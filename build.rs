/* @file

    Build configuration script of RUCI.
    SPDX-License-Identifier: WTFPL

*/
use regex::Regex;
use std::path::{Path, MAIN_SEPARATOR};
use std::{env, fs};
use string_builder::Builder;
use syn::{Expr, Item, Meta};

// Both at compile-time and run-time
#[derive(Debug, Clone)]
struct State {
    current_state: usize,
    next_state: usize,
    target: String, // TBD: Calling functions.
    input_character: char,
}

// Only needs at run-time
struct StateTreeNode {
    value: State,
    child: Option<*mut StateTreeNode>,
    cousin: Option<*mut StateTreeNode>,
}

fn initialize_tree(
    tree: &mut StateTreeNode,
    keys: Vec<(String, String, String, String)>,
) -> (Vec<State>, Vec<usize>) {
    let mut current_state = 1;
    let mut a = Vec::new();
    let mut b;
    let mut prev_state = 0;
    for i in keys {
        let mut tree_walker: &mut StateTreeNode = tree;
        let str_raw = i.3.as_bytes();
        for j in 1..i.3.len() {
            match tree_walker.child {
                None => unsafe {
                    tree_walker.child = Some(Box::leak(Box::<StateTreeNode>::new(StateTreeNode {
                        value: State {
                            current_state: tree_walker.value.next_state,
                            next_state: current_state,
                            target: String::from("None"),
                            input_character: str_raw[j] as char,
                        },
                        child: None,
                        cousin: None,
                    })));
                    current_state += 1;
                    a.push((*tree_walker.child.unwrap()).value.clone());
                    tree_walker = &mut *tree_walker.child.unwrap()
                },
                Some(mut child) => unsafe {
                    while (*child).value.input_character != str_raw[j] as char {
                        match (*child).cousin {
                            None => {
                                (*child).cousin =
                                    Some(Box::leak(Box::<StateTreeNode>::new(StateTreeNode {
                                        value: State {
                                            current_state: tree_walker.value.next_state,
                                            next_state: current_state,
                                            target: String::from("None"),
                                            input_character: str_raw[j] as char,
                                        },
                                        child: None,
                                        cousin: None,
                                    })));
                                a.push((*(*child).cousin.unwrap()).value.clone());
                                current_state += 1;
                            }
                            Some(_) => {}
                        }
                        child = (*child).cousin.unwrap()
                    }
                    tree_walker = &mut *child;
                },
            }
            prev_state = tree_walker.value.next_state;
        }
        // Insert the final state.
        for state in &mut a {
            if state.next_state == prev_state
                && state.input_character == i.3.as_bytes()[i.3.len() - 1] as char
            {
                state.target = format!("Some({}::{})", i.0, i.1);
            }
        }
    }
    // Sort generated vector.
    a.sort_by(|first: &State, second: &State| first.current_state.cmp(&second.current_state));
    // Generate state hint.
    b = Vec::new();
    b.resize(current_state, usize::MAX);
    for i in 0..(&a).len() {
        if b[a[i].current_state] == usize::MAX {
            b[a[i].current_state] = i;
        }
    }
    (a, b)
}

/// Binding Vector:
/// Vec<File, Function, Method, Mapping>
fn generate_binding(path: &str) -> Vec<(String, String, String, String)> {
    let mut i = Vec::new();
    let paths = fs::read_dir(path).unwrap();
    let match_regex =
        Regex::new(r#"^ *(?<method>(GET|POST)) *, *(?<mapping>[a-zA-Z0-9\-.?,'/\\+&%$#_]+) *$"#)
            .unwrap();
    let file_name_format = format!(r#"(?<name>[^{}]+)\.rs$"#, MAIN_SEPARATOR);
    let file_name_regex = Regex::new(&*file_name_format).unwrap();
    for path in paths {
        let path_name = path.unwrap().path();
        let rust_file = fs::read_to_string(&path_name).expect("Unable to read rust file");
        let ast = syn::parse_file(&rust_file).expect(&*rust_file);
        // Iterate file to find functions.
        for item in ast.items {
            match item {
                Item::Fn(item_fn) => {
                    // Good, found a function.
                    // Then find if it has #[doc] attribute.
                    for attr in item_fn.attrs {
                        if attr.path().is_ident("doc") {
                            let Meta::NameValue(attr_meta) = attr.meta else {
                                panic!("No way!")
                            };
                            let Expr::Lit(attr_lit) = attr_meta.value else {
                                panic!("No way!")
                            };
                            let syn::Lit::Str(attr_lit_str) = attr_lit.lit else {
                                panic!("No way!")
                            };
                            let content = attr_lit_str.value();
                            let match_result = match_regex.captures(&*content);
                            match match_result {
                                Some(captures) => {
                                    let file_path_str = path_name.to_str().unwrap();
                                    let file_name_capture = file_name_regex
                                        .captures(file_path_str)
                                        .expect(file_path_str);
                                    i.push((
                                        String::from(
                                            file_name_capture.name("name").unwrap().as_str(),
                                        ),
                                        item_fn.sig.ident.to_string(),
                                        String::from(captures.name("method").unwrap().as_str()),
                                        String::from(captures.name("mapping").unwrap().as_str()),
                                    ));
                                }
                                None => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    i
}

fn main() {
    // Generate bindings
    let bindings = generate_binding("src/controller");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("bindings.gen.rs");
    let mut builder = Builder::default();
    // Generate Tree.
    // Algorithm: Altered Trie-Tree with static allocations for Rust's delight.
    let mut state_tree: StateTreeNode = StateTreeNode {
        value: State {
            current_state: 0,
            next_state: 0,
            target: String::from("None"),
            input_character: '/', // Root node is not used.
        },
        child: None,
        cousin: None,
    };
    let (states, state_hints) = initialize_tree(&mut state_tree, bindings);
    builder.append(
        r"// Generated by build script. DO NOT EDIT!
#[derive(Debug, Copy, Clone)]
struct State {
    current_state: usize,
    next_state:usize,
    target:Option<fn(cgi::Request)->Response>,   // WARNING: Not the same as build script!
    input_character: char
}
",
    );
    builder.append(format!(
        "const STATES_TABLE: [State; {}] = [\n",
        states.len()
    ));
    for i in states {
        builder.append(format!("    State {{ current_state: {}, next_state: {}, target: {}, input_character: '{}' }},\n",
                i.current_state,
                i.next_state,
                i.target,
                i.input_character
        ));
    }
    builder.append(format!(
        "];\nconst STATE_HINTS_TABLE: [usize; {}] = [",
        state_hints.len()
    ));
    for i in state_hints {
        builder.append(format!("{}, ", i));
    }
    builder.append("];\n");
    fs::write(&dest_path, builder.string().expect("Invalid mapping!")).unwrap();

    // Rust toolchain bug
    if std::env::var("CARGO_CFG_TARGET_ENV").unwrap() == "musl" {
        println!(
            "cargo::rustc-link-arg=-Wl,--dynamic-linker=/lib/ld-musl-{}.so.1",
            std::env::var("CARGO_CFG_TARGET_ARCH").unwrap()
        );
    }
}
