use crate::proto::tinykv_server::Tinykv; // proto pakage trait
use crate::proto::{DeleteRequest, GetRequest, PutRequest, TinykvResult};
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
    ) -> Result<tonic::Response<TinykvResult>, tonic::Status> {
        let msg = request.into_inner();
        let cf = msg.cf;
        let k = msg.k;
        let v = msg.v;

        match self.db.put(&cf, &k, &v) {
            Ok(()) => Ok(tonic::Response::new(TinykvResult {
                success: true,
                data: String::from(""),
                error: String::from(""),
            })),
            Err(e) => Ok(tonic::Response::new(TinykvResult {
                success: false,
                data: String::from(""),
                error: e.to_string(),
            })),
        }
    }

    async fn get(
        &self,
        request: tonic::Request<GetRequest>,
    ) -> Result<tonic::Response<TinykvResult>, tonic::Status> {
        let msg = request.into_inner();
        let cf = msg.cf;
        let k = msg.k;

        match self.db.get(&cf, &k) {
            Ok(value) => Ok(tonic::Response::new(TinykvResult {
                success: true,
                data: value,
                error: String::from(""),
            })),
            Err(e) => Ok(tonic::Response::new(TinykvResult {
                success: false,
                data: String::from(""),
                error: e.to_string(),
            })),
        }
    }

    async fn delete(
        &self,
        request: tonic::Request<DeleteRequest>,
    ) -> Result<tonic::Response<TinykvResult>, tonic::Status> {
        let msg = request.into_inner();
        let cf = msg.cf;
        let k = msg.k;

        match self.db.delete(&cf, &k) {
            Ok(_value) => Ok(tonic::Response::new(TinykvResult {
                success: true,
                data: String::from(""),
                error: String::from(""),
            })),
            Err(e) => Ok(tonic::Response::new(TinykvResult {
                success: false,
                data: String::from(""),
                error: e.to_string(),
            })),
        }
    }
}
