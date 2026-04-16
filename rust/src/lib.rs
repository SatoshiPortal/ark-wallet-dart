pub mod ark;
#[cfg(not(feature = "bull_sdk"))]
mod frb_generated;

// Re-export bitcoin types for flutter_rust_bridge
pub use bitcoin::OutPoint;
