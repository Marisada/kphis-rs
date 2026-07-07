use cryptographic_message_syntax::{Bytes, CmsError, Oid, SignedDataBuilder, SignerBuilder};
use std::sync::Arc;
use tracing::warn;
use typst_pdf::PdfSig;
use x509_certificate::{CapturedX509Certificate, InMemorySigningKeyPair};

use kphis_util::error::{AppError, Source};

use crate::state::ApiState;

#[derive(Clone)]
pub struct PdfSigner {
    // x509 pem
    pub signing_certificate: CapturedX509Certificate,
    // pkcs#8 pem
    pub signing_key: Arc<InMemorySigningKeyPair>,
    // Time Stamp Authority url
    pub tsa: Option<String>,
    pub sig: PdfSig,
}

impl PdfSigner {
    pub fn sign(&self, raw: Vec<u8>) -> Result<Vec<u8>, AppError> {
        // Calculate file hash and sign it using the users key
        // ASN.1: CAdES (EN 319 122-1 and 2) builds on CMS IETF RFC 5652.
        // https://www.rfc-editor.org/info/rfc5652

        // Use Cert/Private key to sign data
        let signer_alone = SignerBuilder::new(self.signing_key.as_ref(), self.signing_certificate.clone());

        // Try using a time server. If it fails we continue without it.
        // Alternative time servers:
        // 1: https://freetsa.org/tsr
        // 2: http://timestamp.digicert.com
        let signer = if let Some(tsa) = &self.tsa {
            match signer_alone.clone().time_stamp_url(tsa) {
                Ok(signer) => signer,
                Err(e) => {
                    warn!("Using PDF signer without timestamp because TSA url parsing error: {e}");
                    signer_alone.clone()
                }
            }
        } else {
            signer_alone.clone()
        };
        let signature = match signing(raw.clone(), signer) {
            Ok(sig) => Ok(sig),
            Err(e) => {
                warn!("Using PDF signer without timestamp because Signing: {e}");
                signing(raw, signer_alone)
            }
        };
        signature.map_err(|e| Source::Cms.to_error(500, e, "Signing PDF"))
    }
}

/// call this only after add SIG to PDF's catalog tag
pub fn write_sig_content(mut buf: Vec<u8>, app: &ApiState) -> Result<Vec<u8>, AppError> {
    let contents_point = find_binary_pattern(&buf, b"<BEEFFACE00"); // [190,239,250,206]
    let byte_range_point = find_binary_pattern(&buf, b"[88888888");

    match (contents_point, app.pdf_signer.as_ref()) {
        (Some(cp), Some(signer)) => {
            // we need signature Contents from [cryptographic_message_syntax](https://github.com/indygreg/cryptography-rs)
            // to overwrite 'BEEFFACE00..00' at this stage
            // cryptographic_message_syntax::signing::SignedDataBuilder::build_der() will return Vec<u8>
            // - rsa:4096 sha256: ~2,000 bytes
            // - timestamp: ~5,500 bytes
            // so 'BEEFFACE00..00' length should be >10,000 bytes (>20,000 hex string chars)
            // *NOTE*: please use the same Contents length as 'typst-pdf\src\catalog.rs::write_catalog' function
            // pdf_writer will generate '<BEEFFACE00..00>' from [190,239,250,206,0,0,..,0,0]
            let tail = cp + 22220 + 2; // hex string length(bytes*2), with "<" and ">"
            let tail_n = buf.len() - tail;

            // we need 2 parts of bytes to hash -> ByteRange [head=0, head_n, tail, tail_n]
            // - head part = BOF(0) -> start of sig_content(head_n = contents_point)
            // - tail part = end of sig_content(tail = head_n + sig_content.len()) -> EOF(tail_n = buf.len() - tail)
            // pad with ' ' to fill 37 chars (so max is '[01234567 01234567 01234567 01234567]' '[0 0123456789 0123456789 0123456789a]')
            let byte_range_new = format!("[0 {cp} {tail} {tail_n}]");
            let byte_range = format!("{byte_range_new:<37}");

            // update ByteRange
            let mut buf_with_br = match byte_range_point {
                Some(brp) => {
                    buf.splice(brp..(brp + byte_range.len()), byte_range.as_bytes().to_vec());
                    buf
                }
                None => buf,
            };

            // Construct Raw data to be hash
            let first_range = 0..cp;
            let second_range = tail..tail + tail_n;
            let first_part = &buf_with_br[first_range];
            let second_part = &buf_with_br[second_range];
            // Create new vec without the content part
            let mut raw = Vec::new();
            raw.extend_from_slice(first_part);
            raw.extend_from_slice(second_part);

            let signature = signer.sign(raw)?;
            let signature_hex = hex::encode(signature);
            let signature_bytes = signature_hex.as_bytes();

            // Construct new Contents and insert it into file
            let new_contents_len = signature_bytes.len() + 1;
            let mut new_contents = Vec::with_capacity(new_contents_len);
            new_contents.push(60u8); // "<"
            new_contents.extend_from_slice(signature_bytes);
            buf_with_br.splice(cp..(cp + new_contents_len), new_contents);
            Ok(buf_with_br)
        }
        (_, _) => Ok(buf),
    }
}

fn signing(raw: Vec<u8>, signer_builder: SignerBuilder) -> Result<Vec<u8>, CmsError> {
    SignedDataBuilder::default()
        .content_external(raw)
        .content_type(Oid(Bytes::copy_from_slice(cryptographic_message_syntax::asn1::rfc5652::OID_ID_DATA.as_ref())))
        .signer(signer_builder)
        .build_der()
}

fn find_binary_pattern(bytes: &[u8], pattern: &[u8]) -> Option<usize> {
    if bytes.is_empty() || pattern.is_empty() {
        return None;
    }

    let first_pat_byte = pattern.first().expect("At least 1 byte expected.");
    let mut next_pat_byte = first_pat_byte;
    let mut pattern_index = 0;
    let mut start_index = 0;

    for (index, byte) in bytes.iter().enumerate() {
        if next_pat_byte == byte {
            // Save `start_index` for later
            if pattern_index == 0 {
                start_index = index;
            }
            // Go to next byte of pattern
            pattern_index += 1;
            next_pat_byte = match pattern.get(pattern_index) {
                Some(byte) => byte,
                None => return Some(start_index),
            };
        } else {
            // If pattern breaks or does not match
            pattern_index = 0;
            next_pat_byte = first_pat_byte;
        }
    }

    None
}
