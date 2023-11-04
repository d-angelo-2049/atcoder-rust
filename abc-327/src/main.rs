use regex::Regex;

fn main() {
    a();
    b();
}


fn a() {
    proconio::input! {
        n: String,
        s: String,
    }
    let re = Regex::new("ab|ba").unwrap();

    for _ in re.find_iter(&s) {

        println!("Yes");
        return;
    }
    println!("No");

}

fn b() {

    proconio::input! {
        n: u64,
    }

    for i in 1..16 {
        let mut result: u64 = 1;

        for _ in 1..(i + 1) {
            result = result * i;
        }

        if  result == n {
            println!("{}", i);
            return;           
        }

    }
    println!("-1");

}
