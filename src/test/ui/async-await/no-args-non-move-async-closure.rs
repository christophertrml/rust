// edition:2018

#![feature(async_closure)]

fn main() {
    let _ = async |x: u8| {};
    //~^ ERROR `async` non-`move` closures with arguments are not currently supported
}
