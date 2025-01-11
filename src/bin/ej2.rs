use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() -> io::Result<()> {
    let path = "datac.txt";
    let amount_of_reports = read_matrix_file(path)?;
    println!(
        "La suma de las diferencias absolutas es: {}",
        amount_of_reports
    );
    Ok(())
}

fn read_matrix_file<P>(filename: P) -> io::Result<i32>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut ammount_safe_reports = 0;
    for line in reader.lines() {
        let line = line?;
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if is_row_report_safe(row) {
            ammount_safe_reports += 1
        }
    }
    Ok(ammount_safe_reports)
}

fn is_row_report_safe(row: Vec<i32>) -> bool {
    if row.len() < 2 {
        return true;
    }

    let mut is_desc: Option<bool> = None;

    for i in 1..row.len() {
        let diff = row[i] - row[i - 1];

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if is_desc.is_none() {
            is_desc = Some(diff < 0);
        }

        if (diff < 0 && is_desc == Some(false)) || (diff > 0 && is_desc == Some(true)) {
            return false;
        }
    }

    true
}
