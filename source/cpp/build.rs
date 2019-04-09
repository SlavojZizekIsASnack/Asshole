fn main() {
    cc::Build::new()
        .file("simple/simple.cpp")
        .cpp(true)
        .compile("simple.a");
}
