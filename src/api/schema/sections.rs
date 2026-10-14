use serde::{Deserialize, Serialize};

/// Params for `section.report`: replace the contents of one reported sidebar section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SectionReportParams {
    /// Stable section identity; a later report with the same id replaces this one.
    pub id: String,
    /// Header text rendered above the rows.
    #[serde(default)]
    pub title: String,
    /// Row lines rendered top to bottom.
    #[serde(default)]
    pub rows: Vec<String>,
}
