const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn user_agent() -> String {
	format!("paperback/{}", VERSION)
}
