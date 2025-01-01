use std::io;

fn main() {
    
    println!("I can find 'Nemo' in things you type!");
    
    let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read age. Make you an integer is used.");
        
        let words = guess.split_whitespace();
        
        let mut wordcount = 0;
        let mut found = false;
        
        for word in words {
            wordcount += 1;
            if word == "Nemo" {
                println!("Found Nemo at {}", wordcount);
                found = true;
                break
            }
        }
        
        if found == false {
            println!("You suck!");
        }
        
        // let guess: str =  match guess.trim().parse() {
        //     Ok(num) => num, 
        //     Err(_) => continue,
        // };

        // let age: u32 = guess * 365;

        // println!("You are roughly {age} days old!")
    
}

