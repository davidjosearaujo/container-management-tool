use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};
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

use std::io::prelude::*;
use std::{fs::OpenOptions, path::Path, vec};

use subprocess::Exec;

use toml::Table;

use crate::{
    BuildArgs, ConfigArgs, CopyArgs, CreateArgs, DeleteArgs, ExecuteArgs, ListArgs, StartArgs,
    StopArgs,
};

pub static mut STDOUT: bool = true;
pub static mut STDERR: bool = true;

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

pub fn delete(args: DeleteArgs) -> Vec<String> {
    let mut delete_options: String = String::new();

    if args.force {
        delete_options.push_str(&format!(" --force"));
    }

    if args.snapshots {
        delete_options.push_str(&format!(" --snapshots"));
    }

    if args.rcfile.is_some() {
        delete_options.push_str(&format!(" --rcfile={}", args.rcfile.unwrap()));
    }

    let cmdstr = format!("lxc-destroy --name={}{}", args.name, delete_options,);

    vec![cmdstr]
}

pub fn execute(args: ExecuteArgs) -> Vec<String> {
    let mut execute_options = String::new();

    if let Some(elevated_privileges) = args.elevated_privileges {
        execute_options.push_str(&format!(" --elevated-privileges={}", elevated_privileges));
    }

    if let Some(arch) = args.arch {
        execute_options.push_str(&format!(" --arch={}", arch));
    }

    if let Some(namespaces) = args.namespaces {
        execute_options.push_str(&format!(" --namespaces={}", namespaces));
    }

    if let Some(remount_sys_proc) = args.remount_sys_proc {
        execute_options.push_str(&format!(" --remount-sys-proc={}", remount_sys_proc));
    }

    if args.clear_env {
        execute_options.push_str(" --clear-env");
    }

    if args.keep_env {
        execute_options.push_str(" --keep-env");
    }

    if let Some(pty_log) = args.pty_log {
        execute_options.push_str(&format!(" --pty-log={}", pty_log));
    }

    if args.set_var {
        execute_options.push_str(" --set-var");
    }

    if args.keep_var {
        execute_options.push_str(" --keep-var");
    }

    if let Some(rcfile) = args.rcfile {
        execute_options.push_str(&format!(" --rcfile={}", rcfile));
    }

    if let Some(uid) = args.uid {
        execute_options.push_str(&format!(" --uid={}", uid));
    }

    if let Some(gid) = args.gid {
        execute_options.push_str(&format!(" --gid={}", gid));
    }

    if let Some(context) = args.context {
        execute_options.push_str(&format!(" --context={}", context));
    }

    let cmdstr = format!(
        "lxc-attach --name={} {} -- {}",
        args.name,
        execute_options,
        args.command.join(" ").as_str()
    );

    vec![cmdstr]
}

pub fn start(args: StartArgs) -> Vec<String> {
    let mut start_options = String::new();

    if args.daemon {
        start_options.push_str(" --daemon");
    }

    if args.foreground {
        start_options.push_str(" --foreground");
    }

    if let Some(pidfile) = args.pidfile {
        start_options.push_str(&format!(" --pidfile={}", pidfile));
    }

    if let Some(rcfile) = args.rcfile {
        start_options.push_str(&format!(" --rcfile={}", rcfile));
    }

    if let Some(console) = args.console {
        start_options.push_str(&format!(" --console={}", console));
    }

    if let Some(console_log) = args.console_log {
        start_options.push_str(&format!(" --console-log={}", console_log));
    }

    if args.close_all_fds {
        start_options.push_str(" --close-all-fds");
    }

    if let Some(define) = args.define {
        start_options.push_str(&format!(" --define={}", define));
    }

    if let Some(share_net) = args.share_net {
        start_options.push_str(&format!(" --share-net={}", share_net));
    }

    if let Some(share_ipc) = args.share_ipc {
        start_options.push_str(&format!(" --share-ipc={}", share_ipc));
    }

    if let Some(share_uts) = args.share_uts {
        start_options.push_str(&format!(" --share-uts={}", share_uts));
    }

    if let Some(share_pid) = args.share_pid {
        start_options.push_str(&format!(" --share-pid={}", share_pid));
    }

    let cmdstr = format!("lxc-start --name={}{}", args.name, start_options);

    vec![cmdstr]
}

pub fn stop(args: StopArgs) -> Vec<String> {
    let mut stop_options = String::new();

    if args.reboot {
        stop_options.push_str(" --reboot");
    }

    if args.nowait {
        stop_options.push_str(" --nowait");
    }

    if let Some(timeout) = args.timeout {
        stop_options.push_str(&format!(" --timeout={}", timeout));
    }

    if args.kill {
        stop_options.push_str(" --kill");
    }

    if args.nolock {
        stop_options.push_str(" --nolock");
    }

    if args.nokill {
        stop_options.push_str(" --nokill");
    }

    if let Some(rcfile) = args.rcfile {
        stop_options.push_str(&format!(" --rcfile={}", rcfile));
    }

    let cmdstr = format!("lxc-stop --name={}{}", args.name, stop_options);

    vec![cmdstr]
}

pub fn list(args: ListArgs) -> Vec<String> {
    let mut list_options = String::new();

    if args.line {
        list_options.push_str(" --line");
    }

    if args.fancy {
        list_options.push_str(" --fancy");
    }

    if let Some(fancy_format) = args.fancy_format {
        list_options.push_str(&format!(" --fancy-format={}", fancy_format.join(",")));
    }

    if args.active {
        list_options.push_str(" --active");
    }

    if args.running {
        list_options.push_str(" --running");
    }

    if args.frozen {
        list_options.push_str(" --frozen");
    }

    if args.stopped {
        list_options.push_str(" --stopped");
    }

    if args.defined {
        list_options.push_str(" --defined");
    }

    if let Some(nesting) = args.nesting {
        list_options.push_str(&format!(" --nesting={}", nesting));
    }

    if let Some(filter) = args.filter {
        list_options.push_str(&format!(" --filter={}", filter));
    }

    if let Some(groups) = args.groups {
        list_options.push_str(&format!(" --groups={}", groups.join(",")));
    }

    let cmdstr = format!("lxc-ls{}", list_options);

    vec![cmdstr]
}

pub fn copy(args: CopyArgs) -> Vec<String> {
    let mut copy_options: String = String::from("--recursive");

    // Get source location
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

    // Get destination location
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

    if args.follow_link {
        copy_options.push_str(" --dereference");
    }

    if args.archive {
        copy_options.push_str(" --archive");
    }

    // Copy recursively and follows symbolic links
    let cmdstr = format!("cp {} {} {}", copy_options, source_path, destination_path);

    vec![cmdstr]
}

pub fn config(args: ConfigArgs) -> Vec<String> {
    let mut cmdstr: String = String::new();

    let mut config_options: String = String::new();

    if let Some(state_object) = args.state_object {
        cmdstr.push_str(&format!("lxc-cgroup --name={}", args.name));

        config_options.push_str(&format!(" {}", state_object[0]));
        if state_object.len() > 1 {
            config_options.push_str(&format!(" {}", state_object[1]));
        }
    } else {
        cmdstr.push_str(&format!("lxc-info --name={}", args.name));

        if let Some(config) = args.config {
            config_options.push_str(&format!(" --config={}", config));
        }
    }

    cmdstr.push_str(config_options.as_str());

    vec![cmdstr]
}

pub fn build(args: BuildArgs) -> Vec<String> {
    // Parse build file
    let lxcfilepath = format!("{}/{}", args.path.unwrap(), args.file.unwrap());

    // Parse file
    let contents = std::fs::read_to_string(lxcfilepath).expect("File not found");

    // Create container
    let container_build_file = contents.parse::<Table>().unwrap();

    let image = format!(
        "{}:{}:{}",
        container_build_file["image"]["distro"]
            .to_string()
            .trim_matches('\"'),
        container_build_file["image"]["release"]
            .to_string()
            .trim_matches('\"'),
        container_build_file["image"]["arch"]
            .to_string()
            .trim_matches('\"'),
    );

    let config_option = if container_build_file["image"]
        .as_table()
        .unwrap()
        .contains_key("config")
    {
        Some(container_build_file["image"]["config"].to_string())
    } else {
        Some(String::default())
    };

    let dir = if container_build_file["image"]
        .as_table()
        .unwrap()
        .contains_key("dir")
    {
        Some(container_build_file["image"]["dir"].to_string())
    } else {
        Some(String::default())
    };

    let network = if container_build_file["image"]
        .as_table()
        .unwrap()
        .contains_key("network")
    {
        Some(container_build_file["image"]["network"].to_string())
    } else {
        Some(String::default())
    };

    let container_name = container_build_file["name"]
        .to_string()
        .trim_matches('\"')
        .to_string();

    // Create container_build_file command
    let create_command = create(CreateArgs {
        name: container_name.clone(),
        image: image.clone(),
        config: config_option,
        dir: dir.clone(),
        network,
    });
    // Create container
    run_command(create_command[0].clone());
    if unsafe { STDOUT } {
        println!("[+] Container created");
    }

    // Create a shell script locally with the command
    // and the copy this shell script to the containers
    // /etc/init.d directory and gives it execution privileges
    if container_build_file.contains_key("entrypoint") {
        // Enable container configuration
        let path: String = if dir.clone().is_some_and(|dir| !dir.is_empty()) {
            format!("{}/etc/profile.d/lxcapp.sh", dir.clone().unwrap())
        } else {
            format!(
                "/var/lib/lxc/{}/rootfs/etc/profile.d/lxcapp.sh",
                container_name
            )
        };

        // Create executable for entrypoint
        let mut container_config_file = OpenOptions::new()
            .create_new(true)
            .append(true)
            .open(path)
            .unwrap();

        // Write commands to script
        _ = writeln!(
            container_config_file,
            "#!/bin/sh\n{}",
            container_build_file["entrypoint"]
                .to_string()
                .trim_matches('\"')
                .trim_matches('\'')
                .to_string()
        );
        let _ = container_config_file.flush();

        // Set as an executable
        let mut perm = container_config_file.metadata().unwrap().permissions();
        perm.set_mode(0o555);
        let _ = container_config_file.set_permissions(perm);
    }

    run_command(format!("lxc-start {}", container_name));
    if unsafe { STDOUT } {
        println!("[+] Container started");
    }

    // Handle copies. In this case, the source is always
    // the host and the destination is always the container
    if container_build_file.contains_key("copy") {
        if let Some(copies) = container_build_file["copy"].as_array() {
            for copy_elem in copies {
                let archive: bool = if copy_elem.as_table().unwrap().contains_key("archive") {
                    copy_elem.as_table().unwrap()["archive"].as_bool().unwrap()
                } else {
                    false
                };

                let follow_link: bool = if copy_elem.as_table().unwrap().contains_key("follow_link")
                {
                    copy_elem.as_table().unwrap()["follow_link"]
                        .as_bool()
                        .unwrap()
                } else {
                    false
                };

                let x: &[_] = &['.', '\"'];
                let copy_command = copy(CopyArgs {
                    source: copy_elem["host"].to_string().trim_matches('\"').to_string(),
                    destination: format!(
                        "{}:{}",
                        container_name,
                        copy_elem["container"].to_string().trim_matches(x)
                    ),
                    archive,
                    follow_link,
                });
                // Copy content
                run_command(copy_command[0].clone());
            }
        }
        if unsafe { STDOUT } {
            println!("[+] Content copied to the container");
        }
    }

    // Handle shared volume
    if container_build_file.contains_key("shared") {
        if let Some(locations) = container_build_file["shared"].as_array() {
            for location in locations {
                let location_table = location.as_table().unwrap();
                // Creates mount dir in host
                if !Path::new(&location_table["host"].to_string()).exists() {
                    run_command(format!("mkdir -p {}", location_table["host"]));
                }

                // Enable container configuration
                let mut container_config_file = OpenOptions::new()
                    .append(true)
                    .open(format!("/var/lib/lxc/{}/config", container_name))
                    .unwrap();

                _ = writeln!(
                    container_config_file,
                    "lxc.mount.entry = {} {} none bind,create=dir 0 0",
                    location_table["host"].to_string().trim_matches('\"'),
                    location_table["container"].to_string().trim_matches('\"')
                );
            }
        }
        if unsafe { STDOUT } {
            println!("[+] Shared volumes mounted");
        }
    }

    run_command(format!("lxc-stop {}", container_name));
    run_command(format!("lxc-start {}", container_name));

    if unsafe { STDOUT } {
        println!("[!] Running commands...");
    }

    // Handle run commands
    if container_build_file.contains_key("run") {
        if let Some(commands) = container_build_file["run"].as_array() {
            for command in commands {
                let cmd = command["cmd"].to_string().trim_matches('\"').to_string();
                let run_content_command =
                    format!("lxc-attach {} -- {}", container_name, cmd.clone());
                if unsafe { STDOUT } {
                    println!(" => {}", cmd.clone());
                }
                // Run command in content
                run_command(run_content_command.clone());
            }
        }
    }

    // Handle limits
    if container_build_file.contains_key("limits") {
        let limits_table = container_build_file["limits"].as_table().unwrap();

        for limit in limits_table {
            let config_command = config(ConfigArgs {
                name: container_name.clone(),
                state_object: Some(vec![
                    limit.0.replace("_", ".").to_string(),
                    limit.1.to_string().trim_matches('\"').to_string(),
                ]),
                config: Some(String::default()),
            });
            run_command(config_command[0].clone());
        }
    }

    run_command(format!("lxc-stop {}", container_name));
    run_command(format!("lxc-start {}", container_name));

    return vec!["echo [+] Container created".to_string()];
}

fn run_command(command: String) {
    let p_out = if unsafe { STDOUT } {
        Stdio::inherit()
    } else {
        Stdio::null()
    };
    let p_err = if unsafe { STDERR } {
        Stdio::inherit()
    } else {
        Stdio::null()
    };

    let mut command_and_args: Vec<&str> = command.split_whitespace().collect();
    match Command::new(command_and_args[0])
        .args(command_and_args.split_off(1))
        .stdout(p_out)
        .stderr(p_err)
        .spawn()
    {
        Ok(mut shell) => {
            let _ = shell.wait();
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
