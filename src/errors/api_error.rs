use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Issue getting response")]
    IssueGettingResponse,
    #[error("Issue parsing response")]
    IssueParsingResponse,
    #[error("Request Failed with Error: {0}")]
    RequestFailedWithError(String),
}
