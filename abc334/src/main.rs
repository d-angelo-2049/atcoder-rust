
// https://atcoder.jp/contests/abc334/tasks
fn main() {
    a();
    b();
    c();
}


fn a() {

    proconio::input! {
        bat: i32,
        grab: i32,
    }

    if bat > grab {
        println!("Bat");
    } else {
        println!("Glove");

    }

}

fn b() {

    proconio::input! {
        a: i64,
        m: i64,
        l: i64,
        r: i64,
    }



    if a < l && l < r {
        let kl = divide_and_ceil(l - a, m);
        let kr = (r - a) / m;
        let result = kr -(kl - 1);
        println!("{}", result);


    } else if l == a && l < r {
        let kr = (r - a) / m;
        let result = kr + 1;
        println!("{}", result);


    } else if l < a && a < r {
        let kl = (a - l) / m;
        let kr = (r - a) / m;
        let result = kr + kl + 1;
        println!("{}", result);

    } else if l == r {

        if l == a {
            println!("1");

        } else if a < l {
            let kl = (l - a) % m;
            if kl == 0 {
                println!("1");

            } else {
                println!("0");
            }
 
        } else if l < a {
            let kl = (a - l) % m;

            if kl == 0 {
                println!("1");
            } else {
                println!("0");
            }
        }

    } else if l < r && r == a {
        let kl = (a - l) / m;
        let result = kl + 1;
        println!("{}", result);

    } else if l < r && r < a {
        let kl = (a - l) / m;
        let kr = divide_and_ceil(a - r, m);
        let result = kl - (kr - 1);
        println!("{}", result);
    }


    fn divide_and_ceil(dividend: i64, divisor: i64) -> i64 {
        let quotient = dividend.div_euclid(divisor);
        let remainder = dividend.rem_euclid(divisor);
    
        if remainder > 0 {
            quotient + 1
        } else {
            quotient
        }
    }

}

fn c() {

    proconio::input! {
        n: i64,
        k: i64,
        ak: [i64;k]
    }

    // ここはわかりきっているため　早期リターン
    if k == 1 {
        println!("0");
        return;
    }

    if k % 2 == 0 {
        // 偶数
        let mut sum = 0;
        for (index, item) in ak.iter().enumerate().step_by(2) {

            sum += (item - ak[index + 1]).abs();
        }

        println!("{}", sum);
    } else {
        // 奇数
        let mut min_sum;
        let mut pre_sum = 0;
        let initial_array: Vec<_> = ak.iter()
            .enumerate()
            .filter(|(index, _)| *index != 0)
            .map(|(_, &item)| item)
            .collect();

        // index =0 を外した際の 累積和(min sum と presum の初期値)
        for (index, item) in initial_array.iter().enumerate().step_by(2) {
            pre_sum += (item - initial_array[index + 1]).abs();
        }
        min_sum = pre_sum;

        // index が偶数の時のみ計算対象に入れて pre sum との変動分のみ計算することでO(n)の計算量にとどめる
        for (index, _) in ak.iter().enumerate() {
            if index == 0 || index % 2 == 1 {
                continue;
            }

            let now_sum = pre_sum - (ak[index - 1] - ak[index]).abs() + (ak[index - 2] - ak[index - 1]).abs();
            if min_sum > now_sum {
                min_sum = now_sum;
            }
            pre_sum = now_sum;
        }
        println!("{}", min_sum);

    }
}
