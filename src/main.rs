use rust_kms::KmsEncryption;
use std::{path::Path,env};

#[tokio::main]
async fn main() {
    let kms_encryption = KmsEncryption::new().await;
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let operation = &args[1];
            let file_path = &args[2];
            let path = Path::new(file_path);

            match &operation[..] {
                "decrypt" => {
                    match kms_encryption.decrypt_file(path).await {
                        Ok(_) => eprintln!("File decrypted successfully"),
                        Err(e) => eprintln!("Failed to decrypt file: {:?}", e),
                    }
                },
                _ => {
                    eprintln!("Operation not supported");
                }
            }
        },
        4 => {
            let operation = &args[1];
            let file_path = &args[2];
            let key_id = &args[3];
            let path = Path::new(file_path);

            match &operation[..] {
                "encrypt" => {
                    match kms_encryption.encrypt_file(key_id, path).await {
                        Ok(_) => eprintln!("File encrypted successfully"),
                        Err(e) => eprintln!("Failed to encrypt file: {:?}", e),
                    }
                },
                _ => {
                    eprintln!("Operation not supported");
                }
            }
        },
        _ => {
            eprintln!("Wrong combination of arguments");
        }

    }
}
