#[macro_use]
extern crate log;

#[macro_use]
extern crate quick_error;

extern crate rayon;

pub mod commands;
pub mod config;
pub mod fscrawling;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
