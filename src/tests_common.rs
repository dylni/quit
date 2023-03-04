use std::fmt::Debug;
use std::process::Termination;

use super::ResultInner;
use super::__Result;

impl<T> __Result<T>
where
    T: Termination,
{
    #[track_caller]
    fn unwrap(self) -> T {
        use ResultInner as Inner;

        match self.0 {
            Inner::Result(result) => result,
            Inner::ExitCode(exit_code) => panic!(
                "called `__Result::unwrap()` on an `ExitCode` value: {:?}",
                exit_code,
            ),
        }
    }
}

#[inline]
#[track_caller]
pub fn assert_result<T>(expected: T, result: __Result<T>)
where
    T: Debug + PartialEq<T> + Termination,
{
    assert_eq!(expected, result.unwrap());
}
