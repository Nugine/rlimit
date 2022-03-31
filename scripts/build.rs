fn main() {
    let vars = [
        "TARGET",
        "CARGO_CFG_TARGET_FAMILY",
        "CARGO_CFG_TARGET_OS",
        "CARGO_CFG_TARGET_ARCH",
        "CARGO_CFG_TARGET_VENDOR",
        "CARGO_CFG_TARGET_ENV",
    ];

    use std::io::Write;

    let mut file = std::fs::File::create("target/vars.txt").unwrap();

    for v in vars {
        let result = std::env::var(v);
        if let Ok(s) = result {
            writeln!(&mut file, "{}={}", v, s).unwrap()
        }
    }
}
