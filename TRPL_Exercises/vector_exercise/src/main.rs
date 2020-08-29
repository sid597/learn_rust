/*
Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.

*/
use std::collections::HashMap;

fn main() {
    let mut v = vec![32,2, 4, 65,24, 54, 23,77, 74,5 ,8, 3, 47,85];
    v.sort();
    println!("Vector v is {:?}", v);
    println!("Vector mean is {}",mean(&v));
    
    println!("Vector mean is {}",median(&v));
}

fn mean(v:&Vec<u64>) -> u64{
    let mut sum = 0;
    let mut len = 0;
    for num in v {
        sum += num;
        len +=1 ;
    }
    sum / len
    
}

fn median(v:&Vec<u64>) -> u64 {
    v[v.len()-1] 
}

