use std::env;
use std::str::FromStr;

pub fn parse_arg<T: FromStr>(n: usize) -> Option<Result<T, T::Err>> {
    match env::args().nth(n) {
        Some(opt) => Some(opt.parse::<T>()),
        None      => None,
    }
}