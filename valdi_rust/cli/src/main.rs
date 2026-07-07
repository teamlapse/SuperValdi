use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.get(1).map(String::as_str) == Some("ir-inspect") {
        run_ir_inspect(&args[2..]);
        return;
    }

    let crates = [
        valdi_rust_ir::CRATE_ID,
        valdi_rust_backend::CRATE_ID,
        valdi_rust_codec::CRATE_ID,
        valdi_rust_runtime::CRATE_ID,
        valdi_rust_codegen::CRATE_ID,
    ];
    println!("valdi_rust foundation crates: {}", crates.join(","));
}

fn run_ir_inspect(args: &[String]) {
    let Some(path) = args.first() else {
        eprintln!("usage: valdi_rust_cli ir-inspect <fixture.ir.json> [--backend-tag <tag>]");
        process::exit(2);
    };
    let mut backend_tag = None;
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--backend-tag" => {
                let Some(tag) = args.get(index + 1) else {
                    eprintln!("--backend-tag requires a value");
                    process::exit(2);
                };
                backend_tag = Some(tag.as_str());
                index += 2;
            }
            flag => {
                eprintln!("unknown ir-inspect option: {flag}");
                process::exit(2);
            }
        }
    }

    let input = read_fixture_input(path).unwrap_or_else(|message| {
        eprintln!("{message}");
        process::exit(1);
    });
    let fixture = valdi_rust_codec::json_debug::decode_fixture(&input).unwrap_or_else(|diagnostic| {
        eprintln!("{} {} {}", diagnostic.code, diagnostic.path, diagnostic.message);
        process::exit(1);
    });
    println!("{}", valdi_rust_codec::inspect::inspect_fixture(&fixture, backend_tag));
}

fn read_fixture_input(path: &str) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(input) => Ok(input),
        Err(err) => {
            if Path::new(path).is_relative() {
                if let Ok(working_dir) = env::var("BUILD_WORKING_DIRECTORY") {
                    let candidate = Path::new(&working_dir).join(path);
                    if let Ok(input) = fs::read_to_string(&candidate) {
                        return Ok(input);
                    }
                }
            }
            Err(format!("failed to read {path}: {err}"))
        }
    }
}
