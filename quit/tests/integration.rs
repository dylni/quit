use std::io::Result as IoResult;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Stdio;

use lazy_static::lazy_static;

lazy_static! {
    static ref TEST_PROCESS_BUILD: IoResult<ExitStatus> = {
        let mut command = Command::new("cargo");
        command.arg("build");
        if cfg!(not(debug_assertions)) {
            command.arg("--release");
        }
        command
            .arg("--quiet")
            .current_dir("test_process")
            .spawn()
            .and_then(|mut x| x.wait())
    };
}

fn test_exit_code(exit_code: i32) -> IoResult<()> {
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
    .stdout(Stdio::piped())
    .spawn()?
    .wait_with_output()?;

    assert_eq!(Some(exit_code), output.status.code());
    assert_eq!(b"dropped\n", output.stdout.as_slice());

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
        println!("hello world");
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

    let _ = main();
}

#[test]
fn test_code_0() -> IoResult<()> {
    test_exit_code(0)
}

#[test]
fn test_code_1() -> IoResult<()> {
    test_exit_code(1)
}

#[test]
fn test_code_2() -> IoResult<()> {
    test_exit_code(2)
}

#[test]
fn test_code_10() -> IoResult<()> {
    test_exit_code(10)
}
