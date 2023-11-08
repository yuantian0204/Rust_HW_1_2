fn main() {
    let mut f: [u8; 101] = [0; 101];
    f[0] = 0;
    f[1] = 1;
    println!("F(0) = {:?}", f[0]);
    println!("F(1) = {:?}", f[1]);
    for i in 2..=100 {
        f[i] = f[i - 1] + f[i - 2];
        println!("F({i}) = {:?}", f[i]);
    }
}
