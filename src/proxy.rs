mod config;

use async_trait::async_trait;
use pingora_core::prelude::HttpPeer;
use pingora_core::server::Server;
use pingora_proxy::{ProxyHttp, Session};
use crate::config::ProxyConfig;

struct ReverseProxy {
    pub backend: String,
}

#[async_trait]
impl ProxyHttp for ReverseProxy {
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(&self, session: &mut Session, ctx: &mut Self::CTX) -> pingora_core::Result<Box<HttpPeer>> {
        log_request(&session);
        Ok(Box::from(HttpPeer::new(&self.backend, false, String::from(""))))
    }
}

fn log_request(session: &Session) {
    let method = &session.req_header().method;
    let uri = &session.req_header().uri;
    let version = &session.req_header().version;
    let client_ip = &session.client_addr().unwrap();

    log::info!(
        "New request from {}: {} {} {:?}",
        client_ip,
        method,
        uri,
        version
    );

    for (name, value) in &session.req_header().headers {
        log::info!("Header: {:?} = {:?}", name, value)
    }
}

fn main() {
    env_logger::init();
    let config = ProxyConfig::load_from_file("proxy-config.toml");

    let mut server = Server::new(None).unwrap();
    server.bootstrap();

    let mut proxy = pingora_proxy::http_proxy_service(
        &server.configuration,
        ReverseProxy { backend: config.proxy.backend }
    );
    proxy.add_tcp(&config.proxy.listen);

    server.add_service(proxy);
    server.run_forever()
}