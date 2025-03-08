use borsh::{io::Error, BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use sdk::{Digestable, HyleContract, RunResult};

/// The state of the contract, including the agent action attestation
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct AgentContract {
    pub attestation: AgentActionAttestation,
}

/// Represents the agent action attestation
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct AgentActionAttestation {
    pub agent: String,
    pub action_name: String,
    pub action_successful: bool,
}

impl HyleContract for AgentContract {
    /// Entry point of the contract's logic
    fn execute(&mut self, contract_input: &sdk::ContractInput) -> RunResult {
        // Parse contract inputs
        let (action, ctx) = sdk::utils::parse_raw_contract_input::<AgentAction>(contract_input)?;

        // Execute the contract logic
        match action {
            AgentAction::SetAgentAction { agent, action_name } => {
                self.attestation.agent = agent;
                self.attestation.action_name = action_name;
            }
            AgentAction::MarkExecutionStatus { success } => {
                self.attestation.action_successful = success;
            }
        }

        // program_output might be used to give feedback to the user
        let program_output = format!(
            "Agent: {}, Action: {}, Success: {}",
            self.attestation.agent, self.attestation.action_name, self.attestation.action_successful
        );
        Ok((program_output, ctx, vec![]))
    }
}

/// The action represents the different operations that can be done on the contract
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum AgentAction {
    SetAgentAction { agent: String, action_name: String },
    MarkExecutionStatus { success: bool },
}

/// Utils function for the host
impl AgentContract {
    pub fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        borsh::to_vec(self)
    }
}

/// Utils function for the host
impl AgentAction {
    pub fn as_blob(&self, contract_name: &str) -> sdk::Blob {
        sdk::Blob {
            contract_name: contract_name.into(),
            data: sdk::BlobData(borsh::to_vec(self).expect("failed to encode BlobData")),
        }
    }
}

/// Helpers to transform the contract's state into its on-chain state digest version.
impl Digestable for AgentContract {
    fn as_digest(&self) -> sdk::StateDigest {
        sdk::StateDigest(borsh::to_vec(self).expect("Failed to encode AgentContract"))
    }
}

impl From<sdk::StateDigest> for AgentContract {
    fn from(state: sdk::StateDigest) -> Self {
        borsh::from_slice(&state.0)
            .map_err(|_| "Could not decode hyllar state".to_string())
            .unwrap()
    }
}
