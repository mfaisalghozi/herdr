use crate::api::schema::sections::SectionReportParams;
use crate::api::schema::ResponseResult;
use crate::app::App;

use super::responses::{encode_error, encode_success};

/// Upper bound on reported rows per section, keeping a hostile reporter from growing the
/// client snapshot without limit.
pub(super) const MAX_SECTION_ROWS: usize = 64;

impl App {
    /// `section.report`: store one reported section, replacing any prior report with the same id.
    pub(super) fn handle_section_report(&mut self, id: String, params: SectionReportParams) -> String {
        if params.id.is_empty() {
            return encode_error(id, "invalid_section_id", "section id must not be empty");
        }
        if params.rows.len() > MAX_SECTION_ROWS {
            return encode_error(
                id,
                "section_rows_limit",
                format!("a section may contain at most {MAX_SECTION_ROWS} rows"),
            );
        }
        match self
            .state
            .reported_sections
            .iter_mut()
            .find(|section| section.id == params.id)
        {
            Some(existing) => *existing = params,
            None => self.state.reported_sections.push(params),
        }
        encode_success(id, ResponseResult::Ok {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::schema::{Method, Request, SuccessResponse};

    fn test_app() -> App {
        let (_api_tx, api_rx) = tokio::sync::mpsc::unbounded_channel();
        App::new(
            &crate::config::Config::default(),
            crate::app::AppPolicy::TEST,
            None,
            api_rx,
            crate::api::EventHub::default(),
        )
    }

    fn report(id: &str, rows: &[&str]) -> Request {
        Request {
            id: "req".into(),
            method: Method::SectionReport(SectionReportParams {
                id: id.into(),
                title: id.into(),
                rows: rows.iter().map(|row| (*row).to_string()).collect(),
            }),
        }
    }

    #[test]
    fn report_stores_then_replaces_by_id() {
        let mut app = test_app();
        let ok: SuccessResponse =
            serde_json::from_str(&app.handle_api_request(report("servers", &["5432 postgres"])))
                .unwrap();
        assert_eq!(ok.result, ResponseResult::Ok {});
        assert_eq!(app.state.reported_sections.len(), 1);
        assert_eq!(app.state.reported_sections[0].rows, ["5432 postgres"]);

        app.handle_api_request(report("servers", &["5173 vite"]));
        assert_eq!(app.state.reported_sections.len(), 1);
        assert_eq!(app.state.reported_sections[0].rows, ["5173 vite"]);

        app.handle_api_request(report("services", &["8080 api"]));
        assert_eq!(app.state.reported_sections.len(), 2);
    }

    #[test]
    fn report_rejects_empty_id_and_oversized_rows() {
        let mut app = test_app();
        let empty = app.handle_api_request(Request {
            id: "e".into(),
            method: Method::SectionReport(SectionReportParams {
                id: String::new(),
                title: String::new(),
                rows: Vec::new(),
            }),
        });
        assert!(empty.contains("invalid_section_id"), "{empty}");

        let many = app.handle_api_request(Request {
            id: "m".into(),
            method: Method::SectionReport(SectionReportParams {
                id: "servers".into(),
                title: "servers".into(),
                rows: vec!["row".to_string(); MAX_SECTION_ROWS + 1],
            }),
        });
        assert!(many.contains("section_rows_limit"), "{many}");
        assert!(app.state.reported_sections.is_empty());
    }
}
