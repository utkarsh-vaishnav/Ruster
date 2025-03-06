fn update_light(current: &str) -> String {
    let ns;
    
    match current {
        "green" => {ns = "yellow"},
        "yellow" => {ns = "red"},
        _ => {ns="green"}
    }
    ns.to_string()
}

fn main() {
    let result = update_light("red");
    println!("{}",result);
}