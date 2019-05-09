fn main() {
    cc::Build::new()
        .file("small/small.cpp")
        .cpp(true)
        .compile("libsmall.a");
}
