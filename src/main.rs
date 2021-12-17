
fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    use std::fs::File;
    let mut file = File::open(filename).expect("Input file does not exist or OS bwoke");
    let mut string_buf = String::new();
    use std::io::Read;
    let _ = file.read_to_string(&mut string_buf);

    //let lower_case_string_buf = string_buf.to_lowercase();
    let split_strings = string_buf.split_whitespace();
    use std::collections::HashMap;
    let mut map: HashMap<&str,HashMap<&str,usize>> = HashMap::new();
    let mut iter = split_strings.peekable();
    while let Some(s) = iter.next(){
        let next_value = iter.peek();
        match next_value {
            Some(s2) =>{ map.entry(s)
                .and_modify(|m| {let _ = m.entry(s2).and_modify(|v| {*v+=1;}).or_insert(1);})
                .or_insert({
                    let mut new_map = HashMap::new();
                    new_map.insert(*s2,1);
                    new_map
                    });
                ()
                },
            None => (),
        }
    };

    use rand::distributions::WeightedIndex;
    use rand::prelude::*;
    let mut new_map = HashMap::new();

    for (k,v) in map{
        let mut array = vec![];
        for (k2,v2) in v{
            array.push((k2,v2));
        }
        //println!("{} {:?}", k, array);
        let dist = WeightedIndex::new(array.iter().map(|item| item.1)).unwrap();
        new_map.insert(k, (array,dist));
    }
    let mut rng = rand::thread_rng();
    let mut next_word = *new_map.keys().next().unwrap();
    for _ in 0..400 {
        print!("{} ", next_word);
        let transitions = &new_map[next_word];
        next_word = transitions.0[transitions.1.sample(&mut rng)].0;
    }
    //for (k,v) in new_map.iter(){
     //   println!("key: {:?}\n items: {:?}", k, v.0);
    //}

    
}

