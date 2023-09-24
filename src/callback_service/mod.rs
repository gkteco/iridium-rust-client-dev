pub mod CallBackService {
    use crate::callback_service::{exchange_headers, exchange_url};
    use crate::token_service::TokenService::exchange_code_for_token;
    use reqwest::StatusCode;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::env;
    use warp::http::HeaderMap;

    #[derive(Deserialize, Serialize, Debug)]
    struct TokenResponse {
        access_token: String,
    }
    pub async fn handle_callback(
        params: HashMap<String, String>,
        verifier: String,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        if let (Some(code), Some(state)) = (params.get("code"), params.get("state")) {
            let client = reqwest::Client::new();
            let exchange_url = exchange_url::generate(code, state, &verifier);
            println!("exchange_url: {}", exchange_url);
            let headers = exchange_headers::generate();

            match client.post(&exchange_url).headers(headers).send().await {
                Ok(response) if response.status() == StatusCode::OK => {
                    let json = response.json::<TokenResponse>().await.unwrap();
                    println!("{}", json.access_token);
                    println!("json: {:?}", &json);
                    match get_identity(&json.access_token).await {
                        Ok(response) if response.status() == StatusCode::OK => {
                            let json = response.json::<serde_json::Value>().await.unwrap();
                            println!("json: {:?}", &json);
                        }
                        Ok(response) => {
                            eprintln!("Unexpected response status: {}", response.status());
                        }
                        Err(e) => {
                            eprintln!("Error making request: {}", e);
                        }
                    }
                    Ok(warp::reply::json(&json))
                }
                Ok(response) => {
                    eprintln!("Unexpected response status: {}", response.status());
                    Err(warp::reject())
                }
                Err(e) => {
                    eprintln!("Error making request: {}", e);
                    Err(warp::reject())
                }
            }
        } else {
            Err(warp::reject::not_found())
        }
    }

    pub async fn get_identity(token: &str) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();
        let base_url =
            env::var("RUST_IRIDIUM_BASE_URL").expect("RUST_IRIDIUM_BASE_URL must be set");
        let identities_url = format!("{}identities", base_url);
        let mut headers = HeaderMap::new();
        let bearer = format!("Bearer {}", token);
        headers.insert(
            "Accept",
            "application/vnd.iridium.id.identity-response.1+json"
                .parse()
                .unwrap(),
        );

        headers.insert("Authorization", bearer.parse().unwrap());

        client.get(&identities_url).headers(headers).send().await
    }
}

pub mod exchange_url {
    use dotenvy::dotenv;
    use std::env;

    pub fn generate(code: &str, state: &str, verifier: &str) -> String {
        dotenv().ok();
        let url = env::var("RUST_IRIDIUM_BASE_URL").expect("RUST_IRIDIUM_BASE_URL must be set");
        let redirect_uri = env::var("RUST_PUBLIC_IRIDIUM_REDIRECT_URI")
            .expect("RUST_PUBLIC_IRIDIUM_REDIRECT_URI must be set");
        let client_id = env::var("RUST_PUBLIC_IRIDIUM_CLIENT_ID")
            .expect("RUST_PUBLIC_IRIDIUM_CLIENT_ID must be set");

        format!("{}oauth/token?grant_type=authorization_code&code={}&redirect_uri={}&client_id={}&code_verifier={}&state={}", url, code, redirect_uri, client_id, verifier, state)
    }
}

pub mod exchange_headers {
    pub fn generate() -> reqwest::header::HeaderMap {
        // ... your logic to generate the headers
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());
        headers.insert("Content-Length", "24".parse().unwrap());
        headers
    }
}
