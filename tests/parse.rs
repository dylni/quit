use std::mem::ManuallyDrop;

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

    // SAFETY: Nothing unsafe is used.
    unsafe {
        ManuallyDrop::drop(&mut main::main::<()>());
    }
}
