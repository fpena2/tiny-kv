tonic::include_proto!("tinykv");

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("tinykv_descriptor");

impl KvPair {
    pub fn default() -> KvPair {
        KvPair {
            key: String::from(""),
            value: String::from(""),
        }
    }
    pub fn new(k: &str, v: &str) -> KvPair {
        KvPair {
            key: String::from(k),
            value: String::from(v),
        }
    }
}
