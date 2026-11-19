pub const LOCAL_REPO_PATH: &str = "/opt/aoska/";

pub fn catalog_endpoint() -> String {
    std::env::var("AOSKA_ASMR_URL").unwrap_or_else(|_| "http://127.0.0.1:8765/".into())
}
