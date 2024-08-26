use argh::FromArgs;
use peff::input;
use peff::pe::TargetBinary;

#[derive(FromArgs)]
/// Print DLLs imported by a Windows binary (EXE, DLL, OCX, SYS, etc.). The
/// target binaries will be read and their import sections parsed to generate a
/// list of imported DLLs.
struct Args {
    #[argh(switch)]
    /// print version and exit
    version: Option<bool>,

    #[argh(positional)]
    /// input files to analyze
    target: Vec<String>,
}

fn main() {
    let args: Args = argh::from_env();

    // Print version and exit.
    if args.version.is_some() {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    // Make sure we got at least one input file before proceeding.
    if args.target.is_empty() {
        println!(
            "Try `{} --help` for usage instructions.",
            env!("CARGO_PKG_NAME")
        );
        std::process::exit(0)
    }

    // Build list of target binaries to analyze.
    let mut results = vec![];
    for target in input::build_list(args.target) {
        match TargetBinary::from(&target) {
            Ok(t) => results.push(t),
            Err(e) => eprintln!("Error processing {target:#?}: {e}"),
        }
    }

    // Print results.
    for r in results {
        r.print();
    }
}
