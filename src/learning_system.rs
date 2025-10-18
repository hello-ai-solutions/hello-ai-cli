use chrono::Timelike;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String,
    pub preferences: UserPreferences,
    pub usage_patterns: UsagePatterns,
    pub learning_history: Vec<LearningEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub preferred_languages: Vec<String>,
    pub coding_style: String,
    pub complexity_level: String,
    pub favorite_tools: Vec<String>,
    pub response_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsagePatterns {
    pub most_used_commands: HashMap<String, u32>,
    pub common_file_types: HashMap<String, u32>,
    pub typical_session_length: u32,
    pub peak_usage_hours: Vec<u8>,
    pub error_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEvent {
    pub timestamp: String,
    pub event_type: String,
    pub context: String,
    pub user_feedback: Option<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct PersonalizedSuggestion {
    pub suggestion_type: String,
    pub content: String,
    pub confidence: f32,
    pub reasoning: String,
}

pub struct LearningSystem {
    profiles_dir: String,
    current_user: Option<String>,
}

impl LearningSystem {
    pub fn new() -> Self {
        let profiles_dir = ".amazonq/user_profiles".to_string();
        fs::create_dir_all(&profiles_dir).ok();

        Self {
            profiles_dir,
            current_user: None,
        }
    }

    pub fn set_user(&mut self, user_id: &str) {
        self.current_user = Some(user_id.to_string());
    }

    pub async fn load_or_create_profile(
        &self,
        user_id: &str,
    ) -> Result<UserProfile, Box<dyn std::error::Error>> {
        let profile_path = format!("{}/{}.json", self.profiles_dir, user_id);

        if Path::new(&profile_path).exists() {
            let content = fs::read_to_string(&profile_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(self.create_default_profile(user_id))
        }
    }

    fn create_default_profile(&self, user_id: &str) -> UserProfile {
        UserProfile {
            user_id: user_id.to_string(),
            preferences: UserPreferences {
                preferred_languages: vec!["rust".to_string()],
                coding_style: "functional".to_string(),
                complexity_level: "intermediate".to_string(),
                favorite_tools: vec!["cargo".to_string(), "git".to_string()],
                response_format: "detailed".to_string(),
            },
            usage_patterns: UsagePatterns {
                most_used_commands: HashMap::new(),
                common_file_types: HashMap::new(),
                typical_session_length: 30,
                peak_usage_hours: vec![9, 10, 14, 15],
                error_patterns: Vec::new(),
            },
            learning_history: Vec::new(),
        }
    }

    pub async fn save_profile(
        &self,
        profile: &UserProfile,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let profile_path = format!("{}/{}.json", self.profiles_dir, profile.user_id);
        let content = serde_json::to_string_pretty(profile)?;
        fs::write(&profile_path, content)?;
        Ok(())
    }

    pub async fn record_usage(
        &self,
        user_id: &str,
        command: &str,
        file_type: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut profile = self.load_or_create_profile(user_id).await?;

        // Update command usage
        *profile
            .usage_patterns
            .most_used_commands
            .entry(command.to_string())
            .or_insert(0) += 1;

        // Update file type usage
        if let Some(ft) = file_type {
            *profile
                .usage_patterns
                .common_file_types
                .entry(ft.to_string())
                .or_insert(0) += 1;
        }

        // Record learning event
        profile.learning_history.push(LearningEvent {
            timestamp: chrono::Utc::now().to_rfc3339(),
            event_type: "command_usage".to_string(),
            context: command.to_string(),
            user_feedback: None,
            success: true,
        });

        self.save_profile(&profile).await?;
        Ok(())
    }

    pub async fn record_feedback(
        &self,
        user_id: &str,
        context: &str,
        feedback: &str,
        success: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut profile = self.load_or_create_profile(user_id).await?;

        profile.learning_history.push(LearningEvent {
            timestamp: chrono::Utc::now().to_rfc3339(),
            event_type: "user_feedback".to_string(),
            context: context.to_string(),
            user_feedback: Some(feedback.to_string()),
            success,
        });

        self.save_profile(&profile).await?;
        Ok(())
    }

    pub async fn get_personalized_suggestions(
        &self,
        user_id: &str,
        context: &str,
    ) -> Result<Vec<PersonalizedSuggestion>, Box<dyn std::error::Error>> {
        let profile = self.load_or_create_profile(user_id).await?;
        let mut suggestions = Vec::new();

        // Command suggestions based on usage patterns
        if let Some((most_used_cmd, count)) = profile
            .usage_patterns
            .most_used_commands
            .iter()
            .max_by_key(|(_, &count)| count)
        {
            if *count > 5 {
                suggestions.push(PersonalizedSuggestion {
                    suggestion_type: "command_shortcut".to_string(),
                    content: format!(
                        "You frequently use '{}'. Consider creating an alias for faster access.",
                        most_used_cmd
                    ),
                    confidence: 0.8,
                    reasoning: format!("Used {} times in recent sessions", count),
                });
            }
        }

        // Language-specific suggestions
        for lang in &profile.preferences.preferred_languages {
            if context.contains(lang) {
                suggestions.push(PersonalizedSuggestion {
                    suggestion_type: "language_specific".to_string(),
                    content: format!("Based on your {} preference, consider using language-specific best practices.", lang),
                    confidence: 0.7,
                    reasoning: format!("{} is in your preferred languages", lang),
                });
            }
        }

        // Complexity-based suggestions
        match profile.preferences.complexity_level.as_str() {
            "beginner" => {
                suggestions.push(PersonalizedSuggestion {
                    suggestion_type: "learning_path".to_string(),
                    content: "Try the /explain command to understand code better".to_string(),
                    confidence: 0.9,
                    reasoning: "Beginner level - focus on understanding".to_string(),
                });
            }
            "advanced" => {
                suggestions.push(PersonalizedSuggestion {
                    suggestion_type: "advanced_feature".to_string(),
                    content: "Consider using /refactor for code optimization opportunities"
                        .to_string(),
                    confidence: 0.8,
                    reasoning: "Advanced level - focus on optimization".to_string(),
                });
            }
            _ => {}
        }

        // Time-based suggestions
        let current_hour = chrono::Utc::now().hour() as u8;
        if profile
            .usage_patterns
            .peak_usage_hours
            .contains(&current_hour)
        {
            suggestions.push(PersonalizedSuggestion {
                suggestion_type: "productivity_tip".to_string(),
                content:
                    "This is your peak productivity time! Consider tackling complex tasks now."
                        .to_string(),
                confidence: 0.6,
                reasoning: "Based on your usage patterns".to_string(),
            });
        }

        Ok(suggestions)
    }

    pub async fn adapt_response_style(
        &self,
        user_id: &str,
        base_response: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let profile = self.load_or_create_profile(user_id).await?;

        match profile.preferences.response_format.as_str() {
            "concise" => Ok(self.make_concise(base_response)),
            "detailed" => Ok(self.make_detailed(base_response)),
            "code_focused" => Ok(self.make_code_focused(base_response)),
            _ => Ok(base_response.to_string()),
        }
    }

    fn make_concise(&self, response: &str) -> String {
        // Simplify response for concise preference
        let lines: Vec<&str> = response.lines().collect();
        if lines.len() > 5 {
            format!(
                "{}...\n\n(Use /help for full details)",
                lines[..3].join("\n")
            )
        } else {
            response.to_string()
        }
    }

    fn make_detailed(&self, response: &str) -> String {
        // Add more context for detailed preference
        format!("{}\n\n💡 **Additional Context:**\nThis response was personalized based on your usage patterns. Use /feedback to help improve future suggestions.", response)
    }

    fn make_code_focused(&self, response: &str) -> String {
        // Emphasize code examples
        if response.contains("```") {
            format!("🔧 **Code Focus:**\n{}\n\n💻 **Quick Actions:** Try /refactor or /generate-tests on this code", response)
        } else {
            response.to_string()
        }
    }

    pub async fn update_preferences(
        &self,
        user_id: &str,
        preferences: UserPreferences,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut profile = self.load_or_create_profile(user_id).await?;
        profile.preferences = preferences;
        self.save_profile(&profile).await?;
        Ok(())
    }

    pub async fn get_learning_insights(
        &self,
        user_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let profile = self.load_or_create_profile(user_id).await?;

        let total_commands = profile
            .usage_patterns
            .most_used_commands
            .values()
            .sum::<u32>();
        let most_used = profile
            .usage_patterns
            .most_used_commands
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(cmd, count)| format!("{} ({} times)", cmd, count))
            .unwrap_or("None".to_string());

        let success_rate = if profile.learning_history.is_empty() {
            0.0
        } else {
            let successful = profile
                .learning_history
                .iter()
                .filter(|e| e.success)
                .count();
            (successful as f32 / profile.learning_history.len() as f32) * 100.0
        };

        Ok(format!(
            "# 📊 Your Learning Insights\n\n\
            ## Usage Statistics\n\
            - **Total Commands**: {}\n\
            - **Most Used**: {}\n\
            - **Success Rate**: {:.1}%\n\
            - **Preferred Languages**: {}\n\n\
            ## Personalization\n\
            - **Complexity Level**: {}\n\
            - **Response Style**: {}\n\
            - **Learning Events**: {}\n\n\
            ## Recommendations\n\
            - Continue using your most effective commands\n\
            - Try exploring new features based on your preferences\n\
            - Consider adjusting complexity level as you progress",
            total_commands,
            most_used,
            success_rate,
            profile.preferences.preferred_languages.join(", "),
            profile.preferences.complexity_level,
            profile.preferences.response_format,
            profile.learning_history.len()
        ))
    }
}
