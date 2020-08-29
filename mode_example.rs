use std::collections::HashMap;
fn main() {

    let v = vec![2,3,4,324,1,5,2,4,3,3];
    println!("{}", mode(&v) );
}

fn mode(v:&Vec<u32>) -> u32 {

    let mut hm = HashMap::new();
    let mut mx = 0;
    let mut mx_val:u32=0;
    
    for n in v {
        let count = hm.entry(n).or_insert(0);
        *count +=1;
    }

    for ki in hm{
        if ki.1 > mx {
            mx = ki.1;
            mx_val = *ki.0;
        }
    }
    mx_val
}
