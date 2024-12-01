use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    transaction::Transaction,
    message::Message,
};
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use std::error::Error;
use solana_program::{
    instruction::Instruction,
    pubkey::Pubkey,
};
use std::str::FromStr;

// AES-128-CBC için tip tanımı
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn main() -> Result<(), Box<dyn Error>> {
    // Solana Devnet'e bağlanma
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // Yeni bir anahtar çifti oluşturma
    let payer = Keypair::new();

    // Cüzdana SOL yükleme (airdrop)
    let sig = client.request_airdrop(&payer.pubkey(), 1_000_000_000)?;
    client.confirm_transaction(&sig)?;

    // Akıllı saat verilerini simüle etme
    let smartwatch_data = "HeartRate:72,Steps:1200";

    // Verileri şifreleme
    let key = b"very secret key."; // 16 byte'lık anahtar
    let iv = b"unique nonce 1234"; // 16 byte'lık IV
    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
    let ciphertext = cipher.encrypt_vec(smartwatch_data.as_bytes());

    // Memo programının ID'si
    let memo_program_id = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr")?;

    // Şifrelenmiş veriyi Memo programına eklemek için Instruction oluşturma
    let memo_instruction = Instruction {
        program_id: memo_program_id,
        accounts: vec![],
        data: ciphertext,
    };

    let message = Message::new(&[memo_instruction], Some(&payer.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);

    // İşlemi imzalama ve gönderme
    let recent_blockhash = client.get_latest_blockhash()?;
    transaction.sign(&[&payer], recent_blockhash);
    let signature = client.send_and_confirm_transaction(&transaction)?;

    println!("İşlem İmzası (Transaction Signature): {}", signature);

    Ok(())
}
