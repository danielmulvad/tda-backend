use crate::router::Router;
use axum::{
    extract::Host, handler::HandlerWithoutStateExt, http::StatusCode, http::Uri,
    response::Redirect, BoxError,
};
use axum_server::tls_rustls::RustlsConfig;
use dotenv::dotenv;
use std::{net::SocketAddr, path::PathBuf};

#[derive(Clone, Copy)]
struct Ports {
    http: u16,
    https: u16,
}

async fn redirect_http_to_https(ports: Ports) {
    fn make_https(host: String, uri: Uri, ports: &Ports) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&ports.http.to_string(), &ports.https.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, &ports) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], ports.http));
    tracing::debug!("http redirect listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(redirect.into_make_service())
        .await
        .unwrap();
}

pub struct Server {}

impl Server {
    pub async fn start(&self, router: Option<axum::Router>) {
        dotenv().ok();
        tracing_subscriber::fmt::init();

        let app = match router {
            Some(router) => router,
            None => Router::default().get_router(),
        };
        let ports = Ports {
            http: 7878,
            https: 3000,
        };
        let tls_config = RustlsConfig::from_pem_file(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("self_signed_certs")
                .join("cert.pem"),
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("self_signed_certs")
                .join("key.pem"),
        )
        .await
        .unwrap();
        tokio::spawn(redirect_http_to_https(ports));
        let addr = SocketAddr::from(([127, 0, 0, 1], ports.https));
        tracing::debug!("listening on {}", addr);
        axum_server::bind_rustls(addr, tls_config)
            .serve(app.into_make_service())
            .await
            .unwrap()
    }

    pub fn new() -> Self {
        Self {}
    }
}
