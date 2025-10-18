use chrono::Utc;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

pub async fn log_message(
    logging_config: &crate::LoggingConfig,
    level: &str,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !logging_config.enabled {
        return Ok(());
    }

    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let log_entry = format!("[{}] [{}] {}\n", timestamp, level.to_uppercase(), message);

    match logging_config.provider.as_str() {
        "local" => log_to_local(&logging_config, &log_entry).await,
        "aws" => log_to_s3(&logging_config, &log_entry).await,
        "azure" => log_to_azure_blob(&logging_config, &log_entry).await,
        "gcp" => log_to_gcp_storage(&logging_config, &log_entry).await,
        "oracle" => log_to_oracle_storage(&logging_config, &log_entry).await,
        _ => log_to_local(&logging_config, &log_entry).await,
    }
}

async fn log_to_local(
    config: &crate::LoggingConfig,
    log_entry: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create logs directory if it doesn't exist
    fs::create_dir_all(&config.local_path)?;

    let log_file_path = Path::new(&config.local_path).join(&config.local_filename);

    // Append to log file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)?;

    file.write_all(log_entry.as_bytes())?;
    file.flush()?;

    // TODO: Implement log rotation based on max_file_size_mb and max_files

    Ok(())
}

async fn log_to_s3(
    config: &crate::LoggingConfig,
    log_entry: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if config.aws_s3_bucket.is_empty() {
        return Err("AWS S3 bucket not configured".into());
    }

    // TODO: Implement S3 logging using AWS SDK
    // For now, fallback to local logging
    println!("S3 logging not yet implemented, falling back to local");
    log_to_local(config, log_entry).await
}

async fn log_to_azure_blob(
    config: &crate::LoggingConfig,
    log_entry: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if config.azure_storage_account.is_empty() {
        return Err("Azure storage account not configured".into());
    }

    let _storage_key = if config.azure_storage_key.is_empty() {
        env::var("AZURE_STORAGE_KEY").unwrap_or_default()
    } else {
        config.azure_storage_key.clone()
    };

    // TODO: Implement Azure Blob logging
    // For now, fallback to local logging
    println!("Azure Blob logging not yet implemented, falling back to local");
    log_to_local(config, log_entry).await
}

async fn log_to_gcp_storage(
    config: &crate::LoggingConfig,
    log_entry: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if config.gcp_storage_bucket.is_empty() {
        return Err("GCP storage bucket not configured".into());
    }

    // TODO: Implement GCP Cloud Storage logging
    // For now, fallback to local logging
    println!("GCP Cloud Storage logging not yet implemented, falling back to local");
    log_to_local(config, log_entry).await
}

async fn log_to_oracle_storage(
    config: &crate::LoggingConfig,
    log_entry: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if config.oracle_bucket.is_empty() || config.oracle_namespace.is_empty() {
        return Err("Oracle Object Storage bucket/namespace not configured".into());
    }

    // TODO: Implement Oracle Object Storage logging
    // For now, fallback to local logging
    println!("Oracle Object Storage logging not yet implemented, falling back to local");
    log_to_local(config, log_entry).await
}

pub fn should_log(logging_config: &crate::LoggingConfig, level: &str) -> bool {
    if !logging_config.enabled {
        return false;
    }

    let level_priority = match level.to_lowercase().as_str() {
        "debug" => 0,
        "info" => 1,
        "warn" => 2,
        "error" => 3,
        _ => 1,
    };

    let config_priority = match logging_config.level.to_lowercase().as_str() {
        "debug" => 0,
        "info" => 1,
        "warn" => 2,
        "error" => 3,
        _ => 1,
    };

    level_priority >= config_priority
}
