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

pub type Result<T> = std::result::Result<T, Error>;
