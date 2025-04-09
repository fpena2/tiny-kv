use crate::proto::tinykv_server::Tinykv; // proto pakage trait
use crate::proto::{
    DeleteRequest, DeleteResponse, GetRequest, GetResponse, KvPair, PutRequest, PutResponse,
    ScanRequest, ScanResponse,
};
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
    ) -> Result<tonic::Response<PutResponse>, tonic::Status> {
        let msg = request.into_inner();
        let cf = msg.cf;
        let k = msg.k;
        let v = msg.v;

        match self.db.put(&cf, &k, &v) {
            Ok(()) => Ok(tonic::Response::new(PutResponse {
                error: String::from(""),
            })),
            Err(e) => Ok(tonic::Response::new(PutResponse {
                error: e.to_string(),
            })),
        }
    }

    async fn get(
        &self,
        request: tonic::Request<GetRequest>,
    ) -> Result<tonic::Response<GetResponse>, tonic::Status> {
        let msg = request.into_inner();
        let cf = msg.cf;
        let k = msg.k;

        match self.db.get(&cf, &k) {
            Ok(value) => Ok(tonic::Response::new(GetResponse {
                error: String::from(""),
                value: value,
            })),
            Err(e) => Ok(tonic::Response::new(GetResponse {
                error: e.to_string(),
                value: String::from(""),
            })),
        }
    }

    async fn delete(
        &self,
        request: tonic::Request<DeleteRequest>,
    ) -> Result<tonic::Response<DeleteResponse>, tonic::Status> {
        let msg = request.into_inner();
        let cf = msg.cf;
        let k = msg.k;

        match self.db.delete(&cf, &k) {
            Ok(_value) => Ok(tonic::Response::new(DeleteResponse {
                error: String::from(""),
            })),
            Err(e) => Ok(tonic::Response::new(DeleteResponse {
                error: e.to_string(),
            })),
        }
    }

    async fn scan(
        &self,
        request: tonic::Request<ScanRequest>,
    ) -> Result<tonic::Response<ScanResponse>, tonic::Status> {
        let msg = request.into_inner();
        let cf = msg.cf;
        let k = msg.k;
        let limit: usize = msg.limit.try_into().unwrap();
        match self.db.scan(&cf, &k, limit) {
            Ok(values) => {
                let pairs: Vec<KvPair> = values.iter().map(|p| KvPair::new(&p.0, &p.1)).collect();
                Ok(tonic::Response::new(ScanResponse {
                    error: String::from(""),
                    data: pairs,
                }))
            }
            Err(e) => Ok(tonic::Response::new(ScanResponse {
                error: e.to_string(),
                data: vec![],
            })),
        }
    }
}
