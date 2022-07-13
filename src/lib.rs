enum WinToastDismissalReason {
    UserCanceled,
    ApplicationHidden,
    TimedOut,
}

extern "C" {
    fn IsCompatible() -> cty::c_int;

}

pub fn is_compatible() -> bool {
    unsafe { IsCompatible() == 0 }
}
