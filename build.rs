use rustc_version::Channel;

fn main() {
	println!("cargo::rustc-check-cfg=cfg(nightly)");
    if rustc_version::version_meta().unwrap().channel == Channel::Nightly {
        println!("cargo:rustc-cfg=nightly");
    }
}
