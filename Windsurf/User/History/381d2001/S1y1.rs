use std::path::{Path, PathBuf};

use tokio::{fs::File, io::AsyncWriteExt};
use reqwest::Client;
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;

use crate::core::error::Result;

type HmacSha256 = Hmac<Sha256>;

const DATA_DIR: &str = ".sync-bfu";

pub struct SimpleS3Client {
    client: Client,
    access_key: String,
    secret_key: String,
    region: String,
    endpoint: Option<String>,
}

impl SimpleS3Client {
    pub fn new(access_key: String, secret_key: String, region: String, endpoint: Option<String>) -> Self {
        Self {
            client: Client::new(),
            access_key,
            secret_key,
            region,
            endpoint,
        }
    }

    fn get_endpoint(&self) -> String {
        self.endpoint.clone()
            .unwrap_or_else(|| format!("https://s3.{}.amazonaws.com", self.region))
    }

    async fn sign_request(&self, method: &str, uri: &str, headers: &mut Vec<(&str, &str)>, payload: &str) -> String {
        let now = Utc::now();
        let date_stamp = now.format("%Y%m%d").to_string();
        let time_stamp = now.format("%Y%m%dT%H%M%SZ").to_string();
        
        headers.push(("host", &self.get_endpoint()[8..])); // Remove https://
        headers.push(("x-amz-date", &time_stamp));
        
        let canonical_headers = headers.iter()
            .map(|(k, v)| format!("{}:{}\n", k.to_lowercase(), v))
            .collect::<String>();
        
        let signed_headers = headers.iter()
            .map(|(k, _)| k.to_lowercase())
            .collect::<Vec<_>>()
            .join(";");
        
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method,
            uri,
            "",
            canonical_headers,
            signed_headers,
            hex::encode(sha2::Sha256::digest(payload.as_bytes()))
        );
        
        let credential_scope = format!("{}/{}/s3/aws4_request", date_stamp, self.region);
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            time_stamp,
            credential_scope,
            hex::encode(sha2::Sha256::digest(canonical_request.as_bytes()))
        );
        
        let mut mac = HmacSha256::new_from_slice(format!("AWS4{}", self.secret_key).as_bytes()).unwrap();
        mac.update(date_stamp.as_bytes());
        let date_key = mac.finalize().into_bytes();
        
        let mut mac = HmacSha256::new_from_slice(&date_key).unwrap();
        mac.update(self.region.as_bytes());
        let region_key = mac.finalize().into_bytes();
        
        let mut mac = HmacSha256::new_from_slice(&region_key).unwrap();
        mac.update(b"s3");
        let service_key = mac.finalize().into_bytes();
        
        let mut mac = HmacSha256::new_from_slice(&service_key).unwrap();
        mac.update(b"aws4_request");
        let signing_key = mac.finalize().into_bytes();
        
        let mut mac = HmacSha256::new_from_slice(&signing_key).unwrap();
        mac.update(string_to_sign.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());
        
        format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            self.access_key, credential_scope, signed_headers, signature
        )
    }

    pub async fn list_objects(&self, bucket: &str, prefix: &str) -> Result<Vec<String>> {
        let uri = if prefix.is_empty() {
            format!("/{}/", bucket)
        } else {
            format!("/{}/?list-type=2&prefix={}", bucket, urlencoding::encode(prefix))
        };
        
        let mut headers = vec![];
        let authorization = self.sign_request("GET", &uri, &mut headers, "").await;
        headers.push(("authorization", &authorization));
        
        let url = format!("{}{}", self.get_endpoint(), uri);
        
        let response = self.client
            .get(&url)
            .headers(headers.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect())
            .send()
            .await?;
        
        if response.status().is_success() {
            let text = response.text().await?;
            Ok(vec![]) // TODO: Parse XML response
        } else {
            Err(crate::core::error::SyncError::S3Error(
                format!("S3 request failed: {}", response.status())
            ))
        }
    }
}

pub struct DataManager {
    s3_client: SimpleS3Client,
}

pub struct FullFile {
    pub file: File,
    pub path: PathBuf,
}

impl DataManager {
    pub fn new(s3_client: SimpleS3Client) -> Self {
        Self { s3_client }
    }

    pub async fn list_objects(&self, bucket: &str, prefix: &str) -> Result<Vec<String>> {
        self.s3_client.list_objects(bucket, prefix).await
    }

    pub async fn download_file(
        &self,
        bucket: &str,
        key: &str,
        base_path: &Path,
    ) -> Result<FullFile> {
        let file_path = base_path.join(key);

        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // TODO: Implement S3 download
        let mut file = File::create(&file_path).await?;
        file.write_all(b"placeholder").await?;
        file.flush().await?;

        Ok(FullFile {
            file,
            path: file_path,
        })
    }

    pub fn get_data_dir() -> PathBuf {
        home::home_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join(DATA_DIR)
    }
}
