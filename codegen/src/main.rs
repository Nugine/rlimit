#![deny(clippy::all)]

mod bindings;
mod build;
mod resource;

use codegen_cfg::ast::{Pred, Var};
use codegen_cfg::bool_logic::ast::Expr;
use codegen_cfg::bool_logic::visit_mut::{walk_mut_expr, VisitMut};
use codegen_libc::{search, simplified_expr, CfgItem, RegexSet};
use codegen_writer::{scoped, Codegen};

fn patch_cfg_expr(expr: &mut Expr<Pred>) {
    struct Visitor;

    impl VisitMut<Pred> for Visitor {
        fn visit_mut_expr(&mut self, expr: &mut Expr<Pred>) {
            walk_mut_expr(self, expr);

            if let Expr::Var(Var(Pred { key, value: None })) = expr {
                // ignore `linux_time_bits64`
                if key.as_str() == "linux_time_bits64" {
                    *expr = Expr::Const(true)
                }
            }
        }
    }

    Visitor.visit_mut_expr(expr);
}

fn collect_item_list() -> Vec<CfgItem> {
    let libc_path = "temp/libc";
    let re = RegexSet::new([
        "RLIM",
        "rlimit",
        "RLIMIT_",
        "^CTL_KERN$",
        "^KERN_MAXFILESPERPROC$",
        "^sysctl$",
    ])
    .unwrap();

    let mut item_list = search(libc_path, &re).unwrap();

    for item in &mut item_list {
        patch_cfg_expr(&mut item.cfg);
        item.cfg = simplified_expr(item.cfg.clone());
    }

    item_list
}

fn main() {
    let item_list = collect_item_list();
    let resources = self::resource::collect_resources(&item_list);

    {
        let path = "src/bindings.rs";
        let gen = Codegen::create_file(path).unwrap();
        scoped(gen, || self::bindings::codegen(&item_list));
    }

    {
        let path = "src/resource/generated.rs";
        let gen = Codegen::create_file(path).unwrap();
        scoped(gen, || self::resource::codegen(&resources));
    }

    {
        let path = "build.rs";
        let gen = Codegen::create_file(path).unwrap();
        scoped(gen, || self::build::codegen(&item_list));
    }
}
