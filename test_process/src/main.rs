use std::env;

use lazy_static::lazy_static;

lazy_static! {
    static ref EXIT_CODE: i32 = env::args_os()
        .nth(1)
        .expect("missing argument")
        .into_string()
        .expect("invalid argument")
        .parse()
        .expect("invalid exit code");
}

#[inline(never)]
fn exit() {
    quit::with_code(*EXIT_CODE);
}

#[quit::main]
fn main() {
    let dropped = Dropped();

    exit();

    println!("unreachable");
    drop(dropped);

    struct Dropped();

    impl Drop for Dropped {
        fn drop(&mut self) {
            println!("dropped");
        }
    }
}
