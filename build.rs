use std::process::Command;
use std::env;
use std::path::Path;

const RPI_RUST_TARGET : &'static str = "arm-unknown-linux-gnueabihf";

fn main() {
  let root = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
  let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
  let target = env::var("TARGET").expect("TARGET not set");

  let (cc, ar) = match target.as_str() {
    RPI_RUST_TARGET =>
      (rpi_cmd("cc", root.as_str()), rpi_cmd("ar", root.as_str())),
    _ =>
      ("cc".to_owned(), "ar".to_owned())
  };

  // note that there are a number of downsides to this approach, the comments
  // below detail how to improve the portability of these commands.
  Command::new(cc.as_str()).args(&["src/io/sources/file/linux/aio.c", "-c", "-fPIC", "-o"])
                     .arg(&format!("{}/aio.o", out_dir))
                     .status().expect("cc failed");
  Command::new(ar.as_str()).args(&["crus", "libwwwee-aio.a", "aio.o"])
                    .current_dir(&Path::new(&out_dir))
                    .status().expect("ar failed");

  println!("cargo:rustc-link-search=native={}", out_dir);
  println!("cargo:rustc-link-lib=static=wwwee-aio");
}

fn rpi_cmd(cmd: &'static str, root: &str) -> String {
  const RPI_CC_TARGET : &'static str = "arm-linux-gnueabihf";
  
  format!("{root}/tools/cross_compilers/rpi/arm-bcm2708/{cc_target}/bin/{cc_target}-{cmd}",
    root = root,
    cc_target = RPI_CC_TARGET,
    cmd = cmd)
}
