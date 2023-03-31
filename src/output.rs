use serde;
use serde::ser::SerializeStruct;
use zookeeper::Stat;

/// The cli command output structure
#[derive(Debug, Default, serde::Serialize)]
pub struct OpResult {
    #[serde(default)]
    pub code: OpCode,
    #[serde(default)]
    pub znode_stat: Option<ZnodeStat>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}

#[derive(Debug)]
pub struct ZnodeStat(pub Stat);

/// As the zookeeper::Stat structure has not implemented trait Serialize, so
/// we need to wrap it in ZnodeStat struct and implement the Serialize trait.
impl serde::Serialize for ZnodeStat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
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

/// The execute result of command.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum OpCode {
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
