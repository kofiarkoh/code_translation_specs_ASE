use std::os::raw::c_char;

use std::ffi::CString;
use std::io::{self, Write};
use std::process;

use std::ffi::{CStr};



// Maximum scheme length
pub const MAX_URL_SCHEME_LEN: usize = 32;
// Maximum authority length
pub const MAX_URL_AUTHORITY_LEN: usize = 512;
// Maximum port length
pub const MAX_URL_PORT_LEN: usize = 64;
// Maximum path length
pub const MAX_URL_PATH_LEN: usize = 1024;
// Maximum query length
pub const MAX_URL_QUERY_LEN: usize = 1024;
// Maximum fragment length
pub const MAX_URL_FRAGMENT_LEN: usize = 256;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct URL_PARTS {
    pub scheme: [c_char; MAX_URL_SCHEME_LEN],
    pub authority: [c_char; MAX_URL_AUTHORITY_LEN],
    pub port: [c_char; MAX_URL_PORT_LEN],
    pub path: [c_char; MAX_URL_PATH_LEN],
    pub query: [c_char; MAX_URL_QUERY_LEN],
    pub fragment: [c_char; MAX_URL_FRAGMENT_LEN],
}

impl Default for URL_PARTS {
    fn default() -> Self {
        Self {
            scheme: [0; MAX_URL_SCHEME_LEN],
            authority: [0; MAX_URL_AUTHORITY_LEN],
            port: [0; MAX_URL_PORT_LEN],
            path: [0; MAX_URL_PATH_LEN],
            query: [0; MAX_URL_QUERY_LEN],
            fragment: [0; MAX_URL_FRAGMENT_LEN],
        }
    }
}


fn write_byte(buf: &mut [c_char], i: usize, byte: u8) {
    buf[i] = byte as c_char;
}

/// Parses a URL-like string into fixed-size component buffers.
///
/// Safety/behavior notes:
/// - This mirrors the C function's contract: null pointers are not handled here.
/// - The output struct is always cleared before parsing begins.
/// - On overflow, returns `false` and leaves any already-written prefix intact.
#[unsafe(no_mangle)]
pub extern "C" fn ParseURL(url: *mut c_char, p_url_parts: *mut URL_PARTS) -> bool {
    // The C code assumes non-null pointers and a valid NUL-terminated string.
    // We preserve that same precondition here.
    assert!(!url.is_null(), "ParseURL precondition violated: URL must not be NULL");
    assert!(
        !p_url_parts.is_null(),
        "ParseURL precondition violated: pURL_Parts must not be NULL"
    );

    let parts = unsafe { &mut *p_url_parts };
    *parts = URL_PARTS::default();

    let bytes = unsafe { CStr::from_ptr(url.cast_const()) }.to_bytes();
    let mut string_idx: usize = 0;

    let authority_idx = bytes
        .windows(2)
        .position(|w| w == b"//")
        .unwrap_or(0);

    if authority_idx != 0 || bytes.windows(2).any(|w| w == b"//") {
        let mut i = 0usize;
        while string_idx < authority_idx {
            let c = bytes[string_idx];
            if c == b':' || c == b'/' {
                break;
            }
            if i == MAX_URL_SCHEME_LEN - 1 {
                return false;
            }
            write_byte(&mut parts.scheme, i, c);
            string_idx += 1;
            i += 1;
        }

        // Skip the slashes.
        string_idx = authority_idx + 2;
    }

    let mut authority_ptr = string_idx;
    let mut bracketed = false;

    if authority_ptr < bytes.len() && bytes[authority_ptr] == b'[' {
        bracketed = true;
        authority_ptr += 1;
    }

    let mut i = 0usize;
    while authority_ptr < bytes.len() {
        let c = bytes[authority_ptr];

        // Skip spaces in authority.
        if c == b' ' {
            authority_ptr += 1;
            continue;
        }

        if c == b'/' {
            break;
        }
        if c == b':' && !bracketed {
            break;
        }
        if c == b'?' {
            break;
        }
        if c == b'#' {
            break;
        }

        if bracketed && c == b']' {
            authority_ptr += 1;
            break;
        }

        if i == MAX_URL_AUTHORITY_LEN - 1 {
            return false;
        }
        write_byte(&mut parts.authority, i, c);
        authority_ptr += 1;
        i += 1;
    }

    string_idx = authority_ptr;

    if string_idx < bytes.len() && bytes[string_idx] == b':' {
        string_idx += 1;
        i = 0;
        while string_idx < bytes.len() {
            let c = bytes[string_idx];
            if c == b'/' || c == b'?' || c == b'#' {
                break;
            }
            if i == MAX_URL_PORT_LEN - 1 {
                return false;
            }
            write_byte(&mut parts.port, i, c);
            string_idx += 1;
            i += 1;
        }
    }

    if string_idx < bytes.len() && bytes[string_idx] == b'/' {
        i = 0;
        while string_idx < bytes.len() {
            let c = bytes[string_idx];
            if c == b'?' || c == b'#' {
                break;
            }
            if i == MAX_URL_PATH_LEN - 1 {
                return false;
            }
            write_byte(&mut parts.path, i, c);
            string_idx += 1;
            i += 1;
        }
    }

    if string_idx < bytes.len() && bytes[string_idx] == b'?' {
        i = 0;
        while string_idx < bytes.len() {
            let c = bytes[string_idx];
            if c == b'#' {
                break;
            }
            if i == MAX_URL_QUERY_LEN - 1 {
                return false;
            }
            write_byte(&mut parts.query, i, c);
            string_idx += 1;
            i += 1;
        }
    }

    if string_idx < bytes.len() && bytes[string_idx] == b'#' {
        i = 0;
        while string_idx < bytes.len() {
            let c = bytes[string_idx];
            if i == MAX_URL_FRAGMENT_LEN - 1 {
                return false;
            }
            write_byte(&mut parts.fragment, i, c);
            string_idx += 1;
            i += 1;
        }
    }

    true
}

fn c_buf_to_str(buf: &[c_char]) -> &str {
    // SAFETY:
    // Per the specification and original C code, each field is assumed to be a
    // valid NUL-terminated C string when PrintURL is called with a non-NULL pointer.
    unsafe { CStr::from_ptr(buf.as_ptr()) }
        .to_str()
        .expect("URL_PARTS field must contain valid UTF-8-compatible text")
}

#[unsafe(no_mangle)]
pub extern "C" fn PrintURL(p_url_parts: *mut URL_PARTS) {
    if p_url_parts.is_null() {
        let _ = writeln!(
            io::stderr(),
            "PrintURL Error: Parameter 'pURL_Parts' is NULL"
        );
        return;
    }

    // SAFETY:
    // The caller provides a non-NULL pointer to a readable URL_PARTS object.
    let parts = unsafe { &*p_url_parts };

    let scheme = c_buf_to_str(&parts.scheme);
    let authority = c_buf_to_str(&parts.authority);
    let port = c_buf_to_str(&parts.port);
    let path = c_buf_to_str(&parts.path);
    let query = c_buf_to_str(&parts.query);
    let fragment = c_buf_to_str(&parts.fragment);

    let mut stdout = io::stdout();

    let _ = write!(
        stdout,
        "Scheme: {}\nAuthority: {}\nPort: {}\nPath: {}\nQuery: {}\nFragment: {}\n",
        scheme, authority, port, path, query, fragment
    );

    if !scheme.is_empty() {
        let _ = write!(stdout, "{}://{}", scheme, authority);
    } else if !authority.is_empty() {
        let _ = write!(stdout, "{}", authority);
    }

    if !port.is_empty() {
        let _ = write!(stdout, ":{}", port);
    }
    if !path.is_empty() {
        let _ = write!(stdout, "{}", path);
    }
    if !query.is_empty() {
        let _ = write!(stdout, "{}", query);
    }
    if !fragment.is_empty() {
        let _ = write!(stdout, "{}", fragment);
    }

    let _ = write!(stdout, "\n");
}



fn main() {
    // The C code dynamically allocates URL_PARTS and exits on allocation failure.
    // In Rust, Box allocation is infallible for practical purposes and aborts on OOM
    // rather than returning NULL, so there is no direct equivalent perror path here.
    // This is a Rust/runtime mismatch rather than a behavioral choice in the original logic.
    let mut url_parts = Box::new(URL_PARTS {
        scheme: [0; MAX_URL_SCHEME_LEN],
        authority: [0; MAX_URL_AUTHORITY_LEN],
        port: [0; MAX_URL_PORT_LEN],
        path: [0; MAX_URL_PATH_LEN],
        query: [0; MAX_URL_QUERY_LEN],
        fragment: [0; MAX_URL_FRAGMENT_LEN],
    });

    let test_urls = [
        "http://sullewarehouse.com/login",
        "https://sullewarehouse.com:1000/login",
        "https://sullewarehouse.com:1000/api/get?username=myuser",
        "sullewarehouse.com/register",
        "http://[2001:0db8:85a3:0000:0000:8a2e:0370:7334]/newpage",
        "https://[2001:0db8:85a3:0000:0000:8a2e:0370:7334]:2678/blog",
    ];

    for url in test_urls {
        let c_url = match CString::new(url) {
            Ok(s) => s,
            Err(_) => {
                let _ = writeln!(io::stderr(), "Failed to construct C string for test URL");
                process::exit(1);
            }
        };

        // SAFETY:
        // - `c_url` is a valid NUL-terminated string and lives across the FFI call.
        // - `url_parts` points to writable memory for one `URL_PARTS`.
        // - We ignore the boolean result exactly as the C code does.
        unsafe {
            let _ = ParseURL(c_url.as_ptr() as *mut std::os::raw::c_char, &mut *url_parts);
            PrintURL(&mut *url_parts);
        }

        print!("\n");
    }

}