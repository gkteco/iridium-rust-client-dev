pub mod CallBackService {
    use std::collections::HashMap;
    use crate::token_service::TokenService::exchange_code_for_token;

    pub async fn handle_callback(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {

        let code = params.get("code").unwrap();
        let state = params.get("state").unwrap();
        
        let client = reqwest::Client::new();
        let exchange_url = exchange_url::generate(&code, &state);
        let headers = exchange_headers::generate();

        let response = client.get(&exhange_url).headers(headers).send().await?;

        process_response()
        
    }

    pub mod exchange_url {
        pub fn generate(code: &str, state: &str) -> String {
            let url = env::var("RUST_IRIDIUM_BASE_URL").expect("RUST_IRIDIUM_BASE_URL must be set");
            let redirectUri = env::var("RUST_PUBLIC_IRIDIUM_REDIRECT_URI").expect("RUST_PUBLIC_IRIDIUM_REDIRECT_URI must be set");
            let clientId = env::var("RUST_PUBLIC_IRIDIUM_CLIENT_ID").expect("RUST_PUBLIC_IRIDIUM_CLIENT_ID must be set");

            format!("{}/oauth/token?grant_type=authorization_code&code={}&redirect_uri={}&client_id={}&state={}", url, code, redirect_uri, client_id, state)  
        }
    }

}
