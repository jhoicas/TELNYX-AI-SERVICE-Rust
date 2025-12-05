use aws_sdk_s3::Client as S3Client;
use tracing::{info, error};

#[derive(Clone)]
pub struct S3Service {
    client: S3Client,
    bucket: String,
}

impl S3Service {
    pub async fn new() -> anyhow::Result<Self> {
        let config = aws_config::load_from_env().await;
        let client = S3Client::new(&config);
        let bucket = std::env::var("S3_BUCKET")
            .unwrap_or_else(|_| "telnyx-ai-service".to_string());

        info!("✅ S3 Service inicializado", bucket = bucket);

        Ok(Self { client, bucket })
    }

    pub async fn upload_audio(
        &self,
        key: &str,
        data: Vec<u8>,
    ) -> anyhow::Result<String> {
        let response = self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(aws_sdk_s3::primitives::ByteStream::from(data))
            .content_type("audio/mpeg")
            .acl(aws_sdk_s3::types::ObjectCannedAcl::PublicRead)
            .send()
            .await?;

        let url = format!(
            "https://{}.s3.amazonaws.com/{}",
            self.bucket, key
        );

        info!("✅ Audio subido a S3", url = url);

        Ok(url)
    }

    pub async fn get_url(&self, key: &str) -> String {
        format!(
            "https://{}.s3.amazonaws.com/{}",
            self.bucket, key
        )
    }
}
