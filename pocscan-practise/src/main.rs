mod error;
mod modules;
mod cli;
pub use error::Error;
use clap::{Command, Arg};
fn main() -> Result<(),Error>{
    let cli = Command::new("POC Scan")
        .subcommand(Command::new("modules")
            .about("List all modules"))
        .subcommand(Command::new("scan")
            .about("Scan target(s)")
            .subcommand(Command::new("target")
                .about("Scan a target")
                .arg(Arg::new("target"))
                .arg_required_else_help(true))
            .subcommand(Command::new("targets")
                .about("Scan targets to .txt")
                .arg(Arg::new("targets"))
                .arg_required_else_help(true))
            .arg_required_else_help(true))
        .arg_required_else_help(true)
        .get_matches();
    if let Some(_) = cli.subcommand_matches("modules"){
        cli::modules();
    }else if let Some(matches) = cli.subcommand_matches("scan") {
        if let Some(target) = matches.subcommand_matches("target")  {
            let target = target.get_one::<String>("target").unwrap();
            cli::scan(target)?;
        }else if let Some(targets) = matches.subcommand_matches("targets") {
            let target = targets.get_one::<String>("targets").unwrap();
            cli::scans(target)?;
        }
    }
    Ok(())
}
