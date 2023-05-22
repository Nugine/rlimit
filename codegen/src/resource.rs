use std::collections::HashMap;

use codegen_libc::CfgItem;
use codegen_writer::g;
use codegen_writer::glines;
use rust_utils::default::default;

pub struct Resource {
    ident: String,
    name: String,
    tag: usize,
}

pub fn collect_resources(item_list: &[CfgItem]) -> Vec<Resource> {
    let mut ans: Vec<Resource> = default();

    for item in item_list {
        let name = item.name.as_str();

        if name == "RLIMIT_NLIMITS" {
            continue;
        }

        if let Some(ident) = name.strip_prefix("RLIMIT_") {
            ans.push(Resource {
                ident: ident.to_owned(),
                name: name.to_owned(),
                tag: 0,
            });
        }
    }

    ans.sort_by(|lhs, rhs| Ord::cmp(&lhs.ident, &rhs.ident));
    ans.iter_mut().enumerate().for_each(|(i, r)| r.tag = i + 1);

    ans
}

fn load_docs() -> HashMap<&'static str, Vec<&'static str>> {
    let content = include_str!("./resource.md");

    let mut paragraphs: HashMap<&str, Vec<&str>> = default();
    let mut iter = content.lines().peekable();

    while let Some(line) = iter.next() {
        let heading = line.strip_prefix("# ").unwrap();

        while let Some(s) = iter.peek() {
            if s.is_empty() {
                iter.next();
            } else {
                break;
            }
        }

        let mut lines: Vec<&str> = default();
        while let Some(s) = iter.peek() {
            if s.starts_with("# ") {
                break;
            } else {
                lines.push(iter.next().unwrap());
            }
        }

        while let Some(s) = lines.last() {
            if s.is_empty() {
                lines.pop();
            } else {
                break;
            }
        }

        assert!(paragraphs.insert(heading, lines).is_none());
    }

    paragraphs
}

pub fn codegen(resources: &[Resource]) {
    let docs = load_docs();

    glines![
        "use super::Resource;",           //
        "use super::ParseResourceError;", //
        "use crate::bindings as C;",      //
        "",
    ];

    {
        g!("impl Resource {{");

        for res in resources {
            for doc in &docs[res.name.as_str()] {
                g!("/// {doc}");
            }

            g!(
                "pub const {}: Self = Self {{ tag: {}, value: C::{} }};",
                res.ident,
                res.tag,
                res.name
            );
            g!();
        }

        g!("}}");
        g!();
    }

    {
        g!("impl Resource {{");

        g!("pub(super) const fn find_name_by_tag(tag: u8) -> Option<&'static str> {{");
        g!("    match tag {{");
        for res in resources {
            g!("    {} => Some({:?}),", res.tag, res.name);
        }
        g!("        _ => None,");
        g!("    }}");
        g!("}}");
        g!();

        g!("pub(super) const fn find_ident_by_tag(tag: u8) -> Option<&'static str> {{");
        g!("    match tag {{");
        for res in resources {
            g!("    {} => Some({:?}),", res.tag, res.ident);
        }
        g!("        _ => None,");
        g!("    }}");
        g!("}}");
        g!();

        g!("}}");
    }

    {
        g!("impl std::str::FromStr for Resource {{");
        g!("    type Err = ParseResourceError;");
        g!();
        g!("    fn from_str(s: &str) -> Result<Self, Self::Err> {{");
        g!("        match s {{");
        for res in resources {
            g!("            {:?} => Ok(Self::{}),", res.name, res.ident);
        }
        g!("            _ => Err(ParseResourceError(())),");
        g!("        }}");
        g!("    }}");
        g!("}}");
    }
}
