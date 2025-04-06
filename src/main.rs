use tiny_kv::proto::tinykv_server::TinykvServer;
use tiny_kv::service::Service;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = Service::default();

    Server::builder()
        .add_service(TinykvServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
