use std::io;
use std::process::Command;
use std::process::Stdio;

fn test(exit_code: i32) -> io::Result<()> {
    let output = Command::new(env!("CARGO_BIN_EXE_dropper"))
        .arg(exit_code.to_string())
        .stderr(Stdio::inherit())
        .output()?;

    assert_eq!(b"dropped\n", &*output.stdout);
    assert_eq!(Some(exit_code), output.status.code());

    Ok(())
}

#[test]
fn test_dropper() -> io::Result<()> {
    test(0)?;
    test(1)?;
    test(2)?;
    test(10)
}
