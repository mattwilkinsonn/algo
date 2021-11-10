fn main() {
    let first_number = 204.0;
    let factor = -0.665;
    let _total_numbers = 19.0;
    let sum: String;

    if factor >= 1.0 {
        sum = "Infinity".to_string();
    } else if factor <= -1.0 {
        sum = "Negative Infinity".to_string();
    } else {
        let actual_sum = first_number / (1.0 - factor);

        sum = actual_sum.to_string();
    }

    println!("Sum: {}", sum)
}

// fn geometric_seqence() {
//     let first_number = 25;
//     let factor = -5;
//     let total_numbers = 11;

//     let mut sum = 0;

//     let mut members = vec![];
//     let mut sums = vec![];

//     for n in 0..total_numbers {
//         let member = first_number * i32::pow(factor, n);

//         sum += member;

//         members.push(member);

//         sums.push(sum);
//     }

//     println!("Members: {:?}", members);
//     println!("Sums: {:?}", sums);
// }

// fn arithmetic_sequence() {
//     let first_number = 200;
//     let common_difference = -124;
//     let total_numbers = 10;

//     let mut sum = 0;

//     let mut members = vec![];
//     let mut sums = vec![];

//     for n in 0..total_numbers {
//         let member = first_number + (n * common_difference);

//         sum += member;

//         members.push(member);

//         sums.push(sum);
//     }

//     println!("Members: {:?}", members);
//     println!("Sums: {:?}", sums);
// }
