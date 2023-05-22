use codegen_cfg::ast::{all, Expr};
use codegen_libc::{simplified_expr, CfgItem};
use codegen_writer::g;
use rust_utils::iter::map_collect_vec;

fn find<'a>(item_list: &'a [CfgItem], name: &str) -> &'a CfgItem {
    item_list.iter().find(|item| item.name == name).unwrap()
}

fn find_many_cfg(item_list: &[CfgItem], items: &[&str]) -> Vec<Expr> {
    map_collect_vec(items, |name| find(item_list, name).cfg.clone())
}

fn set_cfg_if(key: &str, cfg: &Expr) {
    g!("let {key} = cfg!({cfg});");
    g!("if {key} {{");
    g!(r#"println!("cargo:rustc-cfg=rlimit__{key}");"#);
    g!("}}");
    g!();
}

fn forward_item_cfg(item_list: &[CfgItem], name: &str) {
    let item = find(item_list, name);
    let key = format!("has_{}", name);
    set_cfg_if(&key, &item.cfg);
}

pub fn codegen(item_list: &[CfgItem]) {
    g!("fn main() {{");

    forward_item_cfg(item_list, "prlimit64");

    {
        let cfg = simplified_expr(all(find_many_cfg(
            item_list,
            &["CTL_KERN", "KERN_MAXFILESPERPROC", "sysctl"],
        )));
        set_cfg_if("get_kern_max_files_per_proc", &cfg);
    }

    g!("}}")
}
