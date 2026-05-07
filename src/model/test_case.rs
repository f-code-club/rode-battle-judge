use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct TestCase {
    pub id: Uuid,
    pub position: f32,
    pub input_path: String,
    pub output_path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
