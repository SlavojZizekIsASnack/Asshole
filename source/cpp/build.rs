fn main() {
    cc::Build::new()
        .file("simple/main.cpp")
        .cpp(true)
        .compile("libassholeai.a");
}
