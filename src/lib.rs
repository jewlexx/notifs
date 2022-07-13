extern "C" {
    fn IsCompatible() -> cty::c_int;
}

pub unsafe fn is_compatible() -> bool {
    IsCompatible() == 0
}
