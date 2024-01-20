
fn main() {
    c();
}


fn a() {
    proconio::input! {
        x: usize,
    }

    let ryu_str = 'o'.to_string().repeat(x);

    println!("L{}ng", ryu_str);

}

fn b() {

    proconio::input! {
        n: u32,
    }

    let binary = format!("{:b}", n);
    let count = binary.chars().rev().take_while(|&c| c == '0').count();
    println!("{}", count);

}

fn c() {

    proconio::input! {
        n: u64,
    }

    if n == 1 {
        println!("0");
    }
    let result = divide_by_five(n-1, String::new());
    println!("{}", result);
}

fn divide_by_five(n: u64, result: String) -> String { 
    if n == 0 {
        return result;
    }

    let quotient = n / 5;
    let remainder = n % 5;
    //println!("数値: {}, 商: {}, 余り: {}", n, quotient, remainder);
    let doubled_remainder = (remainder * 2).to_string();
    let new_str = format!("{}{}", doubled_remainder, result);

    return divide_by_five(quotient, new_str);
}

