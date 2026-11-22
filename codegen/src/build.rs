use std::fmt::Write as _;

use bool_logic::cfg::ast::{all, any, All, Any, Expr, Not, Var};
use libc_cfg::{simplified_expr, CfgItem};
use scoped_writer::g;

fn find<'a>(item_list: &'a [CfgItem], name: &str) -> &'a CfgItem {
    item_list.iter().find(|item| item.name == name).unwrap()
}

fn find_many_cfg(item_list: &[CfgItem], items: &[&str]) -> Vec<Expr> {
    items
        .iter()
        .map(|name| find(item_list, name).cfg.clone())
        .collect()
}

fn cfg_eval(s: &mut String, depth: usize, cfg: &Expr) {
    match cfg {
        Expr::Any(Any(any)) => {
            if depth > 0 {
                write!(s, "(").unwrap();
            }
            let (first, xs) = any.split_first().unwrap();
            cfg_eval(s, depth + 1, first);
            for x in xs {
                write!(s, " || ").unwrap();
                cfg_eval(s, depth + 1, x);
            }
            if depth > 0 {
                write!(s, ")").unwrap();
            }
        }
        Expr::All(All(all)) => {
            if depth > 0 {
                write!(s, "(").unwrap();
            }
            let (first, xs) = all.split_first().unwrap();
            cfg_eval(s, depth + 1, first);
            for x in xs {
                write!(s, " && ").unwrap();
                cfg_eval(s, depth + 1, x);
            }
            if depth > 0 {
                write!(s, ")").unwrap();
            }
        }
        Expr::Not(Not(expr)) => {
            if let Expr::Var(Var(pred)) = &**expr {
                if let Some(val) = &pred.value {
                    write!(s, "{} != {:?}", pred.key, val).unwrap();
                    return;
                }
            }
            write!(s, "!(").unwrap();
            cfg_eval(s, 0, expr);
            write!(s, ")").unwrap();
        }
        Expr::Var(Var(pred)) => match &pred.value {
            Some(val) => write!(s, "{} == {:?}", pred.key, val).unwrap(),
            // A bare flag such as `unix`; cargo exposes it as `CARGO_CFG_<KEY>`.
            None => {
                let var = format!("CARGO_CFG_{}", pred.key.as_str().to_uppercase());
                write!(s, "std::env::var({var:?}).is_ok()").unwrap();
            }
        },
        Expr::Const(_) => unimplemented!(),
    }
}

fn set_cfg_if(key: &str, cfg: &Expr) {
    let cfg = {
        let mut s = String::new();
        cfg_eval(&mut s, 0, cfg);
        s
    };

    g!("let {key} = {cfg};");
    g!(r#"println!("cargo:rustc-check-cfg=cfg(rlimit__{key})");"#);
    g!("if {key} {{");
    g!(r#"println!("cargo:rustc-cfg=rlimit__{key}");"#);
    g!("}}");
    g!();
}

fn forward_item_cfg(item_list: &[CfgItem], name: &str) {
    let item = find(item_list, name);
    let key = format!("has_{name}");
    set_cfg_if(&key, &item.cfg);
}

/// Forwards whether the `name` re-export exists.
///
/// The bindings alias `{name}64` to `{name}` where the 64-bit variant exists,
/// and fall back to `libc::{name}` otherwise, so either variant makes the
/// exported name callable.
fn forward_alias_item_cfg(item_list: &[CfgItem], name: &str) {
    let name64 = format!("{name}64");
    let cfg64 = find(item_list, &name64).cfg.clone();
    let cfg = find(item_list, name).cfg.clone();
    let key = format!("has_{name}");
    set_cfg_if(&key, &simplified_expr(any((cfg64, cfg))));
}

pub fn codegen(item_list: &[CfgItem]) {
    g!("fn main() {{");

    {
        g!("let target_os = std::env::var(\"CARGO_CFG_TARGET_OS\").unwrap();");
        g!("let target_env = std::env::var(\"CARGO_CFG_TARGET_ENV\").unwrap();");
        g!();
    }

    {
        let extra_os = ["switch", "qurt"];
        let values = extra_os.join("\\\",\\\"");
        g!(r#"println!("cargo:rustc-check-cfg=cfg(target_os, values(\"{values}\"))");"#);
        g!();
    }

    {
        // libc custom cfg variables (defined in libc's cfg_alias / build.rs;
        // unknown to rustc, so they must be declared explicitly)
        let extra_cfg = ["gnu_file_offset_bits64"];
        for name in extra_cfg {
            g!(r#"println!("cargo:rustc-check-cfg=cfg({name})");"#);
        }
        g!();
    }

    forward_item_cfg(item_list, "prlimit64");
    forward_alias_item_cfg(item_list, "getrlimit");
    forward_alias_item_cfg(item_list, "setrlimit");

    {
        let cfg = simplified_expr(all(find_many_cfg(
            item_list,
            &["CTL_KERN", "KERN_MAXFILESPERPROC", "sysctl"],
        )));
        set_cfg_if("get_kern_max_files_per_proc", &cfg);
    }

    g!("}}")
}
