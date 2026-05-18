use ring::{hmac, rand};
use ring::rand::SecureRandom;
use ring::error::Unspecified;

fn main() -> Result<(), Unspecified> {
    let mut key_value = [0u8; 48];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let message = "Legitimate important message";
    // Подписываем строку
    let signature = hmac::sign(&key, message.as_bytes());
    // Проверяем корректность подписи
    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;

    Ok(())
}
