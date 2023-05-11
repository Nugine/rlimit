use codegen_writer::g;
use codegen_writer::glines;
use libc_cfg::generate_item_cfg;

fn main() {
    let path = "src/bindings.rs";
    let gen = codegen_writer::Codegen::create_file(path).unwrap();
    codegen_writer::scoped(gen, codegen);
}

fn codegen() {
    let libc_repo_path = "temp/libc";
    let item_list = libc_cfg::generate_item_list(libc_repo_path).unwrap();

    glines!(
        "#![allow(clippy::cast_possible_truncation)]",
        "#![allow(clippy::unnecessary_cast)]",
        "",
    );

    codegen_64(&item_list);
    codegen_inf(&item_list);
    codegen_resources(&item_list);
}

fn codegen_64(item_list: &[libc_cfg::Item]) {
    for name in ["rlimit", "getrlimit", "setrlimit"] {
        let name64 = format!("{}64", name);
        let item64 = item_list.iter().find(|item| item.name == name64).unwrap();
        let cfg64 = generate_item_cfg(item64);

        let item = item_list.iter().find(|item| item.name == name).unwrap();
        let cfg = generate_item_cfg(item);

        g!("#[cfg({cfg64})]");
        g!("pub use libc::{name64} as {name};");
        g!();

        g!("#[cfg(all(not({cfg64}), {cfg}))]");
        g!("pub use libc::{name};");
        g!();
    }
}

fn codegen_inf(item_list: &[libc_cfg::Item]) {
    let name = "RLIM_INFINITY";
    let item = item_list.iter().find(|item| item.name == name).unwrap();
    let cfg = generate_item_cfg(item);

    g!("#[cfg({cfg})]");
    g!("pub const {name}: u64 = libc::{name} as u64;");
    g!();

    g!("#[cfg(not({cfg}))]");
    g!("pub const {name}: u64 = u64::MAX;");
    g!();
}

fn codegen_resources(item_list: &[libc_cfg::Item]) {
    let resources = {
        let mut ans = Vec::new();
        for item in item_list {
            let name = item.name.as_str();

            // TODO
            if matches!(name, "RLIMIT_NLIMITS" | "RLIMIT_THREADS") {
                continue;
            }

            if name.starts_with("RLIMIT_") {
                let cfg = generate_item_cfg(item);
                ans.push((item, cfg));
            }
        }
        ans
    };

    for (item, cfg) in &resources {
        let name = item.name.as_str();

        g!("#[cfg({cfg})]");
        g!("pub const {name}: u8 = libc::{name} as u8;");
        g!();

        g!("#[cfg(not({cfg}))]");
        g!("pub const {name}: u8 = u8::MAX;");
        g!();
    }

    g!("#[cfg(test)]");
    g!("mod tests {{");

    g!("#[allow(clippy::too_many_lines)]");
    g!("#[test]");
    g!("fn resource_range() {{");

    for (item, cfg) in &resources {
        let name = item.name.as_str();

        g!("#[cfg({cfg})]");
        g!("assert!((0..128).contains(&libc::{name}));");
        g!();
    }

    g!("}}");

    g!("}}");
}
