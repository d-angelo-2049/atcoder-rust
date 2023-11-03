fn main() {
    a();
    b();
    c();
    d();
}

//*Hello World */
//*https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_A */
fn a() {
    println!("Hello, world!");
}

//*Cubic */
//*https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_B */
fn b() {
    proconio::input! {
        x: i32,
    }
    println!("{}", x.pow(3));
}

//*Rectangle  */
//*https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_C&lang=en */
fn c() {
    proconio::input! {
        a: i32,
        b: i32,
    }
    let rec: i32 = 2 * (a + b);
    let space: i32 = a * b;
    println!("{} {}", space, rec);
}

//*Watch  */
//*https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_D&lang=en*/
fn d() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let input: i32 = buf.trim().parse().unwrap();

    let h = input / 3600;
    let m  = (input / 60) % 60;
    let s = (input % 3600) % 60;
    println!("{}:{}:{}", h, m, s);
}
