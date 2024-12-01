use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use base64;
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    message::Message,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use std::error::Error;
use std::str::FromStr;

// AES-128-CBC için tip tanımı
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

/// AES-128-CBC ile veriyi şifreler ve Base64 formatına çevirir.
pub fn encrypt_data(
    key: &[u8],
    iv: &[u8],
    data: &str,
) -> Result<String, Box<dyn Error>> {
    if key.len() != 16 || iv.len() != 16 {
        return Err("AES anahtarı ve IV'si AES-128-CBC için 16 bayt uzunluğunda olmalıdır".into());
    }

    let cipher = Aes128Cbc::new_from_slices(key, iv)?;
    let ciphertext = cipher.encrypt_vec(data.as_bytes());
    Ok(base64::encode(&ciphertext))
}

/// Memo programı için bir `Instruction` oluşturur.
pub fn create_memo_instruction(
    memo_program_id: &str,
    data: &str,
) -> Result<Instruction, Box<dyn Error>> {
    let program_id = Pubkey::from_str(memo_program_id)?;
    Ok(Instruction {
        program_id,
        accounts: vec![],
        data: data.as_bytes().to_vec(),
    })
}

/// Solana RPC Client'ını başlatır.
pub fn init_solana_client(rpc_url: &str) -> RpcClient {
    RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed())
}

/// Yeni bir keypair oluşturur.
pub fn create_keypair() -> Keypair {
    Keypair::new()
}

/// Solana ağına bir işlem gönderir.
pub fn send_transaction(
    client: &RpcClient,
    payer: &Keypair,
    instructions: Vec<Instruction>,
) -> Result<String, Box<dyn Error>> {
    let recent_blockhash = client.get_latest_blockhash()?;
    let message = Message::new(&instructions, Some(&payer.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.sign(&[payer], recent_blockhash);

    Ok(client.send_and_confirm_transaction(&transaction)?)
}
