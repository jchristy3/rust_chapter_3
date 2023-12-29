use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    loop {

        println!("Please enter an array index.");

        let mut index = String::new();

        io::stdin().read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        if index >= a.len() {
            println!("Index {} is out of bounds", index);
            continue;
        } else {
            println!("The value of the element at index {} is: {}", index, a[index]);
            break;
        }

    }
}