fn main() {
    a();
    b();
    c();
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
