use rayon::prelude::*;
use std::collections::HashMap;
use zbus::blocking::Connection;
use zbus::zvariant::OwnedObjectPath;

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

#[derive(Debug)]
pub enum ServiceError {
    ZbusError(zbus::Error),
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
impl std::error::Error for ServiceError {}
impl From<zbus::Error> for ServiceError {
    fn from(e: zbus::Error) -> Self {
        Self::ZbusError(e)
    }
}
type Result<T> = std::result::Result<T, ServiceError>;

pub fn get_services() -> Result<Vec<ServiceInfo>> {
    let (system_result, session_result) = rayon::join(
        || {
            Connection::system()
                .map_err(ServiceError::from)
                .and_then(fetch_services_from_connection)
        },
        || {
            Connection::session()
                .map_err(ServiceError::from)
                .and_then(fetch_services_from_connection)
        },
    );

    let mut services = Vec::new();
    if let Ok(s) = system_result {
        services.extend(s);
    }
    if let Ok(s) = session_result {
        services.extend(s);
    }
    Ok(services)
}

fn fetch_services_from_connection(conn: Connection) -> Result<Vec<ServiceInfo>> {
    let (units, unit_files) =
        rayon::join(|| call_list_units(&conn), || call_list_unit_files(&conn));

    let units = units?;
    let enablement_map: HashMap<_, _> = unit_files?
        .into_par_iter()
        .filter_map(|(path, state)| {
            std::path::Path::new(&path)
                .file_name()
                .and_then(|n| n.to_str())
                .map(|name| (name.to_string(), state))
        })
        .collect();

    Ok(units
        .into_par_iter()
        .filter(|u| u.name.ends_with(".service"))
        .map(|unit| {
            let enablement_status = enablement_map
                .get(&unit.name)
                .map(|s| s.as_str())
                .unwrap_or("unknown")
                .into();

            ServiceInfo {
                name: unit.name,
                description: unit.description,
                status: unit.active_state.as_str().into(),
                enablement_status,
            }
        })
        .collect())
}

#[derive(Debug)]
struct UnitInfo {
    name: String,
    description: String,
    active_state: String,
}

fn call_list_units(conn: &Connection) -> Result<Vec<UnitInfo>> {
    let msg = conn.call_method(
        Some("org.freedesktop.systemd1"),
        "/org/freedesktop/systemd1",
        Some("org.freedesktop.systemd1.Manager"),
        "ListUnits",
        &(),
    )?;

    Ok(msg
        .body()
        .deserialize::<Vec<(
            String,
            String,
            String,
            String,
            String,
            String,
            OwnedObjectPath,
            u32,
            String,
            OwnedObjectPath,
        )>>()?
        .into_par_iter()
        .map(|(name, description, _, active_state, ..)| UnitInfo {
            name,
            description,
            active_state,
        })
        .collect())
}

fn call_list_unit_files(conn: &Connection) -> Result<Vec<(String, String)>> {
    conn.call_method(
        Some("org.freedesktop.systemd1"),
        "/org/freedesktop/systemd1",
        Some("org.freedesktop.systemd1.Manager"),
        "ListUnitFiles",
        &(),
    )?
    .body()
    .deserialize()
    .map_err(Into::into)
}
