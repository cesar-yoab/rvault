use clap::Parser;
use rvault::Rvault;

fn main() {
    let rvault_cli = Rvault::parse();
    rvault_cli.exec()
}
