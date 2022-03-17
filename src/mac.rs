#[macro_export]
macro_rules! bi {
    ($x: literal, $base: literal) => {
        BigInt::parse_bytes($x.as_bytes(), $base).unwrap()
    };
}
