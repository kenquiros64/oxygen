
// print_ticket prints a ticket using a thermal printer
pub fn print_ticket(ticket: &Ticket) -> Result<(), anyhow::Error> {
    let mut printer = Printer::new("/dev/usb/lp0")?;
    printer.set_size(2, 2)?;
    printer.set_bold(true)?;
    printer.set_align(Align::Center)?;
    printer.print("TICKET\n")?;
    printer.set_bold(false)?;
    printer.set_size(1, 1)?;
    printer.set_align(Align::Left)?;
    printer.print(&format!("Departure: {}\n", ticket.departure))?;
    printer.print(&format!("Destination: {}\n", ticket.destination))?;
    printer.print(&format!("Stop: {}\n", ticket.stop))?;
    printer.print(&format!("Username: {}\n", ticket.username))?;
    printer.print(&format!("Fare: {}\n", ticket.fare))?;
    printer.print(&format!("Time: {}\n", ticket.time))?;
    printer.print(&format!("Gold: {}\n", ticket.is_gold))?;
    printer.print(&format!("Null: {}\n", ticket.is_null))?;
    printer.print(&format!("Report ID: {}\n", ticket.report_id))?;
    printer.print(&format!("Created At: {}\n", ticket.created_at))?;
    printer.print(&format!("Updated At: {}\n", ticket.updated_at))?;
    printer.cut()?;
    Ok(())
}
