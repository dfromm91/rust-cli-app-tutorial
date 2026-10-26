fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let name_arg = args.get_mut(1);
    let name = match name_arg {
        Some(n) => n,
        None => {
            eprintln!("whoops!");
            return;
        }
    };
    snoopifier(name);
    println!("{name}");
}

fn snoopifier(s: &mut String) {
    s.push_str("izzle");
}
