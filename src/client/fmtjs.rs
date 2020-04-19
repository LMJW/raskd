use prettytable::{Cell, Row, Table};
use raskd::models::Outgoing;

pub fn fmt_one(row: Outgoing) {
    match row {
        Outgoing::Success {
            id,
            name,
            tasktype,
            duration,
            start_at,
            stop_at,
        } => {
            let mut table = Table::new();
            let ids = id.to_string();
            let title_row = Row::new(vec![
                Cell::new("id"),
                Cell::new("name"),
                Cell::new("task_type"),
                Cell::new("duration"),
                Cell::new("start_at"),
                Cell::new("stop_at"),
            ]);
            let data_row = Row::new(vec![
                Cell::from(&ids),
                Cell::from(&name),
                Cell::from(&tasktype),
                Cell::from(&duration),
                Cell::from(&start_at),
                Cell::from(&stop_at),
            ]);
            table.add_row(title_row);
            table.add_row(data_row);
            table.printstd();
        }
        Outgoing::Error { msg } => {
            let mut table = Table::new();
            let row0 = Row::new(vec![Cell::new("error message")]);
            let row1 = Row::new(vec![Cell::from(&msg)]);
            table.add_row(row0);
            table.add_row(row1);
            table.printstd();
        }
    }
}

pub fn fmt_many(rows: Vec<Outgoing>) {
    let mut table = Table::new();
    let title_row = Row::new(vec![
        Cell::new("id"),
        Cell::new("name"),
        Cell::new("task_type"),
        Cell::new("duration"),
        Cell::new("start_at"),
        Cell::new("stop_at"),
    ]);
    table.add_row(title_row);

    for row in rows {
        match row {
            Outgoing::Success {
                id,
                name,
                tasktype,
                duration,
                start_at,
                stop_at,
            } => {
                let ids = id.to_string();
                let data_row = Row::new(vec![
                    Cell::from(&ids),
                    Cell::from(&name),
                    Cell::from(&tasktype),
                    Cell::from(&duration),
                    Cell::from(&start_at),
                    Cell::from(&stop_at),
                ]);
                table.add_row(data_row);
            }
            Outgoing::Error { msg } => {
                let row1 = Row::new(vec![Cell::from(&msg)]);
                table.add_row(row1);
            }
        }
    }

    table.printstd();
}
