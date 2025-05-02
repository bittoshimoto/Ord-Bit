use super::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PType {
  #[serde(rename = "dev-20")]
  Bit20,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
  Transfer,
  Mint,
  Deploy,
  Unknown,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct BIT20 {
  pub p: Option<PType>,
  pub op: Option<Operation>,
  pub tick: Option<String>,
  pub amt: Option<String>,
  pub max: Option<String>,
  pub limit: Option<String>,
}

impl BIT20 {
  pub fn from_json_string(json_str: &str) -> Option<Self> {
    match serde_json::from_str::<BIT20>(json_str) {
      Ok(bit20) => {
        if bit20.is_valid() {
          Some(bit20)
        } else {
          None
        }
      }
      Err(err) => {
        log::debug!("Error deserializing JSON: {}", err);
        None
      }
    }
  }

  fn is_valid(&self) -> bool {
    self.p.is_some()
      && self.tick.is_some()
      && self.clone().op.is_some_and(|op| op != Operation::Unknown)
  }
}
