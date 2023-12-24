fn main() {
    println!("Hello, world!");
}


fn a() {
    proconio::input! {
        n: i32,
        s: i32,
        k: i32
    }

    let sum = 0;

    for _ in 0..n {
        proconio::input! {
            p: i32,
            q: i32

        }

        sum += p * q;
    }

    if sum < s {
        sum += k;

    }

    println!("{}", sum);

}


fn b() {

    proconio::input! {
        k: i32,
        g_max: i32,
        m_max: i32
    }

    let mut g_vol = 0;
    let mut m_vol = 0;




    for _ in 0..k {
        if g_vol >= g_max {
            g_vol = 0;

        } else if m_vol <= 0 {
            m_vol = m_max;
        } else {

            let remain = g_max - g_vol;

            if remain >= m_vol {
                g_vol += m_vol;
                m_vol = 0;
            
            } else if remain < m_vol {
                g_vol += remain;
                m_vol -= remain;
            }
        }
    }

    println!("{} {}", g_vol, m_vol);


}

fn c () {

    proconio::input! {
        n: i32,
        m: i32,
        s: [i32;n]
    }
    
    let mut plain = t_manager {
        stock: m,
        worn: 0,
        buy:0
    };


    let mut logo = t_manager {
        stock: 0,
        worn: 0,
        buy:0
    };
    
    
    for i in s {

        if i == 0 {
            plain.stock += plain.worn;
            plain.worn = 0;

            logo.stock += logo.worn;
            logo.worn = 0;

        } else if i == 1 {
            if plain.stock >= 1 {
                plain.stock -= 1;
                plain.worn += 1;
            } else if logo.stock >= 1 {
                logo.stock -= 1;
                logo.worn +=1
            } else {
                logo.buy += 1;
                logo.worn += 1;
            }

        } else {
            if logo.stock >= 1 {
                logo.stock -= 1;
                logo.worn +=1
            } else {
                logo.buy += 1;
                logo.worn += 1;
            }
        }
    }
    
    println!("{}", logo.buy);
      

}


struct t_manager {
    stock:i32,
    worn:i32,
    buy:i32
}

