use serde::{Deserialize, Serialize};

use super::Button;

#[derive(Debug, Serialize, Deserialize)]
pub struct Keyboard {
    pub buttons: Vec<Vec<Button>>,
}
