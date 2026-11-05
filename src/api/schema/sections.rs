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
    /// Optional per-token actions (e.g. one per visible port), parallel to `rows`.
    #[serde(default)]
    pub row_actions: Vec<Vec<SectionRowAction>>,
}

/// One clickable token within a section row, e.g. a port in `8080 8082 +3 node`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SectionRowAction {
    /// Text token to make clickable inside the row.
    pub token: String,
    /// Workspace to focus if no pane is set.
    #[serde(default)]
    pub workspace_id: Option<String>,
    /// Pane to focus for this token.
    #[serde(default)]
    pub pane_id: Option<String>,
}
