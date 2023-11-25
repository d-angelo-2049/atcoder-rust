
fn main() {
    a();
    b();
}


fn a() {
    proconio::input! {
        n: i32,
        l: i32,
        a: [i32;n]
    }

    let mut count:i32 = 0;
    for i in a {

        if i >= l {
            count += 1;
            
        }
    }

    println!("{}", count);
}

fn b() {

    proconio::input! {
        n: usize,
        l: i32,
        r: i32,
        a: [i32;n]
    };

    let mut x: Vec<i32> = Vec::new();

    for ai in a {

        if ai < l {
            x.push(l);


        } else if l <= ai && ai <= r  {
            x.push(ai);
            
        } else if r < ai {
            x.push(r);

        }

    }

        // Vecの要素をスペースで区切って表示
    let formatted_output: String = x
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>()
    .join(" ");

    println!("{}", formatted_output);

}