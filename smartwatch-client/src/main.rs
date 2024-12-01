// Gerekli dış kütüphaneleri import edin
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::path::Path;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Signer},
    transaction::Transaction,
    message::Message,
    pubkey::Pubkey,
    instruction::Instruction,
};
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use base64::engine::general_purpose::STANDARD;
use base64::Engine as _; // Base64 işlemleri için gerekli
use std::str::FromStr;

// AES-128-CBC için tip tanımı
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn main() -> Result<(), Box<dyn Error>> {
    // .env dosyasını yükleyin (isteğe bağlı)
    dotenv().ok();

    // Çevresel değişkenden keypair yolunu al veya varsayılanı kullan
    let keypair_path = env::var("SOLANA_KEYPAIR_PATH")
        .unwrap_or_else(|_| "/home/nisa/solana-wallets/my-solana-wallet.json".to_string());

    // Payer'ın keypair'ini yükleme
    let payer = read_keypair_file(Path::new(&keypair_path))
        .map_err(|e| format!("{} dosyasını okumada başarısız: {}", keypair_path, e))?;

    // Solana RPC client'ını başlatın
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // Bakiye kontrolü
    let balance = client.get_balance(&payer.pubkey())?;
    println!("Cüzdan Bakiyesi: {} lamports", balance);

    // Bakiye yetersizse airdrop talep etme
    if balance < 1_000_000_000 {
        println!("Yetersiz bakiye. Airdrop talep ediliyor...");
        match client.request_airdrop(&payer.pubkey(), 2_000_000_000) {
            Ok(sig) => {
                println!("Airdrop talebi gönderildi: {}", sig);
                // Airdrop'un onaylanmasını bekleme
                client.confirm_transaction(&sig)?;
                println!("Airdrop onaylandı.");
            },
            Err(e) => {
                println!("Airdrop talebi başarısız oldu: {:?}", e);
                return Err(Box::new(e));
            }
        }
    }

    // Akıllı saat verilerini simüle etme
    let smartwatch_data = "HeartRate:72,Steps:1200";

    // Çevresel değişkenlerden AES anahtarı ve IV'si al
    let aes_key_str = env::var("AES_KEY")
        .expect("AES_KEY çevresel değişkeni ayarlanmadı");
    let key = aes_key_str.as_bytes();

    let aes_iv_str = env::var("AES_IV")
        .expect("AES_IV çevresel değişkeni ayarlanmadı");
    let iv = aes_iv_str.as_bytes();

    // Anahtar ve IV uzunluklarını doğrulama
    if key.len() != 16 || iv.len() != 16 {
        return Err("AES anahtarı ve IV'si AES-128-CBC için 16 bayt olmalıdır".into());
    }

    // Veriyi şifreleme
    let cipher = Aes128Cbc::new_from_slices(key, iv)?;
    let ciphertext = cipher.encrypt_vec(smartwatch_data.as_bytes());

    // Şifrelenmiş veriyi Base64'e çevirme
    let encoded_ciphertext = STANDARD.encode(&ciphertext);

    // Memo programının ID'si
    let memo_program_id = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr")?;

    // Şifrelenmiş veriyi Memo programına eklemek için Instruction oluşturma
    let memo_instruction = Instruction {
        program_id: memo_program_id,
        accounts: vec![],
        data: encoded_ciphertext.into_bytes(),
    };

    // Mesaj ve işlem oluşturma
    let message = Message::new(&[memo_instruction], Some(&payer.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);

    // İşlemi imzalama ve gönderme
    let recent_blockhash = client.get_latest_blockhash()?;
    transaction.sign(&[&payer], recent_blockhash);
    let signature = client.send_and_confirm_transaction(&transaction)?;

    println!("İşlem İmzası (Transaction Signature): {}", signature);

    Ok(())
}
//5VLR3rLxv4BJTKSiiWKyBWHd3PZ4tBepmZxix4Cw9gPseoQsp6ryjDh6XUNEQPbPAZGYbhgHXGodzFU44vtEGs6R işlem imzası
//4DWJkKW881CEMrENCokEs8o6ybZtQgdb9zoWp7LBthy6 pubkey