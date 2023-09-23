use serde::Deserialize;
use std::todo;
pub mod TokenService {
    use reqwest::StatusCode;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct TokenResponse {
        access_token: String,
        token_type: String,
        expires_in: u64,
        refresh_token: Option<String>,
        scope: String,
    }

    pub async fn exchange_code_for_token(code: &str) -> Result<TokenResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        todo!("implement params");
        let params = [("Todo", "todo")];
        /*
        let params = [
            ("client_id", CLIENT_ID),
            ("client_secret", CLIENT_SECRET),
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", REDIRECT_URI),
        ];
         */

        let response = client.post("https://auth-server.com/token")
            .form(&params)
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            let token_response: TokenResponse = response.json().await?;
            Ok(token_response)
        } else {
            todo!("implements when status code is not OK");
        }
    }

}
