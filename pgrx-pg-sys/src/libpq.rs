/** Manually-constructed bindings to libpq

Because pgrx is for extensions which run in the Postgres server, it rarely needs access to libpq.
However, some server-side extensions need to interact with the reality that clients exist.
Unfortunately, doing that means acknowledging that clients need authentication and authorization,
areas of concern that are far beyond what pgrx wants to involve itself with or be responsible for.

We define some types and signatures here to allow a minimal amount of usage of items from libpq,
while largely rejecting the notion that we should involve ourselves in security-laden concerns.

*/

pub mod be {

    unsafe extern "C" {
        pub static mut MyProcPort: *mut Port;
    }

    /// #define SCRAM_MAX_KEY_LEN          PG_SHA256_DIGEST_LENGTH
    /// #define PG_SHA256_DIGEST_LENGTH    32
    const SCRAM_MAX_KEY_LEN: usize = 32;

    #[repr(C)]
    pub struct Port {
        pub sock: crate::pgsocket,
        pub noblock: bool,
        pub proto: crate::ProtocolVersion,
        pub laddr: crate::SockAddr,
        pub raddr: crate::SockAddr,
        pub remote_host: *mut core::ffi::c_char,
        pub remote_hostname: *mut core::ffi::c_char,
        pub remote_hostname_resolv: core::ffi::c_int,
        pub remote_hostname_errcode: core::ffi::c_int,
        pub remote_port: *mut core::ffi::c_char,
        #[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15", feature = "pg16"))]
        pub canAcceptConnections: core::ffi::c_uint,
        #[cfg(feature = "pg18")]
        pub local_host: [core::ffi::c_char; 64],
        pub database_name: *mut core::ffi::c_char,
        pub user_name: *mut core::ffi::c_char,
        pub cmdline_options: *mut core::ffi::c_char,
        pub guc_options: *mut core::ffi::c_char,
        pub application_name: *mut core::ffi::c_char,

        // The remainder is for completeness, so Rust sees Port's layout as correctly as possible.
        // Ideally we would use `extern type` so the remainder of this was seen as of unknown size.
        // An alternative is to simply treat them as private fields, so we do.
        HbaLine: *mut core::ffi::c_void,

        #[cfg(any(feature = "pg14", feature = "pg15"))]
        authn_id: *const core::ffi::c_char,

        default_keepalives_idle: core::ffi::c_int,
        default_keepalives_interval: core::ffi::c_int,
        default_keepalives_count: core::ffi::c_int,
        default_tcp_user_timeout: core::ffi::c_int,
        keepalives_idle: core::ffi::c_int,
        keepalives_interval: core::ffi::c_int,
        keepalives_count: core::ffi::c_int,
        tcp_user_timeout: core::ffi::c_int,

        #[cfg(feature = "pg18")]
        scram_ClientKey: [u8; SCRAM_MAX_KEY_LEN],
        #[cfg(feature = "pg18")]
        scram_ServerKey: [u8; SCRAM_MAX_KEY_LEN],
        #[cfg(feature = "pg18")]
        has_scram_keys: bool,

        // as if ENABLE_GSS == false && ENABLE_SSPI == false
        gss: *mut core::ffi::c_void,

        ssl_in_use: bool,
        peer_cn: *mut core::ffi::c_char,
        #[cfg(any(feature = "pg14", feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
        peer_dn: *mut core::ffi::c_char,
        peer_cert_valid: bool,

        #[cfg(any(feature = "pg17", feature = "pg18"))]
        alpn_used: bool,
        #[cfg(feature = "pg18")]
        last_read_was_eof: bool,

        // NOTE: 5 fields remain on PG17, but two are `#ifdef USE_OPENSSL` in PG17, so treat all
        // as conditioned on PG18, even if that is not strictly accurate for PG17

        // as if USE_OPENSSL == false && ENABLE_SSPI == false
        #[cfg(feature = "pg18")]
        ssl: *mut core::ffi::c_void,
        #[cfg(feature = "pg18")]
        peer: *mut core::ffi::c_void,

        #[cfg(feature = "pg18")]
        raw_buf: *mut core::ffi::c_char,
        #[cfg(feature = "pg18")]
        raw_buf_consumed: isize,
        #[cfg(feature = "pg18")]
        raw_buf_remaining: isize,
    }
}
