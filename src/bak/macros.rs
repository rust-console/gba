//! Contains the macros for the crate.
//!
//! Because (unlike everything else in Rust) a macro has to be declared before
//! use, we place them in their own module and then declare that module at the
//! start of the crate.

/// Assists in defining a newtype wrapper over some base type.
///
/// Note that rustdoc and derives are all the "meta" stuff, so you can write all
/// of your docs and derives in front of your newtype in the same way you would
/// for a normal struct. Then the inner type to be wrapped it name.
///
/// The macro _assumes_ that you'll be using it to wrap numeric types and that
/// it's safe to have a `0` value, so it automatically provides a `const fn`
/// method for `new` that just wraps `0`. Also, it derives Debug, Clone, Copy,
/// Default, PartialEq, and Eq. If all this is not desired you can add `, no
/// frills` to the invocation.
///
/// ```no_run
/// newtype! {
///   /// Records a particular key press combination.
///   KeyInput, u16
/// }
/// newtype! {
///   /// You can't derive most stuff above array size 32, so we add
///   /// the `, no frills` modifier to this one.
///   BigArray, [u8; 200], no frills
/// }
/// ```
#[macro_export]
macro_rules! newtype {
  ($(#[$attr:meta])* $new_name:ident, $v:vis $old_name:ty) => {
    $(#[$attr])*
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct $new_name($v $old_name);
    impl $new_name {
      /// A `const` "zero value" constructor
      pub const fn new() -> Self {
        $new_name(0)
      }
    }
  };
  ($(#[$attr:meta])* $new_name:ident, $v:vis $old_name:ty, no frills) => {
    $(#[$attr])*
    #[repr(transparent)]
    pub struct $new_name($v $old_name);
  };
}

/// Assists in defining a newtype that's an enum.
///
/// First give `NewType = OldType,`, then define the tags and their explicit
/// values with zero or more entries of `TagName = base_value,`. In both cases
/// you can place doc comments or other attributes directly on to the type
/// declaration or the tag declaration.
///
/// The generated enum will get an appropriate `repr` attribute as well as
/// Debug, Clone, Copy, PartialEq, and Eq
///
/// ```no_run
/// newtype_enum! {
///   /// The Foo
///   Foo = u16,
///   /// The Bar
///   Bar = 0,
///   /// The Zap
///   Zap = 1,
/// }
/// ```
#[macro_export]
macro_rules! newtype_enum {
  (
    $(#[$struct_attr:meta])*
    $new_name:ident = $old_name:ident,
    $($(#[$tag_attr:meta])* $tag_name:ident = $base_value:expr,)*
  ) => {
    $(#[$struct_attr])*
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr($old_name)]
    pub enum $new_name {
      $(
        $(#[$tag_attr])*
        $tag_name = $base_value,
      )*
    }
  };
}

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
    use $crate::debug;
    if !debug::is_debugging_disabled() {
      debug::debug_print(debug::DebugLevel::Fatal, &format_args!($($arg)*)).ok();
    }
    debug::crash()
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
    use $crate::debug;
    if !debug::is_debugging_disabled() {
      debug::debug_print(debug::DebugLevel::Error, &format_args!($($arg)*)).ok();
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
macro_rules! warn {
  ($($arg:tt)*) => {{
    use $crate::debug;
    if !debug::is_debugging_disabled() {
      debug::debug_print(debug::DebugLevel::Warning, &format_args!($($arg)*)).ok();
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
    use $crate::debug;
    if !debug::is_debugging_disabled() {
      debug::debug_print(debug::DebugLevel::Info, &format_args!($($arg)*)).ok();
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
    use $crate::debug;
    if !debug::is_debugging_disabled() {
      debug::debug_print(debug::DebugLevel::Debug, &format_args!($($arg)*)).ok();
    }
  }};
}

/// Using timers 0 and 1, performs a crude timing of the expression given.
#[macro_export]
macro_rules! time_this01 {
  ($x:expr) => {{
    use $crate::io::timers::*;
    const NORMAL_ON: TimerControlSetting = TimerControlSetting::new().with_enabled(true);
    const CASCADE_ON: TimerControlSetting =
      TimerControlSetting::new().with_enabled(true).with_tick_rate(TimerTickRate::Cascade);
    const OFF: TimerControlSetting = TimerControlSetting::new();
    TM1CNT_H.write(CASCADE_ON);
    TM0CNT_H.write(NORMAL_ON);
    $x;
    TM0CNT_H.write(OFF);
    TM1CNT_H.write(OFF);
    let end_low = TM0CNT_L.read() as u32;
    let end_high = TM1CNT_L.read() as u32;
    end_high << 16 | end_low
  }};
}
