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

pub struct ExecutionStatusError {
    pub message: String,
    pub test_timeout: bool,
    pub keyword_timeout: bool,
    pub syntax: bool,
    pub exit: bool,
    continue_on_failure: bool,
    pub skip: bool,
    pub return_value: Option<String>,
    pub parent: Option<ExecutionStatusError>,
    pub children: Option<Vec<ExecutionStatusError>>
}

impl ExecutionStatusError {
    fn new(message: String) -> ExecutionStatusError {
        ExecutionStatusError {
            message,
            test_timeout: false,
            keyword_timeout: false,
            syntax: false,
            exit: false,
            continue_on_failure: false,
            skip: false,
            return_value: None,
            parent: None,
            children: None
        }
    }
    pub fn timeout(&self) -> bool {
        self.keyword_timeout || self.test_timeout
    }
    fn dont_continue(&self) -> bool {
        self.timeout() || self.syntax || self.exit
    }
    fn continue_on_failure(&self) -> bool {
        self.continue_on_failure
    }
    fn set_continue_on_failure(&mut self, continue_on_failure: bool) {
        self.continue_on_failure = continue_on_failure;
        if let Some(children) = &mut self.children {
            for child in children {
                child.continue_on_failure = continue_on_failure;
            }
        }
    }
}

pub enum ExecutionStatusErrorEnum {

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