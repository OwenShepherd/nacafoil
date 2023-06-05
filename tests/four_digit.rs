use serde_json::{Serialize, Deserialize};
use std::path::PathBuf;
use nacafoil;

impl Deserialize for nacafoil::Airfoil {
    fn deserialize<S>(&self, deserializer: S) -> Result<S::Ok, S::Error>
    where {
        S
    }
}

