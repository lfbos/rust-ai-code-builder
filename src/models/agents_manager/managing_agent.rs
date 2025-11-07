use crate::models::{
    agent_basic::{
        basic_agent::{AgentState, BasicAgent},
        basic_traits::SpecialFunctions,
    },
    agents::traits::FactSheet,
};

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}
