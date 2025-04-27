use tiny_kv::database::proto::STORAGE_FILE_DESCRIPTOR_SET;
use tiny_kv::database::proto::storage_server::StorageServer;
use tiny_kv::database::service::Service;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let db_service = StorageServer::new(Service::default());

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(STORAGE_FILE_DESCRIPTOR_SET)
        .build_v1()?;

    Server::builder()
        .add_service(reflection_service)
        .add_service(db_service)
        .serve(addr)
        .await?;

    Ok(())
}
