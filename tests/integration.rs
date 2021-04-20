use std::io;
use std::mem::ManuallyDrop;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Stdio;

use lazy_static::lazy_static;

lazy_static! {
    static ref TEST_PROCESS_BUILD: io::Result<ExitStatus> = {
        let mut command = Command::new("cargo");
        let _ = command.arg("build");
        if cfg!(not(debug_assertions)) {
            let _ = command.arg("--release");
        }
        command.arg("--quiet").current_dir("test_process").status()
    };
}

fn test_exit_code(exit_code: i32) -> io::Result<()> {
    assert!(TEST_PROCESS_BUILD
        .as_ref()
        .expect("failed test process build")
        .success());

    let output = Command::new(format!(
        "test_process/target/{}/test_process",
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
    ))
    .arg(exit_code.to_string())
    .stderr(Stdio::inherit())
    .output()?;

    assert_eq!(Some(exit_code), output.status.code());
    assert_eq!(b"dropped\n", &*output.stdout);

    Ok(())
}

#[test]
fn test_empty() {
    #[quit::main]
    fn main() {}

    main();
}

#[test]
fn test_success() {
    #[quit::main]
    fn main() {
        if !"hello world".contains('h') {
            panic!("hello world");
        }
    }

    main();
}

#[should_panic(expected = "hello world")]
#[test]
fn test_panic() {
    #[quit::main]
    fn main() {
        panic!("hello world");
    }

    main();
}

#[test]
fn test_return() {
    #[quit::main]
    fn main() -> Result<(), ()> {
        Ok(())
    }

    assert_eq!(Ok(()), main());
}

#[test]
fn test_code_0() -> io::Result<()> {
    test_exit_code(0)
}

#[test]
fn test_code_1() -> io::Result<()> {
    test_exit_code(1)
}

#[test]
fn test_code_2() -> io::Result<()> {
    test_exit_code(2)
}

#[test]
fn test_code_10() -> io::Result<()> {
    test_exit_code(10)
}

#[test]
fn test_complex_signature() {
    mod main {
        use std::mem::ManuallyDrop;

        #[quit::main]
        #[quit::main]
        pub(super) unsafe extern "C" fn main<T: Copy>() -> ManuallyDrop<()>
        where
            T: Clone,
        {
            ManuallyDrop::new(())
        }
    }

    unsafe {
        ManuallyDrop::drop(&mut main::main::<()>());
    }
}
