use std::fmt::Debug;

use async_trait::async_trait;

use crate::models::{
    agent_basic::basic_agent::{AgentState, BasicAgent},
    agents::traits::FactSheet,
    general::llm::Message,
};

pub trait BasicTraits {
    fn new(objective: String, position: String) -> Self;
    fn update_state(&mut self, new_state: AgentState);
    fn get_objective(&self) -> &String;
    fn get_position(&self) -> &String;
    fn get_state(&self) -> &AgentState;
    fn get_memory(&self) -> &Vec<Message>;
}

#[async_trait]
pub trait SpecialFunctions: Debug {
    // Used to that manager can get attributes from Agents
    fn get_attributes_from_agent(&self) -> &BasicAgent;

    // This function will allow agents to execute their logic
    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
