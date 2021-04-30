use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // ユーザー標準入力
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        // 行の読み込みに失敗しました
        .expect("Failed to read line");
    
    // 次のように予想しました: {user_input_word}
    println!("You guessed: {}", guess);
}
