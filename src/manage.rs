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

use std::path::Path;

use subprocess::Exec;

use crate::{
    quiet_println, ConfigArgs, CopyArgs, CreateArgs, DeleteArgs, ExecuteArgs, ListArgs, StartArgs, StopArgs
};

pub fn create(args: CreateArgs) -> String {
    let mut create_options: String = String::new();
    if args.config.is_some() {
        create_options.push_str(&format!(" --config={}", args.config.unwrap()));
    }

    if args.dir.is_some() {
        if !Path::new(args.dir.clone().unwrap().as_str()).exists() {
                _ = std::fs::create_dir(args.dir.clone().unwrap().as_str());
            }
        create_options.push_str(&format!(" --dir={}",args.dir.unwrap().as_str()));
    }

    // Parse template
    let image: Vec<&str> = args.image.split(':').collect();

    let cmdstr = format!(
        "lxc-create --name={}{} --template=download -- --dist={} --release={} --arch={}",
        args.name, create_options, image[0], image[1], image[2],
    );

    cmdstr
}

pub fn delete(args: DeleteArgs) -> String {
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

    cmdstr
}

pub fn execute(args: ExecuteArgs) -> String {
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
        args.name, execute_options, args.command
    );

    cmdstr
}

pub fn start(args: StartArgs) -> String {
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

    cmdstr
}

pub fn stop(args: StopArgs) -> String {
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

    let cmdstr = format!(
        "lxc-stop --name={}{}",
        args.name,
        stop_options
    );

    cmdstr
}


pub fn list(args: ListArgs) -> String {
    let mut list_options = String::new();

    if args.line {
        list_options.push_str(" --line");
    }

    if args.fancy {
        list_options.push_str(" --fancy");
    }

    if let Some(fancy_format) = args.fancy_format {
        list_options.push_str(&format!(" --fancy-format={}", fancy_format));
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

    let cmdstr = format!(
        "lxc-ls{}",
        list_options
    );

    cmdstr
}


pub fn copy(args: CopyArgs) -> String {
    let mut copy_options: String = String::new();

    // Get source location
    let mut source_path = String::new();
    let source_location: Vec<&str> = args.source.split(':').collect();
    if args.source.contains(':') && source_location.len() > 1{
        // Find rootfs path
        source_path = (Exec::shell(&format!("lxc-info --name={} --config=lxc.rootfs.path", source_location[0])) | Exec::shell("cut -c 19-")).capture().unwrap().stdout_str().trim().to_string();
        source_path.push_str(source_location[1]);
    }else{
        source_path.push_str(source_location[0]);
    }
    

    // Get destination location
    let mut destination_path = String::new();
    let destination_location: Vec<&str> = args.destination.split(':').collect();
    if args.destination.contains(':') && destination_location.len() > 1{
        // Find rootfs path
        destination_path = (Exec::shell(&format!("lxc-info --name={} --config=lxc.rootfs.path", destination_location[0])) | Exec::shell("cut -c 19-")).capture().unwrap().stdout_str().trim().to_string();
        destination_path.push_str(destination_location[1]);
    }else{
        destination_path.push_str(destination_location[0]);
    } 

    if args.follow_link{
        copy_options.push_str(" --dereference");
    }

    if args.archive{
        copy_options.push_str(" --archive");
    }

    // Copy recursively and follows symbolic links
    let cmdstr = format!("cp {} {} {}", copy_options, source_path, destination_path);

    cmdstr
}

// TODO
pub fn config(args: ConfigArgs) -> String {
    let mut config_options: String = String::new();

    let cmdstr = format!("lxc-config --name={}{}", args.name, config_options,);

    cmdstr
}
