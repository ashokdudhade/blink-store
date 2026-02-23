//! Tracing setup and log retention worker.

use anyhow::Result;
use std::path::Path;
use std::time::Duration;
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

/// Holds guards so that logs are flushed on drop.
pub struct LoggingGuard {
    _file_guard: Option<WorkerGuard>,
}

/// Initializes tracing: stdout + optional rolling file in `log_dir`.
/// If `log_dir` is None, only stdout is used.
pub fn init_tracing(log_dir: Option<&Path>, env_filter: Option<&str>) -> Result<LoggingGuard> {
    let filter = env_filter
        .map(EnvFilter::new)
        .unwrap_or_else(|| EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()));

    let (file_guard, file_layer) = if let Some(dir) = log_dir {
        std::fs::create_dir_all(dir)?;
        let file_appender = tracing_appender::rolling::daily(dir, "blink-store.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        let layer = fmt::layer()
            .with_writer(non_blocking)
            .with_ansi(false)
            .with_span_events(FmtSpan::CLOSE);
        (Some(guard), Some(layer))
    } else {
        let (non_blocking, guard) = tracing_appender::non_blocking(std::io::sink());
        let layer = fmt::layer()
            .with_writer(non_blocking)
            .with_ansi(false)
            .with_span_events(FmtSpan::CLOSE);
        (Some(guard), Some(layer))
    };

    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_span_events(FmtSpan::CLOSE);

    tracing_subscriber::registry()
        .with(filter)
        .with(stdout_layer)
        .with(file_layer)
        .try_init()?;

    Ok(LoggingGuard {
        _file_guard: file_guard,
    })
}

/// Spawns a background task that prunes log files in `log_dir` older than `retention`.
/// Log files are assumed to have the pattern `blink-store.log.YYYY-MM-DD` (daily rolling).
pub fn spawn_log_retention_worker(
    log_dir: std::path::PathBuf,
    retention: Duration,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            if let Err(e) = prune_old_logs(&log_dir, retention).await {
                info!(error = %e, action = "log_retention_prune_failed");
            }
        }
    })
}

async fn prune_old_logs(log_dir: &Path, retention: Duration) -> Result<()> {
    let cutoff = std::time::SystemTime::now() - retention;
    let mut entries = tokio::fs::read_dir(log_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() {
            if let Ok(meta) = entry.metadata().await {
                if let Ok(modified) = meta.modified() {
                    if modified < cutoff {
                        let _ = tokio::fs::remove_file(&path).await;
                        info!(path = ?path, action = "pruned_old_log");
                    }
                }
            }
        }
    }
    Ok(())
}
