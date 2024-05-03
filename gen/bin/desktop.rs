fn main() {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    let _ = rs_mobile::main();
}
