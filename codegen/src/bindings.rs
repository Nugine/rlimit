use codegen_cfg::ast::*;
use codegen_libc::{simplified_expr, CfgItem};
use codegen_writer::g;
use codegen_writer::glines;

pub fn codegen(item_list: &[CfgItem]) {
    glines![
        "#![allow(clippy::cast_possible_truncation)]"
        "#![allow(clippy::unnecessary_cast)]"
        ""
    ];

    codegen_64(item_list);
    codegen_inf(item_list);
    codegen_resources(item_list);
}

fn codegen_64(item_list: &[CfgItem]) {
    for name in ["rlimit", "getrlimit", "setrlimit", "prlimit"] {
        let name64 = format!("{}64", name);
        let item64 = item_list.iter().find(|item| item.name == name64).unwrap();
        let cfg64 = item64.cfg.clone();

        let item = item_list.iter().find(|item| item.name == name).unwrap();
        let cfg = item.cfg.clone();

        g!("#[cfg({cfg64})]");
        g!("pub use libc::{name64} as {name};");
        g!();

        let otherwise = simplified_expr(all((not(cfg64), cfg)));
        if otherwise.is_const_false() {
            assert_eq!(name, "prlimit");
        } else {
            g!("#[cfg({otherwise})]");
            g!("pub use libc::{name};");
            g!();
        }
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

            if name == "RLIMIT_NLIMITS" {
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
