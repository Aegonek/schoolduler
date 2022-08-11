macro_rules! verbose {
    ($($x:tt)*) => {
        #[cfg(feature = "verbose")]
            println!($( $x )*);
    }
}

pub(crate) use verbose;