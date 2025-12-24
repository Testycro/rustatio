use serde::Serialize;
use std::sync::OnceLock;
use tauri::Emitter;

// Log event payload
#[derive(Clone, Serialize)]
struct LogEvent {
    timestamp: u64,
    level: String,
    message: String,
}

// Global app handle storage
static APP_HANDLE: OnceLock<AppHandleWrapper> = OnceLock::new();

// Wrapper to make AppHandle Send + Sync
struct AppHandleWrapper {
    handle: tauri::AppHandle,
}

unsafe impl Send for AppHandleWrapper {}
unsafe impl Sync for AppHandleWrapper {}

/// Initialize the logger with the app handle
pub fn init_logger(handle: tauri::AppHandle) {
    let _ = APP_HANDLE.set(AppHandleWrapper { handle });
}

/// Emit a log to the UI (if app handle is available)
fn emit_log(level: &str, message: String) {
    if let Some(wrapper) = APP_HANDLE.get() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_secs(0))
            .as_millis() as u64;

        let log_event = LogEvent {
            timestamp,
            level: level.to_string(),
            message: message.clone(),
        };

        let _ = wrapper.handle.emit("log-event", log_event);
    }
}

/// Log at info level (both to console and UI)
pub fn info(message: String) {
    log::info!("{}", message);
    emit_log("info", message);
}

/// Log at warn level (both to console and UI)
pub fn warn(message: String) {
    log::warn!("{}", message);
    emit_log("warn", message);
}

/// Log at error level (both to console and UI)
pub fn error(message: String) {
    log::error!("{}", message);
    emit_log("error", message);
}

/// Log at debug level (both to console and UI)
pub fn debug(message: String) {
    log::debug!("{}", message);
    emit_log("debug", message);
}

// Macros for easier usage
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::core::logger::info(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::core::logger::warn(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::core::logger::error(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::core::logger::debug(format!($($arg)*))
    };
}
