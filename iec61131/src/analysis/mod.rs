//! Semantic analysis for IEC 61131-3
//!
//! This module provides:
//! - Control flow graph (CFG) construction
//! - Cyclomatic complexity calculation
//! - Nesting depth analysis

mod cfg;
mod nesting;

pub use cfg::{
    count_expression_decisions, Cfg, CfgBuilder, CfgEdge, CfgNode, EdgeKind, NodeId, NodeKind,
};
pub use nesting::max_nesting_depth;
