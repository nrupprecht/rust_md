
#[macro_export]
macro_rules! assert_close {
    ($x:expr, $y:expr, $delta:expr) => {
        if !($x - $y < $delta || $y - $x < $delta) { panic!(); }
    };
}