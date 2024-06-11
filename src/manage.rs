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

use crate::{
    quiet_println, Backingstore, ConfigArgs, CopyArgs, CreateArgs, DeleteArgs, ExecuteArgs, ListArgs, StartArgs, StopArgs
};
use crate::utils;

use core::sync::atomic::Ordering;
use std::rc::Rc;

pub fn create(args: CreateArgs) -> String {
    let mut create_options: String = String::new();
    if args.config.is_some() {
        create_options.push_str(&format!(" --config={}", args.config.unwrap()));
    }

    // Backingstore modes
    let mut backingstore_options: String = String::new();
    match args.sub {
        Some(Backingstore::DIR(o_args)) => {
            backingstore_options.push_str(
                &format!(" --dir={}",
                o_args.dir));
        },
        Some(Backingstore::LVM(o_args)) => {
            backingstore_options.push_str(
                &format!(" --bdev lvm --lvname={} --vgname={} --thinpool={} --fssize={} --fstype={}",
                o_args.lvname.unwrap_or(args.name.clone()),
                o_args.vgname.unwrap(),
                o_args.thinpool.unwrap(),
                o_args.fssize.unwrap(),
                o_args.fstype.unwrap()));
        },
        Some(Backingstore::RBD(o_args)) => {
            backingstore_options.push_str(
                &format!(" --bdev rbd --rbdname={} --rbdpool={}",
                o_args.rbdname.unwrap_or(args.name.clone()),
                o_args.rbdpool.unwrap()));
        },
        Some(Backingstore::ZFS(o_args)) => {
            backingstore_options.push_str(
                &format!(" --bdev=zfs --zfsroot={}",
                o_args.zfsroot.unwrap()));
        },
        Some(Backingstore::LOOP(o_args)) => {
            backingstore_options.push_str(
                &format!(" --bdev=loop --fssize={} --fstype={}",
                o_args.fssize.unwrap(),
                o_args.fstype.unwrap()));
        },
        _ => {}
    }

    let cmdstr= format!(
        "lxc-create --name={} --template={}{}{}",
        args.name,
        args.template,
        create_options,
        backingstore_options,
    );

    return cmdstr
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
    
    let cmdstr= format!(
        "lxc-destroy --name={}{}",
        args.name,
        delete_options,
    );

    return cmdstr;
}

pub fn execute(args: ExecuteArgs) -> String {
    let mut execute_options: String = String::new();

    if args.daemon {
        execute_options.push_str(&format!(" --snapshots"));
    }

    let cmdstr= format!(
        "lxc-execute --name={}{}",
        args.name,
        execute_options,
    );

    return cmdstr;
}

// TODO
pub fn start(args: StartArgs) -> String{
    let mut start_options: String = String::new();

    let cmdstr= format!(
        "lxc-start --name={}{}",
        args.name,
        start_options,
    );

    return cmdstr;
}

// TODO
pub fn stop(args: StopArgs) -> String {
    let mut stop_options: String = String::new();

    let cmdstr= format!(
        "lxc-stop --name={}{}",
        args.name,
        stop_options,
    );

    return cmdstr;
}

// TODO
pub fn list(args: ListArgs) -> String {
    let mut list_options: String = String::new();

    let cmdstr= format!(
        "lxc-list --name={}{}",
        args.name,
        list_options,
    );

    return cmdstr;
}

// TODO
pub fn copy(args: CopyArgs) -> String {
    let mut copy_options: String = String::new();

    let cmdstr= format!(
        "lxc-copy --name={}{}",
        args.name,
        copy_options,
    );

    return cmdstr;
}

// TODO
pub fn config(args: ConfigArgs) -> String {
    let mut config_options: String = String::new();

    let cmdstr= format!(
        "lxc-config --name={}{}",
        args.name,
        config_options,
    );

    return cmdstr;
}