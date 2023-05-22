#![deny(clippy::all)]

mod bindings;

fn main() {
    let item_list = self::bindings::collect_item_list();

    {
        let path = "src/bindings.rs";
        let gen = codegen_writer::Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, || self::bindings::codegen(&item_list));
    }
}
