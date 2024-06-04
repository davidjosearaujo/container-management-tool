// Copyright 2024 David Araújo
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(
    name = "cargo",
    bin_name = "cargo",
    long_about = "Management LXC container tool"
)]
struct CmtCli {
    #[command(subcommand)]
    sub: Subcommands,

    #[arg(short, long, help = "Show all information messages", global = true)]
    verbose: bool,

    #[arg(short, long, help = "Don't show progress information", global = true)]
    quiet: bool,
}

#[derive(Debug, Subcommand)]
enum Subcommands {
    Create(CreateArgs),
    Delete(DeleteArgs),
    Exec(ExecArgs),
}

#[derive(Debug, Args)]
#[command(
    version,
    about,
    long_about = "Create and start instances from images",
    visible_aliases = ["init"]
)]
struct CreateArgs {
    #[arg(required = true)]
    image: String,

    #[arg(short, long, help = "Name for the new container")]
    name: String,

    #[arg(short, long, help = "Config key/value to apply to the new instance")]
    config: Option<String>,

    #[arg(short, long, help = "New key/value to apply to a specific device")]
    device: Option<String>,

    #[arg(long, help = "Create an empty instance")]
    empty: bool,

    #[arg(short, long, help = "Ephemeral instance")]
    ephemeral: bool,

    #[arg(long, help = "Network name")]
    network: Option<String>,

    #[arg(
        long = "no-profiles",
        help = "Create the instance with no profiles applied"
    )]
    no_profiles: bool,

    #[arg(short, long, help = "Profile to apply to the new instance")]
    profile: Option<String>,

    #[arg(short, long, help = "Storage pool name")]
    storage: Option<String>,

    #[arg(long, help = "Cluster member name")]
    target: Option<String>,

    #[arg(short, long, help = "Instance type")]
    instance_type: Option<String>,
}

#[derive(Debug, clap::Args)]
#[command(
    version,
    about,
    long_about = "Delete containers and images",
    visible_aliases = ["rm", "destroy"]
)]
struct DeleteArgs {
    #[arg(
        value_delimiter=',',
        value_names=["instance","[<instance>,....]"],
        help = "Name of containers to delete"
    )]
    id: String,

    #[arg(short, long, help = "Force the removal of running instances")]
    force: bool,

    #[arg(short, long, help = "Require user confirmation")]
    interactive: bool,
}

#[derive(Debug, clap::Args)]
#[command(version, about, long_about = "Execute commands in containers")]
struct ExecArgs {
    #[arg(required=true)]
    command: String,

    #[arg(long, help = "Directory to run the command in (default /root)")]
    cwd: Option<String>,

    #[arg(short = 'n', long, help = "Disable stdin (reads from /dev/null)")]
    disable_stdin: bool,

    #[arg(
        long,
        help = "Environment variable to set (e.g. HOME=/home/foo)"
    )]
    env: Option<String>,

    #[arg(
        short = 't',
        long = "force-interactive",
        help = "Force pseudo-terminal allocation"
    )]
    force_interactive: bool,

    #[arg(
        short = 'T',
        long = "force-noninteractive",
        help = "Disable pseudo-terminal allocation"
    )]
    force_noninteractive: bool,

    #[arg(long, help = "Group ID to run the command as (default 0)")]
    group: Option<String>,

    #[arg(
        long,
        help = "Override the terminal mode (auto, interactive or non-interactive) (default \"auto\")"
    )]
    mode: Option<String>,

    #[arg(long, help = "User ID to run the command as (default 0)")]
    user: Option<String>,
}

fn main() {
    let _cli = CmtCli::parse();
}
