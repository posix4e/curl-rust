#![allow(non_camel_case_types, raw_pointer_derive)]

extern crate libc;
#[cfg(not(target_env = "msvc"))] extern crate libz_sys;
#[cfg(unix)] extern crate openssl_sys;

use libc::{c_void, c_int, c_char, c_uint, c_long};

pub type CURLINFO = c_int;
pub type CURL = c_void;
pub type curl_slist = c_void;
pub type CURLoption = c_int;
pub type CURLMoption = c_int;
pub type C = c_int;

#[repr(C)]
#[derive(Clone, Copy)]
pub enum CURLversion {
    CURL_VERSION_FIRST,
    CURL_VERSION_SECOND,
    CURL_VERSION_THIRD,
    CURL_VERSION_FOURTH,
    CURL_VERSION_LAST /* never actually use this */
}

#[repr(C)]
#[derive(Copy)]
pub struct curl_version_info_data {
    pub age: CURLversion,

    pub version: *const c_char,
    pub version_num: c_uint,
    pub host: *const c_char,
    pub features: c_int,
    pub ssl_version: *const c_char,

    pub ssl_version_num: c_long,

    pub libz_version: *const c_char,

    /* protocols is terminated by an entry with a NULL protoname */
    pub protocols: *const *const c_char,

    /* The fields below this were added in CURL_VERSION_SECOND */
    pub ares: *const c_char,
    pub ares_num: c_int,

    /* This field was added in CURL_VERSION_THIRD */
    pub libidn: *const c_char,

    /* These field were added in CURL_VERSION_FOURTH */
    pub iconv_ver_num: c_int,
    pub libssh_version: *const c_char,
}

impl Clone for curl_version_info_data {
    fn clone(&self) -> Self { *self }
}

pub const CURL_READFUNC_ABORT: c_int = 0x10000000;

pub const CURLINFO_STRING: c_int   = 0x100000;
pub const CURLINFO_LONG: c_int     = 0x200000;
pub const CURLINFO_DOUBLE: c_int   = 0x300000;
pub const CURLINFO_SLIST:  c_int   = 0x400000;
pub const CURLINFO_MASK: c_int     = 0x0fffff;
pub const CURLINFO_TYPEMASK: c_int = 0xf00000;

pub const CURLINFO_EFFECTIVE_URL: CURLINFO = CURLINFO_STRING + 1;
pub const CURLINFO_RESPONSE_CODE: CURLINFO = CURLINFO_LONG + 2;
pub const CURLINFO_TOTAL_TIME: CURLINFO = CURLINFO_DOUBLE + 5;

pub const CURLOPTTYPE_LONG: c_int          = 0;
pub const CURLOPTTYPE_OBJECTPOINT: c_int   = 10_000;
pub const CURLOPTTYPE_FUNCTIONPOINT: c_int = 20_000;
pub const CURLOPTTYPE_OFF_T: c_int         = 30_000;

pub const CURL_VERSION_NOW: CURLversion    = CURLversion::CURL_VERSION_FOURTH;
pub const CURL_VERSION_IPV6:         c_int = (1 << 0);
pub const CURL_VERSION_KERBEROS4:    c_int = (1 << 1);
pub const CURL_VERSION_SSL:          c_int = (1 << 2);
pub const CURL_VERSION_LIBZ:         c_int = (1 << 3);
pub const CURL_VERSION_NTLM:         c_int = (1 << 4);
pub const CURL_VERSION_GSSNEGOTIATE: c_int = (1 << 5);
pub const CURL_VERSION_DEBUG:        c_int = (1 << 6);
pub const CURL_VERSION_ASYNCHDNS:    c_int = (1 << 7);
pub const CURL_VERSION_SPNEGO:       c_int = (1 << 8);
pub const CURL_VERSION_LARGEFILE:    c_int = (1 << 9);
pub const CURL_VERSION_IDN:          c_int = (1 << 10);
pub const CURL_VERSION_SSPI:         c_int = (1 << 11);
pub const CURL_VERSION_CONV:         c_int = (1 << 12);
pub const CURL_VERSION_CURLDEBUG:    c_int = (1 << 13);
pub const CURL_VERSION_TLSAUTH_SRP:  c_int = (1 << 14);
pub const CURL_VERSION_NTLM_WB:      c_int = (1 << 15);
pub const CURL_VERSION_HTTP2:        c_int = (1 << 16);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum CURLcode {
    CURLE_OK = 0,
    CURLE_UNSUPPORTED_PROTOCOL,    /* 1 */
    CURLE_FAILED_INIT,             /* 2 */
    CURLE_URL_MALFORMAT,           /* 3 */
    CURLE_NOT_BUILT_IN,            /* 4 - [was obsoleted in August 2007 for
                                7.17.0, reused in April 2011 for 7.21.5] */
    CURLE_COULDNT_RESOLVE_PROXY,   /* 5 */
    CURLE_COULDNT_RESOLVE_HOST,    /* 6 */
    CURLE_COULDNT_CONNECT,         /* 7 */
    CURLE_FTP_WEIRD_SERVER_REPLY,  /* 8 */
    CURLE_REMOTE_ACCESS_DENIED,    /* 9 a service was denied by the server
                                due to lack of access - when login fails
                                this is not returned. */
    CURLE_FTP_ACCEPT_FAILED,       /* 10 - [was obsoleted in April 2006 for
                                7.15.4, reused in Dec 2011 for 7.24.0]*/
    CURLE_FTP_WEIRD_PASS_REPLY,    /* 11 */
    CURLE_FTP_ACCEPT_TIMEOUT,      /* 12 - timeout occurred accepting server
                                [was obsoleted in August 2007 for 7.17.0,
                                reused in Dec 2011 for 7.24.0]*/
    CURLE_FTP_WEIRD_PASV_REPLY,    /* 13 */
    CURLE_FTP_WEIRD_227_FORMAT,    /* 14 */
    CURLE_FTP_CANT_GET_HOST,       /* 15 */
    CURLE_OBSOLETE16,              /* 16 - NOT USED */
    CURLE_FTP_COULDNT_SET_TYPE,    /* 17 */
    CURLE_PARTIAL_FILE,            /* 18 */
    CURLE_FTP_COULDNT_RETR_FILE,   /* 19 */
    CURLE_OBSOLETE20,              /* 20 - NOT USED */
    CURLE_QUOTE_ERROR,             /* 21 - quote command failure */
    CURLE_HTTP_RETURNED_ERROR,     /* 22 */
    CURLE_WRITE_ERROR,             /* 23 */
    CURLE_OBSOLETE24,              /* 24 - NOT USED */
    CURLE_UPLOAD_FAILED,           /* 25 - failed upload "command" */
    CURLE_READ_ERROR,              /* 26 - couldn't open/read from file */
    CURLE_OUT_OF_MEMORY,           /* 27 */
    /* Note: CURLE_OUT_OF_MEMORY may sometimes indicate a conversion error
       instead of a memory allocation error if CURL_DOES_CONVERSIONS
       is defined
       */
    CURLE_OPERATION_TIMEDOUT,      /* 28 - the timeout time was reached */
    CURLE_OBSOLETE29,              /* 29 - NOT USED */
    CURLE_FTP_PORT_FAILED,         /* 30 - FTP PORT operation failed */
    CURLE_FTP_COULDNT_USE_REST,    /* 31 - the REST command failed */
    CURLE_OBSOLETE32,              /* 32 - NOT USED */
    CURLE_RANGE_ERROR,             /* 33 - RANGE "command" didn't work */
    CURLE_HTTP_POST_ERROR,         /* 34 */
    CURLE_SSL_CONNECT_ERROR,       /* 35 - wrong when connecting with SSL */
    CURLE_BAD_DOWNLOAD_RESUME,     /* 36 - couldn't resume download */
    CURLE_FILE_COULDNT_READ_FILE,  /* 37 */
    CURLE_LDAP_CANNOT_BIND,        /* 38 */
    CURLE_LDAP_SEARCH_FAILED,      /* 39 */
    CURLE_OBSOLETE40,              /* 40 - NOT USED */
    CURLE_FUNCTION_NOT_FOUND,      /* 41 */
    CURLE_ABORTED_BY_CALLBACK,     /* 42 */
    CURLE_BAD_FUNCTION_ARGUMENT,   /* 43 */
    CURLE_OBSOLETE44,              /* 44 - NOT USED */
    CURLE_INTERFACE_FAILED,        /* 45 - CURLOPT_INTERFACE failed */
    CURLE_OBSOLETE46,              /* 46 - NOT USED */
    CURLE_TOO_MANY_REDIRECTS ,     /* 47 - catch endless re-direct loops */
    CURLE_UNKNOWN_OPTION,          /* 48 - User specified an unknown option */
    CURLE_TELNET_OPTION_SYNTAX ,   /* 49 - Malformed telnet option */
    CURLE_OBSOLETE50,              /* 50 - NOT USED */
    CURLE_PEER_FAILED_VERIFICATION, /* 51 - peer's certificate or fingerprint
                                 wasn't verified fine */
    CURLE_GOT_NOTHING,             /* 52 - when this is a specific error */
    CURLE_SSL_ENGINE_NOTFOUND,     /* 53 - SSL crypto engine not found */
    CURLE_SSL_ENGINE_SETFAILED,    /* 54 - can not set SSL crypto engine as
                                default */
    CURLE_SEND_ERROR,              /* 55 - failed sending network data */
    CURLE_RECV_ERROR,              /* 56 - failure in receiving network data */
    CURLE_OBSOLETE57,              /* 57 - NOT IN USE */
    CURLE_SSL_CERTPROBLEM,         /* 58 - problem with the local certificate */
    CURLE_SSL_CIPHER,              /* 59 - couldn't use specified cipher */
    CURLE_SSL_CACERT,              /* 60 - problem with the CA cert (path?) */
    CURLE_BAD_CONTENT_ENCODING,    /* 61 - Unrecognized/bad encoding */
    CURLE_LDAP_INVALID_URL,        /* 62 - Invalid LDAP URL */
    CURLE_FILESIZE_EXCEEDED,       /* 63 - Maximum file size exceeded */
    CURLE_USE_SSL_FAILED,          /* 64 - Requested FTP SSL level failed */
    CURLE_SEND_FAIL_REWIND,        /* 65 - Sending the data requires a rewind
                                that failed */
    CURLE_SSL_ENGINE_INITFAILED,   /* 66 - failed to initialise ENGINE */
    CURLE_LOGIN_DENIED,            /* 67 - user, password or similar was not
                                accepted and we failed to login */
    CURLE_TFTP_NOTFOUND,           /* 68 - file not found on server */
    CURLE_TFTP_PERM,               /* 69 - permission problem on server */
    CURLE_REMOTE_DISK_FULL,        /* 70 - out of disk space on server */
    CURLE_TFTP_ILLEGAL,            /* 71 - Illegal TFTP operation */
    CURLE_TFTP_UNKNOWNID,          /* 72 - Unknown transfer ID */
    CURLE_REMOTE_FILE_EXISTS,      /* 73 - File already exists */
    CURLE_TFTP_NOSUCHUSER,         /* 74 - No such user */
    CURLE_CONV_FAILED,             /* 75 - conversion failed */
    CURLE_CONV_REQD,               /* 76 - caller must register conversion
                                callbacks using curl_easy_setopt options
                                CURLOPT_CONV_FROM_NETWORK_FUNCTION,
                                CURLOPT_CONV_TO_NETWORK_FUNCTION, and
                                CURLOPT_CONV_FROM_UTF8_FUNCTION */
    CURLE_SSL_CACERT_BADFILE,      /* 77 - could not load CACERT file, missing
                                or wrong format */
    CURLE_REMOTE_FILE_NOT_FOUND,   /* 78 - remote file not found */
    CURLE_SSH,                     /* 79 - error from the SSH layer, somewhat
                                generic so the error message will be of
                                interest when this has happened */

    CURLE_SSL_SHUTDOWN_FAILED,     /* 80 - Failed to shut down the SSL
                                connection */
    CURLE_AGAIN,                   /* 81 - socket is not ready for send/recv,
                                wait till it's ready and try again (Added
                                in 7.18.2) */
    CURLE_SSL_CRL_BADFILE,         /* 82 - could not load CRL file, missing or
                                wrong format (Added in 7.19.0) */
    CURLE_SSL_ISSUER_ERROR,        /* 83 - Issuer check failed.  (Added in
                                7.19.0) */
    CURLE_FTP_PRET_FAILED,         /* 84 - a PRET command failed */
    CURLE_RTSP_CSEQ_ERROR,         /* 85 - mismatch of RTSP CSeq numbers */
    CURLE_RTSP_SESSION_ERROR,      /* 86 - mismatch of RTSP Session Ids */
    CURLE_FTP_BAD_FILE_LIST,       /* 87 - unable to parse FTP file list */
    CURLE_CHUNK_FAILED,            /* 88 - chunk callback reported error */
    CURLE_NO_CONNECTION_AVAILABLE, /* 89 - No connection available, the
                                session will be queued */
    CURLE_LAST /* never use! */
}

macro_rules! DEFOPT {
    ($name:ident, $ty:ident, $num:expr) => (
        #[allow(dead_code)]
        pub const $name: CURLoption = $ty + $num;
    )
}

macro_rules! ALIAS {
    ($name:ident, $to:ident) => (
        #[allow(dead_code)]
        pub const $name: CURLoption = $to;
    )
}

DEFOPT!(CURLOPT_FILE,                   CURLOPTTYPE_OBJECTPOINT,     1);
DEFOPT!(CURLOPT_URL,                    CURLOPTTYPE_OBJECTPOINT,     2);
DEFOPT!(CURLOPT_PORT,                   CURLOPTTYPE_LONG,            3);
DEFOPT!(CURLOPT_PROXY,                  CURLOPTTYPE_OBJECTPOINT,     4);
DEFOPT!(CURLOPT_USERPWD,                CURLOPTTYPE_OBJECTPOINT,     5);
DEFOPT!(CURLOPT_PROXYUSERPWD,           CURLOPTTYPE_OBJECTPOINT,     6);
DEFOPT!(CURLOPT_RANGE,                  CURLOPTTYPE_OBJECTPOINT,     7);
/* 8: not used */
DEFOPT!(CURLOPT_INFILE,                 CURLOPTTYPE_OBJECTPOINT,     9);
DEFOPT!(CURLOPT_ERRORBUFFER,            CURLOPTTYPE_OBJECTPOINT,    10);
DEFOPT!(CURLOPT_WRITEFUNCTION,          CURLOPTTYPE_FUNCTIONPOINT,  11);
DEFOPT!(CURLOPT_READFUNCTION,           CURLOPTTYPE_FUNCTIONPOINT,  12);
DEFOPT!(CURLOPT_TIMEOUT,                CURLOPTTYPE_LONG,           13);
DEFOPT!(CURLOPT_INFILESIZE,             CURLOPTTYPE_LONG,           14);
DEFOPT!(CURLOPT_POSTFIELDS,             CURLOPTTYPE_OBJECTPOINT,    15);
DEFOPT!(CURLOPT_REFERER,                CURLOPTTYPE_OBJECTPOINT,    16);
DEFOPT!(CURLOPT_FTPPORT,                CURLOPTTYPE_OBJECTPOINT,    17);
DEFOPT!(CURLOPT_USERAGENT,              CURLOPTTYPE_OBJECTPOINT,    18);
DEFOPT!(CURLOPT_LOW_SPEED_LIMIT,        CURLOPTTYPE_LONG,           19);
DEFOPT!(CURLOPT_LOW_SPEED_TIME,         CURLOPTTYPE_LONG,           20);
DEFOPT!(CURLOPT_RESUME_FROM,            CURLOPTTYPE_LONG,           21);
DEFOPT!(CURLOPT_COOKIE,                 CURLOPTTYPE_OBJECTPOINT,    22);
DEFOPT!(CURLOPT_HTTPHEADER,             CURLOPTTYPE_OBJECTPOINT,    23);
DEFOPT!(CURLOPT_HTTPPOST,               CURLOPTTYPE_OBJECTPOINT,    24);
DEFOPT!(CURLOPT_SSLCERT,                CURLOPTTYPE_OBJECTPOINT,    25);
DEFOPT!(CURLOPT_KEYPASSWD,              CURLOPTTYPE_OBJECTPOINT,    26);
DEFOPT!(CURLOPT_CRLF,                   CURLOPTTYPE_LONG,           27);
DEFOPT!(CURLOPT_QUOTE,                  CURLOPTTYPE_OBJECTPOINT,    28);
DEFOPT!(CURLOPT_WRITEHEADER,            CURLOPTTYPE_OBJECTPOINT,    29);
/* 30: not used */
DEFOPT!(CURLOPT_COOKIEFILE,             CURLOPTTYPE_OBJECTPOINT,    31);
DEFOPT!(CURLOPT_SSLVERSION,             CURLOPTTYPE_LONG,           32);
DEFOPT!(CURLOPT_TIMECONDITION,          CURLOPTTYPE_LONG,           33);
DEFOPT!(CURLOPT_TIMEVALUE,              CURLOPTTYPE_LONG,           34);
/* 35: not used */
DEFOPT!(CURLOPT_CUSTOMREQUEST,          CURLOPTTYPE_OBJECTPOINT,    36);
DEFOPT!(CURLOPT_STDERR,                 CURLOPTTYPE_OBJECTPOINT,    37);
/* 38: not used */
DEFOPT!(CURLOPT_POSTQUOTE,              CURLOPTTYPE_OBJECTPOINT,    39);
DEFOPT!(CURLOPT_WRITEINFO,              CURLOPTTYPE_OBJECTPOINT,    40);
DEFOPT!(CURLOPT_VERBOSE,                CURLOPTTYPE_LONG,           41);
DEFOPT!(CURLOPT_HEADER,                 CURLOPTTYPE_LONG,           42);
DEFOPT!(CURLOPT_NOPROGRESS,             CURLOPTTYPE_LONG,           43);
DEFOPT!(CURLOPT_NOBODY,                 CURLOPTTYPE_LONG,           44);
DEFOPT!(CURLOPT_FAILONERROR,            CURLOPTTYPE_LONG,           45);
DEFOPT!(CURLOPT_UPLOAD,                 CURLOPTTYPE_LONG,           46);
DEFOPT!(CURLOPT_POST,                   CURLOPTTYPE_LONG,           47);
DEFOPT!(CURLOPT_DIRLISTONLY,            CURLOPTTYPE_LONG,           48);
DEFOPT!(CURLOPT_APPEND,                 CURLOPTTYPE_LONG,           50);
DEFOPT!(CURLOPT_NETRC,                  CURLOPTTYPE_LONG,           51);
DEFOPT!(CURLOPT_FOLLOWLOCATION,         CURLOPTTYPE_LONG,           52);
DEFOPT!(CURLOPT_TRANSFERTEXT,           CURLOPTTYPE_LONG,           53);
DEFOPT!(CURLOPT_PUT,                    CURLOPTTYPE_LONG,           54);
/* 55: not used */
DEFOPT!(CURLOPT_PROGRESSFUNCTION,       CURLOPTTYPE_FUNCTIONPOINT,  56);
DEFOPT!(CURLOPT_PROGRESSDATA,           CURLOPTTYPE_OBJECTPOINT,    57);
DEFOPT!(CURLOPT_AUTOREFERER,            CURLOPTTYPE_LONG,           58);
DEFOPT!(CURLOPT_PROXYPORT,              CURLOPTTYPE_LONG,           59);
DEFOPT!(CURLOPT_POSTFIELDSIZE,          CURLOPTTYPE_LONG,           60);
DEFOPT!(CURLOPT_HTTPPROXYTUNNEL,        CURLOPTTYPE_LONG,           61);
DEFOPT!(CURLOPT_INTERFACE,              CURLOPTTYPE_OBJECTPOINT,    62);
DEFOPT!(CURLOPT_KRBLEVEL,               CURLOPTTYPE_OBJECTPOINT,    63);
DEFOPT!(CURLOPT_SSL_VERIFYPEER,         CURLOPTTYPE_LONG,           64);
DEFOPT!(CURLOPT_CAINFO,                 CURLOPTTYPE_OBJECTPOINT,    65);
/* 66: not used */
/* 67: not used */
DEFOPT!(CURLOPT_MAXREDIRS,                  CURLOPTTYPE_LONG,           68);
DEFOPT!(CURLOPT_FILETIME,                   CURLOPTTYPE_LONG,           69);
DEFOPT!(CURLOPT_TELNETOPTIONS,              CURLOPTTYPE_OBJECTPOINT,    70);
DEFOPT!(CURLOPT_MAXCONNECTS,                CURLOPTTYPE_LONG,           71);
DEFOPT!(CURLOPT_CLOSEPOLICY,                CURLOPTTYPE_LONG,           72);
/* 73: not used */
DEFOPT!(CURLOPT_FRESH_CONNECT,              CURLOPTTYPE_LONG,           74);
DEFOPT!(CURLOPT_FORBID_REUSE,               CURLOPTTYPE_LONG,           75);
DEFOPT!(CURLOPT_RANDOM_FILE,                CURLOPTTYPE_OBJECTPOINT,    76);
DEFOPT!(CURLOPT_EGDSOCKET,                  CURLOPTTYPE_OBJECTPOINT,    77);
DEFOPT!(CURLOPT_CONNECTTIMEOUT,             CURLOPTTYPE_LONG,           78);
DEFOPT!(CURLOPT_HEADERFUNCTION,             CURLOPTTYPE_FUNCTIONPOINT,  79);
DEFOPT!(CURLOPT_HTTPGET,                    CURLOPTTYPE_LONG,           80);
DEFOPT!(CURLOPT_SSL_VERIFYHOST,             CURLOPTTYPE_LONG,           81);
DEFOPT!(CURLOPT_COOKIEJAR,                  CURLOPTTYPE_OBJECTPOINT,    82);
DEFOPT!(CURLOPT_SSL_CIPHER_LIST,            CURLOPTTYPE_OBJECTPOINT,    83);
DEFOPT!(CURLOPT_HTTP_VERSION,               CURLOPTTYPE_LONG,           84);
DEFOPT!(CURLOPT_FTP_USE_EPSV,               CURLOPTTYPE_LONG,           85);
DEFOPT!(CURLOPT_SSLCERTTYPE,                CURLOPTTYPE_OBJECTPOINT,    86);
DEFOPT!(CURLOPT_SSLKEY,                     CURLOPTTYPE_OBJECTPOINT,    87);
DEFOPT!(CURLOPT_SSLKEYTYPE,                 CURLOPTTYPE_OBJECTPOINT,    88);
DEFOPT!(CURLOPT_SSLENGINE,                  CURLOPTTYPE_OBJECTPOINT,    89);
DEFOPT!(CURLOPT_SSLENGINE_DEFAULT,          CURLOPTTYPE_LONG,           90);
DEFOPT!(CURLOPT_DNS_USE_GLOBAL_CACHE,       CURLOPTTYPE_LONG,           91);
DEFOPT!(CURLOPT_DNS_CACHE_TIMEOUT,          CURLOPTTYPE_LONG,           92);
DEFOPT!(CURLOPT_PREQUOTE,                   CURLOPTTYPE_OBJECTPOINT,    93);
DEFOPT!(CURLOPT_DEBUGFUNCTION,              CURLOPTTYPE_FUNCTIONPOINT,  94);
DEFOPT!(CURLOPT_DEBUGDATA,                  CURLOPTTYPE_OBJECTPOINT,    95);
DEFOPT!(CURLOPT_COOKIESESSION,              CURLOPTTYPE_LONG,           96);
DEFOPT!(CURLOPT_CAPATH,                     CURLOPTTYPE_OBJECTPOINT,    97);
DEFOPT!(CURLOPT_BUFFERSIZE,                 CURLOPTTYPE_LONG,           98);
DEFOPT!(CURLOPT_NOSIGNAL,                   CURLOPTTYPE_LONG,           99);
DEFOPT!(CURLOPT_SHARE,                      CURLOPTTYPE_OBJECTPOINT,   100);
DEFOPT!(CURLOPT_PROXYTYPE,                  CURLOPTTYPE_LONG,          101);
DEFOPT!(CURLOPT_ACCEPT_ENCODING,            CURLOPTTYPE_OBJECTPOINT,   102);
DEFOPT!(CURLOPT_PRIVATE,                    CURLOPTTYPE_OBJECTPOINT,   103);
DEFOPT!(CURLOPT_HTTP200ALIASES,             CURLOPTTYPE_OBJECTPOINT,   104);
DEFOPT!(CURLOPT_UNRESTRICTED_AUTH,          CURLOPTTYPE_LONG,          105);
DEFOPT!(CURLOPT_FTP_USE_EPRT,               CURLOPTTYPE_LONG,          106);
DEFOPT!(CURLOPT_HTTPAUTH,                   CURLOPTTYPE_LONG,          107);
DEFOPT!(CURLOPT_SSL_CTX_FUNCTION,           CURLOPTTYPE_FUNCTIONPOINT, 108);
DEFOPT!(CURLOPT_SSL_CTX_DATA,               CURLOPTTYPE_OBJECTPOINT,   109);
DEFOPT!(CURLOPT_FTP_CREATE_MISSING_DIRS,    CURLOPTTYPE_LONG,          110);
DEFOPT!(CURLOPT_PROXYAUTH,                  CURLOPTTYPE_LONG,          111);
DEFOPT!(CURLOPT_FTP_RESPONSE_TIMEOUT,       CURLOPTTYPE_LONG,          112);
DEFOPT!(CURLOPT_IPRESOLVE,                  CURLOPTTYPE_LONG,          113);
DEFOPT!(CURLOPT_MAXFILESIZE,                CURLOPTTYPE_LONG,          114);
DEFOPT!(CURLOPT_INFILESIZE_LARGE,           CURLOPTTYPE_OFF_T,         115);
DEFOPT!(CURLOPT_RESUME_FROM_LARGE,          CURLOPTTYPE_OFF_T,         116);
DEFOPT!(CURLOPT_MAXFILESIZE_LARGE,          CURLOPTTYPE_OFF_T,         117);
DEFOPT!(CURLOPT_NETRC_FILE,                 CURLOPTTYPE_OBJECTPOINT,   118);
DEFOPT!(CURLOPT_USE_SSL,                    CURLOPTTYPE_LONG,          119);
DEFOPT!(CURLOPT_POSTFIELDSIZE_LARGE,        CURLOPTTYPE_OFF_T,         120);
DEFOPT!(CURLOPT_TCP_NODELAY,                CURLOPTTYPE_LONG,          121);
/* 122 - 128: not used */
DEFOPT!(CURLOPT_FTPSSLAUTH,                 CURLOPTTYPE_LONG,          129);
DEFOPT!(CURLOPT_IOCTLFUNCTION,              CURLOPTTYPE_FUNCTIONPOINT, 130);
DEFOPT!(CURLOPT_IOCTLDATA,                  CURLOPTTYPE_OBJECTPOINT,   131);
/* 132, CURLOPTTYPE_133: not used */
DEFOPT!(CURLOPT_FTP_ACCOUNT,                CURLOPTTYPE_OBJECTPOINT,   134);
DEFOPT!(CURLOPT_COOKIELIST,                 CURLOPTTYPE_OBJECTPOINT,   135);
DEFOPT!(CURLOPT_IGNORE_CONTENT_LENGTH,      CURLOPTTYPE_LONG,          136);
DEFOPT!(CURLOPT_FTP_SKIP_PASV_IP,           CURLOPTTYPE_LONG,          137);
DEFOPT!(CURLOPT_FTP_FILEMETHOD,             CURLOPTTYPE_LONG,          138);
DEFOPT!(CURLOPT_LOCALPORT,                  CURLOPTTYPE_LONG,          139);
DEFOPT!(CURLOPT_LOCALPORTRANGE,             CURLOPTTYPE_LONG,          140);
DEFOPT!(CURLOPT_CONNECT_ONLY,               CURLOPTTYPE_LONG,          141);
DEFOPT!(CURLOPT_CONV_FROM_NETWORK_FUNCTION, CURLOPTTYPE_FUNCTIONPOINT, 142);
DEFOPT!(CURLOPT_CONV_TO_NETWORK_FUNCTION,   CURLOPTTYPE_FUNCTIONPOINT, 143);
DEFOPT!(CURLOPT_CONV_FROM_UTF8_FUNCTION,    CURLOPTTYPE_FUNCTIONPOINT, 144);
DEFOPT!(CURLOPT_MAX_SEND_SPEED_LARGE,       CURLOPTTYPE_OFF_T,         145);
DEFOPT!(CURLOPT_MAX_RECV_SPEED_LARGE,       CURLOPTTYPE_OFF_T,         146);
DEFOPT!(CURLOPT_FTP_ALTERNATIVE_TO_USER,    CURLOPTTYPE_OBJECTPOINT,   147);
DEFOPT!(CURLOPT_SOCKOPTFUNCTION,            CURLOPTTYPE_FUNCTIONPOINT, 148);
DEFOPT!(CURLOPT_SOCKOPTDATA,                CURLOPTTYPE_OBJECTPOINT,   149);
DEFOPT!(CURLOPT_SSL_SESSIONID_CACHE,        CURLOPTTYPE_LONG,          150);
DEFOPT!(CURLOPT_SSH_AUTH_TYPES,             CURLOPTTYPE_LONG,          151);
DEFOPT!(CURLOPT_SSH_PUBLIC_KEYFILE,         CURLOPTTYPE_OBJECTPOINT,   152);
DEFOPT!(CURLOPT_SSH_PRIVATE_KEYFILE,        CURLOPTTYPE_OBJECTPOINT,   153);
DEFOPT!(CURLOPT_FTP_SSL_CCC,                CURLOPTTYPE_LONG,          154);
DEFOPT!(CURLOPT_TIMEOUT_MS,                 CURLOPTTYPE_LONG,          155);
DEFOPT!(CURLOPT_CONNECTTIMEOUT_MS,          CURLOPTTYPE_LONG,          156);
DEFOPT!(CURLOPT_HTTP_TRANSFER_DECODING,     CURLOPTTYPE_LONG,          157);
DEFOPT!(CURLOPT_HTTP_CONTENT_DECODING,      CURLOPTTYPE_LONG,          158);
DEFOPT!(CURLOPT_NEW_FILE_PERMS,             CURLOPTTYPE_LONG,          159);
DEFOPT!(CURLOPT_NEW_DIRECTORY_PERMS,        CURLOPTTYPE_LONG,          160);
DEFOPT!(CURLOPT_POSTREDIR,                  CURLOPTTYPE_LONG,          161);
DEFOPT!(CURLOPT_SSH_HOST_PUBLIC_KEY_MD5,    CURLOPTTYPE_OBJECTPOINT,   162);
DEFOPT!(CURLOPT_OPENSOCKETFUNCTION,         CURLOPTTYPE_FUNCTIONPOINT, 163);
DEFOPT!(CURLOPT_OPENSOCKETDATA,             CURLOPTTYPE_OBJECTPOINT,   164);
DEFOPT!(CURLOPT_COPYPOSTFIELDS,             CURLOPTTYPE_OBJECTPOINT,   165);
DEFOPT!(CURLOPT_PROXY_TRANSFER_MODE,        CURLOPTTYPE_LONG,          166);
DEFOPT!(CURLOPT_SEEKFUNCTION,               CURLOPTTYPE_FUNCTIONPOINT, 167);
DEFOPT!(CURLOPT_SEEKDATA,                   CURLOPTTYPE_OBJECTPOINT,   168);
DEFOPT!(CURLOPT_CRLFILE,                    CURLOPTTYPE_OBJECTPOINT,   169);
DEFOPT!(CURLOPT_ISSUERCERT,                 CURLOPTTYPE_OBJECTPOINT,   170);
DEFOPT!(CURLOPT_ADDRESS_SCOPE,              CURLOPTTYPE_LONG,          171);
DEFOPT!(CURLOPT_CERTINFO,                   CURLOPTTYPE_LONG,          172);
DEFOPT!(CURLOPT_USERNAME,                   CURLOPTTYPE_OBJECTPOINT,   173);
DEFOPT!(CURLOPT_PASSWORD,                   CURLOPTTYPE_OBJECTPOINT,   174);
DEFOPT!(CURLOPT_PROXYUSERNAME,              CURLOPTTYPE_OBJECTPOINT,   175);
DEFOPT!(CURLOPT_PROXYPASSWORD,              CURLOPTTYPE_OBJECTPOINT,   176);
DEFOPT!(CURLOPT_NOPROXY,                    CURLOPTTYPE_OBJECTPOINT,   177);
DEFOPT!(CURLOPT_TFTP_BLKSIZE,               CURLOPTTYPE_LONG,          178);
DEFOPT!(CURLOPT_SOCKS5_GSSAPI_SERVICE,      CURLOPTTYPE_OBJECTPOINT,   179);
DEFOPT!(CURLOPT_SOCKS5_GSSAPI_NEC,          CURLOPTTYPE_LONG,          180);
DEFOPT!(CURLOPT_PROTOCOLS,                  CURLOPTTYPE_LONG,          181);
DEFOPT!(CURLOPT_REDIR_PROTOCOLS,            CURLOPTTYPE_LONG,          182);
DEFOPT!(CURLOPT_SSH_KNOWNHOSTS,             CURLOPTTYPE_OBJECTPOINT,   183);
DEFOPT!(CURLOPT_SSH_KEYFUNCTION,            CURLOPTTYPE_FUNCTIONPOINT, 184);
DEFOPT!(CURLOPT_SSH_KEYDATA,                CURLOPTTYPE_OBJECTPOINT,   185);
DEFOPT!(CURLOPT_MAIL_FROM,                  CURLOPTTYPE_OBJECTPOINT,   186);
DEFOPT!(CURLOPT_MAIL_RCPT,                  CURLOPTTYPE_OBJECTPOINT,   187);
DEFOPT!(CURLOPT_FTP_USE_PRET,               CURLOPTTYPE_LONG,          188);
DEFOPT!(CURLOPT_RTSP_REQUEST,               CURLOPTTYPE_LONG,          189);
DEFOPT!(CURLOPT_RTSP_SESSION_ID,            CURLOPTTYPE_OBJECTPOINT,   190);
DEFOPT!(CURLOPT_RTSP_STREAM_URI,            CURLOPTTYPE_OBJECTPOINT,   191);
DEFOPT!(CURLOPT_RTSP_TRANSPORT,             CURLOPTTYPE_OBJECTPOINT,   192);
DEFOPT!(CURLOPT_RTSP_CLIENT_CSEQ,           CURLOPTTYPE_LONG,          193);
DEFOPT!(CURLOPT_RTSP_SERVER_CSEQ,           CURLOPTTYPE_LONG,          194);
DEFOPT!(CURLOPT_INTERLEAVEDATA,             CURLOPTTYPE_OBJECTPOINT,   195);
DEFOPT!(CURLOPT_INTERLEAVEFUNCTION,         CURLOPTTYPE_FUNCTIONPOINT, 196);
DEFOPT!(CURLOPT_WILDCARDMATCH,              CURLOPTTYPE_LONG,          197);
DEFOPT!(CURLOPT_CHUNK_BGN_FUNCTION,         CURLOPTTYPE_FUNCTIONPOINT, 198);
DEFOPT!(CURLOPT_CHUNK_END_FUNCTION,         CURLOPTTYPE_FUNCTIONPOINT, 199);
DEFOPT!(CURLOPT_FNMATCH_FUNCTION,           CURLOPTTYPE_FUNCTIONPOINT, 200);
DEFOPT!(CURLOPT_CHUNK_DATA,                 CURLOPTTYPE_OBJECTPOINT,   201);
DEFOPT!(CURLOPT_FNMATCH_DATA,               CURLOPTTYPE_OBJECTPOINT,   202);
DEFOPT!(CURLOPT_RESOLVE,                    CURLOPTTYPE_OBJECTPOINT,   203);
DEFOPT!(CURLOPT_TLSAUTH_USERNAME,           CURLOPTTYPE_OBJECTPOINT,   204);
DEFOPT!(CURLOPT_TLSAUTH_PASSWORD,           CURLOPTTYPE_OBJECTPOINT,   205);
DEFOPT!(CURLOPT_TLSAUTH_TYPE,               CURLOPTTYPE_OBJECTPOINT,   206);
DEFOPT!(CURLOPT_TRANSFER_ENCODING,          CURLOPTTYPE_LONG,          207);
DEFOPT!(CURLOPT_CLOSESOCKETFUNCTION,        CURLOPTTYPE_FUNCTIONPOINT, 208);
DEFOPT!(CURLOPT_CLOSESOCKETDATA,            CURLOPTTYPE_OBJECTPOINT,   209);
DEFOPT!(CURLOPT_GSSAPI_DELEGATION,          CURLOPTTYPE_LONG,          210);
DEFOPT!(CURLOPT_DNS_SERVERS,                CURLOPTTYPE_OBJECTPOINT,   211);
DEFOPT!(CURLOPT_ACCEPTTIMEOUT_MS,           CURLOPTTYPE_LONG,          212);
DEFOPT!(CURLOPT_TCP_KEEPALIVE,              CURLOPTTYPE_LONG,          213);
DEFOPT!(CURLOPT_TCP_KEEPIDLE,               CURLOPTTYPE_LONG,          214);
DEFOPT!(CURLOPT_TCP_KEEPINTVL,              CURLOPTTYPE_LONG,          215);
DEFOPT!(CURLOPT_SSL_OPTIONS,                CURLOPTTYPE_LONG,          216);
DEFOPT!(CURLOPT_MAIL_AUTH,                  CURLOPTTYPE_OBJECTPOINT,   217);
DEFOPT!(CURLOPT_SASL_IR,                    CURLOPTTYPE_LONG,          218);
DEFOPT!(CURLOPT_XFERINFOFUNCTION,           CURLOPTTYPE_FUNCTIONPOINT, 219);
DEFOPT!(CURLOPT_XOAUTH2_BEARER,             CURLOPTTYPE_OBJECTPOINT,   220);
DEFOPT!(CURLOPT_DNS_INTERFACE,              CURLOPTTYPE_OBJECTPOINT,   221);
DEFOPT!(CURLOPT_DNS_LOCAL_IP4,              CURLOPTTYPE_OBJECTPOINT,   222);
DEFOPT!(CURLOPT_DNS_LOCAL_IP6,              CURLOPTTYPE_OBJECTPOINT,   223);
DEFOPT!(CURLOPT_LOGIN_OPTIONS,              CURLOPTTYPE_OBJECTPOINT,   224);
DEFOPT!(CURLOPT_SSL_ENABLE_NPN,             CURLOPTTYPE_LONG,          225);
DEFOPT!(CURLOPT_SSL_ENABLE_ALPN,            CURLOPTTYPE_LONG,          226);
DEFOPT!(CURLOPT_EXPECT_100_TIMEOUT_MS,      CURLOPTTYPE_LONG,          227);
DEFOPT!(CURLOPT_PROXYHEADER,                CURLOPTTYPE_OBJECTPOINT,   228);
DEFOPT!(CURLOPT_HEADEROPT,                  CURLOPTTYPE_LONG,          229);

// Option aliases
ALIAS!(CURLOPT_POST301, CURLOPT_POSTREDIR);
ALIAS!(CURLOPT_SSLKEYPASSWD, CURLOPT_KEYPASSWD);
ALIAS!(CURLOPT_FTPAPPEND, CURLOPT_APPEND);
ALIAS!(CURLOPT_FTPLISTONLY, CURLOPT_DIRLISTONLY);
ALIAS!(CURLOPT_FTP_SSL, CURLOPT_USE_SSL);
ALIAS!(CURLOPT_SSLCERTPASSWD, CURLOPT_KEYPASSWD);
ALIAS!(CURLOPT_KRB4LEVEL, CURLOPT_KRBLEVEL);
ALIAS!(CURLOPT_READDATA, CURLOPT_INFILE);
ALIAS!(CURLOPT_WRITEDATA, CURLOPT_FILE);
ALIAS!(CURLOPT_HEADERDATA, CURLOPT_WRITEHEADER);
ALIAS!(CURLOPT_XFERINFODATA, CURLOPT_PROGRESSDATA);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum CURLMcode {
	CURLM_CALL_MULTI_PERFORM = -1, /* please call curl_multi_perform() or
                                    curl_multi_socket*() soon */
  CURLM_OK,
  CURLM_BAD_HANDLE,      /* the passed-in handle is not a valid CURLM handle */
  CURLM_BAD_EASY_HANDLE, /* an easy handle was not good/valid */
  CURLM_OUT_OF_MEMORY,   /* if you ever get this, you're in deep sh*t */
  CURLM_INTERNAL_ERROR,  /* this is a libcurl bug */
  CURLM_BAD_SOCKET,      /* the passed in socket argument did not match */
  CURLM_UNKNOWN_OPTION,  /* curl_multi_setopt() with unsupported option */
  CURLM_ADDED_ALREADY,   /* an easy handle already added to a multi handle was
                            attempted to get added - again */
  CURLM_LAST
}

 /* This is the FILE * or void * the regular output should be written to. */
  DEFOPT!(WRITEDATA, OBJECTPOINT, 1);

  /* The full URL to get/put */
  DEFOPT!(URL, OBJECTPOINT, 2);

  /* Port number to connect to, if other than default. */
  DEFOPT!(PORT, LONG, 3);

  /* Name of proxy to use. */
  DEFOPT!(PROXY, OBJECTPOINT, 4);

  /* "user:password;options" to use when fetching. */
  DEFOPT!(USERPWD, OBJECTPOINT, 5);

  /* "user:password" to use with proxy. */
  DEFOPT!(PROXYUSERPWD, OBJECTPOINT, 6);

  /* Range to get, specified as an ASCII string. */
  DEFOPT!(RANGE, OBJECTPOINT, 7);

  /* not used */

  /* Specified file stream to upload from (use as input): */
  DEFOPT!(READDATA, OBJECTPOINT, 9);

  /* Buffer to receive error messages in, must be at least CURL_ERROR_SIZE
   * bytes big. If this is not used, error messages go to stderr instead: */
  DEFOPT!(ERRORBUFFER, OBJECTPOINT, 10);

  /* Function that will be called to store the output (instead of fwrite). The
   * parameters will use fwrite() syntax, make sure to follow them. */
  DEFOPT!(WRITEFUNCTION, FUNCTIONPOINT, 11);

  /* Function that will be called to read the input (instead of fread). The
   * parameters will use fread() syntax, make sure to follow them. */
  DEFOPT!(READFUNCTION, FUNCTIONPOINT, 12);

  /* Time-out the read operation after this amount of seconds */
  DEFOPT!(TIMEOUT, LONG, 13);

  /* If the CURLOPT_INFILE is used, this can be used to inform libcurl about
   * how large the file being sent really is. That allows better error
   * checking and better verifies that the upload was successful. -1 means
   * unknown size.
   *
   * For large file support, there is also a _LARGE version of the key
   * which takes an off_t type, allowing platforms with larger off_t
   * sizes to handle larger files.  See below for INFILESIZE_LARGE.
   */
  DEFOPT!(INFILESIZE, LONG, 14);

  /* POST static input fields. */
  DEFOPT!(POSTFIELDS, OBJECTPOINT, 15);
  /* Set the referrer page (needed by some CGIs) */
  DEFOPT!(REFERER, OBJECTPOINT, 16);

  /* Set the FTP PORT string (interface name, named or numerical IP address)
     Use i.e '-' to use default address. */
  DEFOPT!(FTPPORT, OBJECTPOINT, 17);

  /* Set the User-Agent string (examined by some CGIs) */
  DEFOPT!(USERAGENT, OBJECTPOINT, 18);

  /* If the download receives less than "low speed limit" bytes/second
   * during "low speed time" seconds, the operations is aborted.
   * You could i.e if you have a pretty high speed connection, abort if
   * it is less than 2000 bytes/sec during 20 seconds.
   */

  /* Set the "low speed limit" */
  DEFOPT!(LOW_SPEED_LIMIT, LONG, 19);

  /* Set the "low speed time" */
  DEFOPT!(LOW_SPEED_TIME, LONG, 20);

  /* Set the continuation offset.
   *
   * Note there is also a _LARGE version of this key which uses
   * off_t types, allowing for large file offsets on platforms which
   * use larger-than-32-bit off_t's.  Look below for RESUME_FROM_LARGE.
   */
  DEFOPT!(RESUME_FROM, LONG, 21);

  /* Set cookie in request: */
  DEFOPT!(COOKIE, OBJECTPOINT, 22);

  /* This points to a linked list of headers, struct curl_slist kind. This
     list is also used for RTSP (in spite of its name) */
  DEFOPT!(HTTPHEADER, OBJECTPOINT, 23);

  /* This points to a linked list of post entries, struct curl_httppost */
  DEFOPT!(HTTPPOST, OBJECTPOINT, 24);

  /* name of the file keeping your private SSL-certificate */
  DEFOPT!(SSLCERT, OBJECTPOINT, 25);

  /* password for the SSL or SSH private key */
  DEFOPT!(KEYPASSWD, OBJECTPOINT, 26);

  /* send TYPE parameter? */
  DEFOPT!(CRLF, LONG, 27);

  /* send linked-list of QUOTE commands */
  DEFOPT!(QUOTE, OBJECTPOINT, 28);

  /* send FILE * or void * to store headers to, if you use a callback it
     is simply passed to the callback unmodified */
  DEFOPT!(HEADERDATA, OBJECTPOINT, 29);
  /* point to a file to read the initial cookies from, also enables
     "cookie awareness" */
  DEFOPT!(COOKIEFILE, OBJECTPOINT, 31);

  /* What version to specifically try to use.
     See CURL_SSLVERSION defines below. */
  DEFOPT!(SSLVERSION, LONG, 32);

  /* What kind of HTTP time condition to use, see defines */
  DEFOPT!(TIMECONDITION, LONG, 33);

  /* Time to use with the above condition. Specified in number of seconds
     since 1 Jan 1970 */
  DEFOPT!(TIMEVALUE, LONG, 34);

  /* 35 = OBSOLETE */

  /* Custom request, for customizing the get command like
     HTTP: DELETE, TRACE and others
     FTP: to use a different list command
     */
  DEFOPT!(CUSTOMREQUEST, OBJECTPOINT, 36);

  /* HTTP request, for odd commands like DELETE, TRACE and others */
  DEFOPT!(STDERR, OBJECTPOINT, 37);

  /* 38 is not used */

  /* send linked-list of post-transfer QUOTE commands */
  DEFOPT!(POSTQUOTE, OBJECTPOINT, 39);

  DEFOPT!(OBSOLETE40, OBJECTPOINT, 40); /* OBSOLETE, do not use! */

  DEFOPT!(VERBOSE, LONG, 41);      /* talk a lot */
  DEFOPT!(HEADER, LONG, 42);       /* throw the header out too */
  DEFOPT!(NOPROGRESS, LONG, 43);   /* shut off the progress meter */
  DEFOPT!(NOBODY, LONG, 44);       /* use HEAD to get http document */
  DEFOPT!(FAILONERROR, LONG, 45);  /* no output on http error codes >= 400 */
  DEFOPT!(UPLOAD, LONG, 46);       /* this is an upload */
  DEFOPT!(POST, LONG, 47);         /* HTTP POST method */
  DEFOPT!(DIRLISTONLY, LONG, 48);  /* bare names when listing directories */

  DEFOPT!(APPEND, LONG, 50);       /* Append instead of overwrite on upload! */

  /* Specify whether to read the user+password from the .netrc or the URL.
   * This must be one of the CURL_NETRC_* enums below. */
  DEFOPT!(NETRC, LONG, 51);

  DEFOPT!(FOLLOWLOCATION, LONG, 52);  /* use Location: Luke! */

  DEFOPT!(TRANSFERTEXT, LONG, 53); /* transfer data in text/ASCII format */
  DEFOPT!(PUT, LONG, 54);          /* HTTP PUT */

  /* 55 = OBSOLETE */

  /* DEPRECATED
   * Function that will be called instead of the internal progress display
   * function. This function should be defined as the curl_progress_callback
   * prototype defines. */
  DEFOPT!(PROGRESSFUNCTION, FUNCTIONPOINT, 56);

  /* Data passed to the CURLOPT_PROGRESSFUNCTION and CURLOPT_XFERINFOFUNCTION
     callbacks */
  DEFOPT!(PROGRESSDATA, OBJECTPOINT, 57);

  /* We want the referrer field set automatically when following locations */
  DEFOPT!(AUTOREFERER, LONG, 58);

  /* Port of the proxy, can be set in the proxy string as well with:
     "[host]:[port]" */
  DEFOPT!(PROXYPORT, LONG, 59);

  /* size of the POST input data, if strlen() is not good to use */
  DEFOPT!(POSTFIELDSIZE, LONG, 60);

  /* tunnel non-http operations through a HTTP proxy */
  DEFOPT!(HTTPPROXYTUNNEL, LONG, 61);

  /* Set the interface string to use as outgoing network interface */
  DEFOPT!(INTERFACE, OBJECTPOINT, 62);

  /* Set the krb4/5 security level, this also enables krb4/5 awareness.  This
   * is a string, 'clear', 'safe', 'confidential' or 'private'.  If the string
   * is set but doesn't match one of these, 'private' will be used.  */
  DEFOPT!(KRBLEVEL, OBJECTPOINT, 63);

  /* Set if we should verify the peer in ssl handshake, set 1 to verify. */
  DEFOPT!(SSL_VERIFYPEER, LONG, 64);

  /* The CApath or CAfile used to validate the peer certificate
     this option is used only if SSL_VERIFYPEER is true */
  DEFOPT!(CAINFO, OBJECTPOINT, 65);

  /* 66 = OBSOLETE */
  /* 67 = OBSOLETE */

  /* Maximum number of http redirects to follow */
  DEFOPT!(MAXREDIRS, LONG, 68);

  /* Pass a long set to 1 to get the date of the requested document (if
     possible)! Pass a zero to shut it off. */
  DEFOPT!(FILETIME, LONG, 69);

  /* This points to a linked list of telnet options */
  DEFOPT!(TELNETOPTIONS, OBJECTPOINT, 70);

  /* Max amount of cached alive connections */
 DEFOPT!(MAXCONNECTS, LONG, 71);

  DEFOPT!(OBSOLETE72, LONG, 72); /* OBSOLETE, do not use! */

  /* 73 = OBSOLETE */

  /* Set to explicitly use a new connection for the upcoming transfer.
     Do not use this unless you're absolutely sure of this, as it makes the
     operation slower and is less friendly for the network. */
  DEFOPT!(FRESH_CONNECT, LONG, 74);

  /* Set to explicitly forbid the upcoming transfer's connection to be re-used
     when done. Do not use this unless you're absolutely sure of this, as it
     makes the operation slower and is less friendly for the network. */
  DEFOPT!(FORBID_REUSE, LONG, 75);

  /* Set to a file name that contains random data for libcurl to use to
     seed the random engine when doing SSL connects. */
  DEFOPT!(RANDOM_FILE, OBJECTPOINT, 76);

  /* Set to the Entropy Gathering Daemon socket pathname */
  DEFOPT!(EGDSOCKET, OBJECTPOINT, 77);

  /* Time-out connect operations after this amount of seconds, if connects are
     OK within this time, then fine... This only aborts the connect phase. */
  DEFOPT!(CONNECTTIMEOUT, LONG, 78);

  /* Function that will be called to store headers (instead of fwrite). The
   * parameters will use fwrite() syntax, make sure to follow them. */
  DEFOPT!(HEADERFUNCTION, FUNCTIONPOINT, 79);

  /* Set this to force the HTTP request to get back to GET. Only really usable
     if POST, PUT or a custom request have been used first.
   */
  DEFOPT!(HTTPGET, LONG, 80);

  /* Set if we should verify the Common name from the peer certificate in ssl
   * handshake, set 1 to check existence, 2 to ensure that it matches the
   * provided hostname. */
  DEFOPT!(SSL_VERIFYHOST, LONG, 81);

  /* Specify which file name to write all known cookies in after completed
     operation. Set file name to "-" (dash) to make it go to stdout. */
  DEFOPT!(COOKIEJAR, OBJECTPOINT, 82);

  /* Specify which SSL ciphers to use */
  DEFOPT!(SSL_CIPHER_LIST, OBJECTPOINT, 83);

  /* Specify which HTTP version to use! This must be set to one of the
     CURL_HTTP_VERSION* enums set below. */
  DEFOPT!(HTTP_VERSION, LONG, 84);

  /* Specifically switch on or off the FTP engine's use of the EPSV command. By
     default, that one will always be attempted before the more traditional
     PASV command. */
  DEFOPT!(FTP_USE_EPSV, LONG, 85);

  /* type of the file keeping your SSL-certificate ("DER", "PEM", "ENG") */
  DEFOPT!(SSLCERTTYPE, OBJECTPOINT, 86);

  /* name of the file keeping your private SSL-key */
  DEFOPT!(SSLKEY, OBJECTPOINT, 87);

  /* type of the file keeping your private SSL-key ("DER", "PEM", "ENG") */
  DEFOPT!(SSLKEYTYPE, OBJECTPOINT, 88);

  /* crypto engine for the SSL-sub system */
  DEFOPT!(SSLENGINE, OBJECTPOINT, 89);

  /* set the crypto engine for the SSL-sub system as default
     the param has no meaning...
   */
  DEFOPT!(SSLENGINE_DEFAULT, LONG, 90);

  /* Non-zero value means to use the global dns cache */
  DEFOPT!(DNS_USE_GLOBAL_CACHE, LONG, 91); /* DEPRECATED, do not use! */

  /* DNS cache timeout */
  DEFOPT!(DNS_CACHE_TIMEOUT, LONG, 92);

  /* send linked-list of pre-transfer QUOTE commands */
  DEFOPT!(PREQUOTE, OBJECTPOINT, 93);

  /* set the debug function */
  DEFOPT!(DEBUGFUNCTION, FUNCTIONPOINT, 94);

  /* set the data for the debug function */
  DEFOPT!(DEBUGDATA, OBJECTPOINT, 95);

  /* mark this as start of a cookie session */
  DEFOPT!(COOKIESESSION, LONG, 96);

  /* The CApath directory used to validate the peer certificate
     this option is used only if SSL_VERIFYPEER is true */
  DEFOPT!(CAPATH, OBJECTPOINT, 97);

  /* Instruct libcurl to use a smaller receive buffer */
  DEFOPT!(BUFFERSIZE, LONG, 98);

  /* Instruct libcurl to not use any signal/alarm handlers, even when using
     timeouts. This option is useful for multi-threaded applications.
     See libcurl-the-guide for more background information. */
  DEFOPT!(NOSIGNAL, LONG, 99);

  /* Provide a CURLShare for mutexing non-ts data */
  DEFOPT!(SHARE, OBJECTPOINT, 100);

  /* indicates type of proxy. accepted values are CURLPROXY_HTTP (default);
     CURLPROXY_SOCKS4, CURLPROXY_SOCKS4A and CURLPROXY_SOCKS5. */
  DEFOPT!(PROXYTYPE, LONG, 101);

  /* Set the Accept-Encoding string. Use this to tell a server you would like
     the response to be compressed. Before 7.21.6, this was known as
     CURLOPT_ENCODING */
  DEFOPT!(ACCEPT_ENCODING, OBJECTPOINT, 102);

  /* Set pointer to private data */
  DEFOPT!(PRIVATE, OBJECTPOINT, 103);

  /* Set aliases for HTTP 200 in the HTTP Response header */
  DEFOPT!(HTTP200ALIASES, OBJECTPOINT, 104);

  /* Continue to send authentication (user+password) when following locations,
     even when hostname changed. This can potentially send off the name
     and password to whatever host the server decides. */
  DEFOPT!(UNRESTRICTED_AUTH, LONG, 105);

  /* Specifically switch on or off the FTP engine's use of the EPRT command (
     it also disables the LPRT attempt). By default, those ones will always be
     attempted before the good old traditional PORT command. */
  DEFOPT!(FTP_USE_EPRT, LONG, 106);

  /* Set this to a bitmask value to enable the particular authentications
     methods you like. Use this in combination with CURLOPT_USERPWD.
     Note that setting multiple bits may cause extra network round-trips. */
  DEFOPT!(HTTPAUTH, LONG, 107);

  /* Set the ssl context callback function, currently only for OpenSSL ssl_ctx
     in second argument. The function must be matching the
     curl_ssl_ctx_callback proto. */
  DEFOPT!(SSL_CTX_FUNCTION, FUNCTIONPOINT, 108);

  /* Set the userdata for the ssl context callback function's third
     argument */
  DEFOPT!(SSL_CTX_DATA, OBJECTPOINT, 109);

  /* FTP Option that causes missing dirs to be created on the remote server.
     In 7.19.4 we introduced the convenience enums for this option using the
     CURLFTP_CREATE_DIR prefix.
  */
  DEFOPT!(FTP_CREATE_MISSING_DIRS, LONG, 110);

  /* Set this to a bitmask value to enable the particular authentications
     methods you like. Use this in combination with CURLOPT_PROXYUSERPWD.
     Note that setting multiple bits may cause extra network round-trips. */
  DEFOPT!(PROXYAUTH, LONG, 111);

  /* FTP option that changes the timeout, in seconds, associated with
     getting a response.  This is different from transfer timeout time and
     essentially places a demand on the FTP server to acknowledge commands
     in a timely manner. */
  DEFOPT!(FTP_RESPONSE_TIMEOUT, LONG, 112);

  /* Set this option to one of the CURL_IPRESOLVE_* defines (see below) to
     tell libcurl to resolve names to those IP versions only. This only has
     affect on systems with support for more than one, i.e IPv4 _and_ IPv6. */
  DEFOPT!(IPRESOLVE, LONG, 113);
  /* Set this option to limit the size of a file that will be downloaded from
     an HTTP or FTP server.

     Note there is also _LARGE version which adds large file support for
     platforms which have larger off_t sizes.  See MAXFILESIZE_LARGE below. */
  DEFOPT!(MAXFILESIZE, LONG, 114);

  /* See the comment for INFILESIZE above, but in short, specifies
   * the size of the file being uploaded.  -1 means unknown.
   */
  DEFOPT!(INFILESIZE_LARGE, OFF_T, 115);

  /* Sets the continuation offset.  There is also a LONG version of this;
   * look above for RESUME_FROM.
   */
  DEFOPT!(RESUME_FROM_LARGE, OFF_T, 116);

  /* Sets the maximum size of data that will be downloaded from
   * an HTTP or FTP server.  See MAXFILESIZE above for the LONG version.
   */
  DEFOPT!(MAXFILESIZE_LARGE, OFF_T, 117);

  /* Set this option to the file name of your .netrc file you want libcurl
     to parse (using the CURLOPT_NETRC option). If not set, libcurl will do
     a poor attempt to find the user's home directory and check for a .netrc
     file in there. */
  DEFOPT!(NETRC_FILE, OBJECTPOINT, 118);

  /* Enable SSL/TLS for FTP, pick one of:
     CURLUSESSL_TRY     - try using SSL, proceed anyway otherwise
     CURLUSESSL_CONTROL - SSL for the control connection or fail
     CURLUSESSL_ALL     - SSL for all communication or fail
  */
  DEFOPT!(USE_SSL, LONG, 119);

  /* The _LARGE version of the standard POSTFIELDSIZE option */
  DEFOPT!(POSTFIELDSIZE_LARGE, OFF_T, 120);

  /* Enable/disable the TCP Nagle algorithm */
  DEFOPT!(TCP_NODELAY, LONG, 121);

  /* 122 OBSOLETE, used in 7.12.3. Gone in 7.13.0 */
  /* 123 OBSOLETE. Gone in 7.16.0 */
  /* 124 OBSOLETE, used in 7.12.3. Gone in 7.13.0 */
  /* 125 OBSOLETE, used in 7.12.3. Gone in 7.13.0 */
  /* 126 OBSOLETE, used in 7.12.3. Gone in 7.13.0 */
  /* 127 OBSOLETE. Gone in 7.16.0 */
  /* 128 OBSOLETE. Gone in 7.16.0 */

  /* When FTP over SSL/TLS is selected (with CURLOPT_USE_SSL); this option
     can be used to change libcurl's default action which is to first try
     "AUTH SSL" and then "AUTH TLS" in this order, and proceed when a OK
     response has been received.

     Available parameters are:
     CURLFTPAUTH_DEFAULT - let libcurl decide
     CURLFTPAUTH_SSL     - try "AUTH SSL" first, then TLS
     CURLFTPAUTH_TLS     - try "AUTH TLS" first, then SSL
  */
  DEFOPT!(FTPSSLAUTH, LONG, 129);

  DEFOPT!(IOCTLFUNCTION, FUNCTIONPOINT, 130);
  DEFOPT!(IOCTLDATA, OBJECTPOINT, 131);

  /* 132 OBSOLETE. Gone in 7.16.0 */
  /* 133 OBSOLETE. Gone in 7.16.0 */

  /* zero terminated string for pass on to the FTP server when asked for
     "account" info */
  DEFOPT!(FTP_ACCOUNT, OBJECTPOINT, 134);

  /* feed cookies into cookie engine */
  DEFOPT!(COOKIELIST, OBJECTPOINT, 135);

  /* ignore Content-Length */
  DEFOPT!(IGNORE_CONTENT_LENGTH, LONG, 136);

  /* Set to non-zero to skip the IP address received in a 227 PASV FTP server
     response. Typically used for FTP-SSL purposes but is not restricted to
     that. libcurl will then instead use the same IP address it used for the
     control connection. */
  DEFOPT!(FTP_SKIP_PASV_IP, LONG, 137);

  /* Select "file method" to use when doing FTP, see the curl_ftpmethod
     above. */
  DEFOPT!(FTP_FILEMETHOD, LONG, 138);

  /* Local port number to bind the socket to */
  DEFOPT!(LOCALPORT, LONG, 139);

  /* Number of ports to try, including the first one set with LOCALPORT.
     Thus, setting it to 1 will make no additional attempts but the first.
  */
  DEFOPT!(LOCALPORTRANGE, LONG, 140);

  /* no transfer, set up connection and let application use the socket by
     extracting it with CURLINFO_LASTSOCKET */
  DEFOPT!(CONNECT_ONLY, LONG, 141);

  /* Function that will be called to convert from the
     network encoding (instead of using the iconv calls in libcurl) */
  DEFOPT!(CONV_FROM_NETWORK_FUNCTION, FUNCTIONPOINT, 142);

  /* Function that will be called to convert to the
     network encoding (instead of using the iconv calls in libcurl) */
  DEFOPT!(CONV_TO_NETWORK_FUNCTION, FUNCTIONPOINT, 143);

  /* Function that will be called to convert from UTF8
     (instead of using the iconv calls in libcurl)
     Note that this is used only for SSL certificate processing */
  DEFOPT!(CONV_FROM_UTF8_FUNCTION, FUNCTIONPOINT, 144);
  /* if the connection proceeds too quickly then need to slow it down */
  /* limit-rate: maximum number of bytes per second to send or receive */
  DEFOPT!(MAX_SEND_SPEED_LARGE, OFF_T, 145);
  DEFOPT!(MAX_RECV_SPEED_LARGE, OFF_T, 146);

  /* Pointer to command string to send if USER/PASS fails. */
  DEFOPT!(FTP_ALTERNATIVE_TO_USER, OBJECTPOINT, 147);

  /* callback function for setting socket options */
  DEFOPT!(SOCKOPTFUNCTION, FUNCTIONPOINT, 148);
  DEFOPT!(SOCKOPTDATA, OBJECTPOINT, 149);

  /* set to 0 to disable session ID re-use for this transfer, default is
     enabled (== 1) */
  DEFOPT!(SSL_SESSIONID_CACHE, LONG, 150);

  /* allowed SSH authentication methods */
  DEFOPT!(SSH_AUTH_TYPES, LONG, 151);

  /* Used by scp/sftp to do public/private key authentication */
  DEFOPT!(SSH_PUBLIC_KEYFILE, OBJECTPOINT, 152);
  DEFOPT!(SSH_PRIVATE_KEYFILE, OBJECTPOINT, 153);

  /* Send CCC (Clear Command Channel) after authentication */
  DEFOPT!(FTP_SSL_CCC, LONG, 154);

  /* Same as TIMEOUT and CONNECTTIMEOUT, but with ms resolution */
  DEFOPT!(TIMEOUT_MS, LONG, 155);
  DEFOPT!(CONNECTTIMEOUT_MS, LONG, 156);

  /* set to zero to disable the libcurl's decoding and thus pass the raw body
     data to the application even when it is encoded/compressed */
  DEFOPT!(HTTP_TRANSFER_DECODING, LONG, 157);
  DEFOPT!(HTTP_CONTENT_DECODING, LONG, 158);

  /* Permission used when creating new files and directories on the remote
     server for protocols that support it, SFTP/SCP/FILE */
  DEFOPT!(NEW_FILE_PERMS, LONG, 159);
  DEFOPT!(NEW_DIRECTORY_PERMS, LONG, 160);

  /* Set the behaviour of POST when redirecting. Values must be set to one
     of CURL_REDIR* defines below. This used to be called CURLOPT_POST301 */
  DEFOPT!(POSTREDIR, LONG, 161);

  /* used by scp/sftp to verify the host's public key */
  DEFOPT!(SSH_HOST_PUBLIC_KEY_MD5, OBJECTPOINT, 162);

  /* Callback function for opening socket (instead of socket(2)). Optionally,
     callback is able change the address or refuse to connect returning
     CURL_SOCKET_BAD.  The callback should have type
     curl_opensocket_callback */
  DEFOPT!(OPENSOCKETFUNCTION, FUNCTIONPOINT, 163);
  DEFOPT!(OPENSOCKETDATA, OBJECTPOINT, 164);

  /* POST volatile input fields. */
  DEFOPT!(COPYPOSTFIELDS, OBJECTPOINT, 165);

  /* set transfer mode (;type=<a|i>) when doing FTP via an HTTP proxy */
  DEFOPT!(PROXY_TRANSFER_MODE, LONG, 166);

  /* Callback function for seeking in the input stream */
  DEFOPT!(SEEKFUNCTION, FUNCTIONPOINT, 167);
  DEFOPT!(SEEKDATA, OBJECTPOINT, 168);

  /* CRL file */
  DEFOPT!(CRLFILE, OBJECTPOINT, 169);

  /* Issuer certificate */
  DEFOPT!(ISSUERCERT, OBJECTPOINT, 170);

  /* (IPv6) Address scope */
  DEFOPT!(ADDRESS_SCOPE, LONG, 171);

  /* Collect certificate chain info and allow it to get retrievable with
     CURLINFO_CERTINFO after the transfer is complete. */
  DEFOPT!(CERTINFO, LONG, 172);

  /* "name" and "pwd" to use when fetching. */
  DEFOPT!(USERNAME, OBJECTPOINT, 173);
  DEFOPT!(PASSWORD, OBJECTPOINT, 174);

    /* "name" and "pwd" to use with Proxy when fetching. */
  DEFOPT!(PROXYUSERNAME, OBJECTPOINT, 175);
  DEFOPT!(PROXYPASSWORD, OBJECTPOINT, 176);

  /* Comma separated list of hostnames defining no-proxy zones. These should
     match both hostnames directly, and hostnames within a domain. For
     example, local.com will match local.com and www.local.com, but NOT
     notlocal.com or www.notlocal.com. For compatibility with other
     implementations of this, .local.com will be considered to be the same as
     local.com. A single * is the only valid wildcard, and effectively
     disables the use of proxy. */
  DEFOPT!(NOPROXY, OBJECTPOINT, 177);

  /* block size for TFTP transfers */
  DEFOPT!(TFTP_BLKSIZE, LONG, 178);

  /* Socks Service */
  DEFOPT!(SOCKS5_GSSAPI_SERVICE, OBJECTPOINT, 179);

  /* Socks Service */
  DEFOPT!(SOCKS5_GSSAPI_NEC, LONG, 180);

  /* set the bitmask for the protocols that are allowed to be used for the
     transfer, which thus helps the app which takes URLs from users or other
     external inputs and want to restrict what protocol(s) to deal
     with. Defaults to CURLPROTO_ALL. */
  DEFOPT!(PROTOCOLS, LONG, 181);

  /* set the bitmask for the protocols that libcurl is allowed to follow to,
     as a subset of the CURLOPT_PROTOCOLS ones. That means the protocol needs
    to be set in both bitmasks to be allowed to get redirected to. Defaults
     to all protocols except FILE and SCP. */
  DEFOPT!(REDIR_PROTOCOLS, LONG, 182);

  /* set the SSH knownhost file name to use */
  DEFOPT!(SSH_KNOWNHOSTS, OBJECTPOINT, 183);

  /* set the SSH host key callback, must point to a curl_sshkeycallback
     function */
  DEFOPT!(SSH_KEYFUNCTION, FUNCTIONPOINT, 184);

  /* set the SSH host key callback custom pointer */
  DEFOPT!(SSH_KEYDATA, OBJECTPOINT, 185);

  /* set the SMTP mail originator */
  DEFOPT!(MAIL_FROM, OBJECTPOINT, 186);

  /* set the SMTP mail receiver(s) */
  DEFOPT!(MAIL_RCPT, OBJECTPOINT, 187);

  /* FTP: send PRET before PASV */
  DEFOPT!(FTP_USE_PRET, LONG, 188);

  /* RTSP request method (OPTIONS, SETUP, PLAY, etc...) */
  DEFOPT!(RTSP_REQUEST, LONG, 189);

  /* The RTSP session identifier */
  DEFOPT!(RTSP_SESSION_ID, OBJECTPOINT, 190);

  /* The RTSP stream URI */
  DEFOPT!(RTSP_STREAM_URI, OBJECTPOINT, 191);

  /* The Transport: header to use in RTSP requests */
  DEFOPT!(RTSP_TRANSPORT, OBJECTPOINT, 192);

  /* Manually initialize the client RTSP CSeq for this handle */
  DEFOPT!(RTSP_CLIENT_CSEQ, LONG, 193);

  /* Manually initialize the server RTSP CSeq for this handle */
  DEFOPT!(RTSP_SERVER_CSEQ, LONG, 194);

  /* The stream to pass to INTERLEAVEFUNCTION. */
  DEFOPT!(INTERLEAVEDATA, OBJECTPOINT, 195);

  /* Let the application define a custom write method for RTP data */
  DEFOPT!(INTERLEAVEFUNCTION, FUNCTIONPOINT, 196);

  /* Turn on wildcard matching */
  DEFOPT!(WILDCARDMATCH, LONG, 197);

  /* Directory matching callback called before downloading of an
     individual file (chunk) started */
  DEFOPT!(CHUNK_BGN_FUNCTION, FUNCTIONPOINT, 198);

  /* Directory matching callback called after the file (chunk)
     was downloaded, or skipped */
  DEFOPT!(CHUNK_END_FUNCTION, FUNCTIONPOINT, 199);

  /* Change match (fnmatch-like) callback for wildcard matching */
  DEFOPT!(FNMATCH_FUNCTION, FUNCTIONPOINT, 200);

  /* Let the application define custom chunk data pointer */
  DEFOPT!(CHUNK_DATA, OBJECTPOINT, 201);

  /* FNMATCH_FUNCTION user pointer */
  DEFOPT!(FNMATCH_DATA, OBJECTPOINT, 202);

  /* send linked-list of name:port:address sets */
  DEFOPT!(RESOLVE, OBJECTPOINT, 203);

  /* Set a username for authenticated TLS */
  DEFOPT!(TLSAUTH_USERNAME, OBJECTPOINT, 204);

  /* Set a password for authenticated TLS */
  DEFOPT!(TLSAUTH_PASSWORD, OBJECTPOINT, 205);

  /* Set authentication type for authenticated TLS */
  DEFOPT!(TLSAUTH_TYPE, OBJECTPOINT, 206);

  /* Set to 1 to enable the "TE:" header in HTTP requests to ask for
     compressed transfer-encoded responses. Set to 0 to disable the use of TE:
     in outgoing requests. The current default is 0, but it might change in a
     future libcurl release.

     libcurl will ask for the compressed methods it knows of, and if that
     isn't any, it will not ask for transfer-encoding at all even if this
     option is set to 1.

  */
  DEFOPT!(TRANSFER_ENCODING, LONG, 207);

  /* Callback function for closing socket (instead of close(2)). The callback
     should have type curl_closesocket_callback */
  DEFOPT!(CLOSESOCKETFUNCTION, FUNCTIONPOINT, 208);
  DEFOPT!(CLOSESOCKETDATA, OBJECTPOINT, 209);

  /* allow GSSAPI credential delegation */
  DEFOPT!(GSSAPI_DELEGATION, LONG, 210);

  /* Set the name servers to use for DNS resolution */
  DEFOPT!(DNS_SERVERS, OBJECTPOINT, 211);

  /* Time-out accept operations (currently for FTP only) after this amount
     of miliseconds. */
  DEFOPT!(ACCEPTTIMEOUT_MS, LONG, 212);

  /* Set TCP keepalive */
  DEFOPT!(TCP_KEEPALIVE, LONG, 213);

  /* non-universal keepalive knobs (Linux, AIX, HP-UX, more) */
  DEFOPT!(TCP_KEEPIDLE, LONG, 214);
  DEFOPT!(TCP_KEEPINTVL, LONG, 215);
  /* Enable/disable specific SSL features with a bitmask, see CURLSSLOPT_* */
  DEFOPT!(SSL_OPTIONS, LONG, 216);

  /* Set the SMTP auth originator */
  DEFOPT!(MAIL_AUTH, OBJECTPOINT, 217);

  /* Enable/disable SASL initial response */
  DEFOPT!(SASL_IR, LONG, 218);

  /* Function that will be called instead of the internal progress display
   * function. This function should be defined as the curl_xferinfo_callback
   * prototype defines. (Deprecates CURLOPT_PROGRESSFUNCTION) */
  DEFOPT!(XFERINFOFUNCTION, FUNCTIONPOINT, 219);

  /* The XOAUTH2 bearer token */
  DEFOPT!(XOAUTH2_BEARER, OBJECTPOINT, 220);

  /* Set the interface string to use as outgoing network
   * interface for DNS requests.
   * Only supported by the c-ares DNS backend */
  DEFOPT!(DNS_INTERFACE, OBJECTPOINT, 221);

  /* Set the local IPv4 address to use for outgoing DNS requests.
   * Only supported by the c-ares DNS backend */
  DEFOPT!(DNS_LOCAL_IP4, OBJECTPOINT, 222);

  /* Set the local IPv4 address to use for outgoing DNS requests.
   * Only supported by the c-ares DNS backend */
  DEFOPT!(DNS_LOCAL_IP6, OBJECTPOINT, 223);

  /* Set authentication options directly */
  DEFOPT!(LOGIN_OPTIONS, OBJECTPOINT, 224);

  /* Enable/disable TLS NPN extension (http2 over ssl might fail without) */
  DEFOPT!(SSL_ENABLE_NPN, LONG, 225);

  /* Enable/disable TLS ALPN extension (http2 over ssl might fail without) */
  DEFOPT!(SSL_ENABLE_ALPN, LONG, 226);

  /* Time to wait for a response to a HTTP request containing an
   * Expect: 100-continue header before sending the data anyway. */
  DEFOPT!(EXPECT_100_TIMEOUT_MS, LONG, 227);

  /* This points to a linked list of headers used for proxy requests only,
     struct curl_slist kind */
  DEFOPT!(PROXYHEADER, OBJECTPOINT, 228);

  /* Pass in a bitmask of "header options" */
  DEFOPT!(HEADEROPT, LONG, 229);

  /* The public key in DER form used to validate the peer public key
     this option is used only if SSL_VERIFYPEER is true */
  DEFOPT!(PINNEDPUBLICKEY, OBJECTPOINT, 230);

  /* Path to Unix domain socket */
  DEFOPT!(UNIX_SOCKET_PATH, OBJECTPOINT, 231);

  /* Set if we should verify the certificate status. */
  DEFOPT!(SSL_VERIFYSTATUS, LONG, 232);

  /* Set if we should enable TLS false start. */
  DEFOPT!(SSL_FALSESTART, LONG, 233);

  /* Do not squash dot-dot sequences */
  DEFOPT!(PATH_AS_IS, LONG, 234);

  /* Proxy Service Name */
  DEFOPT!(PROXY_SERVICE_NAME, OBJECTPOINT, 235);

  /* Service Name */
  DEFOPT!(SERVICE_NAME, OBJECTPOINT, 236);

  /* Wait/don't wait for pipe/mutex to clarify */
  DEFOPT!(PIPEWAIT, LONG, 237);
  ALIAS!(LONG, CURLOPTTYPE_LONG);
  ALIAS!(OBJECTPOINT, CURLOPTTYPE_OBJECTPOINT);
  ALIAS!(FUNCTIONPOINT, CURLOPTTYPE_FUNCTIONPOINT);
  ALIAS!(OFF_T, CURLOPTTYPE_OFF_T);

extern {
	
	// Easy curl mode (one thread per connection)
    pub fn curl_easy_strerror(code: CURLcode) -> *const c_char;
    pub fn curl_easy_init() -> *mut CURL;
    pub fn curl_easy_setopt(curl: *mut CURL, option: CURLoption, ...) -> CURLcode;
    pub fn curl_easy_perform(curl: *mut CURL) -> CURLcode;
    pub fn curl_easy_cleanup(curl: *mut CURL);
    pub fn curl_easy_getinfo(curl: *const CURL, info: CURLINFO, ...) -> CURLcode;
    
    // Multi curl mode
    pub fn curl_multi_init() -> *mut CURL;
    pub fn curl_multi_setopt(curl: *mut CURL, option: CURLMoption, ...) -> CURLMcode;
    
    pub fn curl_global_cleanup();

    pub fn curl_slist_append(list: *mut curl_slist,
                             val: *const u8) -> *mut curl_slist;
    pub fn curl_slist_free_all(list: *mut curl_slist);

    pub fn curl_version() -> *const c_char;
    pub fn curl_version_info(t: CURLversion) -> *mut curl_version_info_data;
}
