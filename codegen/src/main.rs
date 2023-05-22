#![deny(clippy::all)]

mod bindings;
mod build;
mod resource;

use codegen_libc::{search, CfgItem, RegexSet};
use codegen_writer::{scoped, Codegen};

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
    search(libc_path, &re).unwrap()
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
