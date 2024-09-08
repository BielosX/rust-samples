pub mod macros {
    #[macro_export]
    macro_rules! count_tts {
        () => { 0 };
        ($odd:tt $($a:tt $b:tt)*) => { (count_tts!($($a)*) << 1) | 1 };
        ($($a:tt $even:tt)*) => { count_tts!($($a)*) << 1 };
    }
}