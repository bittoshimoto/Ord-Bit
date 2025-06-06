use serde_json::{json, Value};
use {
    crate::{Inscription, InscriptionId, SatPoint},
    bitcoin::Txid,
    serde::{Deserialize, Serialize},
};
use crate::bit20::deploy::Deploy;
use crate::bit20::errors::{JSONError};
use crate::bit20::mint::Mint;
use crate::bit20::OperationType;
use crate::bit20::params::PROTOCOL_LITERAL;
use crate::bit20::transfer::Transfer;
use anyhow::{Result};

// collect the inscription operation.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct InscriptionOp {
    pub txid: Txid,
    pub action: Action,
    pub inscription_number: Option<u64>,
    pub inscription_id: InscriptionId,
    pub old_satpoint: SatPoint,
    pub new_satpoint: Option<SatPoint>,
}

// the act of marking an inscription.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Action {
    New {
        inscription: Inscription,
    },
    Transfer,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Deploy(Deploy),
    Mint(Mint),
    InscribeTransfer(Transfer),
    Transfer(Transfer),
}

impl Operation {
    pub fn op_type(&self) -> OperationType {
        match self {
            Operation::Deploy(_) => OperationType::Deploy,
            Operation::Mint(_) => OperationType::Mint,
            Operation::InscribeTransfer(_) => OperationType::InscribeTransfer,
            Operation::Transfer(_) => OperationType::Transfer,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(tag = "op")]
enum RawOperation {
    #[serde(rename = "deploy")]
    Deploy(Deploy),
    #[serde(rename = "mint")]
    Mint(Mint),
    #[serde(rename = "transfer")]
    Transfer(Transfer),
}

pub(crate) fn deserialize_bit20_operation(
    inscription: &Inscription,
    action: &Action,
) -> Result<Operation> {
    let content_body = std::str::from_utf8(inscription.body().ok_or(JSONError::InvalidJson)?)?;
    if content_body.len() < 40 {
        return Err(JSONError::NotBIT20Json.into());
    }

    let content_type = inscription
        .content_type()
        .ok_or(JSONError::InvalidContentType)?;

    if content_type != "text/plain"
        && content_type != "text/plain;charset=utf-8"
        && content_type != "text/plain;charset=UTF-8"
        && content_type != "text/plain; charset=utf-8"
        && content_type != "text/plain; charset=UTF-8"
        && content_type != "application/json"
        && !content_type.starts_with("text/plain;")
    {
        return Err(JSONError::UnSupportContentType.into());
    }
    let raw_operation = match deserialize_bit20(content_body) {
        Ok(op) => op,
        Err(e) => {
            return Err(e.into());
        }
    };

    match action {
        Action::New { .. } => match raw_operation {
            RawOperation::Deploy(deploy) => Ok(Operation::Deploy(deploy)),
            RawOperation::Mint(mint) => Ok(Operation::Mint(mint)),
            RawOperation::Transfer(transfer) => Ok(Operation::InscribeTransfer(transfer)),
        },
        Action::Transfer => match raw_operation {
            RawOperation::Transfer(transfer) => Ok(Operation::Transfer(transfer)),
            _ => Err(JSONError::NotBIT20Json.into()),
        },
    }
}

fn deserialize_bit20(s: &str) -> Result<RawOperation, JSONError> {
    let value: Value = serde_json::from_str(s).map_err(|_| JSONError::InvalidJson)?;
    if value.get("p") != Some(&json!(PROTOCOL_LITERAL)) {
        return Err(JSONError::NotBIT20Json);
    }

    serde_json::from_value(value).map_err(|e| JSONError::ParseOperationJsonError(e.to_string()))
}
