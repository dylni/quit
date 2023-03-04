use std::env;

#[inline(never)]
fn exit(exit_code: u8) -> ! {
    quit::with_code(exit_code);
}

#[allow(unreachable_code)]
#[quit::main]
fn main() {
    struct Dropped();

    impl Drop for Dropped {
        fn drop(&mut self) {
            println!("dropped");
        }
    }

    let _dropped = Dropped();

    let exit_code = env::args_os()
        .nth(1)
        .expect("missing argument")
        .into_string()
        .expect("invalid argument")
        .parse()
        .expect("invalid exit code");
    exit(exit_code);

    println!("unreachable");
}
