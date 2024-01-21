
fn main() {
    b();
}


fn a() {

    proconio::input! {
        n: i32,
        scores: [i32;2 * n]
    }

    let mut x_result = 0;
    let mut y_result = 0;
    for chunk in scores.chunks(2) {
        if let Some(first_element) = chunk.first() {
            x_result += first_element;
        }

        if let Some(second_element) = chunk.get(1) {
            y_result += second_element;
        }
    }

    if x_result > y_result {
        println!("Takahashi");


    } else if x_result < y_result {
        println!("Aoki");

        
    } else {
        println!("Draw");
        
    }


}

fn b() {

    proconio::input! {
        n: String,
    }

    let mut result = "Yes";
    let mut prev_optional: Option<char> = None;
    for element in n.chars() {
        match prev_optional {
            Some(prev_value) => {
                match prev_value {
                    'B' => {
                        if element == 'B' || element == 'C' {
                            prev_optional = Some(element);
                        } else {
                            result = "No";
                            break;
                        }
                    },
                    'C' => {
                        if element == 'C' {
                            prev_optional = Some(element);
                        } else {
                            result = "No";
                            break;
                        }
                    },
                    _ => {
                        prev_optional = Some(element);
                    }
                    
                }
            },
            None => {
                // 初期値はそのまま prevに格納
                prev_optional = Some(element);
            }
            
        }

    }
    println!("{}", result);

}