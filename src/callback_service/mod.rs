pub mod CallBackService {
    use std::collections::HashMap;
    use crate::token_service::TokenService::exchange_code_for_token;

    pub async fn handle_callback(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
        if let Some(code) = params.get("code") {
            // Exchange code for an access token
            let token_response = exchange_code_for_token(code).await;
            match token_response {
                Ok(token) => {
                    // Use the token (e.g., to access protected resources)
                    // For this example, we'll just return it
                    Ok(warp::reply::json(&token))
                }
                Err(e) => {
                    eprintln!("Error fetching token: {}", e);
                    Err(warp::reject::not_found())
                }
            }
        } else {
            Err(warp::reject::not_found())
        }
    }

}
