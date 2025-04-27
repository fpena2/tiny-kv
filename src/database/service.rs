use crate::database::proto::storage_server::Storage; // proto package trait
use crate::database::proto::{
    DeleteRequest, DeleteResponse, GetRequest, GetResponse, KvPair, PutRequest, PutResponse,
    ScanRequest, ScanResponse,
};
use std::num::NonZeroUsize;

use crate::database::memory_storage::MemoryStorage;

#[derive(Debug, Default)]
pub struct Service {
    db: MemoryStorage,
}

#[tonic::async_trait]
impl Storage for Service {
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
            value,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::proto::{DeleteRequest, GetRequest, PutRequest, ScanRequest};
    use tonic::Request;

    #[tokio::test]
    async fn test_put() {
        let service = Service::default();
        let response = service
            .put(Request::new(PutRequest {
                cf: "c".to_string(),
                k: "k".to_string(),
                v: "v".to_string(),
            }))
            .await
            .unwrap();

        assert_eq!(response.get_ref().error, "");
    }

    #[tokio::test]
    async fn test_get() {
        let service = Service::default();

        service
            .put(Request::new(PutRequest {
                cf: "test_cf".to_string(),
                k: "test_key".to_string(),
                v: "test_value".to_string(),
            }))
            .await
            .unwrap();

        let response = service
            .get(Request::new(GetRequest {
                cf: "test_cf".to_string(),
                k: "test_key".to_string(),
            }))
            .await
            .unwrap();

        assert_eq!(response.get_ref().error, "");
        assert_eq!(response.get_ref().value, "test_value");
    }

    #[tokio::test]
    async fn test_delete() {
        let service = Service::default();
        service
            .put(Request::new(PutRequest {
                cf: "test_cf".to_string(),
                k: "test_key".to_string(),
                v: "test_value".to_string(),
            }))
            .await
            .unwrap();

        let response = service
            .delete(Request::new(DeleteRequest {
                cf: "test_cf".to_string(),
                k: "test_key".to_string(),
            }))
            .await
            .unwrap();

        assert_eq!(response.get_ref().error, "");

        let response = service
            .get(Request::new(GetRequest {
                cf: "test_cf".to_string(),
                k: "test_key".to_string(),
            }))
            .await;

        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_scan() {
        let service = Service::default();

        for i in 0..5 {
            service
                .put(Request::new(PutRequest {
                    cf: "test_cf".to_string(),
                    k: format!("key_{}", i),
                    v: format!("value_{}", i),
                }))
                .await
                .unwrap();
        }

        // Scan shall start at "key_2" and pull 3 items (inclusive)(e.g. keys 2, 3, 4)
        let response = service
            .scan(Request::new(ScanRequest {
                cf: "test_cf".to_string(),
                k: "key_2".to_string(),
                limit: 3,
            }))
            .await
            .unwrap();

        let data = &response.get_ref().data;
        assert_eq!(data.len(), 3, "Should return exactly 3 items");
        assert_eq!(
            data,
            &[
                KvPair::new("key_2", "value_2"),
                KvPair::new("key_3", "value_3"),
                KvPair::new("key_4", "value_4")
            ],
            "Should return keys 2, 3, 4 in order with correct values"
        );
    }
}
