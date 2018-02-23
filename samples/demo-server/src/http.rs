use futures;
use futures::future::Future;
use hyper;
use hyper::{Method, StatusCode};
use hyper::header::ContentType;
use hyper::server::{Http, NewService, Request, Response, Service};
use opcua_server::server_metrics::ServerMetrics;
use serde_json;
use std::io;
use std::sync::{Arc, RwLock};

/// This is our metrics service, the thing called to handle requests coming from hyper
struct MetricsService {
    server_metrics: Arc<RwLock<ServerMetrics>>
}

impl MetricsService {
    fn new(server_metrics: Arc<RwLock<ServerMetrics>>) -> MetricsService {
        MetricsService { server_metrics }
    }
}

impl Service for MetricsService {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    // The future representing the eventual Response the call will resolve to
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();
        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                let content = include_str!("index.html");
                response.set_body(content);
            }
            (&Method::Get, "/metrics") => {
                use std::ops::Deref;
                // Send metrics data as json
                let json = {
                    let server_metrics = self.server_metrics.read().unwrap();
                    serde_json::to_string_pretty(server_metrics.deref()).unwrap()
                };
                response.headers_mut().set(ContentType::json());
                response.set_body(json);
            }
            _ => {
                response.set_status(StatusCode::NotFound);
            }
        }
        Box::new(futures::future::ok(response))
    }
}

struct MetricsServiceFactory {
    server_metrics: Arc<RwLock<ServerMetrics>>
}

impl NewService for MetricsServiceFactory {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = MetricsService;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(MetricsService::new(self.server_metrics.clone()))
    }
}

/// Runs an http server on the specified binding address, serving out the supplied server metrics
pub fn run_http_server(address: &str, server_metrics: Arc<RwLock<ServerMetrics>>) {
    // info!("HTTP server is running on {} to provide OPC UA server metrics", address);
    let address = address.parse().unwrap();
    let metrics_factory = MetricsServiceFactory { server_metrics };
    let server = Http::new().bind(&address, metrics_factory).unwrap();
    server.run().unwrap();
}
