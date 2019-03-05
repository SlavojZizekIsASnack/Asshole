fn main() {
    cc::Build::new()
        .file("cpp_ai/main.cpp")
        .cpp(true)
        .compile("libassholeai.a");
}