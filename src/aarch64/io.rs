#[macro_export]
macro_rules! HVC {
    ($($arg:tt)*) => ($crate::aarch64::uart_console::println("HVC:" + format_args!($($arg)*)));
}

#[macro_export]
macro_rules! SMC {
    ($($arg:tt)*) => ($crate::aarch64::uart_console::println("SMC:" + format_args!($($arg)*)));
}

#[macro_export]
macro_rules! ERROR {
    ($($arg:tt)*) => ($crate::aarch64::uart_console::println("ERROR:" + format_args!($($arg)*)));
}

#[macro_export]
macro_rules! INFO {
    ($($arg:tt)*) => ($crate::aarch64::uart_console::println("INFO:" + format_args!($($arg)*)));
}