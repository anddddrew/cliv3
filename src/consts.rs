pub const fn get_user_agent() -> &'static str {
    concat!("cli-rs", env!("CARGO_PKG_VERSION"))
}

pub const TICK_STRING: &str = "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ";

pub const PLUGINS: &[&str] = &["PostgreSQL", "MySQL", "Redis", "MongoDB"];

pub const NO_SERVICE_LINKED: &str =
    "No service linked and no plugins found\nRun `railway service` to link a service";
pub const ABORTED_BY_USER: &str = "Aborted by user";
