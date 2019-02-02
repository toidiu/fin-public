// Helper macros to make life easier.

macro_rules! loge (
    (target: $target:expr, $($arg:tt)*) => (
        log!(target: $target, log::Level::Error, "{}: {}", line!(), $($arg)*);
    );
    ($($arg:tt)*) => (
        log!(log::Level::Error, "{}: {}", line!(), $($arg)*);
    )
);
