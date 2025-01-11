use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() -> io::Result<()> {
    let path = "datac.txt";
    let (list1, list2) = read_two_column_file(path)?;
    let result = ej1(list1, list2);
    println!("La suma de las diferencias absolutas es: {}", result);
    Ok(())
}
fn read_two_column_file<P>(filename: P) -> io::Result<(Vec<i32>, Vec<i32>)>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut columns = line.split_whitespace();
        if let (Some(col1), Some(col2)) = (columns.next(), columns.next()) {
            list1.push(col1.parse().unwrap());
            list2.push(col2.parse().unwrap());
        }
    }

    Ok((list1, list2))
}
fn ej1(mut list1: Vec<i32>, mut list2: Vec<i32>) -> i32 {
    list1.sort();
    list2.sort();
    let mut i = 0;
    let mut j = 0;
    let mut result = Vec::new();
    while i < list1.len() && j < list2.len() {
        result.push((list1[i] - list2[j]).abs());
        i += 1;
        j += 1;
    }
    let sum: i32 = result.iter().sum();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        let result = ej1(list1, list2);
        assert_eq!(result, 11)
    }
}
