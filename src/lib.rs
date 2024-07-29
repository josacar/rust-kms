use aws_sdk_kms::primitives::Blob;
use aws_sdk_kms::{Client, Error};
use aws_config::BehaviorVersion;
use std::fs;
use std::path::Path;
use std::io::{self, Write};

pub struct KmsEncryption {
    client: Client,
}

impl KmsEncryption {
    pub async fn new() -> Self {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&config);
        KmsEncryption { client }
    }

    pub async fn encrypt_file(&self, key_id: &str, file_path: &Path) -> Result<(), Error> {
        let data = fs::read(file_path)
            .expect("Should have been able to read the file");

        let blob = Blob::new(data);

        let encrypt_output = self.client
            .encrypt()
            .key_id(key_id)
            .plaintext(blob)
            .send()
            .await
            .expect("Error in encrypting the file");

        let ciphertext_blob = encrypt_output.ciphertext_blob
            .expect("Failed to get the blob");

        let bytes = ciphertext_blob.as_ref();
        let result = io::stdout().write_all(bytes).unwrap();

        Ok(result)
    }

    pub async fn decrypt_file(&self, file_path: &Path) -> Result<(), Error> {
        let ciphertext_blob = fs::read(file_path)
            .expect("Failed to read the file");

        let blob = Blob::new(ciphertext_blob);

        let decrypt_output = self.client
            .decrypt()
            .ciphertext_blob(blob)
            .send()
            .await?;

        let plaintext = decrypt_output.plaintext
            .expect("Failed to get plaintext");

        let result = io::stdout().write_all(plaintext.as_ref()).unwrap();

        Ok(result)
    }
}
