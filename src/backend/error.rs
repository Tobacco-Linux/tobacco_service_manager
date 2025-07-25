use zbus::Error as ZbusError;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ServiceError {
    ZbusError(ZbusError),
    ParseError(String),
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZbusError(e) => write!(f, "D-Bus error: {}", e),
            Self::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl From<ZbusError> for ServiceError {
    fn from(e: ZbusError) -> Self {
        Self::ZbusError(e)
    }
}

pub type Result<T> = std::result::Result<T, ServiceError>;
