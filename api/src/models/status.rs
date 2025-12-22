use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Status {
    pub auth: bool,
}
