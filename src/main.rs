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
    sub: Option<Subcommands>,

    #[arg(
        short = 'o',
        long,
        value_name = "FILE",
        help = "Output log to FILE instead of stderr",
        global = true
    )]
    logfile: bool,

    #[arg(
        short,
        long,
        value_name = "LEVEL",
        help = "Set log priority to LEVEL",
        global = true
    )]
    logpriority: bool,

    #[arg(short, long, help = "Don't show progress information", global = true)]
    quiet: bool,

    #[arg(
        short = 'P',
        long,
        value_name = "PATH",
        help = "Use specified container path",
        global = true
    )]
    lxcpath: bool,
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
    #[arg(short, long, help = "Name for the new container", required = true)]
    name: String,

    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Config key/value to apply to the new instance"
    )]
    config: Option<String>,

    #[arg(
        short,
        long,
        value_name = "TEMPLATE",
        help = "Template to use to setup container",
        required = true
    )]
    template: String,

    #[command(subcommand)]
    sub: Option<Backingstore>,

    #[arg(long, help = "Network name")]
    network: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Backingstore {
    DIR(DIRArgs),
    LVM(LVMArgs),
    ZFS(ZFSArgs),
    LOOP(LOOPArgs),
}

#[derive(Debug, Args)]
#[command(
    about,
    long_about = "The container root filesystem will be a directory under /var/lib/lxc/container/rootfs"
)]
struct DIRArgs {
    #[arg(long, value_name = "DIR", help = "Place rootfs directory under DIR")]
    dir: Option<String>,
}

#[derive(Debug, Args)]
#[command(about, long_about = "An lvm block device will be used")]
struct LVMArgs {
    #[arg(long, value_name = "LVNAME", help = "Use LVM lv name LVNAME")]
    lvname: Option<String>,

    #[arg(
        long,
        value_name = "VG",
        default_value = "lxc",
        help = "Use LVM in volume group called VG"
    )]
    vgname: Option<String>,

    #[arg(
        long,
        value_name = "TP",
        default_value = "lxc",
        help = "Use LVM thin pool called TP"
    )]
    thinpool: Option<String>,
}

#[derive(Debug, Args)]
#[command(
    about,
    long_about = "A complete ZFS filesystem will be created for the container"
)]
struct ZFSArgs {
    #[arg(
        long,
        value_name = "PATH",
        default_value = "tank/lxc",
        help = "Create zfs under given zfsroot"
    )]
    zfsroot: Option<String>,
}

#[derive(Debug, Args)]
#[command(about, long_about = "Can set type and size in a lvm block")]
struct LOOPArgs {
    #[arg(
        long,
        value_name = "TYPE",
        default_value = "ext4",
        help = "Create fstype TYPE"
    )]
    fstype: Option<String>,

    #[arg(
        long,
        value_name = "SIZE[U]",
        default_value = "1G",
        help = "Create filesystem of size SIZE * unit U (b|B|k|Km|Mg|G|t|T)"
    )]
    fssize: Option<String>,
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
        value_delimiter = ',',
        value_name = "NAME",
        help = "Name of containers to delete [comma-separated]"
    )]
    name: Vec<String>,

    #[arg(short, long, help = "destroy including all snapshots")]
    snapshots: bool,

    #[arg(short, long, help = "Force the removal of running instances")]
    force: bool,

    #[arg(long, value_name = "FILE", help = "Load configuration file FILE")]
    rcfile: bool,
}

#[derive(Debug, clap::Args)]
#[command(
    version,
    about,
    long_about = "Execute commands in containers",
    visible_aliases = ["execute"])]
struct ExecArgs {
    #[arg(
        required=true,
        value_name="NAME",
        help="NAME of the container"
    )]
    name: String,

    #[arg(
        required = true,
        help="COMMAND to execute into this container")]
    command: String,
    
    #[arg(short, long, help = "Daemonize the container")]
    daemon: bool,

    #[arg(long, default_value="/root", help = "Directory to run the command in")]
    cwd: Option<String>,

    #[arg(short, long, value_delimiter = ',', help = "Environment variable(s) to set [e.g. HOME=/home/foo][comma-separated]")]
    env: Option<Vec<String>>,

    #[arg(long, value_name="FILE", help = "Environment variable FILE")]
    envfile: Option<String>,

    #[arg(short, long, default_value="0", help = "User ID to run the command as")]
    uid: Option<String>,

    #[arg(short, long, default_value="0", help = "Group ID to run the command as")]
    gid: Option<String>,
}

fn main() {
    let _cli = CmtCli::parse();
}
