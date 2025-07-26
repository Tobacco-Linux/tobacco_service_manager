use super::{error::Result, models::UnitInfo};
use rayon::prelude::*;
use serde::ser;
use std::collections::HashMap;
use zbus::blocking::{Connection, Proxy};
use zbus::zvariant::{DynamicType, OwnedObjectPath};

#[derive(Clone)]
pub struct SystemdServiceManager;

impl SystemdServiceManager {
    pub fn new() -> Self {
        Self
    }

    pub fn get_services(&self) -> Result<Vec<super::models::ServiceInfo>> {
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

    fn fetch_services(
        &self,
        conn_result: zbus::Result<Connection>,
    ) -> Result<Vec<super::models::ServiceInfo>> {
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

                super::models::ServiceInfo {
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
        self.call_manager_method(&conn, "StartUnit", &(unit_name, "replace"))?;
        Ok(())
    }

    pub fn stop_unit(&self, unit_name: &str) -> Result<()> {
        let conn = Connection::system()?;
        self.call_manager_method(&conn, "StopUnit", &(unit_name, "replace"))?;
        Ok(())
    }

    pub fn enable_unit(&self, unit_name: &str) -> Result<()> {
        let conn = Connection::system()?;
        let proxy = self.get_manager_proxy(&conn)?;
        proxy.call_method("EnableUnitFiles", &(vec![unit_name], false, true))?;
        Ok(())
    }

    pub fn disable_unit(&self, unit_name: &str) -> Result<()> {
        let conn = Connection::system()?;
        let proxy = self.get_manager_proxy(&conn)?;
        proxy.call_method("DisableUnitFiles", &(vec![unit_name], false))?;
        Ok(())
    }

    fn call_manager_method<T>(
        &self,
        conn: &Connection,
        method: &str,
        args: &T,
    ) -> Result<zbus::Message>
    where
        T: ser::Serialize + DynamicType,
    {
        let proxy = self.get_manager_proxy(conn)?;
        proxy.call_method(method, args).map_err(Into::into)
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
}
