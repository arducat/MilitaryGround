use std::io; 
 
fn prefix(arr: &[i32]) -> Vec<i32> { 
    let mut pref_sums = vec![0]; 
    for &i in arr { 
         let last_sum = *pref_sums.last().unwrap(); 
         pref_sums.push(last_sum + i); 
    } 
    pref_sums 
} 
 
fn main() { 
    let massiv = vec![2, 4, 7, 1]; 
    let pref = prefix(&massiv); 
    let [otkuda, kuda] = io::stdin()
        .lines()
        .take(2)
        .map(|v| Ok::<usize, io::Error>(v?.trim().parse::<usize>().unwrap()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap()[..] else { panic!("shit") }; 
 
    println!("{:?}", pref); 
 
    let m = pref[kuda] - pref[otkuda-1]; 
    println!("{}", m); 
}

