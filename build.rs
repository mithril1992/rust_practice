extern crate gcc;

fn main() {
    gcc::Build::new()
        .file("src/c/c_hello.c")
        .include("src/bin")
        .compile("libc_hello.a")
}