use tiny_kv::proto;
use tiny_kv::proto::tinykv_server::TinykvServer;
use tiny_kv::service::Service;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let db_service = TinykvServer::new(Service::default());

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    Server::builder()
        .add_service(reflection_service)
        .add_service(db_service)
        .serve(addr)
        .await?;

    Ok(())
}
