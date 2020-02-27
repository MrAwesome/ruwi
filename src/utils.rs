use std::thread;
use crate::errors::*;

#[inline]
pub(crate) fn await_thread<T>(handle: thread::JoinHandle<T>) -> Result<T, RuwiError> {
    handle.join().or_else(|_| {
        Err(rerr!(
            RuwiErrorKind::FailedToSpawnThread,
            "Failed to spawn thread."
        ))
    })
}

pub(crate) fn loop_check(loop_protection: &mut u16, loop_max: u16) -> Result<(), RuwiError> {
    if *loop_protection >= loop_max {
        return Err(rerr!(
            RuwiErrorKind::LoopProtectionMaxExceeded,
            format!("Infinite loop suspected in main loop! If you just retried {} times by hand, consider the author very impressed and feel free to make a diff to bump this number. Exiting...", loop_max)
        ));
    }

    *loop_protection += 1;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_loop_check_allows_at_least_5_loops() -> Result<(), RuwiError> {
        let mut start = 0;
        let max = 1000;
        for _ in 1..5 {
            loop_check(&mut start, max)?;
        }
        assert_eq![start, 4];
        loop_check(&mut start, max)
    }

    #[test]
    fn test_loop_failure() -> Result<(), RuwiError> {
        let mut start = 0;
        let max = 2;

        loop_check(&mut start, max)?;
        loop_check(&mut start, max)?;

        let final_loop_res = loop_check(&mut start, max);

        if let Err(RuwiError{kind: RuwiErrorKind::LoopProtectionMaxExceeded, desc: _}) = final_loop_res {
            Ok(())
        } else {
            dbg!(&final_loop_res);
            panic!("Loop protection did not fail when expected!");
        }
            
    }
}
