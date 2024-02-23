use csv;
use prettytable::{row, Cell, Row, Table};
use std::error::Error;

fn read_csv_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(file_path)?;

    // Create a new table
    let mut table = Table::new();

    // Add header row
    table.add_row(row!["Name", "Age", "Location"]);

    // Iterate over each record
    for result in reader.records() {
        // Extract the record
        let record = result?;

        // Create an empty row
        let mut row = Row::empty();

        // Add cell to the row
        for field in record.iter() {
            let cell = Cell::new(field);
            row.add_cell(cell);
        }

        // Add row to the table
        table.add_row(row);
    }

    // Print the table
    table.printstd();

    Ok(())
}

fn main() {
    // If errors come up then print them
    if let Err(e) = read_csv_file("./example.csv") {
        eprintln!("Error reading CSV file: {}", e);
    }
}
