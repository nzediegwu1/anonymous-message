use axum::{
    routing::{get, post, MethodRouter},
    Router,
};

use super::controllers::{find_user, find_users, handle_login, handle_signup, hello_world};

fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}

fn post_auth() -> Router {
    route("/signup", post(handle_signup)).route("/login", post(handle_login))
}

fn get_auth() -> Router {
    route("/", get(hello_world))
        .route("/users", get(find_users))
        .route("/users/:user_id", get(find_user))
}

pub fn auth_routes() -> Router {
    let auth_api = Router::new().merge(get_auth()).merge(post_auth());
    auth_api
}
