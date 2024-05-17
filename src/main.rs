use clap::Parser;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "cargo", bin_name = "cargo", long_about="Management LXC container tool")]
enum CmtCli {
    Create(CreateArgs),
    Delete(DeleteArgs),
    Exec(ExecArgs),
}

#[derive(Debug, clap::Args)]
#[command(version, about, long_about = "Create and start instances from images")]
struct CreateArgs {
    #[arg(short, long, required = true)]
    image: String,
    
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, clap::Args)]
#[command(version, about, long_about = "Delete containers and images")]
struct DeleteArgs {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    hash: String,
}

#[derive(Debug, clap::Args)]
#[command(version, about, long_about = "Execute commands in containers")]
struct ExecArgs {
    #[arg(short, long)]
    command: String,
}

fn main() {
    let _cli = CmtCli::parse();
}