use aws_sdk_s3::Client as S3Client;
use tracing::{info, error};

#[derive(Clone)]
pub struct S3Service {
    client: S3Client,
    bucket: String,
    region: String,
}

impl S3Service {
    pub async fn new() -> anyhow::Result<Self> {
        let config = aws_config::load_from_env().await;
        let client = S3Client::new(&config);
        let bucket = std::env::var("S3_BUCKET")
            .unwrap_or_else(|_| "telnyx-ai-service".to_string());
        let region = std::env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string());

        info!("✅ S3 Service inicializado. Bucket: {}", bucket);

        Ok(Self { client, bucket, region })
    }

    pub async fn upload_audio(
        &self,
        key: &str,
        data: Vec<u8>,
    ) -> anyhow::Result<String> {

        let _response = self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(aws_sdk_s3::primitives::ByteStream::from(data))
            .content_type("audio/mpeg")
            .acl(aws_sdk_s3::types::ObjectCannedAcl::PublicRead)
            .send()
            .await?;

        // Construir URL pública incluyendo la región (más fiable)
        let url = format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            self.bucket, self.region, key
        );

        info!("✅ Audio subido a S3. URL: {}", url);
        
        Ok(url)
    }

    pub async fn get_url(&self, key: &str) -> String {
        format!(
            "https://{}.s3.amazonaws.com/{}",
            self.bucket, key
        )
    }
}
