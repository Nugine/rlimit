use std::fmt::Write as _;

use bool_logic::cfg::ast::{all, All, Any, Expr, Not, Var};
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
            let pred = &expr.as_var().unwrap().0;
            let val = pred.value.as_ref().unwrap();
            write!(s, "{} != {:?}", pred.key, val).unwrap();
        }
        Expr::Var(Var(pred)) => {
            let val = pred.value.as_ref().unwrap();
            write!(s, "{} == {:?}", pred.key, val).unwrap();
        }
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

pub fn codegen(item_list: &[CfgItem]) {
    g!("fn main() {{");

    {
        g!("let target_os = std::env::var(\"CARGO_CFG_TARGET_OS\").unwrap();");
        g!("let target_env = std::env::var(\"CARGO_CFG_TARGET_ENV\").unwrap();");
        g!("let target_arch = std::env::var(\"CARGO_CFG_TARGET_ARCH\").unwrap();");
        g!();
    }

    {
        let extra_os = ["switch"];
        let values = extra_os.join("\",\"");
        g!(r#"println!("cargo:rustc-check-cfg=cfg(target_os, values(\"{values}\"))");"#)
    }

    forward_item_cfg(item_list, "prlimit64");

    {
        let cfg = simplified_expr(all(find_many_cfg(
            item_list,
            &["CTL_KERN", "KERN_MAXFILESPERPROC", "sysctl"],
        )));
        set_cfg_if("get_kern_max_files_per_proc", &cfg);
    }

    {
        g!(r#"
        let asm_syscall = cfg!(feature = "asm_syscall")
            && target_arch == "x86_64"
            && (target_os == "linux" || target_os == "android");
        println!("cargo:rustc-check-cfg=cfg(rlimit__asm_syscall)");
        if asm_syscall {{
            println!("cargo:rustc-cfg=rlimit__asm_syscall");
        }}
        "#);
    }

    g!("}}")
}
