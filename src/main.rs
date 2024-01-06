use rand::{seq::SliceRandom};

fn main() {
    let password_length = 8;

    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%&"
                .chars()
                .collect::<Vec<char>>();

    let mut rng = rand::thread_rng();

    let password: String = (0..password_length)
        .map(|_| *chars.choose(&mut rng).unwrap())
        .collect();

    println!("Your new password is: {}", password);
}