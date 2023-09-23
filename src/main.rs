
mod auth_url_service;
mod callback_service;
mod token_service;

use auth_url_service::AuthUrlService;
use callback_service::CallBackService;
use token_service::TokenService;
use tokio;
use warp::{Filter, http::StatusCode, Reply};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::http::Uri;
use warp::reply::Response;
use crate::auth_url_service::AuthUrlService::get_iridium_auth_url;
use crate::callback_service::CallBackService::handle_callback;

const CLIENT_ID: &str = todo!();
const CLIENT_SECRET: &str = todo!();
const REDIRECT_URI: &str = "http://localhost:3030/callback";




#[tokio::main]
async fn main() {

    let auth = warp::path!("auth")
        .map( || {
            //redirect to iridum server
            warp::redirect(Uri::from_static(todo!()))
        });



    //call back
    let callback = warp::path!("callback")
        .and(warp::query::<HashMap<String, String>>())
        .and_then(handle_callback);

    let routes = auth.or(callback);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}



