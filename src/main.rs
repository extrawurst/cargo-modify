#![forbid(unsafe_code)]
#![deny(unused_imports)]
#![deny(unused_must_use)]
#![deny(dead_code)]
#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(clippy::match_like_matches_macro)]
#![deny(clippy::needless_update)]

fn main() {
    let p = "Cargo.toml";
    let file = std::fs::read_to_string(p).unwrap();
    let original_toml: toml::Value = toml::from_slice(file.as_bytes()).unwrap();
    let mut new_toml = original_toml.clone();
    new_toml
        .as_table_mut()
        .and_then(|t| t.get_mut("package"))
        .and_then(|package| package.as_table_mut())
        .map(|package| package.entry("resolver").or_insert("2".into()));

    if new_toml != original_toml {
        let s = toml::to_string(&new_toml).unwrap();
        std::fs::write(p, s).unwrap();
        println!("changes written");
    } else {
        println!("nothing changed");
    }
}
