pub mod matrix;
pub mod metrics;
pub mod vector;

pub use matrix::{multiply, Matrix};
pub use metrics::cmap::CmapMetrics;
pub use vector::{dot_product, Vector};
