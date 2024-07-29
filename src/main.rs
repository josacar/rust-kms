use rust_kms::KmsEncryption;
use std::path::Path;

#[tokio::main]
async fn main() {
    let kms_encryption = KmsEncryption::new().await;
    let key_id = "a5807bb6-cde2-41f4-983d-f5f2f270170f";  // Replace with your AWS KMS key ID

    let file_path = Path::new("file.txt");

    // Encrypt the file
    match kms_encryption.encrypt_file(key_id, file_path).await {
        Ok(_) => eprintln!("File encrypted successfully"),
        Err(e) => eprintln!("Failed to encrypt file: {:?}", e),
    }

    println!("\n");

    let file_path = Path::new("file.encrypted");

    // Decrypt the file
    match kms_encryption.decrypt_file(file_path).await {
        Ok(_) => eprintln!("File decrypted successfully"),
        Err(e) => eprintln!("Failed to decrypt file: {:?}", e),
    }
}
