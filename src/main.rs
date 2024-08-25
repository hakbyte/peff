use argh::FromArgs;

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
    if args.target.len() == 0 {
        println!(
            "Try `{} --help` for usage instructions.",
            env!("CARGO_PKG_NAME")
        );
        std::process::exit(0)
    }

    // TODO: build list of target binaries to analyze.
    for _ in args.target {}
}
