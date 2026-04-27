为 `ora` 这样一个旨在作为 AI Agent 研发底座的项目设计 Git 包装器，核心挑战在于**如何将非结构化的 Git CLI 输出转化为对 Agent 友好、结构化且强类型的 Rust 数据**，同时保证极高的稳定性和错误可溯源性。

针对 `gitlancer` 的需求，以下是为您梳理的架构设计与模块划分方案：

### 一、 整体模块架构图

为了保证高可扩展性，建议将 `gitlancer` 拆分为职责单一的模块：

```text
gitlancer/
├── src/
│   ├── lib.rs
│   ├── error.rs        # 统一分层错误体系 (thiserror)
│   ├── core/           # 核心执行器与基础结构
│   │   ├── executor.rs # 封装 std::process::Command，处理 stdout/stderr
│   │   ├── repo.rs     # Repository 核心抽象
│   │   └── worktree.rs # Worktree 核心抽象
│   ├── models/         # 向上层提供的结构化数据实体
│   │   ├── diff.rs     # FileDiff, Hunk 等
│   │   └── status.rs   # ChangeStatus, FileList 等
│   ├── ops/            # 具体 Git 操作的领域拆分 (易于扩展新功能)
│   │   ├── worktree.rs # worktree 的 add/list/prune/exclude
│   │   ├── commit.rs   # 阶段、提交
│   │   └── diff.rs     # diff 解析
│   └── parser/         # 专门负责解析 Git CLI 输出 (如 --porcelain)
```

---

### 二、 核心数据结构设计 (Domain Entities)

由于操作既可能在主仓库（Bare/Main repo）发生，也可能在特定的 Worktree 发生，建议将两者分离，使用组合或借用模式。

```rust
use std::path::{Path, PathBuf};

/// 代表基础的 Git 仓库
#[derive(Debug, Clone)]
pub struct Repository {
    pub root_path: PathBuf,
}

/// 代表一个具体的 Worktree
#[derive(Debug, Clone)]
pub struct Worktree {
    /// 关联的主仓库
    pub repo: Repository,
    /// worktree 所在的实际路径
    pub path: PathBuf,
}

impl Repository {
    pub fn new(path: impl AsRef<Path>) -> Self { ... }
    
    /// 从当前仓库派生/获取一个 worktree
    pub fn worktree(&self, path: impl AsRef<Path>) -> Worktree {
        Worktree {
            repo: self.clone(),
            path: path.as_ref().to_path_buf(),
        }
    }
}
```

---

### 三、 分层错误体系 (基于 `thiserror`)

这是系统的地基。底层的 Git 报错往往杂乱无章，需要通过正则或退出码将其分层，以便上层的 `ora` 代理能根据不同的错误类型做出自愈决策（例如：遇到 `WorktreeLocked`，Agent 可能会决定先执行 unlock）。

```rust
use thiserror::Error;
use std::path::PathBuf;

/// 全局顶级错误
#[derive(Error, Debug)]
pub enum GitlancerError {
    #[error("I/O error occurred: {0}")]
    Io(#[from] std::io::Error),

    #[error("Git execution failed: {0}")]
    Execution(#[from] ExecutionError),

    #[error("Worktree operation failed: {0}")]
    Worktree(#[from] WorktreeError),

    #[error("Failed to parse git output: {0}")]
    Parse(#[from] ParseError),
}

/// 细分的 Worktree 错误
#[derive(Error, Debug)]
pub enum WorktreeError {
    #[error("Worktree already exists at {0}")]
    AlreadyExists(PathBuf),
    
    #[error("Worktree is locked: {reason}")]
    Locked { reason: String },
    
    #[error("Path is not a valid git repository or worktree")]
    NotARepository,
}

/// Git 进程执行错误
#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Git command exited with code {code}: {stderr}")]
    Failed {
        code: i32,
        stderr: String,
    },
    #[error("Git executable not found")]
    GitNotFound,
}
```

---

### 四、 核心执行层与解析层 (Executor & Parser)

**1. 统一执行器 (Executor):**
不要在每个操作里直接写 `std::process::Command::new("git")`。构建一个内部的 `Executor`，统一处理超时、环境变量、工作目录（主仓库还是 worktree 目录）以及错误拦截。

**2. 解析器 (Parser):**
作为 AI Agent 的底座，向上层提供纯文本是毫无意义的。必须尽可能使用 Git 的底层机器可读输出（如 `git status --porcelain=v2`、`git diff --name-status` 等）。

```rust
// models/status.rs
pub enum FileState {
    Modified,
    Added,
    Deleted,
    Untracked,
}

pub struct ChangedFile {
    pub path: String,
    pub state: FileState,
}

// ops/diff.rs
impl Worktree {
    /// 查看 worktree 中的状态变更
    pub fn get_changing_files(&self) -> Result<Vec<ChangedFile>, GitlancerError> {
        // 调用底层的 Executor 执行: git -C <worktree_path> status --porcelain
        let output = self.execute(&["status", "--porcelain"])?;
        // 将输出交给 parser
        crate::parser::parse_status(&output)
    }
}
```

---

### 五、 针对特定需求的架构应对策略

1.  **管理 Worktree (应对各种工作区状态与错误)：**
    在 `ops/worktree.rs` 中封装复杂的逻辑流。比如在添加 worktree 时，先检查路径是否存在，捕获执行失败时的 `stderr` 并匹配关键字（如 "already exists", "locked"），将其转化为精确的 `WorktreeError` 变体。
2.  **动态 Exclude 路径：**
    可以通过操作 `.git/info/exclude` 文件或 worktree 特有的局部 exclude 文件来实现。在 `Worktree` 结构体上挂载类似 `pub fn add_exclude(&self, pattern: &str) -> Result<(), Error>` 的方法，由执行层自动定位到正确的 `info/exclude` 路径进行无损的文件读写。
3.  **方便扩展新功能：**
    采用 **Trait 扩展模式**（Extension Traits）。例如，如果你想增加一套实验性的 `bisect` 功能，你不需要修改核心的 `Repository` 结构，只需要新建一个 `trait GitBisect`，然后为 `Repository` 实现它。这样核心代码非常干净，新功能即插即用。

在这个设计下，`gitlancer` 相当于在裸露的 Git CLI 引擎外加装了一层强类型的传感器和错误处理主板。


代码中无法处理的错误返回错误码和文本，在 ora 界面上提示用户，由用户解决

路径绝对化： CLI 很容易搞乱相对路径。gitlancer 接收和向前端传递的所有路径（path）都必须是经过 fs::canonicalize 处理的绝对路径，这能规避 90% 的低级错误。

环境隔离： 执行 Git 命令时，显式关闭 pager 和设置语言。Command::new("git") 时注入 env("GIT_TERMINAL_PROMPT", "0")（禁止 Git 弹出交互式密码输入框卡死 GUI）以及 env("LANG", "C")（确保底层的少量文本输出不受用户系统本地化语言影响，保证 fallback 解析的稳定性）。
