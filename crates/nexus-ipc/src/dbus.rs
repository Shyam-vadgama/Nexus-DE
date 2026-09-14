/// D-Bus Service Name
pub const NEXUS_DBUS_SERVICE: &str = "org.nexus.DE";

/// D-Bus Object Path
pub const NEXUS_DBUS_PATH: &str = "/org/nexus/DE";

/// D-Bus Main Interface Name
pub const NEXUS_DBUS_INTERFACE: &str = "org.nexus.DE";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dbus_constants() {
        assert_eq!(NEXUS_DBUS_SERVICE, "org.nexus.DE");
        assert_eq!(NEXUS_DBUS_PATH, "/org/nexus/DE");
        assert_eq!(NEXUS_DBUS_INTERFACE, "org.nexus.DE");
    }
}
