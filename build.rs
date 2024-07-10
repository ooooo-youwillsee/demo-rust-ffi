
fn main() {
    println!("cargo:rustc-link-search=native={}", "./libgit2/build");
}
