//! Shared production library boundary for Interstellar Meridian.
//!
//! T4.1 establishes only the Cargo package and executable. Simulation,
//! drivers and Bevy runtime integration belong to later packages.

/// Identifies the production bootstrap without advancing game time.
pub const BOOTSTRAP_MESSAGE: &str = "Interstellar Meridian | T4.1 bootstrap";
