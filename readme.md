# About
This is a simple none-interactive cli command tool that CRUD with zookeeper server, and print the result in serialized json format.


## Output json format
```json
{
    "code": "success", // or "failed"
    "znode_stat": {...}, // znode information if exists
    "value": "hello,world", // value of the znode
    "error": "", // error message if any
}
```

# Usage example
## Create
### create znode with given value
```bash
./zkcli --address 127.0.0.1:2181 create --path /test --value "hello,world"
```
output:
```json
{"code":"success","znode_stat":{"czxid":14178674,"mzxid":14178758,"ctime":1680261144362,"mtime":1680273483665,"version":10,"cversion":0,"aversion":0,"ephemeral_owner":0,"data_length":11,"num_children":0,"pzxid":14178674},"value":null,"error":null}
```

### create znode with fix-sized random value
```bash
./zkcli --address 127.0.0.1:2181 create --path /test --random-size 20
```
output:
```json
{"code":"success","znode_stat":{"czxid":14178674,"mzxid":14178762,"ctime":1680261144362,"mtime":1680273803350,"version":11,"cversion":0,"aversion":0,"ephemeral_owner":0,"data_length":20,"num_children":0,"pzxid":14178674},"value":null,"error":null}
```

## Set
### set a not exist znode
```bash
./zkcli --address 127.0.0.1:2181 set --path /node-not-exist 
```
output:
```json
{"code":"failed","znode_stat":null,"value":null,"error":"Zookeeper Error: NoNode"}
```

### set znode with given value
```bash
./zkcli --address 127.0.0.1:2181 set --path /test --value 'winter-is-coming'
```
output:
```json
{"code":"success","znode_stat":{"czxid":14178674,"mzxid":14178768,"ctime":1680261144362,"mtime":1680273899237,"version":12,"cversion":0,"aversion":0,"ephemeral_owner":0,"data_length":16,"num_children":0,"pzxid":14178674},"value":null,"error":null}
```

### set znode with fix-sized random value
```bash
./zkcli --address 127.0.0.1:2181 set --path /test --random-size 20
```
output:
```json
{"code":"success","znode_stat":{"czxid":14178674,"mzxid":14178772,"ctime":1680261144362,"mtime":1680274008100,"version":13,"cversion":0,"aversion":0,"ephemeral_owner":0,"data_length":20,"num_children":0,"pzxid":14178674},"value":null,"error":null}
```
## Get
```bash
./zkcli --address 127.0.0.1:2181 get --path /test
```
output:
```json
{"code":"success","znode_stat":{"czxid":14178674,"mzxid":14178772,"ctime":1680261144362,"mtime":1680274008100,"version":13,"cversion":0,"aversion":0,"ephemeral_owner":0,"data_length":20,"num_children":0,"pzxid":14178674},"value":"t5ijuhp1awdgcmdz6kmf","error":null}
```

get value that znode not exists
```bash
./zkcli --address 127.0.0.1:2181 get --path /node-not-exists
```
output:
```json
{"code":"failed","znode_stat":null,"value":null,"error":"Zookeeper Error: NoNode"}
```

## Exists
check a znode exists or not.
```bash
./zkcli --address 127.0.0.1:2181 exists --path /test
```
output(success with znode_stat information):
```json
{"code":"success","znode_stat":{"czxid":14178674,"mzxid":14178772,"ctime":1680261144362,"mtime":1680274008100,"version":13,"cversion":0,"aversion":0,"ephemeral_owner":0,"data_length":20,"num_children":0,"pzxid":14178674},"value":null,"error":null}
```

```bash
./zkcli --address 127.0.0.1:2181 exists --path /node-not-exist
```
output(success with null znode_stat information):
```json
{"code":"success","znode_stat":null,"value":null,"error":null}
```

## Delete
delete a znode
```bash
./zkcli --address 127.0.0.1:2181 delete --path /test
```
output:
```json
{"code":"success","znode_stat":null,"value":null,"error":null}
```

## DeleteAll
delete znode recursively
```bash
./zkcli --address 127.0.0.1:2181 deleteall --path /test
```
output:
```json
{"code":"success","znode_stat":null,"value":null,"error":null}
```
