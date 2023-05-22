use codegen_cfg::ast::*;
use codegen_libc::{search, simplified_expr, CfgItem, RegexSet};
use codegen_writer::g;
use codegen_writer::glines;

fn main() {
    let path = "src/bindings.rs";
    let gen = codegen_writer::Codegen::create_file(path).unwrap();
    codegen_writer::scoped(gen, codegen);
}

fn codegen() {
    let libc_path = "temp/libc";
    let re = RegexSet::new(["RLIM", "rlimit", "RLIMIT_"]).unwrap();

    let item_list = search(libc_path, &re).unwrap();

    glines!(
        "#![allow(clippy::cast_possible_truncation)]",
        "#![allow(clippy::unnecessary_cast)]",
        "",
    );

    codegen_64(&item_list);
    codegen_inf(&item_list);
    codegen_resources(&item_list);
}

fn codegen_64(item_list: &[CfgItem]) {
    for name in ["rlimit", "getrlimit", "setrlimit"] {
        let name64 = format!("{}64", name);
        let item64 = item_list.iter().find(|item| item.name == name64).unwrap();
        let cfg64 = item64.cfg.clone();

        let item = item_list.iter().find(|item| item.name == name).unwrap();
        let cfg = item.cfg.clone();

        g!("#[cfg({cfg64})]");
        g!("pub use libc::{name64} as {name};");
        g!();

        let otherwise = simplified_expr(all((not(cfg64), cfg)));

        g!("#[cfg({otherwise})]");
        g!("pub use libc::{name};");
        g!();
    }
}

fn codegen_inf(item_list: &[CfgItem]) {
    let name = "RLIM_INFINITY";
    let item = item_list.iter().find(|item| item.name == name).unwrap();
    let cfg = &item.cfg;

    g!("#[cfg({cfg})]");
    g!("pub const {name}: u64 = libc::{name} as u64;");
    g!();

    g!("#[cfg(not({cfg}))]");
    g!("pub const {name}: u64 = u64::MAX;");
    g!();
}

fn codegen_resources(item_list: &[CfgItem]) {
    let resources = {
        let mut ans = Vec::new();
        for item in item_list {
            let name = item.name.as_str();

            // TODO
            if matches!(name, "RLIMIT_NLIMITS" | "RLIMIT_THREADS") {
                continue;
            }

            if name.starts_with("RLIMIT_") {
                ans.push(item);
            }
        }
        ans
    };

    for item in &resources {
        let name = item.name.as_str();
        let cfg = &item.cfg;

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

    for item in &resources {
        let name = item.name.as_str();
        let cfg = &item.cfg;

        g!("#[cfg({cfg})]");
        g!("assert!((0..128).contains(&libc::{name}));");
        g!();
    }

    g!("}}");

    g!("}}");
}
