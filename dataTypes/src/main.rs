fn main() {
    // println!("Hello, world!");

    let a: u32 = "69".parse().expect("This is not N INTEGER");
    println!("a: {}",a);

    let zx: i64 = 987_000_41;
    println!("{}",zx);
    
    let b: char = 'g';
    println!("{}",b);

    let f: f32 = 0.64;
    println!("{}",f);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
    println!("{},{},{},{},{}",sum,difference,product,quotient,remainder);

    let emoji = '📞';
    println!("{}",emoji);

    let tup: (u32, f64, char) = (23, 45.123, 'c');
    let tup2 = (512, 1.0, 'w');
    let (p,q,r) = tup2;
    println!("{},{},{}",p,q,r);
    println!("{}", tup.0);

    let arr: [u32; 5] = [1,23,456,7,8];
    let arr2 = [3; 5];
    println!("{},{}",arr[3], arr2[3]);
}
