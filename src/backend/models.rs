#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub description: String,
    pub status: ServiceStatus,
    pub enablement_status: EnablementStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServiceStatus {
    Active,
    Inactive,
    Failed,
    Activating,
    Deactivating,
    Unknown(String),
}
// models.rs

impl From<&str> for ServiceStatus {
    fn from(s: &str) -> Self {
        match s {
            "active" => Self::Active,
            "inactive" => Self::Inactive,
            "failed" => Self::Failed,
            "activating" => Self::Activating,
            "deactivating" => Self::Deactivating,
            _ => Self::Unknown(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnablementStatus {
    Enabled,
    Disabled,
    Static,
    Indirect,
    Generated,
    Transient,
    Unknown(String),
}

impl From<&str> for EnablementStatus {
    fn from(s: &str) -> Self {
        match s {
            "enabled" => Self::Enabled,
            "disabled" => Self::Disabled,
            "static" => Self::Static,
            "indirect" => Self::Indirect,
            "generated" => Self::Generated,
            "transient" => Self::Transient,
            _ => Self::Unknown(s.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnitInfo {
    pub name: String,
    pub description: String,
    pub active_state: String,
}
