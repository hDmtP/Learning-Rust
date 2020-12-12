fn main() {
    // name should be celciusto_farenhite
    let ans: f32 = temp_c_2f(15.7);
    println!("{}", ans);
}

fn temp_c_2f(c: f32) -> f32 {
    ((9.0*c) /5.0) +32.0
}
