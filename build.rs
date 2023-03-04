fn main() {
    cc::Build::new()
        .file("src/answer.c")
        .opt_level(2)
        // .flag("-mabi=lp64d") // Uncomment to work around
        .compile("answer");
}
