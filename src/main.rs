mod callback_service;
mod pkce_service;
mod state_generator_service;
mod token_service;
mod url_generator_service;

use crate::callback_service::CallBackService::handle_callback;
use crate::state_generator_service::StateGenerator;
use crate::url_generator_service::UrlGeneratorService;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use tokio;
use warp::http::Uri;
use warp::{http::StatusCode, Filter, Reply};

#[tokio::main]
async fn main() {
    let state = StateGenerator::generate();
    let verifier = generate_random_string();
    let pkce_code = pkce_service::Pkce_Service::generate_code_challenge(&verifier);

    if let Ok(code_challenge) = pkce_code {
        let auth = warp::path!("auth").map(move || {
            let uri = Uri::from_str(&UrlGeneratorService::getIridiumAuthUrl(
                &state,
                &code_challenge,
            ))
            .unwrap();
            warp::redirect(uri)
        });

        //call back
        let callback = warp::path!("callback")
            .and(warp::query::<HashMap<String, String>>())
            .and_then(move |params: HashMap<String, String>| {
                handle_callback(params, verifier.clone())
            });

        let routes = auth.or(callback);

        warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
    } else {
        eprintln!("Error generating code challenge")
    }
}

fn generate_random_string() -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    random_string
}
