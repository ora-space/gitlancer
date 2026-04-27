use crate::domain::refs::CommitId;
use crate::error::ParseError;
use crate::git::commit::CommitResponse;

/// Parses the latest commit identifier and summary once the commit readback flow is implemented.
pub fn parse_commit_response(_stdout: &str) -> Result<CommitResponse, ParseError> {
    Err(ParseError::Unimplemented {
        feature: "parse_commit_response",
    })
}

/// Parses one commit identifier from the first non-empty line of stdout.
pub fn parse_commit_id(stdout: &str) -> Result<CommitId, ParseError> {
    let line = stdout
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .ok_or(ParseError::MissingLine)?;

    Ok(CommitId::new(line.to_string()))
}
