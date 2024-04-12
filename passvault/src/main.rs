mod psentry;

use crate::psentry::prompt;
use crate::psentry::read_passwords_from_file;

fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clr();
    let ascii = r#"
    
    ______________
    /             /|
   /             / |
  /____________ /  |
 | ___________ |   |
 ||           ||   |
 ||           ||   |
 ||           ||   |
 ||___________||   |
 |   _______   |  /
/|  (_______)  | /
( |_____________|/
\
.=======================.
| ::::::::::::::::  ::: |
| ::::::::::::::[]  ::: |
|   -----------     ::: |
`-----------------------'
    
    
    "#;

    println!("{ascii}");

    loop {
        println!("Password manager menu : ");
        println!("1. Add Entry ");
        println!("2. List Entry ");
        println!("3. Search Entry ");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {}
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords : {}", err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Services = {}
                        - Username = {}
                        - Password = {} ",
                        item.service, item.usernaem, item.password
                    )
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords : {}", err);
                    Vec::new()
                });

                let search = prompt("Search :");
                for item  in &services {
                    if item.service.as_str() == search.as_str(){
                        println!(
                            "Service = {}
                            - Username : {}
                            - Password : {}",
                            item.service, item.username, item.password
                        );
                    }
                }
            }
            "4" => {
                clr();
                println!("Have a great day!");
                break;
            }
            _ => println!("Invalid choice. ")
        }
        println!("\n\n");
    }
}
