
mod auth_url_service;
mod callback_service;
mod token_service;
mod state_generator_service;
mod url_generator_service;

use auth_url_service::AuthUrlService;
use callback_service::CallBackService;
use token_service::TokenService;
use tokio;
use warp::{Filter, http::StatusCode, Reply};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use warp::http::Uri;
use warp::reply::Response;
use crate::auth_url_service::AuthUrlService::get_iridium_auth_url;
use crate::callback_service::CallBackService::handle_callback;
use crate::state_generator_service::StateGenerator;
use crate::url_generator_service::UrlGeneratorService::getIridiumAuthUrl;





#[tokio::main]
async fn main() {
    let state = StateGenerator::generate();

    let auth = warp::path!("auth")
        .map( move || {
            let uri = Uri::from_str(&getIridiumAuthUrl(&state)).unwrap();
            warp::redirect(uri)
        });



    //call back
    let callback = warp::path!("callback")
        .and(warp::query::<HashMap<String, String>>())
        .and_then(handle_callback);

    let routes = auth.or(callback);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}



