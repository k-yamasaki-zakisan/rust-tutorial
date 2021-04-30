fn main() {
    // mutで再代入を許可する(mutを宣言しないと再代入不可)
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    // 定数
    const MAX_POINT: u32 = 100_000;

    // シャドーイング(letを複数宣言して上書き)
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let guess: u32 = "42".parse()
        .expect("Not a number!");
    
    let x = 2.0;

    let y: f32 = 3.0;

    // 足し算
    let sum = 5 + 10;
    // 引き算
    let difference = 95.5 - 4.3;
    // 掛け算
    let product = 4 * 30;
    // 割り算
    let quotient = 56.7 / 32.2;
    // 余り
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; // 明示的型注釈付きで

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];
    let index = 0;
    let element = a[index];
    println!("The value of element is: {}", element);

    another_function();
}

fn another_function() {
    println!("test function");
}
