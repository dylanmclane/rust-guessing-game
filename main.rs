// standard input library
use std::io::stdin;

// random number dependency
use rand::Rng;

fn main() {
    '_outer_loop: loop {

        // let _number: u32 = 10;
        let _number: i32 = rand::thread_rng().gen_range(1..=15);

        println! ("Pick a number >>>");

        loop {
            
            let mut line = String::new();

            let _input = stdin().read_line(&mut line);

            let guess: Option<i32> = _input.ok().map_or(None, |_| line.trim().parse().ok());

            match guess{
                None => println!("enter a number..."),
                Some(n) if n == _number => {
                    println!("Bravo! You guessed it!");
                    break '_outer_loop;
                }
                Some(n) if n < _number => println!("Too low"),
                Some(n) if n > _number => println!("Too high"),
                Some(_) => println!("Error!")
            }
        }
    }
}
