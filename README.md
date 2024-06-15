# Container Management Tool

# Usage

## Prerequisites

Before you can compile and run this Rust program, you need to have the following dependencies installed:

1. **Rust**: The programming language in which the program is written. You can install Rust using rustup, the recommended installer for the Rust programming language.
To install Rust, run the following command in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Follow the on-screen instructions to complete the installation. After installation, make sure to **update your PATH environment variable**:

```bash
source $HOME/.cargo/env
```

3. **Cargo**: The Rust package manager, which is installed automatically with Rust. It helps in managing dependencies and building projects.

## Compilation

To compile the Rust program, navigate to the root directory of your project (where Cargo.toml is located) and run the following command:

```bash
cargo build --release
```

This command compiles the project in release mode, optimizing for performance. The compiled binary will be located in the target/release directory.

## Running the Program

Once the program is compiled, you can run it directly from the terminal.

```bash
./target/release/cmt
```

After this, you can just follow the tools help dialog, complete with examples and long explanations.

```bash
$ ./cmt --help
Management LXC container tool

Usage: cargo [OPTIONS] [COMMAND]

Commands:
  create   Create and start instances from images [aliases: init, new]
  delete   Delete containers and images [aliases: rm, destroy]
  execute  Execute commands in containers [aliases: exec]
  start    Start containers [aliases: up, boot]
  stop     Stop containers [aliases: halt, terminate]
  list     List containers [aliases: ls, show, sh]
  copy     Copy files/folders between a container and the local filesystem [aliases: cp]
  config   Get or set the configurations for a container [aliases: cf]
  build    Build an image from a LXCfile
  help     Print this message or the help of the given subcommand(s)

Options:
  -o, --logfile <FILE>
          Output log to FILE instead of stderr

  -l, --logpriority <LEVEL>
          Set log priority to LEVEL

  -q, --quiet
          Don't show progress information

  -P, --lxcpath <PATH>
          Use specified container path

  -h, --help
          Print help (see a summary with '-h')
```

# Code and Implementation

This tool is implemented in two main files: `main.rs` and `manage.rs`. The first file, `main.rs`, handles user interactions and the CLI implementation. The second file, `manage.rs`, translates user input into `lxc` commands and other necessary function calls.

## Main.rs

The CLI for this tool is largely based on the `lxc` suite of commands and also takes some inspiration from the Docker CLI. The `Subcommands` enum represents the various commands that the user can execute. Here's a breakdown of each subcommand:

```rust
enum Subcommands {
    Create(CreateArgs),
    Delete(DeleteArgs),
    Execute(ExecuteArgs),
    Start(StartArgs),
    Stop(StopArgs),
    List(ListArgs),
    Copy(CopyArgs),
    Config(ConfigArgs),
    Build(BuildArgs),
}
```

- Create: Used to create a new container.
- Delete: Used to delete an existing container.
- Execute: Used to execute a command inside a container.
- Start: Used to start a stopped container.
- Stop: Used to stop a running container.
- List: Lists all the containers.
- Copy: Used to copy a container or snapshot.
- Config: Used to configure container settings.
- Build: Used to build a container from a configuration file.

Each subcommand in the `Subcommands` enum is mapped to a corresponding function in `manage.rs`, which translates the user's input into the appropriate `lxc` command or other necessary function calls. This structure ensures that the CLI is both intuitive and flexible, leveraging the power of the lxc suite and the familiarity of Docker-like commands.

### Command building

This code snippet is responsible for building and executing commands based on the user's input from the CLI. It maps each subcommand to its corresponding function in the manage module, constructs the command string, and then executes it. Here is a detailed explanation of the code:

```rust
// Build command based on subcommands.
let mut cmdstr: Vec<String> = Vec::new();
match cli.sub {
    Some(Subcommands::Create(args)) => cmdstr = manage::create(args),
    Some(Subcommands::Delete(args)) => cmdstr = manage::delete(args),
    Some(Subcommands::Execute(args)) => cmdstr = manage::execute(args),
    Some(Subcommands::Start(args)) => cmdstr = manage::start(args),
    Some(Subcommands::Stop(args)) => cmdstr = manage::stop(args),
    Some(Subcommands::List(args)) => cmdstr = manage::list(args),
    Some(Subcommands::Copy(args)) => cmdstr = manage::copy(args),
    Some(Subcommands::Config(args)) => cmdstr = manage::config(args),
    Some(Subcommands::Build(args)) => cmdstr = manage::build(args),
    _ => {}
};
```

1. **Command Building**: The match statement checks which subcommand was provided by the user (e.g., Create, Delete, Execute, etc.). It then calls the corresponding function in the manage module, passing the arguments to that function. Each function returns a vector of command strings (cmdstr).

```rust
for cmd in cmdstr {
    // Quiet mode redirects everything to /dev/null
    let mut command_and_args: Vec<&str> = cmd.split_whitespace().collect();
```

2. **Command Splitting**: The code iterates over each command string in `cmdstr`. It splits each command string into its individual components (command and arguments) and stores them in a vector `command_and_args`.

```rust
let mut stdout = Stdio::inherit();
let mut stderr = Stdio::inherit();
if cli.quiet {
    stdout = Stdio::null();
    stderr = Stdio::null();
}
```

3. **Output Redirection**: If the `quiet` flag is set in the CLI arguments (`cli.quiet`), both `stdout` and `stderr` are redirected to `/dev/null`, effectively silencing the command output. Otherwise, the command output is inherited from the parent process.

```rust
match Command::new(command_and_args[0])
    .args(command_and_args.split_off(1))
    .stdout(stdout)
    .stderr(stderr)
    .spawn()
{
    Ok(mut shell) => {
        let _ = shell.wait();
    }
    Err(e) => {
        println!("{:?}", e);
    }
}
```

4. **Command Execution**: The `Command::new `function is used to create a new command from the first element of `command_and_args` (the actual command). The remaining elements are the arguments, which are passed using `.args()`. The command is then executed with the specified `stdout` and `stderr` configurations.

Overall, this code allows the CLI tool to dynamically build and execute commands based on user input, with options to control the verbosity of the output.

## Manage.rs

`Manage.rs` will, based on the arguments captured, build commands and/or edit files.

For example, creating a container is achieved like this:

```rust
pub fn create(args: CreateArgs) -> Vec<String> {
    let mut create_options: String = String::new();
    if args.config.is_some() && !args.config.as_ref().unwrap().is_empty() {
        create_options.push_str(&format!(" --config={}", args.config.unwrap()));
    }

    if args.dir.is_some() && !args.dir.as_ref().unwrap().is_empty() {
        if !Path::new(args.dir.clone().unwrap().as_str()).exists() {
            _ = std::fs::create_dir(args.dir.clone().unwrap().as_str());
        }
        create_options.push_str(&format!(" --dir={}", args.dir.unwrap().as_str()));
    }

    if args.network.is_some() && !args.network.as_ref().unwrap().is_empty() {
        create_options.push_str(&format!(" --network={}", args.network.unwrap()));
    }

    // Parse template
    let image: Vec<&str> = args.image.split(':').collect();

    let cmdstr = format!(
        "lxc-create --name={}{} --template=download -- --dist={} --release={} --arch={}",
        args.name, create_options, image[0], image[1], image[2],
    );

    vec![cmdstr]
}
```

It will literally build the command string so it can than be called by a subprocess.

## Copying Files

Not all commands function uniformly. By default, `lxc` lacks a built-in tool for copying files to the container's filesystem. However, given direct access, there's nothing preventing us from simply transferring the file. It's crucial to note that users may opt to mount the root of the filesystem in a location other than the default. Hence, it's necessary to verify this location before proceeding.

This CLI takes inspiration from Docker CLI but expands upon it significantly. It allows bidirectional file copying not only between the host and containers but also between different containers.

```rust
let mut copy_options: String = String::from("--recursive");
```

1. Initializes copy_options with default flags (--recursive) to support recursive copying.

```rust
let mut source_path = String::new();
let source_location: Vec<&str> = args.source.split(':').collect();
if args.source.contains(':') && source_location.len() > 1 {
    // Find rootfs path
    source_path = (Exec::shell(&format!(
        "lxc-info --name={} --config=lxc.rootfs.path",
        source_location[0]
    )) | Exec::shell("cut -c 19-"))
    .capture()
    .unwrap()
    .stdout_str()
    .trim()
    .to_string();
    source_path.push_str(source_location[1]);
} else {
    source_path.push_str(source_location[0]);
}
```

2. **Source path determination**: Determines the source path based on user-provided input (args.source). If the source includes a colon (:), indicating a specific location within the container's filesystem, it retrieves the root filesystem path using lxc-info and adjusts accordingly.

```rust
let mut destination_path = String::new();
let destination_location: Vec<&str> = args.destination.split(':').collect();
if args.destination.contains(':') && destination_location.len() > 1 {
    // Find rootfs path
    destination_path = (Exec::shell(&format!(
        "lxc-info --name={} --config=lxc.rootfs.path",
        destination_location[0]
    )) | Exec::shell("cut -c 19-"))
    .capture()
    .unwrap()
    .stdout_str()
    .trim()
    .to_string();
    destination_path.push_str(destination_location[1]);
} else {
    destination_path.push_str(destination_location[0]);
}
```

3. **Destination path determination**: Similar to the source path determination, retrieves the destination path based on user-provided input (args.destination). Adjusts the path if a specific location within the container's filesystem is indicated.

## Configuring via `cgroups`

The `config` function showcases the dynamic container configuration capabilities of the LXC CLI. By leveraging `lxc` commands, it eliminates the need to directly modify the `/proc/cgroup` file. This approach mitigates the risk of inadvertently affecting system stability with erroneous statements, ensuring precise and controlled container configuration.

```rust
if let Some(state_object) = args.state_object {
    cmdstr.push_str(&format!("lxc-cgroup --name={}", args.name));

    config_options.push_str(&format!(" {}", state_object[0]));
    if state_object.len() > 1 {
        config_options.push_str(&format!(" {}", state_object[1]));
    }
} 
```

## Parsing build LXCfiles.toml and building containers

The `LXCfile.toml` is a rudimentary attempt to design an explicit container definition, akin to Docker's use of Dockerfiles. This approach not only facilitates container replication but also simplifies the upfront configuration of containers.

```toml
name = "mycontainer"

[image]
distro = "alpine"
release = "3.19"
arch = "amd64"
dir = /path/to/rootfs

[limits]
cpuset_cpus = "1,2"

[[copy]]
host = "."
container = "/"
follow_link = true

[[copy]]
host = "../../README.md"
container = "./README.md"
archive = true

[[shared]]
host = "/home/davidjosearaujo/Downloads"
container = "mount/point"

[[run]]
cmd = "touch bye"
```

This LXCfile.toml for example, it defines the configuration for an LXC container named "mycontainer". It includes an entrypoint script, image details, resource limits, file copying instructions, shared directories, and build-time commands.

# Videos

Videos demonstrating the tools capabilities can be found [here](./docs/videos/).

In the videos we can see the following capabilities:

- [Video 1](./docs/videos/create_and_delete_container.mp4)
   1. Create a new container.
   2. Start and stop a container.
   3. List all containers.
   4. Destroy a container. 
- [Video 2](./docs/videos/copying_files_to_container_attach_shell.mp4)
  1. Attach shell to a container.
  2. Copy files to a container.
- [Video 3](./docs/videos/runtime_cpu_limit_change.mp4)
  1. Execute commands in a container.
  2. Change `cgroups` proprieties (e.g. CPU in the video) in runtime.
- [Video 4](./docs/videos/build_file_and_shared_volume.mp4)
  1. Define a container via an explicit file (LXCfile.toml).
  2. Build a container via an LXCfile.tml.
  3. Mount shared volumes between host and container.