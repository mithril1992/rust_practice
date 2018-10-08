extern crate rust_sample;

#[link(name="c_hello", kind="static")]
extern {
    fn hello();
}

fn main() {
    unsafe {
        hello();
    };
    rust_sample::foo::r_hello();
}
