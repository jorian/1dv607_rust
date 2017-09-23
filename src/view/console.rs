use std::io;

pub struct Console {
    
}

impl Console {
    fn get_input(&self, input_str: String) -> String {        
        println!("{}", input_str);
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        return input
    }

    pub fn start_program(&self) {
        println!("Welcome to the membership register!");
        println!("What would you like to do?");
    
        loop {
            println!("Which register would you like to use?");

            let choice: String = self.get_input(String::from("1) Member, 2) Boat"));

            //println!("You chose option: {}", choice);

            match choice.trim().as_ref() {
                "1" => { 
                    println!("You chose option 1) Member"); 
                    self.end_here();
                    break;
                },
                "2" => { println!("You chose option 2) Boat"); break }, 
                _ => println!("You chose the wrong option, please try again.")
            }

        }
    }

    fn end_here(&self) {
        println!("Did loop end?");
        let choice: String = self.get_input(String::from("Second question!"));
    }
}