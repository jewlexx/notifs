#[cxx::bridge]
mod ffi {
    enum WinToastDismissalReason {
        UserCanceled = 0,
        ApplicationHidden = 1,
        TimedOut = 2,
    }

    extern "C++" {
        include!("../include/WinToasts/src/wintoastlib.h");
    }
}

extern "C" {
    fn IsCompatible() -> cty::c_int;

}

pub fn is_compatible() -> bool {
    unsafe { IsCompatible() == 0 }
}
