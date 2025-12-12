pub mod telnyx;
pub mod claude;
pub mod session;
pub mod s3;
pub mod elevenlabs;
pub mod app_state;
pub mod deepgram_ws;

pub use app_state::AppState;
pub use session::SessionManager;
pub use telnyx::TelnyxService;
pub use deepgram_ws::DeepgramWebSocket;
pub use claude::ClaudeService;
pub use s3::S3Service;
pub use elevenlabs::ElevenLabsService;

use dashmap::DashMap;
use std::sync::Arc;

pub type Sessions = Arc<DashMap<String, crate::models::SessionInfo>>;
