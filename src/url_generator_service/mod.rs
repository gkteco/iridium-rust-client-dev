
pub mod UrlGeneratorService {
    use dotenvy::dotenv;
    use std::env;

    pub fn getIridiumAuthUrl(state: &str) -> String {
        dotenv().ok();

        let base_url = env::var("RUST_IRIDIUM_BASE_URL").expect("IRIDIUM_BASE_URL must be set");
        let redirectUri = env::var("RUST_PUBLIC_IRIDIUM_REDIRECT_URI").expect("RUST_PUBLIC_IRIDIUM_REDIRECT_URI must be set");
        let clientId = env::var("RUST_PUBLIC_IRIDIUM_CLIENT_ID").expect("RUST_PUBLIC_IRIDIUM_CLIENT_ID must be set");

        format!("{}login?response_type=code&state={}&redirect_uri={}&client_id={}", base_url, state, redirectUri, clientId)
    }
}
