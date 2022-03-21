fn main()
{
	println!("cargo:rustc-link-search=native=./build/libc");
	println!("cargo:rustc-link-lib=static=c");
	println!("cargo:rerun-if-changed=build/libc");
}
