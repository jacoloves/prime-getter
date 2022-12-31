use std::env;

fn calc_prime(ad: i32) -> Vec<i32> {
    let mut tmp_ad = ad;
    let mut v = Vec::new();
    loop {
        for i in 2..=ad {
            if tmp_ad % i == 0 {
                v.push(i);
                tmp_ad = tmp_ad / i;
                break;
            }
        }
        if tmp_ad == 0 || tmp_ad == 1 {
            break;
        }
    }
    v
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let ad_str = &args[1];

    let ad: i32 = ad_str.parse().unwrap();

    let v = calc_prime(ad);

    let mut cnt: i32 = 0;

    if v.len() == 1 {
        println!("!!!prime number!!!");
        println!("[{}]", v[0]);
    } else {
        println!("not prime number");
        print!("[");
        for e in &v {
            cnt = cnt + 1;
            if cnt == v.len().try_into().unwrap() {
                print!("{}", e);
            } else {
                print!("{}, ", e);
            }
        }
        println!("]");
    }
}

#[test]
fn test_calc_prime() {
    let args: Vec<String> = env::args().collect();

    let ad_str = &args[1];

    let ad: i32 = ad_str.parse().unwrap();
    let v: Vec<i32> = calc_prime(ad);
    let mut x: i32 = 1;
    for e in v {
        x = x * e;
    }

    assert_eq!(x, ad);
}
