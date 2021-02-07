/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */
#![feature(with_options)]

extern crate proc_macro;

use std::{collections::HashMap, io::{Read, Write}, path::Path};

use fs2::FileExt;
use regex::Regex;
use syn::{DeriveInput, Lit, Meta, parse_macro_input};
use proc_macro::TokenStream;


#[proc_macro_derive(Component, attributes(component_ns))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    /* This is all awful. Never do this. Please. But anyway... */
    /* We make the macro stateful using IO relative to where the build command is run */
    let (ident, namespace) = parse_input(parse_macro_input!(input));
    let name = &format!("{}_{}", namespace, ident);
    let id = get_or_generate_id(name);
    format!(
        r#"impl Component for {} {{
            const ID: ComponentID = ComponentID({});
            const ID_STR: &'static str = "{}";
        }}"#, ident, id, name
    ).parse().unwrap()
}

fn parse_input<'a>(input: DeriveInput) -> (String, String) {
    let DeriveInput { attrs, ident, data, .. } = input;

    if !(matches!(data, syn::Data::Struct(_))) {
        panic!("Usage of #[Component] on a non-struct type");
    }

    let namespace = match attrs.get(0) {
        Some(v) => {
            if let Meta::NameValue(v) = v.parse_meta().unwrap() {
                if let Lit::Str(v) = v.lit {
                    let v = v.value();
                    if Regex::new(r"\s").unwrap().is_match(&v) { panic!("Namespace cannot contain whitespace") }
                    v
                } else {
                    panic!("Expected string literal");
                }
            } else {
                panic!("Expected namespace = '<value>'");
            }
        },
        _ => panic!("Namespace string required"),
    };

    return (ident.to_string(), namespace);
}

fn get_or_generate_id(name: &str) -> u64 {
    let base_dir = Path::new("./build/generated/butterscotch_ecs/");
    std::fs::create_dir_all(base_dir.clone()).expect("Could not create output directory");
    
    let filename = base_dir.join("gen_component_ids.txt");
    let mut file = std::fs::File::with_options().create(true).read(true).write(true).open(filename).expect("Could not open file.");
    file.lock_exclusive().expect("could not lock file");

    let mut contents = "".to_owned();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let id_map = construct_id_map(&contents);

    let id = match id_map.get(name) {
        Some(v) => *v,
        None => {
            let mut ids = id_map.values().collect::<Vec<_>>();
            ids.sort();

            let mut prev_id = 0;
            for id in ids {
                let diff = id - prev_id;
                if diff > 1 { break; }
                prev_id = *id;
            }

            let next_id = prev_id + 1;
            file.write_fmt(format_args!("{} {}\n", name, next_id)).expect("Could not write id to file");
            next_id
        }
    };

    file.unlock().expect("Failed to unlock file. Clean cargo and try again.");

    return id;
}


fn construct_id_map(input: &str) -> HashMap<String, u64> {
    let mut map = HashMap::<String, u64>::new();
    let regex = Regex::new(r#"([a-zA-Z0-9_]+)\s+(\d+)"#).expect("Could not construct regex");

    for pair in regex.captures_iter(&input) {
        let name = &pair[1];
        let id = pair[2].parse::<u64>().expect("Bad id file.");
        map.insert(name.to_owned(), id);
    }

    return map;
}