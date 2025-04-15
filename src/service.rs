use crate::proto::tinykv_server::Tinykv; // proto pakage trait
use crate::proto::{
    DeleteRequest, DeleteResponse, GetRequest, GetResponse, KvPair, PutRequest, PutResponse,
    ScanRequest, ScanResponse,
};
use crate::storange::Storage;
use std::num::NonZeroUsize;

#[derive(Debug, Default)]
pub struct Service {
    db: Storage,
}

#[tonic::async_trait]
impl Tinykv for Service {
    async fn put(
        &self,
        request: tonic::Request<PutRequest>,
    ) -> Result<tonic::Response<PutResponse>, tonic::Status> {
        let msg = request.into_inner();
        let cf = msg.cf;
        let k = msg.k;
        let v = msg.v;

        // TODO: catch the errors and pass them in the response as string
        let _ = self
            .db
            .put(&cf, &k, &v)
            .map_err(|e| tonic::Status::internal(format!("Database error: {}", e)))?;

        Ok(tonic::Response::new(PutResponse {
            error: String::from(""),
        }))
    }

    async fn get(
        &self,
        request: tonic::Request<GetRequest>,
    ) -> Result<tonic::Response<GetResponse>, tonic::Status> {
        let msg = request.into_inner();
        let cf = msg.cf;
        let k = msg.k;

        let value = self
            .db
            .get(&cf, &k)
            .map_err(|e| tonic::Status::internal(format!("Database error: {}", e)))?;

        Ok(tonic::Response::new(GetResponse {
            error: String::from(""),
            value: value,
        }))
    }

    async fn delete(
        &self,
        request: tonic::Request<DeleteRequest>,
    ) -> Result<tonic::Response<DeleteResponse>, tonic::Status> {
        let msg = request.into_inner();
        let cf = msg.cf;
        let k = msg.k;

        let _deleted_value = self
            .db
            .delete(&cf, &k)
            .map_err(|e| tonic::Status::internal(format!("Database error: {}", e)))?;

        Ok(tonic::Response::new(DeleteResponse {
            error: String::from(""),
        }))
    }

    async fn scan(
        &self,
        request: tonic::Request<ScanRequest>,
    ) -> Result<tonic::Response<ScanResponse>, tonic::Status> {
        let msg = request.into_inner();
        let cf = msg.cf;
        let k = msg.k;
        let limit = msg.limit;

        let limit: NonZeroUsize = match limit.try_into() {
            Ok(usize_limit) => NonZeroUsize::new(usize_limit)
                .ok_or_else(|| tonic::Status::invalid_argument("Limit cannot be zero"))?,
            Err(_) => {
                return Err(tonic::Status::invalid_argument(
                    "Limit too large for this architecture",
                ));
            }
        };

        let values = self
            .db
            .scan(&cf, &k, limit)
            .map_err(|e| tonic::Status::internal(format!("Database error: {}", e)))?;

        let pairs: Vec<KvPair> = values.iter().map(|p| KvPair::new(&p.0, &p.1)).collect();
        Ok(tonic::Response::new(ScanResponse { data: pairs }))
    }
}
