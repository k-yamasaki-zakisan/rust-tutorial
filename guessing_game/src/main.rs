use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // 乱数の作成
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    // whileloop機能
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            // 例外処理
            .expect("Failed to read line");
        
        // 整数型を指定、入力が違ったらもう一度入力を促す 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        // 整数型を指定
        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!");

        println!("You guessed: {}", guess);

        // 数字の大小の比較
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
