use rust_kms::KmsEncryption;
use std::path::Path;
use clap::{Parser, Subcommand};

/// KMS Encryption Tool
#[derive(Parser, Debug)]
#[clap(author = "Jose Luis Salas", version = "0.1.0", about = "Encrypts/decrypts files using KMS", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Encrypts a file using KMS")]
    Encrypt {
        #[clap(help = "File to encrypt")]
        file: String,
        #[clap(help = "KMS key ID")]
        key_id: String,
    },
    #[clap(about = "Decrypts a file using KMS")]
    Decrypt {
        #[clap(help = "File to decrypt")]
        file: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt { file, key_id } => {
            let path = Path::new(&file);
            let kms_encryption = KmsEncryption::new();
            match kms_encryption.await.encrypt_file(&key_id, path).await {
                Ok(_) => eprintln!("File encrypted successfully"),
                Err(e) => eprintln!("Failed to encrypt file: {:?}", e),
            }
        }
        Commands::Decrypt { file } => {
            let path = Path::new(&file);
            let kms_encryption = KmsEncryption::new();
            match kms_encryption.await.decrypt_file(path).await {
                Ok(_) => eprintln!("File decrypted successfully"),
                Err(e) => eprintln!("Failed to decrypt file: {:?}", e),
            }
        }
    }
}

