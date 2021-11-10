fn main() {
    let mut set_a = vec![5, 7, 20, 55];
    let mut set_b = vec![55, 5, 20, 72];

    set_a.sort_unstable();
    set_b.sort_unstable();

    if set_a == set_b {
        println!("Sets are equal")
    } else {
        println!("Sets are not equal");
    }
}
