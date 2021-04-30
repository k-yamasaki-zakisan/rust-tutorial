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
}
