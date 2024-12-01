use crate::models::report::Report;
use anyhow::Context;

pub struct ReportRepository {
    db: rusqlite::Connection,
}

impl ReportRepository {
    // new creates a new instance of the ReportRepository
    pub fn new(db: rusqlite::Connection) -> Self {
        ReportRepository { db }
    }

    // add_report adds a new report in the database
    pub fn add_report(&self, report: Report) -> Result<(), anyhow::Error> {
        let mut stmt = self.db.prepare(
            "INSERT INTO reports (username, created_at) \
                 VALUES (?1, ?2)",
        )?;
        stmt.execute(&[&report.username, &report.created_at])
            .context("Failed to insert report")?;

        Ok(())
    }

    // find_report_by_username finds a report in the database by username
    pub fn find_report_by_username(&self, username: &str) -> Result<Option<Report>, anyhow::Error> {
        let mut stmt = self
            .db
            .prepare("SELECT * FROM reports WHERE username = ?1")?;
        let mut rows = stmt.query(&[&username])?;

        let report = if let Some(row) = rows.next()? {
            Some(Report {
                id: row.get(0)?,
                username: row.get(1)?,
                created_at: row.get(2)?,
                partial_closed_at: row.get(3)?,
                closed_at: row.get(4)?,
            })
        } else {
            None
        };

        Ok(report)
    }
}
