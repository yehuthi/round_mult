use rustc_version::Channel;

fn main() {
    if rustc_version::version_meta().unwrap().channel == Channel::Nightly {
        println!("cargo:rustc-cfg=nightly");
    }
}
