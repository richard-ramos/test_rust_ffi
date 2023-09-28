fn main() {
    println!("cargo:rustc-link-arg=-lmyfunc");
    println!("cargo:rustc-link-arg=-L../clib/");
}
