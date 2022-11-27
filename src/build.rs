/// Symlink the executable into ~/.local/bin
#[cfg(target_os = "linux")]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    use std::env;
    use std::path::PathBuf;
    use std::str::FromStr;

    let home = match env::var("HOME") {
        Ok(home) => match PathBuf::from_str(home.as_str()) {
            Ok(home) => home,
            Err(_) => return,
        },
        Err(_) => return,
    };

    let cargo_bin = home.join(".cargo/bin");
    let local_bin = home.join(".local/bin");
    let cargo_bin_x = cargo_bin.join("wallpapergen");
    let local_bin_x = local_bin.join("wallpapergen");

    if std::fs::create_dir_all(&local_bin).is_ok()
        && std::os::unix::fs::symlink(&cargo_bin_x, &local_bin_x).is_ok()
    {
        println!(
            "Created symlink: {:?} -> {:?}",
            local_bin.display(),
            cargo_bin.display()
        );
    }
}
