// extern crate handlebars;

#[macro_use]
extern crate log;

extern crate pathdiff;

#[macro_use]
extern crate quick_error;

extern crate rayon;

pub mod commands;

pub mod fscrawling;

pub mod notification;


quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: std::io::Error) {
            cause(err)
            description(err.description())
        }
        Utf8(err: std::str::Utf8Error) {
            description("utf8 error")
        }
        Message(str: &'static str) {
            description("error message")
        }
        Other(err: Box<std::error::Error>) {
            cause(&**err)
            description(err.description())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
