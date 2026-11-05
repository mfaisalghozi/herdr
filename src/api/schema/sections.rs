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
    /// Optional workspace id per row, parallel to `rows`. Clicking a row
    /// focuses that workspace.
    #[serde(default)]
    pub row_workspaces: Vec<Option<String>>,
    /// Optional pane id per row, parallel to `rows`. When set, clicking the
    /// row focuses that pane (and its tab/workspace).
    #[serde(default)]
    pub row_panes: Vec<Option<String>>,
}
