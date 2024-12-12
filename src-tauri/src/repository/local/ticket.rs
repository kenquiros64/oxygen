use crate::models::ticket::Ticket;
use anyhow::Context;

pub struct TicketRepository {
    db: rusqlite::Connection,
}

impl TicketRepository {
    // new creates a new instance of the TicketRepository
    pub fn new(db: rusqlite::Connection) -> Self {
        TicketRepository { db }
    }


    // add_multiple_tickets adds multiple tickets in the database
    pub fn add_multiple_tickets(&self, tickets: Vec<Ticket>) -> Result<(), anyhow::Error> {
        let mut stmt = self.db.prepare(
            "INSERT INTO tickets (departure, destination, stop, username, fare, time, is_gold, \
                 is_null, report_id, created_at, updated_at) \
                 VALUES (?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        )?;
        for ticket in tickets {
            stmt.execute(&[
                &ticket.departure,
                &ticket.destination,
                &ticket.stop,
                &ticket.username,
                &ticket.fare.to_string(),
                &ticket.time,
                &ticket.is_gold.to_string(),
                &ticket.is_null.to_string(),
                &ticket.report_id.to_string(),
                &ticket.created_at,
                &ticket.updated_at,
            ])
            .context("Failed to insert ticket")?;
        }

        Ok(())
    }

    // find_ticket finds a ticket in the database
    pub fn find_ticket(&self, id: i64) -> Result<Option<Ticket>, anyhow::Error> {
        let mut stmt = self.db.prepare("SELECT * FROM tickets WHERE id = ?1")?;
        let mut rows = stmt.query(&[&id])?;

        let ticket = if let Some(row) = rows.next()? {
            Some(Ticket {
                id: row.get(0)?,
                departure: row.get(1)?,
                destination: row.get(2)?,
                stop: row.get(3)?,
                username: row.get(4)?,
                fare: row.get(5)?,
                time: row.get(6)?,
                is_gold: row.get(7)?,
                is_null: row.get(8)?,
                report_id: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        } else {
            None
        };

        Ok(ticket)
    }

    // update_ticket_as_null updates a ticket in the database as null
    pub fn update_ticket_as_null(&self, id: i64) -> Result<(), anyhow::Error> {
        let mut stmt = self.db.prepare("UPDATE tickets SET is_null = 'true' WHERE id = ?1")?;
        stmt.execute(&[&id]).context("Failed to update ticket as null")?;

        Ok(())
    }

    // fetch_tickets_by_username_created_at_report_id fetches all tickets from the database
    pub fn fetch_tickets_by_username_created_at_report_id(
        &self,
        username: &str,
        created_at: &str,
        report_id: &str,
    ) -> Result<Vec<Ticket>, anyhow::Error> {
        let mut stmt = self.db.prepare(
            "SELECT * FROM tickets WHERE username = ?1 AND created_at = ?2 AND report_id = ?3",
        )?;
        let mut rows = stmt.query(&[&username, &created_at, &report_id])?;

        let mut tickets = Vec::new();
        while let Some(row) = rows.next()? {
            tickets.push(Ticket {
                id: row.get(0)?,
                departure: row.get(1)?,
                destination: row.get(2)?,
                stop: row.get(3)?,
                username: row.get(4)?,
                fare: row.get(5)?,
                time: row.get(6)?,
                is_gold: row.get(7)?,
                is_null: row.get(8)?,
                report_id: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            });
        }

        Ok(tickets)
    }
}
