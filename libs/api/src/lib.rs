extern crate handlebars;

#[macro_use]
extern crate log;

extern crate pathdiff;

#[macro_use]
extern crate quick_error;

extern crate rayon;

// extern crate text_table;

pub mod commands;
pub mod context;
pub mod fscrawling;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
