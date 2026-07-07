[cryptography-rs](https://github.com/indygreg/cryptography-rs)
410c6512b317857dd905968984b15f2ee7a56d11
- change blocking reqwest client to `ureq` client to prevent calling blocking task in tokio async thread
- change github action schedule and clippy test
> cargo test --workspace
> cargo clippy --workspace --all-targets
> cargo clippy --workspace --all-targets --no-default-features --features reqwest
> last check date = 2026-06-01

## cryptographic-message-syntax/Cargo.toml
```diff
  reqwest = { version = "0.13.2", default-features = false, features = ["blocking", "rustls"], optional = true }
+ ureq = { version = "3", optional = true }
```

```diff
    [features]
-   default = ["http"]
-   http = ["dep:reqwest"]
+   default = ["reqwest"]
+   reqwest = ["dep:reqwest"]
+   ureq = ["dep:ureq"]
```

## cryptographic-message-syntax/src/lib.rs

add this to prevent enabled both `reqwest` and `ureq` feature at the same time
```rust
#[cfg(all(feature = "reqwest", feature = "ureq"))]
compile_error!("feature \"reqwest\" and feature \"ureq\" cannot be enabled at the same time");
```

:79, 81, 84, 182, 240, 269 (all cfg)
```diff
-   #[cfg(feature = "http")]
+   #[cfg(any(feature = "reqwest", feature = "ureq"))]
```

## cryptographic-message-syntax/src/signing.rs
:27
```diff
- reqwest::IntoUrl,
```

:34 add
```rust
#[cfg(feature = "reqwest")]
use reqwest::IntoUrl;

#[cfg(feature = "ureq")]
use ureq::http;
```

:73
```diff
    /// Time-Stamp Protocol (TSP) server HTTP URL to use.
+   #[cfg(feature = "reqwest")]
    time_stamp_url: Option<reqwest::Url>,
+   /// Time-Stamp Protocol (TSP) server HTTP URL to use.
+   #[cfg(feature = "ureq")]
+   time_stamp_url: Option<http::Uri>,
```

:176
```diff
+   #[cfg(feature = "reqwest")]
-   pub fn time_stamp_url(mut self, url: impl IntoUrl) -> Result<Self, reqwest::Error> {
+   pub fn time_stamp_url(mut self, url: impl IntoUrl) -> Result<Self, TimeStampError> {
```

add
```rust
    /// Obtain a time-stamp token from a server.
    ///
    /// If this is called, the URL must be a server implementing the Time-Stamp Protocol
    /// (TSP) as defined by RFC 3161. At signature generation time, the server will be
    /// contacted and the time stamp token response will be added as an unsigned attribute
    /// on the [SignedData] instance.
    #[cfg(feature = "ureq")]
    pub fn time_stamp_url<T>(mut self, url: T) -> Result<Self, TimeStampError>
    where
        http::Uri: TryFrom<T>,
    {
        let uri = http::Uri::try_from(url)
            .map_err(|_| TimeStampError::Http(String::from("Invalid Uri")))?;
        self.time_stamp_url = Some(uri);
        Ok(self)
    }
```

## cryptographic-message-syntax/src/time_stamp_protocol.rs
:20
```diff
-    reqwest::IntoUrl,
```

:24 add
```rust
#[cfg(feature = "reqwest")]
use reqwest::IntoUrl;

#[cfg(feature = "ureq")]
use ureq::http;
```

:36
```diff
pub enum TimeStampError {
    Io(std::io::Error),
+   #[cfg(feature = "reqwest")]
    Reqwest(reqwest::Error),
+   #[cfg(feature = "ureq")]
+   Ureq(ureq::Error),
    Asn1Decode(DecodeError<Infallible>),
-   Http(&'static str),
+   Http(String),
    Random,
```

:52
```diff
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => f.write_fmt(format_args!("I/O error: {}", e)),
+           #[cfg(feature = "reqwest")]
            Self::Reqwest(e) => f.write_fmt(format_args!("HTTP error: {}", e)),
+           #[cfg(feature = "ureq")]
+           Self::Ureq(e) => f.write_fmt(format_args!("HTTP client error: {}", e)),
            Self::Asn1Decode(e) => f.write_fmt(format_args!("ASN.1 decode error: {}", e)),
```

:79
```diff
+   #[cfg(feature = "reqwest")]
    impl From<reqwest::Error> for TimeStampError {
        fn from(e: reqwest::Error) -> Self {
            Self::Reqwest(e)
        }
    }

+   #[cfg(feature = "ureq")]
+   impl From<ureq::Error> for TimeStampError {
+       fn from(e: ureq::Error) -> Self {
+           Self::Ureq(e)
+       }
+   }

+   #[cfg(feature = "ureq")]
+   impl From<http::Error> for TimeStampError {
+       fn from(e: http::Error) -> Self {
+           Self::Http(e.to_string())
+       }
+   }
```

:179
```diff
+   #[cfg(feature = "reqwest")]
    pub fn time_stamp_request_http(
        url: impl IntoUrl,
        request: &TimeStampReq,
```

:221 time_stamp_request_http()
```diff
    } else {
-       Err(TimeStampError::Http("bad HTTP response"))
+       Err(TimeStampError::Http(String::from("bad HTTP response")))
    }
```

Add
```rust
/// Send a [TimeStampReq] to a server via HTTP.
#[cfg(feature = "ureq")]
pub fn time_stamp_request_http<T>(
    url: T,
    request: &TimeStampReq,
) -> Result<TimeStampResponse, TimeStampError>
where
    http::Uri: TryFrom<T>,
    <http::Uri as TryFrom<T>>::Error: Into<http::Error>,
{
    let mut body = Vec::<u8>::new();
    request
        .encode_ref()
        .write_encoded(bcder::Mode::Der, &mut body)?;

    let req = http::Request::post(url)
        .header("Content-Type", HTTP_CONTENT_TYPE_REQUEST)
        .body(body)?;
    let mut response = ureq::run(req)?;

    if response.status().is_success()
        && response.headers().get("Content-Type")
            == Some(&http::header::HeaderValue::from_static(
                HTTP_CONTENT_TYPE_RESPONSE,
            ))
    {
        let response_bytes = response.body_mut().read_to_vec()?;

        let res = TimeStampResponse(Constructed::decode(
            response_bytes.as_ref(),
            bcder::Mode::Der,
            TimeStampResp::take_from,
        )?);

        // Verify nonce was reflected, if present.
        if res.is_success() {
            if let Some(tst_info) = res.tst_info()? {
                if tst_info.nonce != request.nonce {
                    return Err(TimeStampError::NonceMismatch);
                }
            }
        }

        Ok(res)
    } else {
        Err(TimeStampError::Http(String::from("bad HTTP response")))
    }
}
```

:279
```diff
+   #[cfg(feature = "reqwest")]
    pub fn time_stamp_message_http(
        url: impl IntoUrl,
        message: &[u8],
        digest_algorithm: DigestAlgorithm,
    ) -> Result<TimeStampResponse, TimeStampError> {
-       let mut h = digest_algorithm.digester();
-       h.update(message);
-       let digest = h.finish();

-       let mut random = [0u8; 8];
-       ring::rand::SystemRandom::new()
-           .fill(&mut random)
-           .map_err(|_| TimeStampError::Random)?;

-       let request = TimeStampReq {
-           version: Integer::from(1),
-           message_imprint: MessageImprint {
-               hash_algorithm: digest_algorithm.into(),
-               hashed_message: OctetString::new(bytes::Bytes::copy_from_slice(digest.as_ref())),
-           },
-           req_policy: None,
-           nonce: Some(Integer::from(u64::from_le_bytes(random))),
-           cert_req: Some(true),
-           extensions: None,
-       };

+       let request = create_request(message, digest_algorithm)?;
        time_stamp_request_http(url, &request)
    }
```

:288 add
```rust

/// Send a Time-Stamp request for a given message to an HTTP URL.
///
/// This is a wrapper around [time_stamp_request_http] that constructs the low-level
/// ASN.1 request object with reasonable defaults.
#[cfg(feature = "ureq")]
pub fn time_stamp_message_http<T>(
    url: T,
    message: &[u8],
    digest_algorithm: DigestAlgorithm,
) -> Result<TimeStampResponse, TimeStampError>
where
    http::Uri: TryFrom<T>,
    <http::Uri as TryFrom<T>>::Error: Into<http::Error>,
{
    let request = create_request(message, digest_algorithm)?;
    time_stamp_request_http(url, &request)
}

fn create_request(
    message: &[u8],
    digest_algorithm: DigestAlgorithm,
) -> Result<TimeStampReq, TimeStampError> {
    let mut h = digest_algorithm.digester();
    h.update(message);
    let digest = h.finish();

    let mut random = [0u8; 8];
    ring::rand::SystemRandom::new()
        .fill(&mut random)
        .map_err(|_| TimeStampError::Random)?;

    Ok(TimeStampReq {
        version: Integer::from(1),
        message_imprint: MessageImprint {
            hash_algorithm: digest_algorithm.into(),
            hashed_message: OctetString::new(bytes::Bytes::copy_from_slice(digest.as_ref())),
        },
        req_policy: None,
        nonce: Some(Integer::from(u64::from_le_bytes(random))),
        cert_req: Some(true),
        extensions: None,
    })
}
```

## .github/workflow/workspace.yml
:4
```diff
on:
  push:
  pull_request:
-  schedule:
-    - cron: '13 15 * * *'
```

:50
```diff
      - name: Run Clippy
        if: ${{ matrix.rust_toolchain == 'stable' || matrix.rust_toolchain == 'beta' }}
        run: |
-         cargo clippy --workspace --all-targets --all-features
+         cargo clippy --workspace --all-targets
+         cargo clippy --workspace --all-targets --no-default-features --features ureq
```