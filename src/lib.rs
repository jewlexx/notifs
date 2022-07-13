enum WinToastDismissalReason {
    UserCanceled,
    ApplicationHidden,
    TimedOut,
}

#[cxx::bridge]
mod ffi {
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
