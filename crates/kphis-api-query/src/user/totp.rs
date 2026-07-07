use totp_rs::{Algorithm, Secret, TOTP};

use kphis_util::{
    datetime::get_timestamp_server,
    error::{AppError, Source},
};

/// return (qr, secret key)
pub fn new_totp_encoded_key(loginname: &str, service_name: &str) -> Result<(String, String), AppError> {
    let secret = Secret::default();
    let secret_bytes = secret.to_bytes().map_err(|e| Source::Totp.to_error(500, e, "ParseKey"))?;
    let qr = gen_qr(loginname, service_name, secret_bytes)?;

    Ok((qr, secret.to_encoded().to_string()))
}

/// return (qr, secret key)
pub fn new_totp(loginname: &str, service_name: &str) -> Result<(String, Vec<u8>), AppError> {
    let secret = Secret::default();
    let secret_bytes = secret.to_bytes().map_err(|e| Source::Totp.to_error(500, e, "ParseKey"))?;
    let qr = gen_qr(loginname, service_name, secret_bytes.clone())?;

    Ok((qr, secret_bytes))
}

fn gen_qr(loginname: &str, service_name: &str, secret: Vec<u8>) -> Result<String, AppError> {
    let totper = TOTP::new(Algorithm::SHA1, 6, 0, 30, secret, Some(service_name.to_owned()), loginname.to_owned()).map_err(|e| Source::Totp.to_error(500, e, "New TOTP"))?;
    let qr = totper.get_qr_base64().map_err(|e| Source::Totp.to_error(500, e, "Get QR-CODE"))?;

    Ok(qr)
}

pub fn verify_totp_encoded_key(loginname: &str, token_2fa: &str, secret_encoded: &str, service_name: &str) -> Result<bool, AppError> {
    let secret = Secret::Encoded(secret_encoded.to_owned()).to_bytes().map_err(|e| Source::Totp.to_error(401, e, "Verify TOTP"))?;
    verify_totp(loginname, token_2fa, secret, service_name)
}

pub fn verify_totp(loginname: &str, token_2fa: &str, secret: Vec<u8>, service_name: &str) -> Result<bool, AppError> {
    let totp = TOTP::new(Algorithm::SHA1, 6, 0, 30, secret, Some(service_name.to_owned()), loginname.to_owned()).map_err(|e| Source::Totp.to_error(401, e, "Verify Password"))?;
    let now = get_timestamp_server()?;
    let result = totp.check(token_2fa, now) || totp.check(token_2fa, now.saturating_sub(30)) || totp.check(token_2fa, now.saturating_add(30));

    Ok(result)
}
