fn main() {
    let out_file = "rsrt0.o";
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let profile = std::env::var("PROFILE").unwrap();
    let out_dir_file = format!("{}/{}", out_dir, out_file);
    let mut debug = vec![];
    if profile == "debug" {
        debug.push("-g");
    }
    let as_output = std::process::Command::new("arm-none-eabi-as")
        .args(&["-o", out_dir_file.as_str()])
        .arg("-mthumb-interwork")
        .arg("-mcpu=arm7tdmi")
        .arg("src/rsrt0.S")
        .args(&debug)
        .output()
        .expect("failed to run arm-none-eabi-as");
    if !as_output.status.success() {
        panic!("{}", String::from_utf8_lossy(&as_output.stderr));
    }
    //
    println!("cargo:rustc-link-search={}", out_dir);
}
