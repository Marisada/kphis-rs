use totp_rs::{Algorithm, Builder, Secret};

use kphis_util::{
    datetime::get_timestamp_server,
    error::{AppError, Source},
};

/// return (qr, secret key)
pub fn new_totp_encoded_key(loginname: &str, service_name: &str) -> Result<(String, String), AppError> {
    let secret = Secret::default();
    let secret_bytes = secret.as_bytes();
    let qr = gen_qr(loginname, service_name, secret_bytes)?;

    Ok((qr, secret.to_base32()))
}

/// return (qr, secret key)
pub fn new_totp(loginname: &str, service_name: &str) -> Result<(String, Vec<u8>), AppError> {
    let secret = Secret::default();
    let secret_bytes = secret.as_bytes();
    let qr = gen_qr(loginname, service_name, secret_bytes)?;

    Ok((qr, secret_bytes.to_vec()))
}

fn gen_qr(loginname: &str, service_name: &str, secret: &[u8]) -> Result<String, AppError> {
    let totper = Builder::new()
        .with_algorithm(Algorithm::SHA1)
        .with_digits(6)
        .with_skew(0)
        .with_step_duration(30)
        .with_secret(secret)
        .with_issuer(Some(service_name))
        .with_account_name(loginname)
        .build()
        .map_err(|e| Source::Totp.to_error(500, e, "New TOTP"))?;
    let qr = totper.to_qr_base64().map_err(|e| Source::Totp.to_error(500, e, "Get QR-CODE"))?;

    Ok(qr)
}

/// return Some(step number)
pub fn verify_totp_encoded_key(loginname: &str, token_2fa: &str, secret_encoded: &str, service_name: &str) -> Result<Option<u64>, AppError> {
    let secret = Secret::try_from_base32(secret_encoded).map_err(|e| Source::Totp.to_error(401, e, "Verify TOTP"))?;
    verify_totp(loginname, token_2fa, secret.as_bytes(), service_name)
}

/// return Some(step number)
pub fn verify_totp(loginname: &str, token_2fa: &str, secret: &[u8], service_name: &str) -> Result<Option<u64>, AppError> {
    let totp = Builder::new()
        .with_algorithm(Algorithm::SHA1)
        .with_digits(6)
        .with_skew(0)
        .with_step_duration(30)
        .with_secret(secret)
        .with_issuer(Some(service_name))
        .with_account_name(loginname)
        .build()
        .map_err(|e| Source::Totp.to_error(401, e, "Verify Password"))?;
    let now = get_timestamp_server()?;
    let result = totp.check(token_2fa, now).or_else(|| totp.check(token_2fa, now.saturating_sub(30))).or_else(|| totp.check(token_2fa, now.saturating_add(30)));

    Ok(result)
}
