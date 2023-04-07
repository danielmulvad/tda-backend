use crate::{td_client::TDAmeritradeClientAuthentication, AppState};
use axum::{extract::State, http::StatusCode, Json};

#[derive(serde::Serialize)]
pub struct GetAuthorizationUrlResponse {
    authorization_url: String,
}

pub async fn get_authorization_url(State(state): State<AppState>) -> (StatusCode, Json<GetAuthorizationUrlResponse>) {
    let authorization_url = state.td_client.get_authorization_url();
    (StatusCode::OK, Json(GetAuthorizationUrlResponse { authorization_url }))
}

#[cfg(test)]
mod tests {
    use crate::router::Router;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_get_authorization_url() {
        let router = Router::default().get_router();
        let server = TestServer::new(router.into_make_service()).unwrap();
        let res = server.get("/api/get_authorization_url").await;
        assert_eq!(res.status_code(), 200);
    }
}
