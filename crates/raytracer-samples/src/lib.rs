#[cfg(test)]
mod samples;

#[macro_export]
macro_rules! assert_eq_ppm {
    ($content: expr, $target: literal) => {
        assert_eq!($content, include_str!($target))
    };
}
