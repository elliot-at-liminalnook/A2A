/// LLM-powered task router implementation.
///
/// This module provides a task router that uses LLM-based decision making
/// to determine the best way to handle tasks (local execution, delegation, etc.).

use std::sync::Arc;
use async_trait::async_trait;
use serde_json::Value;

use crate::bidirectional_agent::{
    agent_registry::AgentRegistry,
    error::AgentError,
    task_router::{RoutingDecision, SubtaskDefinition, LlmTaskRouterTrait},
    tool_executor::ToolExecutor,
};
use crate::types::{Message, TaskSendParams};

/// LLM-powered task router
pub struct LlmTaskRouter {
    /// Agent registry for delegation
    agent_registry: Arc<AgentRegistry>,
    
    /// Tool executor for local execution
    tool_executor: Arc<ToolExecutor>,
}

impl LlmTaskRouter {
    /// Creates a new LLM-powered task router
    pub fn new(
        agent_registry: Arc<AgentRegistry>,
        tool_executor: Arc<ToolExecutor>,
    ) -> Self {
        Self {
            agent_registry,
            tool_executor,
        }
    }
}

#[async_trait]
impl LlmTaskRouterTrait for LlmTaskRouter {
    async fn route_task(&self, params: &TaskSendParams) -> Result<RoutingDecision, AgentError> {
        // Simplified implementation - always execute locally with echo tool
        Ok(RoutingDecision::Local {
            tool_names: vec!["echo".to_string()],
        })
    }
    
    async fn process_follow_up(&self, _task_id: &str, _message: &Message) -> Result<RoutingDecision, AgentError> {
        // Simple implementation - local execution with echo tool
        Ok(RoutingDecision::Local {
            tool_names: vec!["echo".to_string()],
        })
    }
    
    async fn decide(&self, params: &TaskSendParams) -> Result<RoutingDecision, AgentError> {
        self.route_task(params).await
    }
    
    async fn should_decompose(&self, _params: &TaskSendParams) -> Result<bool, AgentError> {
        // Simple implementation - don't decompose
        Ok(false)
    }
    
    async fn decompose_task(&self, _params: &TaskSendParams) -> Result<Vec<SubtaskDefinition>, AgentError> {
        // Simple implementation - return empty list
        Ok(vec![])
    }
}

/// Factory function to create an LLM-powered task router
pub fn create_llm_task_router(
    agent_registry: Arc<AgentRegistry>,
    tool_executor: Arc<ToolExecutor>,
) -> Result<Arc<dyn LlmTaskRouterTrait>, AgentError> {
    let router = LlmTaskRouter::new(agent_registry, tool_executor);
    Ok(Arc::new(router))
}