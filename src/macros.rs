extern crate backtrace;

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! ic {
    ($($args:tt)*) => {};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! ice {
    ($($args:tt)*) => {};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! ic {
    () => {{
        let (line, file, formatter) = (line!(), file!(), $crate::PRINTER.read().unwrap());
        eprintln!("{}", formatter.ic(line, file));
    }};

    ($x:expr) => {{
        let (line, file, formatter) = (line!(), file!(), $crate::PRINTER.read().unwrap());
        eprintln!("{}", formatter.ic_expr(&$x, stringify!($x), line, file));
        $x
    }};

    ($annotation:expr, $x:expr) => {{
        let (line, file, formatter) = (line!(), file!(), $crate::PRINTER.read().unwrap());
        eprintln!(
            "{}",
            formatter.ic_annotated($annotation, &$x, stringify!($x), line, file)
        );
        $x
    }};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! ice {
    // Wrap the result in a block so the expanded code can be matched as an $expr in the testing macro.
    () => {{
        let backtrace = $crate::Backtrace::new();
        let parsed = $crate::parsed_backtrace::ParsedBacktrace::new(&backtrace);
        let formatter = &$crate::PRINTER.read().unwrap();
        eprintln!("{}", formatter.ice(parsed));
    }};

    ($x:expr) => {{
        let backtrace = $crate::Backtrace::new();
        let parsed = $crate::parsed_backtrace::ParsedBacktrace::new(&backtrace);
        let formatter = &$crate::PRINTER.read().unwrap();
        eprintln!("{}", formatter.ice_expr(&$x, stringify!($x), parsed));
        $x
    }};

    ($annotation: expr, $x:expr) => {{
        let backtrace = $crate::Backtrace::new();
        let parsed = $crate::parsed_backtrace::ParsedBacktrace::new(&backtrace);
        let formatter = &$crate::PRINTER.read().unwrap();
        eprintln!(
            "{}",
            formatter.ice_annotated($annotation, &$x, stringify!($x), parsed)
        );
        $x
    }};
}
