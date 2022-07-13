fn main() {
    cc::Build::new()
        .cpp(true)
        .include("include/WinToast/src")
        .file("src/lib.cpp")
        .compile("wintoast");
}
