#[cfg(feature = "log")]
pub(crate) use log::debug;

#[cfg(not(feature = "log"))]
macro_rules! debug {
    ($($arg:tt)*) => {
        eprintln!("[WTK DEBUG] {}", format_args!($($arg)*));
    }
}

#[cfg(not(feature = "log"))]
#[allow(unused_macros)]
macro_rules! error {
    ($($arg:tt)*) => {
        eprintln!("[WTK ERROR] {}", format_args!($($arg)*));
    }
}

#[cfg(not(feature = "log"))]
pub(crate) use debug;
