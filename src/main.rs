fn main() {
    for i in 1..256 {
        print!("\x1b[38;5;{i}m{i: >3} ");
        if i % 17 == 0 {
            println!("");
        }
    }
    print!("\x1b[0m");
    println!("\\x1b[38;5;IDm # Set Foreground Color By Id");
    println!("\\x1b[48;5;IDm # Set Background Color By Id");
    println!("\\x1b[0m       # Return to Default Style Set");
    println!("\\x1bc         # Clears Screen of Text");
}
