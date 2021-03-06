// extern crate handlebars;

#[macro_use]
extern crate log;

extern crate pathdiff;

#[macro_use]
extern crate quick_error;

extern crate rayon;

pub mod application;

pub mod commands;

pub mod fs;

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
