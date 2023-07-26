pub const IDENT_STR: &str = "    ";
pub const IDENT: &[u8] = IDENT_STR.as_bytes();
pub const IDENT_SIZE: usize = IDENT_STR.len();

#[cfg(windows)]
pub const LINE_BREAK: &[u8; 1] = b"\n";

#[cfg(windows)]
pub const LINE_BREAK_STR: &str = "\n";

#[cfg(not(windows))]
pub const LINE_BREAK: &[u8; 1] = b"\n";

#[cfg(not(windows))]
pub const LINE_BREAK_STR: &str = "\n";
