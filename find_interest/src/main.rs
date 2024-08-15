fn main() {
    let mut pricipal = String::new();
    let mut rate = String::new();
    let mut time = String::new();
    println!("Enter the pricipal amount: ");
    std::io::stdin().read_line(&mut pricipal).expect("Failed to read line");
    let p = pricipal.trim().parse::<f64>().unwrap();
    println!("Enter the rate: ");
    std::io::stdin().read_line(&mut rate).expect("Failed to read line");
    let r = rate.trim().parse::<f64>().unwrap();
    println!("Enter the time: ");
    std::io::stdin().read_line(&mut time).expect("Failed to read line");
    let t = time.trim().parse::<f64>().unwrap();

    simple_interest(p, r, t);
    compound_interest(p, r, t);
}

fn simple_interest(p:f64,r:f64,t:f64) {
    let si = (p * r * t) / 100.0;
    println!("Simple Interest => ({p} X {r} X {t})/100 = {}", si);
}

fn compound_interest(p:f64,r:f64,t:f64) {
    let ci = p * (1.0 + r/100.0).powf(t);
    println!("Compound Interest => {p} X (1 + {r}/100)^{t} = {}", ci);
}
