#![deny(clippy::all)]

mod bindings;
mod build;
mod resource;

use std::fs::File;
use std::io::BufWriter;

use bool_logic::cfg::ast::Expr;
use bool_logic::cfg::ast::{Pred, Var};
use bool_logic::visit_mut::{walk_mut_expr, VisitMut};
use libc_cfg::{search, simplified_expr, CfgItem, RegexSet};

fn patch_cfg_expr(expr: &mut Expr) {
    struct Visitor;

    impl VisitMut<Pred> for Visitor {
        fn visit_mut_expr(&mut self, expr: &mut Expr) {
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

fn write_file(path: &str, f: impl FnOnce()) {
    let mut writer = BufWriter::new(File::create(path).unwrap());
    scoped_writer::scoped(&mut writer, f)
}

fn main() {
    let item_list = collect_item_list();
    let resources = self::resource::collect_resources(&item_list);

    {
        let path = "src/bindings.rs";
        write_file(path, || self::bindings::codegen(&item_list));
    }

    {
        let path = "src/resource/generated.rs";
        write_file(path, || self::resource::codegen(&resources));
    }

    {
        let path = "build.rs";
        write_file(path, || self::build::codegen(&item_list));
    }
}
