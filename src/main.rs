use anyhow::Result;
use rmcp::{
    handler::server::router::tool::ToolRouter,
    model::{ErrorData as McpError, *},
    tool, tool_handler, tool_router,
    transport::stdio,
    ServerHandler, ServiceExt,
};
use std::future::Future;

#[derive(Debug, Clone)]
pub struct PingServer {
    tool_router: ToolRouter<PingServer>,
}

#[tool_router]
impl PingServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// A simple ping tool that responds with pong
    #[tool(description = "A simple ping tool that responds with pong")]
    async fn ping(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text("pong")]))
    }
}

#[tool_handler]
impl ServerHandler for PingServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "A simple MCP server with a ping tool that responds with pong".to_string(),
            ),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let service = PingServer::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
