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
    Execute(ExecuteArgs),
    Start(StartArgs),
    Stop(StopArgs),
    List(ListArgs),
    Copy(CopyArgs),
    Configuration(ConfigurationArgs),
}

#[derive(Debug, Args)]
#[command(
    version,
    about,
    long_about = "Create and start instances from images",
    visible_aliases = ["init", "new"]
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
    long_about = "The container root filesystem will be a directory"
)]
struct DIRArgs {
    #[arg(
        value_name = "DIR",
        default_value = "/var/lib/lxc/container/rootfs",
        help = "Place rootfs directory under DIR"
    )]
    dir: String,
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

#[derive(Debug, Args)]
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
    rcfile: Option<String>,
}

#[derive(Debug, Args)]
#[command(
    version,
    about,
    long_about = "Execute commands in containers",
    visible_aliases = ["exec"])]
struct ExecuteArgs {
    #[arg(required = true, value_name = "NAME", help = "NAME of the container")]
    name: String,

    #[arg(required = true, help = "COMMAND to execute into this container")]
    command: String,

    #[arg(short, long, help = "Daemonize the container")]
    daemon: bool,

    #[arg(
        long,
        default_value = "/root",
        help = "Directory to run the command in"
    )]
    cwd: Option<String>,

    #[arg(
        short,
        long,
        value_delimiter = ',',
        help = "Environment variable(s) to set [e.g. HOME=/home/foo][comma-separated]"
    )]
    env: Option<Vec<String>>,

    #[arg(long, value_name = "FILE", help = "Environment variable FILE")]
    envfile: Option<String>,

    #[arg(
        short,
        long,
        default_value = "0",
        help = "User ID to run the command as"
    )]
    uid: Option<String>,

    #[arg(
        short,
        long,
        default_value = "0",
        help = "Group ID to run the command as"
    )]
    gid: Option<String>,
}

#[derive(Debug, Args)]
#[command(
    version,
    about,
    long_about = "Start containers",
    visible_aliases = ["up", "boot"]
)]
struct StartArgs {
    #[arg(
        short,
        long,
        value_name = "NAME",
        help = "NAME of the container",
        required = true
    )]
    name: String,

    #[arg(short, long, help = "Daemonize the container (default)")]
    daemon: bool,

    #[arg(
        short = 'F',
        long,
        help = "Start with the current tty attached to /dev/console"
    )]
    foreground: bool,

    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Create a file with the process id"
    )]
    pidfile: Option<String>,

    #[arg(long, value_name = "FILE", help = "Load configuration file FILE")]
    rcfile: Option<String>,

    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Use specified FILE for the container console"
    )]
    console: Option<String>,

    #[arg(
        short = 'L',
        long,
        value_name = "FILE",
        help = "Log container console output to FILE"
    )]
    console_log: Option<String>,

    #[arg(
        short = 'C',
        long,
        help = "If any fds are inherited, close them (Note: --daemon implies --close-all-fds)"
    )]
    close_all_fds: bool,

    #[arg(
        short = 's',
        long,
        value_name = "KEY=VAL",
        help = "Assign VAL to configuration variable KEY"
    )]
    define: Option<String>,

    #[arg(
        long,
        value_name = "NAME",
        help = "Share a network namespace with another container or pid"
    )]
    share_net: Option<String>,

    #[arg(
        long,
        value_name = "NAME",
        help = "Share an IPC namespace with another container or pid"
    )]
    share_ipc: Option<String>,

    #[arg(
        long,
        value_name = "NAME",
        help = "Share a UTS namespace with another container or pid"
    )]
    share_uts: Option<String>,

    #[arg(
        long,
        value_name = "NAME",
        help = "Share a PID namespace with another container or pid"
    )]
    share_pid: Option<String>,
}

#[derive(Debug, Args)]
#[command(
    version,
    about,
    long_about = "Stop containers",
    visible_aliases = ["halt", "terminate"]
)]
struct StopArgs {
    #[arg(
        short,
        long,
        value_name = "NAME",
        help = "NAME of the container",
        required = true
    )]
    name: String,

    #[arg(short, long, help = "Reboot the container")]
    reboot: bool,

    #[arg(
        short = 'W',
        long,
        help = "Don't wait for shutdown or reboot to complete"
    )]
    nowait: bool,

    #[arg(
        short,
        long,
        value_name = "T",
        help = "Wait T seconds before hard-stopping"
    )]
    timeout: Option<u64>,

    #[arg(
        short,
        long,
        help = "Kill container rather than request clean shutdown"
    )]
    kill: bool,

    #[arg(long, help = "Avoid using API locks")]
    nolock: bool,

    #[arg(
        long,
        help = "Only request clean shutdown, don't force kill after timeout"
    )]
    nokill: bool,

    #[arg(long, value_name = "FILE", help = "Load configuration file FILE")]
    rcfile: Option<String>,
}

#[derive(Debug, Args)]
#[command(
    version,
    about,
    long_about = "List containers",
    visible_aliases = ["ls", "show", "sh"]
)]
struct ListArgs {
    #[arg(short = '1', long, help = "Show one entry per line")]
    line: bool,

    #[arg(short, long, help = "Use a fancy, column-based output")]
    fancy: bool,

    #[arg(
        short = 'F',
        long,
        value_name = "COLUMNS",
        help = "Comma separated list of columns to show in the fancy output (valid columns: NAME, STATE, PID, RAM, SWAP, AUTOSTART, GROUPS, INTERFACE, IPV4 and IPV6, UNPRIVILEGED)"
    )]
    fancy_format: Option<String>,

    #[arg(long, help = "List only active containers")]
    active: bool,

    #[arg(long, help = "List only running containers")]
    running: bool,

    #[arg(long, help = "List only frozen containers")]
    frozen: bool,

    #[arg(long, help = "List only stopped containers")]
    stopped: bool,

    #[arg(long, help = "List only defined containers")]
    defined: bool,

    #[arg(
        long,
        value_name = "NUM",
        help = "List nested containers up to NUM levels of nesting (default is 5)"
    )]
    nesting: Option<u32>,

    #[arg(
        long,
        value_name = "REGEX",
        help = "Filter container names by regular expression"
    )]
    filter: Option<String>,

    #[arg(
        short,
        long,
        value_name = "GROUPS",
        value_delimiter = ',',
        help = "Comma separated list of groups a container must have to be displayed"
    )]
    groups: Option<Vec<String>>,
}

#[derive(Debug, Args)]
#[command(
    version,
    about,
    long_about = "Copy files/folders between a container and the local filesystem",
    visible_aliases = ["cp"]
)]
struct CopyArgs {
    #[arg(
        value_name = "[CONTAINER]:SRC_PATH",
        help = "Source path (can be from the local host or from a container)"
    )]
    source: String,

    #[arg(
        value_name = "[CONTAINER]:DEST_PATH",
        help = "Destination path (can be to the local host or to a container)"
    )]
    destination: String,

    #[arg(short, long, help = "Archive mode (copy all uid/gid information)")]
    archive: Option<bool>,

    #[arg(short = 'L', long, help = "Always follow symbol link in SRC_PATH")]
    follow_link: Option<bool>,
}


// Aggregate the functionalities of:
//  -   lxc-cgroups ✔
//  -   lxc-info
#[derive(Debug, Args)]
#[command(
    version,
    about,
    long_about = "Get or set the configurations for a container",
    visible_aliases = ["cf", "config"]
)]
struct ConfigurationArgs {
    #[arg(value_name = "NAME", help = "Name of container")]
    name: String,

    #[arg(long, value_name = "FILE", help = "Load configuration file FILE")]
    rcfile: Option<String>,

    #[arg(long, value_delimiter = ',', value_name = "value", help = "Value of a state object (for example, 'cpuset.cpus')")]
    state_object: Option<Vec<String>>,

    // TODO: add lxc-info capabilities
}

fn main() {
    let _cli = CmtCli::parse();
}
