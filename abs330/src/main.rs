
fn main() {
    a();
    b();
    c()
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

fn c() {

    proconio::input! {
        d: f64,
    };

    let mut result = 1414214;

    for x in 0..(d.sqrt().floor() as i32) + 1 {
        for y in 0..(d.sqrt().floor() + 1 as i32) + 1 {
            
            let diff = (((x * x) + (y * y)) - d.floor() as i32).abs();
            if result > diff {
                result = diff;
            }
        }

    }

    println!("{}", result);

    let root_d = d.sqrt().floor() as i32;


    for x in 0..(d.sqrt().floor() as i32) + 1 {

        let c = x * x - root_d;
        let mut diff = 0;

        if c >= 0 {
            diff = c.abs()

        } else {
            diff = 0;

        }

    }
}