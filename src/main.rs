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

const HELP: &str = "\
cargo-modify

USAGE:
  cargo modify <MODE> [OPTIONS]
FLAGS:
  -h, --help            Prints help information
OPTIONS:
  --v2=true             If not `true` removes new resolver entry (default: true)
MODE:
  new-resolver          Sets package.resolver=\"2\"
";

struct Args {
    new_resolver: bool,
    mode: String,
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let p = "Cargo.toml";
    let file = std::fs::read_to_string(p).unwrap();
    let original_toml: toml::Value = toml::from_slice(file.as_bytes()).unwrap();
    let mut new_toml = original_toml.clone();

    if &args.mode == "new-resolver" {
        println!("mode: new-resolver (arg: {})", args.new_resolver);
        if args.new_resolver {
            new_toml
                .as_table_mut()
                .and_then(|t| t.get_mut("package"))
                .and_then(|package| package.as_table_mut())
                .map(|package| package.entry("resolver").or_insert("2".into()));
        } else {
            new_toml
                .as_table_mut()
                .and_then(|t| t.get_mut("package"))
                .and_then(|package| package.as_table_mut())
                .map(|package| package.remove("resolver"));
        }
    } else {
        eprintln!("unknown mode: '{}'", args.mode);
        std::process::exit(1);
    }

    if new_toml != original_toml {
        let s = toml::to_string(&new_toml).unwrap();
        std::fs::write(p, s).unwrap();
        println!("changes written");
    } else {
        println!("nothing changed");
    }
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let _modify: String = pargs.free_from_str().unwrap_or_default();

    let args = Args {
        new_resolver: pargs.opt_value_from_str("--v2")?.unwrap_or(true),
        mode: pargs.free_from_str().unwrap_or_default(),
    };

    Ok(args)
}
