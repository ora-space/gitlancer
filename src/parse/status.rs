use crate::error::ParseError;
use crate::git::status::StatusEntry;

/// Parses porcelain-v2 status output once the full machine-readable status schema is implemented.
pub fn parse_status_v2(_stdout: &str) -> Result<Vec<StatusEntry>, ParseError> {
    Err(ParseError::Unimplemented {
        feature: "parse_status_v2",
    })
}
