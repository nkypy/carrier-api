#![deny(unsafe_code)]

#[macro_use]
extern crate serde;
extern crate xmlparser;

mod de;
mod ser;
mod error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
