#[allow(unused)]
macro_rules! impl_gpio_pins {
    ($($px: ident:($P: expr, $N: expr);)+) => {
/// GPIO pads available from SyterKit.
pub struct Pads {
    $(
    pub $px: ::allwinner_rt::soc::d1::Pad<$P, $N>,
    )+
}

impl Pads {
    #[doc(hidden)]
    #[inline]
    pub fn __new() -> Self {
        Self {
            $(
            $px: ::allwinner_rt::soc::d1::Pad::__new(),
            )+
        }
    }
}
    };
}

/// Prints to the standard output.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::_print(core::format_args!($($arg)*));
    }
}

/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => {{
        $crate::_print(core::format_args!($($arg)*));
        $crate::println!();
    }}
}
