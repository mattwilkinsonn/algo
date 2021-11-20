use rand::Rng;

fn main() {
    let mut hash_table = HashTable::new();

    let mut rng = rand::thread_rng();

    let generated_values = (0..32)
        .map(|_| rng.gen_range(10000..99999))
        .collect::<Vec<i32>>();

    for value in &generated_values {
        // println!("\ninserting {}", &value);
        hash_table.insert(*value);
    }
    let average_probing = hash_table.average_insert_probing();

    // 2 keys in the hash table
    let mut keys = (0..2)
        .map(|_| generated_values[rng.gen_range(0..generated_values.len())])
        .collect::<Vec<i32>>();

    // 2 random keys
    (0..2).for_each(|_| {
        let random_key = rng.gen_range(10000..99999);

        keys.push(random_key);
    });

    for key in keys {
        println!("\nSearching for {}", &key);
        hash_table.search_and_print(key);
    }

    println!("\n{:?}", hash_table);
    println!("\nAverage linear probing per insert: {}", average_probing);
    println!(
        "Average linear probing per search: {}",
        hash_table.average_search_probing()
    );
}
#[derive(Debug)]
struct HashTable {
    arr: [i32; 50],
    insert_probe_count: i32,
    insert_count: i32,
    search_probe_count: i32,
    search_count: i32,
}

impl HashTable {
    fn new() -> HashTable {
        HashTable {
            arr: [0; 50],
            insert_probe_count: 0,
            insert_count: 0,
            search_probe_count: 0,
            search_count: 0,
        }
    }

    fn insert(&mut self, value: i32) {
        let mut index = (value % 50) as usize;
        let starting_index = index;

        if self.arr[index] != 0 {
            while self.arr[index] != 0 {
                // println!("collision at {}", index);
                index += 1;
                self.insert_probe_count += 1;

                if index >= 50 {
                    index = 0;
                }

                if index == starting_index {
                    panic!("Hash table is full");
                }
            }
        }
        self.insert_count += 1;
        // println!("inserting {} at {}", value, index);
        self.arr[index] = value;
    }

    fn search(&mut self, value: i32) -> bool {
        let mut index = (value % 50) as usize;
        let starting_index = index;
        self.search_count += 1;

        while self.arr[index] != value && self.arr[index] != 0 {
            index += 1;

            if index == starting_index {
                return false;
            }

            self.search_probe_count += 1;

            if index >= 50 {
                index = 0;
            }
        }

        true
    }

    fn search_and_print(&mut self, value: i32) {
        let exists = self.search(value);

        match exists {
            true => println!("Key {} found", value),
            false => println!("Key {} does not exist in the hash table", value),
        }
    }

    fn average_insert_probing(&self) -> f32 {
        self.insert_probe_count as f32 / self.insert_count as f32
    }

    fn average_search_probing(&self) -> f32 {
        self.search_probe_count as f32 / self.search_count as f32
    }
}
