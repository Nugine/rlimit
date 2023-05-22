#![deny(clippy::all)]

mod bindings;
mod resource;

fn main() {
    let item_list = self::bindings::collect_item_list();
    let resources = self::resource::collect_resources(&item_list);

    {
        let path = "src/bindings.rs";
        let gen = codegen_writer::Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, || self::bindings::codegen(&item_list));
    }

    {
        let path = "src/resource/generated.rs";
        let gen = codegen_writer::Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, || self::resource::codegen(&resources));
    }
}
