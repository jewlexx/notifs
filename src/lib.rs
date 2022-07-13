#[cxx::bridge]
mod ffi {
    enum WinToastDismissalReason {
        UserCanceled = 0,
        ApplicationHidden = 1,
        TimedOut = 2,
    }

    unsafe extern "C++" {
        include!("../include/WinToasts/src/wintoastlib.h");

        include!("lib.cpp");

        fn IsCompatible() -> u8;
    }
}

pub fn is_compatible() -> bool {
    ffi::IsCompatible() == 0
}
