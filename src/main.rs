use std::thread;
use std::time;

use clap::Parser;
use serde;
use serde::ser::SerializeStruct;
use serde_json;
use zookeeper::{Stat, WatchedEvent, Watcher, ZkError, ZooKeeper, ZooKeeperExt};

mod cmd;

struct LoggingWatcher;

impl Watcher for LoggingWatcher {
    fn handle(&self, _e: WatchedEvent) {}
}


#[derive(Debug, Default, serde::Serialize)]
struct OpResult {
    #[serde(default)]
    code: OpCode,
    #[serde(default)]
    znode_stat: Option<ZnodeStat>,
    #[serde(default)]
    value: Option<String>,
    #[serde(default)]
    error: Option<String>,
}

#[derive(Debug)]
struct ZnodeStat(Stat);

impl serde::Serialize for ZnodeStat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        let mut state = serializer.serialize_struct("OpStat", 11)?;
        state.serialize_field("czxid", &self.0.czxid)?;
        state.serialize_field("mzxid", &self.0.mzxid)?;
        state.serialize_field("ctime", &self.0.ctime)?;
        state.serialize_field("mtime", &self.0.mtime)?;
        state.serialize_field("version", &self.0.version)?;
        state.serialize_field("cversion", &self.0.cversion)?;
        state.serialize_field("aversion", &self.0.aversion)?;
        state.serialize_field("ephemeral_owner", &self.0.ephemeral_owner)?;
        state.serialize_field("data_length", &self.0.data_length)?;
        state.serialize_field("num_children", &self.0.num_children)?;
        state.serialize_field("pzxid", &self.0.pzxid)?;
        state.end()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum OpCode {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for OpCode {
    fn default() -> Self {
        OpCode::Success
    }
}

/// try to connect to the server by the given address
fn connect_zk(addr: &str) -> Result<ZooKeeper, ZkError> {
    let mut retry = 0;
    loop {
        match ZooKeeper::connect(addr, time::Duration::from_secs(5), LoggingWatcher) {
            Ok(zk_cli) => {
                return Ok(zk_cli);
            }
            Err(e) => {
                println!("Error connecting to ZooKeeper: {}", e);
                retry += 1;
                if retry >= 10 {
                    return Err(e);
                }
                thread::sleep(time::Duration::from_millis(10));
            }
        }
    }
}

fn gen_random_data(size: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(size);
    for _ in 0..size {
        data.push(rand::random::<u8>());
    }
    data
}

fn create(address: &str, arg: cmd::Create) -> OpResult {
    let zk_cli = connect_zk(address).unwrap();
    zk_cli.ensure_path(arg.path.as_str()).unwrap();
    let data;
    if arg.random_size > 0 || arg.value.is_none() {
        data = gen_random_data(arg.random_size);
    } else {
        data = arg.value.unwrap().as_bytes().to_vec();
    }
    let res: OpResult;
    match zk_cli.set_data(&arg.path, data, None) {
        Err(e) => {
            res = OpResult { code: OpCode::Failed, error: Some(e.to_string()), ..Default::default() };
        }
        Ok(s) => {
            res = OpResult { code: OpCode::Success, znode_stat: Some(ZnodeStat(s)), ..Default::default() };
        }
    };
    _ = zk_cli.close();
    res
}

fn set(address: &str, arg: &cmd::Set) -> OpResult {
    let zk_cli = connect_zk(address).unwrap();
    zk_cli.ensure_path(arg.path.as_str()).unwrap();
    let data;
    if arg.random_size > 0 || arg.value.is_none() {
        data = gen_random_data(arg.random_size);
    } else {
        data = arg.value.clone().unwrap().as_bytes().to_vec();
    }
    let res: OpResult;
    match zk_cli.set_data(&arg.path, data, None) {
        Err(e) => {
            res = OpResult { code: OpCode::Failed, error: Some(e.to_string()), ..Default::default() };
        }
        Ok(s) => {
            res = OpResult { code: OpCode::Success, znode_stat: Some(ZnodeStat(s)), ..Default::default() };
        }
    };
    _ = zk_cli.close();
    res
}

fn delete(address: &str, arg: &cmd::Delete) -> OpResult {
    let zk_cli = connect_zk(address).unwrap();
    let res: OpResult;
    match zk_cli.delete(arg.path.as_str(), None) {
        Ok(()) => { res = OpResult { code: OpCode::Success, ..Default::default() } }
        Err(_) => { res = OpResult { code: OpCode::Failed, ..Default::default() } }
    }
    _ = zk_cli.close();
    res
}

fn delete_all(address: &str, arg: &cmd::DeleteAll) -> OpResult {
    let zk_cli = connect_zk(address).unwrap();
    let res: OpResult;
    match zk_cli.delete_recursive(arg.path.as_str()) {
        Ok(()) => { res = OpResult { code: OpCode::Success, ..Default::default() } }
        Err(_) => { res = OpResult { code: OpCode::Failed, ..Default::default() } }
    }
    _ = zk_cli.close();
    res
}

fn exists(address: &str, arg: &cmd::Exists) -> OpResult {
    let zk_cli = connect_zk(address).unwrap();
    let mut res: OpResult;
    match zk_cli.exists(arg.path.as_str(), false) {
        Ok(s) => {
            res = OpResult { code: OpCode::Success, ..Default::default() };
            match s {
                Some(a) => {
                    res.znode_stat = Some(ZnodeStat(a));
                }
                None => {
                    res.znode_stat = None;
                }
            }
        }
        Err(_) => { res = OpResult { code: OpCode::Failed, ..Default::default() } }
    }
    _ = zk_cli.close();
    res
}

fn get(address: &str, arg: cmd::Get) -> OpResult {
    let zk_cli = connect_zk(address).unwrap();
    let res: OpResult;
    match zk_cli.get_data(arg.path.as_str(), false) {
        Ok(s) => {
            res = OpResult {
                code: OpCode::Success,
                znode_stat: Some(ZnodeStat(s.1)),
                value: String::from_utf8(s.0).ok(),
                ..Default::default()
            }
        }
        Err(_) => {
            res = OpResult {
                code: OpCode::Failed,
                ..Default::default()
            }
        }
    }
    _ = zk_cli.close();
    res
}


fn main() {
    let cli = cmd::Cli::parse();
    let res;
    match cli.command.unwrap() {
        cmd::SubCommands::Create(arg) => {
            res = create(cli.address.as_str(), arg);
        }
        cmd::SubCommands::Set(arg) => {
            res = set(cli.address.as_str(), &arg);
        }
        cmd::SubCommands::Delete(arg) => {
            res = delete(cli.address.as_str(), &arg);
        }
        cmd::SubCommands::DeleteAll(arg) => {
            res = delete_all(cli.address.as_str(), &arg);
        }
        cmd::SubCommands::Exists(arg) => {
            res = exists(cli.address.as_str(), &arg);
        }
        cmd::SubCommands::Get(arg) => {
            res = get(cli.address.as_str(), arg);
        }
    };
    println!("{}", serde_json::to_string(&res).unwrap());
}
