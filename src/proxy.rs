use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Method, Request, Response, Server, StatusCode, upgrade::Upgraded, http};
use log;
use tokio::net::TcpStream;
use futures_util::future::try_join;

type HttpClient = Client<hyper::client::HttpConnector>;

pub struct Proxy<'a> {
    host: &'a str,
}

impl<'a> Proxy<'a> {
    pub fn new(host: &'a str) -> Self {
        Proxy {
            host: host,
        }
    }

    pub async fn serve(&self) {
        let addr: SocketAddr = self.host.parse().unwrap();
        let client = HttpClient::new();

        let make_service = make_service_fn(move |_| {
            let client = client.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |req| proxy(client.clone(), req)))
            }
        });

        log::info!("serving on: {}", self.host);

        let server = Server::bind(&addr).serve(make_service);

        if let Err(e) = server.await {
            log::error!("server error: {}", e);
        }
    }
}

async fn proxy(client: HttpClient, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    log::info!("req: {:?}", req);
    println!("req: {:?}", req.uri().authority().map(|a| a.as_str()).unwrap());
    if req.method() == Method::CONNECT {
        if let Some(addr) = host_addr(req.uri()) {
            tokio::task::spawn(async move {
                match req.into_body().on_upgrade().await {
                    Ok(upgraded) => {
                        if let Err(e) = tunnel(upgraded, addr).await {
                            eprintln!("server io error: {}", e);
                        };
                    }
                    Err(e) => eprintln!("upgrade error: {}", e),
                }
            });

            Ok(Response::new(Body::empty()))
        } else {
            log::error!("CONNECT host illegal: {:?}", req.uri());
            let mut resp = Response::new(Body::from("CONNECT host illegal"));
            *resp.status_mut() = StatusCode::BAD_REQUEST;

            Ok(resp)
        }
        //client.request(req).await

    } else {
        client.request(req).await
    }
}

fn host_addr(uri: &http::Uri) -> Option<SocketAddr> {
    uri.authority().and_then(|auth| auth.as_str().parse().ok())
}

async fn tunnel(upgraded: Upgraded, addr: SocketAddr) -> std::io::Result<()> {
    // Connect to remote server
    let mut server = TcpStream::connect(addr).await?;

    // Proxying data
    let amounts = {
        let (mut server_rd, mut server_wr) = server.split();
        let (mut client_rd, mut client_wr) = tokio::io::split(upgraded);

        let client_to_server = tokio::io::copy(&mut client_rd, &mut server_wr);
        let server_to_client = tokio::io::copy(&mut server_rd, &mut client_wr);

        try_join(client_to_server, server_to_client).await
    };
   
    Ok(())
}