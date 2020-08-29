use std::collections::HashMap;

fn main() {

    // INITILIZATION
    // __________________________________________________

    // Empty HashMap
    let mut hm:HashMap<_,_> = HashMap::new();

    // Collect method 

    let hm_from_vec:HashMap<String, u32> = vec![(String::from("Third"), 3), (String::from("Forth"), 4)].into_iter().collect() ; 

    println!("HashMap from vector is {:?}", hm_from_vec);

    // INSERT
    // _________________________________________________

    // Insert method 
    hm.insert("fisrt", 1);
    hm.insert("second", 2);
    println!("Mutable hash table is {:?}",hm);


    println!("Keys in hash map {:?}",hm.keys());
    println!("Values in hash map {:?}",hm.values());
    println!("Length of hash map {:?}",hm.len());
    println!("Is this hash map empty {:?}",hm.is_empty());
    
}
