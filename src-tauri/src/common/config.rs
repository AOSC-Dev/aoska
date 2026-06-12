// aosc-os-asmr -> AOSC OS Software Manager Repo  
pub const ASM_ENDPOINT: &str = "https://raw.githubusercontent.com/AOSC-Dev/aosc-os-asmr/stable";
pub const ASM_INDEX_PATH: &str = "aoska_index.json";
pub const ASM_RECOMMEND_INDEX_PATH: &str = "recommend_index.json";
pub const LOCAL_REPO_PATH: &str = "/opt/aoska/";

#[cfg(test)]
mod tests {
    use super::ASM_ENDPOINT;

    #[test]
    fn asm_endpoint_uses_valid_https_raw_github_url() {
        assert!(ASM_ENDPOINT.starts_with("https://"));
        assert!(ASM_ENDPOINT.contains("raw.githubusercontent.com/AOSC-Dev/aosc-os-asmr/"));
        assert!(!ASM_ENDPOINT.starts_with("https:/raw."));
    }
}
