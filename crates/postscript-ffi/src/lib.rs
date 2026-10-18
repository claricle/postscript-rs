//! C ABI surface over postscript-core for the postscript gem's
//! binding (leptris/parsanol-ffi style: opaque handles + buffer
//! protocol; UTF-8 on both faces).

static RS_VERSION: &[u8] = b"0.1.0\0";

/// Placeholder ABI entry point — replaced by the real surface in #4.
#[no_mangle]
pub extern "C" fn postscript_rs_version() -> *const std::os::raw::c_char {
    RS_VERSION.as_ptr() as *const std::os::raw::c_char
}
