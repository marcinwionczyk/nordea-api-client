use exitcode::ExitCode;

pub struct BasicError {
    pub message: String,
    pub details: String,
}

impl Default for BasicError {
    fn default() -> Self {
        BasicError {
            message: "".to_string(),
            details: "".to_string(),
        }
    }
}

/// Used when the provided test data is invalid.
///
/// DataErrors are not be caught by keywords that run other keywords
/// (e.g. `Run Keyword And Expect Error`). Libraries should thus use
/// this exception with care.
pub struct DataError {
    pub message: String,
    pub details: String,
    pub syntax: bool
}

impl Default for DataError {
    fn default() -> Self {
        DataError {
            message: "".to_string(),
            details: "".to_string(),
            syntax: false,
        }
    }
}

/// Used when a test or keyword timeout occurs. This exception is handled
/// specially so that execution of the current test is always stopped
/// immediately, and it is not caught by keywords executing other
/// keywords (e.g. `Run Keyword And Expect Error`).
pub struct TimeoutError {
    pub message: String,
    pub details: String,
    pub test_timeout: bool
}

pub trait ExecutionStatusTrait {
    fn timeout(&self) -> bool;
    fn dont_continue(&self) -> bool;
    fn continue_on_failure(&self) -> bool;
    fn continue_on_failure_mut(&mut self, continue_on_failure: bool);
}

pub enum AutomatonError{
    FrameworkError(BasicError),
    DataError(DataError),
    VariableError(BasicError),
    KeywordError(BasicError),
    TimeoutError(TimeoutError),
    /// Used by argument parser with --help or --version.
    Information,

}