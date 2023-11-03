
fn main() {
    a();
    b();
    c();
    d();
}

fn a() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let a : i32= iter.next().unwrap().parse().unwrap();
    let b : i32= iter.next().unwrap().parse().unwrap();

    if a < b {
        println!("a < b");

    } else if a > b {
        println!("a > b");
    } else {
        println!("a == b");
    }
}

fn b() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let a : i32= iter.next().unwrap().parse().unwrap();
    let b : i32= iter.next().unwrap().parse().unwrap();
    let c : i32= iter.next().unwrap().parse().unwrap();

    if a < b && b < c{
        println!("Yes");
    } else {
        println!("No");
    }
}

fn c() {

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let mut array:Vec<i32>  = vec![
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    ];

    array.sort();

    println!("{} {} {}", array[0], array[1], array[2]);

}

//*Circle in a Rectangle */
//*https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_2_D&lang=ja */
fn d() {

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let w : i32= iter.next().unwrap().parse().unwrap();
    let h : i32= iter.next().unwrap().parse().unwrap();
    let x : i32= iter.next().unwrap().parse().unwrap();
    let y : i32= iter.next().unwrap().parse().unwrap();
    let r : i32= iter.next().unwrap().parse().unwrap();

    // 4 points

    let top = Point { 
        x: x,
        y: y + r,
    };

    let bottom = Point {
        x: x,
        y: y - r

    };

    let right = Point {
        x: x + r,
        y: y,
    };

    let left = Point {
        x: x - r,
        y: y,
    };

    if top.y > h || bottom.y < 0 || right.x > w || left.x < 0 {
        println!("No");
    } else {
        println!("Yes");
    }

}

struct Point {
    x:i32,
    y:i32
}
