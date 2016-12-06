#[macro_use]
extern crate error_chain;
extern crate a;

error_chain! {
    links {
        A(a::Error, a::ErrorKind);
    }
}
