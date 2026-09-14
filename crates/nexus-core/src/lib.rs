//! NEXUS-DE Core Library
//!
//! Provides core domain models, configuration schema, tier state representations,
//! and design system algorithms (Material You dynamic theming).

pub mod config;
pub mod theme;
pub mod tier;

pub use tier::NexusTier;
