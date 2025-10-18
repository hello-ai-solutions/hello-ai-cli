use crate::local_pii_scanner::LocalPiiScanner;
use crate::Config;
use colored::Colorize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AgentRole {
    pub name: String,
    pub prompt_template: String,
}

#[derive(Debug, Clone)]
pub struct TaskPlan {
    pub description: String,
    pub agents: Vec<AgentRole>,
    pub execution_order: Vec<String>,
}

pub struct Orchestrator {
    available_agents: HashMap<String, AgentRole>,
    pii_scanner: LocalPiiScanner,
}

impl Orchestrator {
    pub fn new() -> Self {
        let mut available_agents = HashMap::new();

        // Define available agent roles
        available_agents.insert("security".to_string(), AgentRole {
            name: "Security Analyst".to_string(),
            prompt_template: "You are a security expert. Analyze the following for security vulnerabilities and best practices:".to_string(),
        });

        available_agents.insert("architect".to_string(), AgentRole {
            name: "Solution Architect".to_string(),
            prompt_template: "You are a solution architect. Design and analyze the following system architecture:".to_string(),
        });

        available_agents.insert(
            "devops".to_string(),
            AgentRole {
                name: "DevOps Engineer".to_string(),
                prompt_template: "You are a DevOps expert. Analyze and provide solutions for:"
                    .to_string(),
            },
        );

        available_agents.insert("troubleshooter".to_string(), AgentRole {
            name: "System Troubleshooter".to_string(),
            prompt_template: "You are a troubleshooting expert. Analyze the following logs/issues and provide solutions:".to_string(),
        });

        available_agents.insert(
            "compliance".to_string(),
            AgentRole {
                name: "Compliance Officer".to_string(),
                prompt_template:
                    "You are a compliance expert. Review the following for regulatory compliance:"
                        .to_string(),
            },
        );

        Self {
            available_agents,
            pii_scanner: LocalPiiScanner::new(
                "http://localhost:11434".to_string(),
                "qwen2.5-coder:14b".to_string(),
            ),
        }
    }

    pub async fn execute_orchestrated_task(
        &mut self,
        request: &str,
        _config: &Config,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Step 1: PII Scan
        println!("{}", "🔍 Scanning request for PII...".dimmed());
        match self.pii_scanner.scan_text(request).await {
            Ok(pii_results) => {
                if pii_results.has_pii {
                    println!("{} PII detected in request", "⚠️".yellow());
                    return Err("Request contains PII. Please remove sensitive information.".into());
                }
            }
            Err(_) => {
                println!(
                    "{}",
                    "⚠️ PII scan failed, proceeding with caution...".dimmed()
                );
            }
        }

        // Step 2: Analyze request and create task plan
        println!("{}", "🧠 Orchestrator analyzing request...".dimmed());
        let task_plan = self.create_task_plan(request).await?;

        println!("{} Task Plan Created:", "📋".bright_blue());
        println!("  {} {}", "Task:".dimmed(), task_plan.description);
        println!("  {} {}", "Agents:".dimmed(), task_plan.agents.len());

        // Step 3: Execute agents in order
        let mut results = HashMap::new();

        for agent_name in &task_plan.execution_order {
            if let Some(agent) = task_plan.agents.iter().find(|a| &a.name == agent_name) {
                println!("{} Executing agent: {}", "🤖".dimmed(), agent.name.dimmed());

                let agent_result = self.execute_agent(agent, request, &results).await?;
                results.insert(agent_name.clone(), agent_result);

                println!("{} {} completed", "✓".green().dimmed(), agent.name.dimmed());
            }
        }

        // Step 4: Synthesize final response
        println!("{}", "🎯 Synthesizing final response...".dimmed());
        let final_response = self.synthesize_response(&task_plan, &results).await?;

        Ok(final_response)
    }

    async fn create_task_plan(
        &self,
        request: &str,
    ) -> Result<TaskPlan, Box<dyn std::error::Error>> {
        let mut selected_agents = Vec::new();
        let mut execution_order = Vec::new();

        // Analyze request to determine required agents
        if request.contains("security")
            || request.contains("vulnerability")
            || request.contains("best practices")
        {
            if let Some(agent) = self.available_agents.get("security") {
                selected_agents.push(agent.clone());
                execution_order.push("Security Analyst".to_string());
            }
        }

        if request.contains("microservice")
            || request.contains("architecture")
            || request.contains("dependency")
        {
            if let Some(agent) = self.available_agents.get("architect") {
                selected_agents.push(agent.clone());
                execution_order.push("Solution Architect".to_string());
            }
        }

        if request.contains("logs") || request.contains("troubleshoot") || request.contains("debug")
        {
            if let Some(agent) = self.available_agents.get("troubleshooter") {
                selected_agents.push(agent.clone());
                execution_order.push("System Troubleshooter".to_string());
            }
        }

        if request.contains("deploy")
            || request.contains("ci/cd")
            || request.contains("infrastructure")
        {
            if let Some(agent) = self.available_agents.get("devops") {
                selected_agents.push(agent.clone());
                execution_order.push("DevOps Engineer".to_string());
            }
        }

        if request.contains("compliance")
            || request.contains("audit")
            || request.contains("governance")
        {
            if let Some(agent) = self.available_agents.get("compliance") {
                selected_agents.push(agent.clone());
                execution_order.push("Compliance Officer".to_string());
            }
        }

        // If no specific agents identified, use general troubleshooter
        if selected_agents.is_empty() {
            if let Some(agent) = self.available_agents.get("troubleshooter") {
                selected_agents.push(agent.clone());
                execution_order.push("System Troubleshooter".to_string());
            }
        }

        Ok(TaskPlan {
            description: self.extract_task_description(request),
            agents: selected_agents,
            execution_order,
        })
    }

    async fn execute_agent(
        &self,
        agent: &AgentRole,
        request: &str,
        previous_results: &HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Build context from previous agent results
        let mut context = String::new();
        if !previous_results.is_empty() {
            context.push_str("Previous agent findings:\n");
            for (agent_name, result) in previous_results {
                context.push_str(&format!("{}: {}\n", agent_name, result));
            }
            context.push_str("\n");
        }

        // Create agent-specific prompt
        let _full_prompt = format!(
            "{}\n\nContext: {}\n\nRequest: {}",
            agent.prompt_template, context, request
        );

        // Simulate agent execution (in real implementation, this would call the specific LLM)
        let response = match agent.name.as_str() {
            "Security Analyst" => self.simulate_security_analysis(request).await,
            "Solution Architect" => self.simulate_architecture_analysis(request).await,
            "DevOps Engineer" => self.simulate_devops_analysis(request).await,
            "System Troubleshooter" => self.simulate_troubleshooting_analysis(request).await,
            "Compliance Officer" => self.simulate_compliance_analysis(request).await,
            _ => "Analysis completed".to_string(),
        };

        Ok(response)
    }

    async fn simulate_security_analysis(&self, request: &str) -> String {
        format!(
            "Security Analysis Results:
• Authentication: Implement multi-factor authentication
• Authorization: Use role-based access control (RBAC)
• Data Protection: Encrypt data at rest and in transit
• Network Security: Use VPC with private subnets
• Monitoring: Enable security logging and alerting
• Compliance: Follow OWASP Top 10 guidelines

Specific to your request: {}",
            if request.contains("microservice") {
                "Implement service mesh for secure inter-service communication"
            } else if request.contains("logs") {
                "Ensure logs don't contain sensitive data"
            } else {
                "Apply security best practices for your use case"
            }
        )
    }

    async fn simulate_architecture_analysis(&self, request: &str) -> String {
        format!(
            "Architecture Analysis Results:
• Design Pattern: Microservices with API Gateway
• Data Flow: Event-driven architecture with message queues
• Scalability: Horizontal scaling with load balancers
• Resilience: Circuit breaker pattern and retry mechanisms
• Dependencies: Minimize coupling between services
• Monitoring: Distributed tracing and health checks

Specific recommendations: {}",
            if request.contains("dependency") {
                "Use dependency injection and service discovery"
            } else if request.contains("troubleshoot") {
                "Implement comprehensive logging and monitoring"
            } else {
                "Follow 12-factor app principles"
            }
        )
    }

    async fn simulate_devops_analysis(&self, request: &str) -> String {
        format!(
            "DevOps Analysis Results:
• CI/CD Pipeline: Automated testing and deployment
• Infrastructure: Infrastructure as Code (Terraform)
• Containerization: Docker with Kubernetes orchestration
• Monitoring: Prometheus and Grafana stack
• Backup Strategy: Automated backups with point-in-time recovery
• Deployment: Blue-green deployment strategy

Implementation steps: {}",
            if request.contains("microservice") {
                "Set up service mesh and container registry"
            } else if request.contains("logs") {
                "Configure centralized logging with ELK stack"
            } else {
                "Implement GitOps workflow"
            }
        )
    }

    async fn simulate_troubleshooting_analysis(&self, request: &str) -> String {
        format!(
            "Troubleshooting Analysis Results:
• Log Analysis: Check application and system logs
• Performance Metrics: Monitor CPU, memory, and network usage
• Error Patterns: Identify recurring error messages
• Dependencies: Verify external service connectivity
• Resource Constraints: Check for resource bottlenecks
• Configuration: Validate configuration settings

Action items: {}",
            if request.contains("logs") {
                "Parse logs for error patterns and performance issues"
            } else if request.contains("microservice") {
                "Check service-to-service communication"
            } else {
                "Perform root cause analysis"
            }
        )
    }

    async fn simulate_compliance_analysis(&self, request: &str) -> String {
        format!(
            "Compliance Analysis Results:
• Data Privacy: GDPR/CCPA compliance requirements
• Security Standards: SOC 2 Type II certification
• Industry Regulations: Sector-specific compliance (HIPAA, PCI-DSS)
• Audit Trail: Comprehensive logging and monitoring
• Access Controls: Principle of least privilege
• Data Retention: Automated data lifecycle management

Compliance checklist: {}",
            if request.contains("security") {
                "Implement security controls per compliance framework"
            } else if request.contains("logs") {
                "Ensure audit logs meet retention requirements"
            } else {
                "Document compliance procedures"
            }
        )
    }

    async fn synthesize_response(
        &self,
        task_plan: &TaskPlan,
        results: &HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut final_response = String::new();

        final_response.push_str(&format!("🎭 Orchestrated Analysis Complete\n"));
        final_response.push_str(&format!("Task: {}\n\n", task_plan.description));

        for agent_name in &task_plan.execution_order {
            if let Some(result) = results.get(agent_name) {
                final_response.push_str(&format!("## {} Report\n{}\n\n", agent_name, result));
            }
        }

        final_response.push_str("## Summary\n");
        final_response.push_str("All agents have completed their analysis. Review the individual reports above for detailed findings and recommendations.\n");

        Ok(final_response)
    }

    fn extract_task_description(&self, request: &str) -> String {
        if request.contains("security best practices") {
            "Security Best Practices Analysis".to_string()
        } else if request.contains("microservice dependency") {
            "Microservice Dependency Analysis".to_string()
        } else if request.contains("analyze logs") {
            "Log Analysis and Troubleshooting".to_string()
        } else if request.contains("troubleshoot") {
            "System Troubleshooting".to_string()
        } else {
            "General System Analysis".to_string()
        }
    }
}
