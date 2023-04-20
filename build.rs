
pub fn main() {
	let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
	println!("cargo:rerun-if-changed={}/link.x", manifest_dir);
	println!("cargo:rustc-link-arg=-T{}/link.x", manifest_dir);
}