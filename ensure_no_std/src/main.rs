#![feature(start)]

#![deny(warnings)]

#![no_std]

#[cfg(windows)]
#[link(name="msvcrt")]
extern { }

mod no_std {
    use core::panic::PanicInfo;
    use exit_no_std::exit;

    #[panic_handler]
    extern fn panic(_info: &PanicInfo) -> ! {
        exit(99)
    }
}

use arrayvec::ArrayVec;
use iter_identify_first_last::IteratorIdentifyFirstLastExt;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    assert_eq!(
        [1, 2, 3, 4].into_iter().identify_first_last().collect::<ArrayVec<_, 4>>().as_slice(),
        &[(true, false, 1), (false, false, 2), (false, false, 3), (false, true, 4)]
    );
    0
}
