use rand::Rng;
use std::io;

// struct DataF {
//     elements: Vec<f32>,
// }

// impl DataF {
//     fn data_f_create(self, length: u32) -> Self {
//         assert!(length < u32::MAX);

//         Self {
//             elements: vec![[0; length]],
//         }
//     }

//     fn data_f_sum(self) -> f32 {
//         self.elements.iter().sum()
//     }
// }

fn main() {
    let mut data: Vec<i32> = Vec::new();
    let mut user_input = String::new();

    let mut mean: f32 = 0f32;
    let _stdev: f32;

    println!("Choose number of random elements to add to data: ");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Invalid input");
    let user_input: u32 = user_input.trim().parse().expect("Not a number");

    if user_input == 0 || user_input > 20 {
        println!("Invalid input! Choose num between 0 and 20");
        return;
    }

    for _n in 0..=user_input {
        let i = rand::thread_rng().gen_range(0..=100);
        data.push(i);
    }

    println!("Data is {:?}", data);

    for element in data.iter() {
        mean += *element as f32;
    }

    if mean != 0f32 {
        mean /= user_input as f32;
    }

    println!("Mean = {mean}");
}
