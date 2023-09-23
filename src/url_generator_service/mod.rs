


pub mod UrlGeneratorService {
    use std::env;
    use dotenv::dotenv;
    pub fn getIridiumAuthUrl(state: &str) -> String {
        dotenv().ok();

        let base_url = env::var("IRIDIUM_BASE_URL");
        let redirectUri = env::var("RUST_PUBLIC_IRIDIUM_REDIRECT_URI");
        let clientId = env::var("RUST_PUBLIC_IRIDIUM_CLIENT_ID");

        format!("{}/login?response_type=code&state={}&redirect_uri={}&client_id={}", base_url.unwrap(), state, redirectUri.unwrap(), clientId.unwrap())
    }
}