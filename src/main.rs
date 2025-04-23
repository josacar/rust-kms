use rust_kms::KmsEncryption;
use std::{path::Path,env};

fn help() {
    eprintln!("\nusage:
rust_kms encrypt <file> <kms_id>
    Encrypts a file content to STDOUT with the given kms_id
rust_kms decrypt <file>
    Decrypts the file content to STDOUT
");
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let kms_encryption = KmsEncryption::new();

    match args.len() {
        3 => {
            let operation = &args[1];
            let file_path = &args[2];
            let path = Path::new(file_path);

            match &operation[..] {
                "decrypt" => {
                    match kms_encryption.await.decrypt_file(path).await {
                        Ok(_) => eprintln!("File decrypted successfully"),
                        Err(e) => eprintln!("Failed to decrypt file: {:?}", e),
                    }
                },
                _ => {
                    eprintln!("Operation not supported");
                    help();
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
                    match kms_encryption.await.encrypt_file(key_id, path).await {
                        Ok(_) => eprintln!("File encrypted successfully"),
                        Err(e) => eprintln!("Failed to encrypt file: {:?}", e),
                    }
                },
                _ => {
                    eprintln!("Operation not supported");
                    help();
                }
            }
        },
        _ => {
            eprintln!("Wrong combination of arguments");
            help();
        }

    }
}
