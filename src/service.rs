use crate::proto::tinykv_server::Tinykv; // proto pakage trait
use crate::proto::{ActionResult, DeleteRequest, GetRequest, PutRequest};
use crate::storange::Storage;

#[derive(Debug, Default)]
pub struct Service {
    db: Storage,
}

#[tonic::async_trait]
impl Tinykv for Service {
    async fn put(
        &self,
        request: tonic::Request<PutRequest>,
    ) -> Result<tonic::Response<ActionResult>, tonic::Status> {
        println!("Got a request: {:?}", request);
        let reply = ActionResult {
            result: format!("Hello!"),
        };
        Ok(tonic::Response::new(reply))
    }

    async fn get(
        &self,
        request: tonic::Request<GetRequest>,
    ) -> Result<tonic::Response<ActionResult>, tonic::Status> {
        todo!()
    }

    async fn delete(
        &self,
        request: tonic::Request<DeleteRequest>,
    ) -> Result<tonic::Response<ActionResult>, tonic::Status> {
        todo!()
    }
}
