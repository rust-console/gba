/// Delivers a fatal message to the emulator debug output, and crashes
/// the the game.
///
/// This works basically like `println`. You should avoid null ASCII values.
/// Furthermore on mGBA, there is a maximum length of 255 bytes per message.
///
/// This has no effect outside of a supported emulator.
#[macro_export]
macro_rules! fatal {
  ($($arg:tt)*) => {{
    use $crate::debugging;
    if !debugging::is_debugging_disabled() {
      debugging::debug_print(debugging::DebugLevel::Fatal, &format_args!($($arg)*)).ok();
    }
    debugging::crash()
  }};
}

/// Delivers a error message to the emulator debug output.
///
/// This works basically like `println`. You should avoid null ASCII values.
/// Furthermore on mGBA, there is a maximum length of 255 bytes per message.
///
/// This has no effect outside of a supported emulator.
#[macro_export]
macro_rules! error {
  ($($arg:tt)*) => {{
    use $crate::debugging;
    if !debugging::is_debugging_disabled() {
      debugging::debug_print(debugging::DebugLevel::Error, &format_args!($($arg)*)).ok();
    }
  }};
}

/// Delivers a warning message to the emulator debug output.
///
/// This works basically like `println`. You should avoid null ASCII values.
/// Furthermore on mGBA, there is a maximum length of 255 bytes per message.
///
/// This has no effect outside of a supported emulator.
#[macro_export]
macro_rules! warning {
  ($($arg:tt)*) => {{
    use $crate::debugging;
    if !debugging::is_debugging_disabled() {
      debugging::debug_print(debugging::DebugLevel::Warning, &format_args!($($arg)*)).ok();
    }
  }};
}

/// Delivers an info message to the emulator debug output.
///
/// This works basically like `println`. You should avoid null ASCII values.
/// Furthermore on mGBA, there is a maximum length of 255 bytes per message.
///
/// This has no effect outside of a supported emulator.
#[macro_export]
macro_rules! info {
  ($($arg:tt)*) => {{
    use $crate::debugging;
    if !debugging::is_debugging_disabled() {
      debugging::debug_print(debugging::DebugLevel::Info, &format_args!($($arg)*)).ok();
    }
  }};
}

/// Delivers a debug message to the emulator debug output.
///
/// This works basically like `println`. You should avoid null ASCII values.
/// Furthermore on mGBA, there is a maximum length of 255 bytes per message.
///
/// This has no effect outside of a supported emulator.
#[macro_export]
macro_rules! debug {
  ($($arg:tt)*) => {{
    use $crate::debugging;
    if !debugging::is_debugging_disabled() {
      debugging::debug_print(debugging::DebugLevel::Debug, &format_args!($($arg)*)).ok();
    }
  }};
}
