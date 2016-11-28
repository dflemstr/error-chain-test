#[macro_use]
extern crate error_chain;
extern crate a;

error_chain! {
  links {
    a::Error, A;
  }
}

