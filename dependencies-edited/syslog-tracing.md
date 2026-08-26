[syslog-tracing](https://github.com/sp1ff/syslog-tracing)

> last check date = 2026-08-20

## syslog-tracing/tracing-rfc-5424/src/byte-util.rs
remove unimplemented
: 25
```diff
    #[cfg(not(unix))]
    pub fn bytes_from_os_str(s: std::ffi::OsString) -> Vec<u8> {
-       unimplemented!("bytes_from_os_str is not supported on non-Unix.");
+       s.to_string_lossy().as_bytes().to_vec()
    }
```

## syslog-tracing/tracing-rfc-5424/src/layer.rs
move unused import
: 27
```diff
    use crate::{
        formatter::SyslogFormatter,
-       rfc3164::Rfc3164,
        rfc5424::Rfc5424,
        tracing::{TracingFormatter, TrivialTracingFormatter},
        transport::{Transport, UdpTransport},
    };
```
: 34
```diff
    #[cfg(unix)]
-   use crate::transport::UnixSocket;
+   use crate::{
+       rfc3164::Rfc3164,
+       transport::UnixSocket,
+   };
```

:445
```diff
    #[test]
    fn test_structured_data() {
+       use crate::rfc5424::escape;

        // Test with include_target enabled
        let f = Rfc5424::builder()
```

add escape to tests
```diff
-       let expected_file = CALLSITE.metadata().file().unwrap();
+       let expected_file = escape(CALLSITE.metadata().file().unwrap());
```

## syslog-tracing/tracing-rfc-5424/src/transport.rs
fix escape test
: 621
```diff
               // Optionally include target
                if has_target {
-                   let escaped = target
-                       .replace('\\', "\\\\")
-                       .replace('"', "\\\"")
-                       .replace(']', "\\]");
+                   let escaped = escape(target);
                    buf.put_slice(format!(" target=\"{}\"", escaped).as_bytes());
                }

                // Optionally include module path
                if has_module {
                    if let Some(module_path) = module {
-                       let escaped = module_path
-                           .replace('\\', "\\\\")
-                           .replace('"', "\\\"")
-                           .replace(']', "\\]");
+                       let escaped = escape(module_path);
                        buf.put_slice(format!(" module=\"{}\"", escaped).as_bytes());
                    }
                }
```

:642
```diff
                    if let Some(file) = metadata.file() {
-                       let escaped = file
-                           .replace('\\', "\\\\")
-                           .replace('"', "\\\"")
-                           .replace(']', "\\]");
+                       let escaped = escape(file);
                        buf.put_slice(format!(" file=\"{}\"", escaped).as_bytes());
                    }
```

add escape fn
```rust
pub fn escape(source: &str) -> String {
    source
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace(']', "\\]")
}
```

## syslog-tracing/tracing-rfc-5424/src/transport.rs
not test some doc
: 43
```diff
    //! To send messages over UDP to a local Unix socket:
    //!
-   //! ```rust
+   //! ```text
    //! use tracing_rfc_5424::transport::UnixSocket;
    //! let transpo = UnixSocket::new("/i/am/not/there.s");
    //! assert!(transpo.is_err()); // no such socket, after all
```

move unused import
: 75
```diff
-   use std::{net::TcpStream, path::Path};
    use std::net::TcpStream;
```

: 78
```diff
    #[cfg(unix)]
-   use std::os::unix::net::{UnixDatagram, UnixStream};
+   use std::{
+       os::unix::net::{UnixDatagram, UnixStream},
+       path::Path,
+   };
```

Fix os-error 10022 in Windows
:151
```diff
    /// Sending syslog messages via UDP datagrams.
    pub struct UdpTransport {
        socket: std::net::UdpSocket,
+       address: std::net::SocketAddr,
    }
```

:159
```diff
    pub fn new<A: std::net::ToSocketAddrs>(addr: A) -> Result<UdpTransport> {
        // Bind to any available port on localhost...
        let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
+       // Note: send after connect again in windows may throw error: An invalid argument was supplied. (os error 10022)
+       //
-       // and connect to the syslog daemon at `addr`...
-       socket.connect(addr)?;
+       // // and connect to the syslog daemon at `addr`...
+       // socket.connect(addr)?;
+
+       let address = addr.to_socket_addrs()?.next().ok_or_else(|| Error::Io {
+           source: std::io::Error::new(std::io::ErrorKind::InvalidInput, "could not resolve to any addresses"),
+           back: Backtrace::new(),
+       })?;
        // and we're done!
-       Ok(UdpTransport { socket })
+       Ok(UdpTransport { socket, address })
    }
```

:175
```diff
    type Error = Error;
    fn send(&self, buf: F::Output) -> std::result::Result<(), Self::Error> {
-       self.socket.send(&buf)?;
+       self.socket.send_to(&buf, &self.address)?;
        Ok(())
    }
```

## syslog-tracing/ChangeLog
add log
```
2026-08-23  Marisada Pitaktham  <p.marisada@gmail.com>

	- Implement `bytes_from_os_str()` for non-unix
	- Move unused imports of non-unix to unix imports
	- Fix test error from escaped source file path in non-unix
	- Fix doc on UnixSocket in non-unix
    - Fix os-error 10022 on Windows by changing UdpTransport `send()` to `send_to()`
```
