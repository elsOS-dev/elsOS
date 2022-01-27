fn main()
{
	println!("cargo:rustc-link-search=native=./");
	println!("cargo:rustc-link-lib=static=c");
	println!("cargo:rerun-if-changed=libc.a");
}
