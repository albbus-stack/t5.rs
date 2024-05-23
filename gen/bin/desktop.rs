#![windows_subsystem = "windows"]
fn main() {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    let _ = t5_rs::main();
}
