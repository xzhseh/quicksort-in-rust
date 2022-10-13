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
    if index != 0 { // Prevent the possible 'substract to overflow' problem
        quicksort(sort_vec, begin, index - 1);
    }
    quicksort(sort_vec, index + 1, end);
}

fn partition(sort_vec: &mut Vec<i32>, begin: usize, end: usize) -> usize {
    // Here I choose the last element of the current vector as the pivot of quicksort
    let last = sort_vec[end];
    let mut ptr_for_smaller: i32 = match begin {
        0 => -10,
        _ => (begin - 1) as i32,
    };
    for i in begin..end {
        if sort_vec[i] < last {
            match ptr_for_smaller {
                -10 => ptr_for_smaller = 0,
                _ => ptr_for_smaller += 1,
            }
            // Swap the values in sort_vec[i] and sort_vec[ptr_for_smaller]
            let tmp_val = sort_vec[ptr_for_smaller as usize];
            sort_vec[ptr_for_smaller as usize] = sort_vec[i];
            sort_vec[i] = tmp_val;
        } else {
           continue; 
        }
    }
    
    if ptr_for_smaller == -10 {
        ptr_for_smaller = -1;
    }

    // Swap to make sure the pivot is in the right place
    ptr_for_smaller += 1;
    let tmp_val = sort_vec[ptr_for_smaller as usize];
    sort_vec[ptr_for_smaller as usize] = sort_vec[end];
    sort_vec[end] = tmp_val;

    ptr_for_smaller as usize
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

