//! plceye - PLC Code Rule Detector
//!
//! Open source static analyzer for PLC files that detects code rules
//! and potential issues in PLC programs.
//!
//! ## Supported Formats
//!
//! - **L5X** - Rockwell Automation Studio 5000
//! - **PLCopen XML** - IEC 61131-3 standard exchange format
//!
//! ## Supported Rules (Open Source)
//!
//! - **S0001: unused_tags** - Tags defined but never used
//! - **S0002: undefined_tags** - Tags referenced but not defined
//! - **S0003: empty_routines** - Routines with no logic
//! - **S0004: unused_aois** - AOIs defined but never called
//! - **S0005: unused_datatypes** - User-defined types never used
//! - **M0001: cyclomatic_complexity** - ST routines with high complexity
//! - **M0003: deep_nesting** - Control structures nested too deeply
//!
//!
//! ## CLI Usage
//!
//! ```bash
//! # Analyze a file
//! plceye project.L5X
//!
//! # With custom config
//! plceye --config plceye.toml project.L5X
//! ```

pub mod analysis;
mod config;
mod detector;
mod error;
mod loader;
mod report;
mod rules;

// Core types
pub use config::{
    ComplexityConfig, EmptyRoutinesConfig, GeneralConfig, NestingConfig, RuleConfig,
    UndefinedTagsConfig, UnusedAoisConfig, UnusedDataTypesConfig, UnusedTagsConfig,
};
pub use detector::RuleDetector;
pub use error::{ConfigErrorKind, Error, L5xParseErrorKind, Result};
pub use loader::{FileFormat, LoadedProject};
pub use report::{Report, Rule, RuleKind, Severity};

// Analysis types (for extensions)
pub use analysis::{analyze_controller, ParseStats, ProjectAnalysis};
pub use analysis::{analyze_plcopen_project, PlcopenAnalysis, PlcopenStats};

// Re-export parser crates for extensions
pub use iec61131;
pub use l5x;
pub use plcopen;
