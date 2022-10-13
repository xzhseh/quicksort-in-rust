use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};

fn quicksort(sort_vec: &mut Vec<i32>, begin: usize, end: usize) {
    if begin >= end {
        return;
    }
    // Get the index after partition, Now sort_vec[index] is just the correct pivot
    let index = partition(sort_vec, begin, end);
    // Sort the other two halves
    quicksort(sort_vec, begin, index - 1);
    quicksort(sort_vec, index + 1, end);
}

fn partition(sort_vec: &mut Vec<i32>, begin: usize, end: usize) -> usize {
    // Here I choose middle element of the current vector as the pivot of quicksort
    let middle = begin + (end - begin) / 2;
    middle
}

fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let mut buffer = String::new();
    let _size = File::open(filename)?.read_to_string(&mut buffer)?;
    Ok(buffer.split('\n').filter(|x| *x != "").map(|x| x.to_string()).collect())
}

fn main() -> Result<(), Box<dyn Error>> {
    let buffer = read_file_lines(&"quicksort.in".to_string())?;
    let len = buffer[0].parse::<usize>()?;
    let mut quicksort_vec = buffer[1].split(' ').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    quicksort(&mut quicksort_vec, 0, len - 1);
    println!("The final result {:?}", quicksort_vec);
    let final_result = quicksort_vec.iter().map(|x| {
        let mut tmp = x.to_string();
        tmp.push(' ');
        tmp
    }).collect::<String>();
    let final_result = final_result.trim();
    let mut file = File::create("quicksort.out").expect("Error in creating quicksort.out!");
    file.write(&final_result.as_bytes()).expect("Error in writing to quicksort.out!");
    Ok(())
}

