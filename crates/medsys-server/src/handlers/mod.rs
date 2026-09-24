//! Módulo agrupador de manejadores HTTP.

pub mod condition;
pub mod encounter;
pub mod health;
pub mod legacy;
pub mod observation;
pub mod patient;

pub use condition::{get_condition, list_conditions};
pub use encounter::{get_encounter, list_encounters};
pub use health::health_check;
pub use legacy::{get_legacy_patient_full, list_legacy_patients};
pub use observation::{get_observation, list_observations};
pub use patient::{get_patient, list_patients};
