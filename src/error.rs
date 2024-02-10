use thiserror::Error;

#[derive(Debug, Error)]
#[error("{0} is not implemented")]
pub struct NotImplementedError(String);

impl NotImplementedError {
    pub fn new(msg: &str) -> Self {
        NotImplementedError(msg.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_implemented_error() {
        let error = NotImplementedError::new("test");
        assert_eq!(format!("{}", error), "test is not implemented");
    }
}
