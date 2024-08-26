use argh::FromArgs;
use peff::input;
use peff::pe::TargetBinary;

/// Show DLLs imported by a Windows binary (EXE, DLL, OCX, SYS, etc.)
#[derive(FromArgs)]
struct Args {
    /// print version and exit
    #[argh(switch)]
    version: Option<bool>,

    /// suppress errors
    #[argh(switch, short = 'q')]
    quiet: Option<bool>,

    /// input files to analyze
    #[argh(positional)]
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
        let program_name = env!("CARGO_PKG_NAME");
        println!("Usage: {program_name} [<target...>] [--version] [-q]");
        println!("Try '{program_name} --help' for more information.");
        std::process::exit(0)
    }

    // Build list of target binaries to analyze.
    for target in input::build_list(args.target) {
        match TargetBinary::from(&target) {
            Ok(t) => t.print(),
            Err(e) => {
                if args.quiet.is_none() {
                    println!("Error processing {target:#?}: {e}");
                }
            }
        }
    }
}
