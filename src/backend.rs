use zbus::blocking::Connection;
use zbus::zvariant::OwnedObjectPath;

/*
 * returns both system and session services.
 */
pub fn get_services() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok([Connection::system()?, Connection::session()?]
        .into_iter()
        .flat_map(|connection| {
            connection
                .call_method(
                    Some("org.freedesktop.systemd1"),
                    "/org/freedesktop/systemd1",
                    Some("org.freedesktop.systemd1.Manager"),
                    "ListUnits",
                    &(),
                )
                .and_then(|message| {
                    message.body().deserialize::<Vec<(
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
                    )>>()
                })
                .map(|units| {
                    units
                        .into_iter()
                        .filter_map(|unit| unit.0.ends_with(".service").then(|| unit.0))
                })
                .unwrap()
        })
        .collect())
}
