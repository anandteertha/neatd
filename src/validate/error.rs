pub enum ValidationError {
    MissingField {
        field: String,
        section: String,
    },
    InvalidType {
        field: String,
        section: String,
        expected: String,
        got: String,
    },
    InvalidEnumValue {
        field: String,
        section: String,
        expected: Vec<String>,
        got: String,
    },
    InvalidPath {
        path: String,
        reason: String,
    },
    InvalidGlob {
        pattern: String,
        reason: String,
    },
    RuleError {
        rule_name: String,
        reason: String,
    },
}

impl ValidationError {
    pub fn to_string(&self) -> String {
        match self {
            ValidationError::MissingField { field, section } => {
                format!("Missing required field '{field}' in section '[{section}]'")
            }
            ValidationError::InvalidType {
                field,
                section,
                expected,
                got,
            } => {
                format!(
                    "Invalid type at field '{field}' in section '[{section}]'\nExpected: {expected}\tGot: {got}"
                )
            }
            ValidationError::InvalidEnumValue {
                field,
                section,
                expected,
                got,
            } => {
                let expected = expected.join(",");
                format!(
                    "Invalid value at field '{field}' in section '[{section}]'\nExpected: {expected}\tGot: {got}"
                )
            }
            ValidationError::InvalidPath { path, reason } => {
                format!("Invalid path '{path}'\nreason: {reason}")
            }
            ValidationError::InvalidGlob { pattern, reason } => {
                format!("Invalid glob pattern '{pattern}'\nreason: {reason}")
            }
            ValidationError::RuleError { rule_name, reason } => {
                format!("Invalid rule '{rule_name}'\nreason: {reason}")
            }
        }
    }
}

pub type ValidationResult = Result<(), Vec<ValidationError>>;
