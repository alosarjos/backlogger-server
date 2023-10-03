use self::{config::Config, router::Router};

use axum::{routing::IntoMakeService, Server as AxumServer};
use hyper::server::conn::AddrIncoming;

pub mod config;
pub mod controller;
pub mod router;

pub struct Server {
    pub config: Config,
    pub axum_server: AxumServer<AddrIncoming, IntoMakeService<axum::Router>>,
}

impl Server {
    pub fn new(config: Config) -> Self {
        let router = Router::new();
        let address = format!("{}:{}", config.host(), config.port());

        let axum_server =
            AxumServer::bind(&address.parse().unwrap()).serve(router.get_as_service());

        Self {
            config,
            axum_server,
        }
    }

    pub async fn run(self) -> Result<(), hyper::Error> {
        self.axum_server.await
    }
}