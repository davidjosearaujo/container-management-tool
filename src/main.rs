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

mod manage;
mod utils;

use clap::{Args, Parser, Subcommand};
use std::sync::atomic::Ordering;

#[macro_use]
extern crate lazy_static;

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
    logfile: Option<String>,

    #[arg(
        short,
        long,
        value_name = "LEVEL",
        help = "Set log priority to LEVEL",
        global = true
    )]
    logpriority: Option<String>,

    #[arg(short, long, help = "Don't show progress information", global = true)]
    quiet: bool,

    #[arg(
        short = 'P',
        long,
        value_name = "PATH",
        help = "Use specified container path",
        global = true
    )]
    lxcpath: Option<String>,
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
    Config(ConfigArgs),
}

#[derive(Debug, Args)]
#[command(
    version,
    about,
    long_about = "Create and start instances from images",
    visible_aliases = ["init", "new"]
)]
struct CreateArgs {
    #[arg(help = "Name for the new container", required = true)]
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
        value_name = "IMAGE",
        default_value = "alpine:3.19:amd64",
        help = "Image to use to setup container"
    )]
    image: String,

    #[arg(
        short,
        long,
        value_name = "DIR",
        default_value = "/var/lib/lxc/container/rootfs",
        help = "Place rootfs directory under DIR"
    )]
    dir: Option<String>,

    #[arg(long, help = "Network name")]
    network: Option<String>,
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
        help = "Name of containers to delete"
    )]
    name: String,

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

    #[arg(
        short,
        long,
        help = "Use elevated privileges instead of those of the container. If you don't specify privileges to be elevated as OR'd list: CAP, CGROUP and LSM (capabilities, cgroup and restrictions, respectively) then all of them will be elevated. WARNING: This may leak privileges into the container. Use with care."
    )]
    elevated_privileges: Option<String>,

    #[arg(
        short,
        long,
        help = "Use ARCH for program instead of container's own architecture."
    )]
    arch: Option<String>,

    #[arg(
        short,
        long,
        help = "Don't attach to all the namespaces of the container
                    but just to the following OR'd list of flags: MOUNT, PID, UTSNAME, IPC, USER or NETWORK. WARNING: Using -s implies -e with all privileges elevated, it may therefore leak privileges into the container. Use with care."
    )]
    namespaces: Option<String>,

    #[arg(
        short = 'R',
        long,
        help = "Remount /sys and /proc if not attaching to the mount namespace when using -s in order to properly reflect the correct namespace context. See the lxc-attach(1) manual page for details"
    )]
    remount_sys_proc: Option<String>,

    #[arg(
        long,
        help = "Clear all environment variables before attaching. The attached shell/program will start with only container=lxc set."
    )]
    clear_env: bool,

    #[arg(
        long,
        help = "Keep all current environment variables. This is the current default behaviour, but is likely to change in the future."
    )]
    keep_env: bool,

    #[arg(
        short = 'L',
        long,
        value_name = "FILE",
        help = "Log pty output to FILE"
    )]
    pty_log: Option<String>,

    #[arg(
        short = 'v',
        long,
        help = "Set an additional variable that is seen by the attached program in the container. May be specified multiple times."
    )]
    set_var: bool,

    #[arg(
        long,
        help = "Keep an additional environment variable. Only applicable if --clear-env is specified. May be used"
    )]
    keep_var: bool,

    #[arg(
        short = 'f',
        long,
        value_name = "FILE",
        help = "Load configuration file FILE"
    )]
    rcfile: Option<String>,

    #[arg(
        short,
        long,
        default_value = "0",
        help = "Execute COMMAND with UID inside the container"
    )]
    uid: Option<String>,

    #[arg(
        short,
        long,
        default_value = "0",
        help = "Execute COMMAND with GID inside the container"
    )]
    gid: Option<String>,

    #[arg(
        short,
        long,
        value_name="context",
        help = "SELinux Context to transition into"
    )]
    context: Option<String>,
}

#[derive(Debug, Args)]
#[command(
    version,
    about,
    long_about = "Start containers",
    visible_aliases = ["up", "boot"]
)]
struct StartArgs {
    #[arg(value_name = "NAME", help = "NAME of the container", required = true)]
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
        value_name = "[CONTAINER:]SRC_PATH",
        help = "Source path (can be from the local host or from a container)"
    )]
    source: String,

    #[arg(
        value_name = "[CONTAINER:]DEST_PATH",
        help = "Destination path (can be to the local host or to a container)"
    )]
    destination: String,

    #[arg(short, long, help = "Archive mode (copy all uid/gid information)")]
    archive: bool,

    #[arg(short = 'L', long, help = "Always follow symbol link in SRC_PATH")]
    follow_link: bool,
}

// Aggregates lxc-info and lxc-cgroup
#[derive(Debug, Args)]
#[command(
    version,
    about,
    long_about = "Get or set the configurations for a container",
    visible_aliases = ["cf"]
)]
struct ConfigArgs {
    #[arg(value_name = "NAME", help = "Name of container")]
    name: String,

    #[arg(
        long,
        value_name = "VALUE",
        value_delimiter = ':',
        help = "Value of a state object (for example, 'cpuset.cpus:0,3)"
    )]
    state_object: Option<Vec<String>>,

    #[arg(
        short,
        long,
        value_name = "KEY",
        help = "Show configuration variable KEY from running container"
    )]
    config: Option<String>,

    #[arg(short, long, help = "Shows the IP addresses")]
    ips: bool,

    #[arg(short, long, help = "Shows the process id of the init container")]
    pid: bool,

    #[arg(short = 'S', long, help = "Shows usage stats")]
    stats: bool,

    #[arg(short = 'H', long, help = "Shows stats as raw numbers, not humanized")]
    no_humanize: bool,

    #[arg(short, long, help = "shows the state of the container")]
    state: bool,
}

fn main() {
    match CmtCli::try_parse() {
        Ok(cli) => {
            quiet_println!("CLI arguments parsed successfully");

            // Quiet mode suppresses all stdout
            if cli.quiet {
                utils::QUIET.store(true, Ordering::SeqCst);
            }

            // Command's global flags
            let mut global_options: String = String::new();

            if cli.logfile.is_some() {
                global_options.push_str(&format!(" --logfile={}", cli.logfile.unwrap()));
            }

            if cli.logpriority.is_some() {
                global_options.push_str(&format!(" --logpriority={}", cli.logpriority.unwrap()));
            }

            if cli.lxcpath.is_some() {
                global_options.push_str(&format!(" --lxcpath={}", cli.lxcpath.unwrap()));
            }

            // Build command based on subcommands.
            let mut cmdstr: String = String::new();
            match cli.sub {
                Some(Subcommands::Create(args)) => cmdstr = manage::create(args),
                Some(Subcommands::Delete(args)) => cmdstr = manage::delete(args),
                Some(Subcommands::Execute(args)) => cmdstr = manage::execute(args),
                Some(Subcommands::Start(args)) => cmdstr = manage::start(args),
                Some(Subcommands::Stop(args)) => cmdstr = manage::stop(args),
                Some(Subcommands::List(args)) => cmdstr = manage::list(args),
                Some(Subcommands::Copy(args)) => cmdstr = manage::copy(args),
                Some(Subcommands::Config(args)) => cmdstr = manage::config(args),
                _ => {}
            };

            // TODO: Execute command
            quiet_println!("{:?}", cmdstr);
            //let executable_command: String = format!();
            //Exec::shell(executable_command);
        }
        Err(e) => {
            quiet_println!("Error parsing input! Please try again.\n");
            _ = e.print();
        }
    }
}
