use tonic::transport::Server;
use tower_http::trace::{TraceLayer};
use crate::logging::Logger;
use crate::services::blockchain_service;


mod services;
mod logging;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse().unwrap();
    println!("Server listening on {}", addr);


    Server::builder()
        .accept_http1(true)
        .layer(TraceLayer::new_for_grpc().on_request(Logger).on_response(Logger).on_failure(Logger))
        .add_service(
            // Use tonic_web::enable to deal with CORS and OPTIONS requests
            tonic_web::enable(blockchain_service::blockchain::blockchain_service_server::BlockchainServiceServer::new(blockchain_service::BlockchainServiceImpl))
        )
        .serve(addr)
        .await?;

    Ok(())
}