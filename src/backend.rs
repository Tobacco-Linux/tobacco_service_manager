// backend.rs
use rayon::prelude::*;
use std::collections::HashMap;
use users::get_current_uid;
use zbus::Error as ZbusError;
use zbus::blocking::{Connection, Proxy};
use zbus::zvariant::{OwnedObjectPath, Value};

const ACTION_ID: &str = "org.freedesktop.systemd1.manage-units";

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

#[derive(Debug, Clone)]
struct UnitInfo {
    pub name: String,
    pub description: String,
    pub active_state: String,
}

#[derive(Clone)]
pub struct SystemdServiceManager;

impl SystemdServiceManager {
    pub fn new() -> Self {
        Self
    }

    pub fn get_services(&self) -> Result<Vec<ServiceInfo>> {
        let (system_result, session_result) = rayon::join(
            || self.fetch_services(Connection::system()),
            || self.fetch_services(Connection::session()),
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

    fn fetch_services(&self, conn_result: zbus::Result<Connection>) -> Result<Vec<ServiceInfo>> {
        let conn = conn_result?;
        let (units, unit_files) = rayon::join(
            || self.call_list_units(&conn),
            || self.call_list_unit_files(&conn),
        );
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

    fn call_list_units(&self, conn: &Connection) -> Result<Vec<UnitInfo>> {
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

    fn call_list_unit_files(&self, conn: &Connection) -> Result<Vec<(String, String)>> {
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

    pub fn start_unit(&self, unit_name: &str) -> Result<()> {
        let conn = Connection::system()?;

        if !self.authorize(ACTION_ID, &conn)? {
            return Err(ServiceError::ParseError(format!(
                "Not authorized to start unit '{}'",
                unit_name
            )));
        }

        let proxy = self.get_manager_proxy(&conn)?;
        proxy.call_method("StartUnit", &(unit_name, "replace"))?;
        Ok(())
    }

    pub fn stop_unit(&self, unit_name: &str) -> Result<()> {
        let conn = Connection::system()?;

        if !self.authorize(ACTION_ID, &conn)? {
            return Err(ServiceError::ParseError(format!(
                "Not authorized to stop unit '{}'",
                unit_name
            )));
        }

        let proxy = self.get_manager_proxy(&conn)?;
        proxy.call_method("StopUnit", &(unit_name, "replace"))?;
        Ok(())
    }

    pub fn enable_unit(&self, unit_name: &str) -> Result<()> {
        let conn = Connection::system()?;

        if !self.authorize(ACTION_ID, &conn)? {
            return Err(ServiceError::ParseError(format!(
                "Not authorized to stop unit '{}'",
                unit_name
            )));
        }

        let proxy = self.get_manager_proxy(&conn)?;
        proxy.call_method("EnableUnitFiles", &(vec![unit_name], false, true))?;
        Ok(())
    }

    pub fn disable_unit(&self, unit_name: &str) -> Result<()> {
        let conn = Connection::system()?;

        if !self.authorize(ACTION_ID, &conn)? {
            return Err(ServiceError::ParseError(format!(
                "Not authorized to stop unit '{}'",
                unit_name
            )));
        }

        let proxy = self.get_manager_proxy(&conn)?;
        proxy.call_method("DisableUnitFiles", &(vec![unit_name], false))?;
        Ok(())
    }

    fn get_manager_proxy<'a>(&self, conn: &'a Connection) -> Result<Proxy<'a>> {
        Proxy::new(
            conn,
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            "org.freedesktop.systemd1.Manager",
        )
        .map_err(Into::into)
    }

    fn authorize<'a>(&self, action_id: &str, conn: &'a Connection) -> Result<bool> {
        let mut subject = HashMap::new();
        subject.insert("pid", Value::from(std::process::id()));
        subject.insert("start-time", Value::from(0u64));
        subject.insert("uid", Value::from(get_current_uid()));
        let subject = ("unix-process", subject);

        let reply = conn.call_method(
            Some("org.freedesktop.PolicyKit1"),
            "/org/freedesktop/PolicyKit1/Authority",
            Some("org.freedesktop.PolicyKit1.Authority"),
            "CheckAuthorization",
            &(subject, action_id, HashMap::<&str, &str>::new(), 1u32, ""),
        );

        match reply {
            Ok(msg) => {
                let (is_authenticated, _, _): (bool, bool, HashMap<String, String>) =
                    msg.body().deserialize()?;
                Ok(is_authenticated)
            }
            Err(e) => {
                eprintln!("Polkit check failed: {}", e);
                Ok(false)
            }
        }
    }
}
