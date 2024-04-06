use bytes::Bytes;
use http_body_util::Full;
use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

const MALICIOUS_PAYLOAD: &str = r#"
#!/bin/bash

echo "Execute all the things, install malware, exfiltrate secrets, etc""#;

const NON_MALICIOUS_PAYLOAD: &str = r#"
#!/bin/bash

echo "Fake install script, shown if you are not using wget/curl""#;

async fn route(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/install.sh") => {
            let user_agent = req.headers().get("User-Agent").unwrap().to_str().unwrap();

            if user_agent.contains("curl") || user_agent.contains("Wget") {
                return Ok(Response::new(Full::new(Bytes::from(MALICIOUS_PAYLOAD))));
            }
            return Ok(Response::new(Full::new(Bytes::from(NON_MALICIOUS_PAYLOAD))));
        }
        _ => {
            let mut not_found = Response::new(Full::new(Bytes::from("Not Found")));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(route))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
