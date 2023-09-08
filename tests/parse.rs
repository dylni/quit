#![cfg(feature = "__unstable_tests")]

use quit::__Result;
use quit::tests_common as common;

#[test]
fn test_empty() {
    #[quit::main]
    fn main() {}

    common::assert_result((), main());
}

#[test]
fn test_success() {
    #[quit::main]
    fn main() {
        if !"hello world".contains('h') {
            panic!("hello world");
        }
    }

    common::assert_result((), main());
}

#[should_panic(expected = "hello world")]
#[test]
fn test_panic() {
    #[quit::main]
    fn main() {
        panic!("hello world");
    }

    common::assert_result((), main());
}

#[test]
fn test_return() {
    #[quit::main]
    fn main() -> Result<(), ()> {
        Ok(())
    }

    common::assert_result(Ok(()), main());
}

#[test]
const fn test_complex_signature() {
    mod main {
        #[allow(improper_ctypes_definitions)]
        #[quit::main]
        #[quit::main]
        pub(super) async unsafe extern "C" fn main(
            _u1: u8,
            _u2: u16,
        ) -> Result<(), u32> {
            Ok(())
        }
    }

    async fn _test_type() -> __Result<__Result<Result<(), u32>>> {
        // SAFETY: Nothing unsafe is used.
        unsafe { main::main(0, 0).await }
    }
}
