use chrono::{Local, Timelike};
use std::{env::args, process::Command, thread::sleep, time::Duration};

const ZERO: &str = r#"
     ####    
   ##    ##  
 ##        ##
 ##        ##
 ##        ##
 ##        ##
 ##        ##
 ##        ##
  ##      ## 
    ######   
"#;

const ONE: &str = r#"
     ######
    ### ###
   ###  ###
  ###   ###
        ###
        ###
        ###
        ###
        ###
        ###
"#;

const TWO: &str = r#"
   #######  
  ##########
 ##      ###
         ###
        ### 
     #####  
   ####     
  ###       
 ###        
 ###########
"#;

const THREE: &str = r#"
  #########
 ##     ###
        ###
        ###
    ###### 
        ###
        ###
        ###
 ##     ###
  #########
"#;

const FOUR: &str = r#"
     ##### 
    ###### 
   ### ### 
  ###  ### 
 ###   ### 
 ##########
 ##########
       ### 
       ### 
       ### 
"#;

const FIVE: &str = r#"
 ##########
 ##########
 ###       
 ###       
 ######### 
        ###
        ###
        ###
 ##    ### 
  #######  
"#;

const SIX: &str = r#"
   ######## 
  ###    ###
 ###        
 ###        
 #########  
 ###    ### 
 ###    ### 
 ###    ### 
  ###  ###  
   ######   
"#;

const SEVEN: &str = r#"
 ##########
 ##########
       ### 
      ###  
     ###   
    ###    
   ###     
  ###      
  ###      
  ###      
"#;

const EIGHT: &str = r#"
   #######  
  ###   ### 
 ###     ###
 ###     ###
  ###   ### 
   #######  
  ###   ### 
 ###     ###
 ###     ###
  ######### 
"#;

const NINE: &str = r#"
   #######  
  ###   ### 
 ###     ###
 ###     ###
  ###   ####
   #########
         ###
        ### 
 ###   ###  
  #######   
"#;

const SEP: &str = r#"
   
###
###
###
   
   
###
###
###
   
"#;

fn number_to_ascii(num: char) -> String {
    match num {
        '0' => ZERO.to_string(),
        '1' => ONE.to_string(),
        '2' => TWO.to_string(),
        '3' => THREE.to_string(),
        '4' => FOUR.to_string(),
        '5' => FIVE.to_string(),
        '6' => SIX.to_string(),
        '7' => SEVEN.to_string(),
        '8' => EIGHT.to_string(),
        '9' => NINE.to_string(),
        _ => "".to_string(),
    }
}

fn write_time(hour: u32, min: u32, sec: u32, nosec: bool) {
    let time_str: String;
    if nosec {
        time_str = format!("{:02}:{:02}", hour, min);
    } else {
        time_str = format!("{:02}:{:02}:{:02}", hour, min, sec);
    }
    let mut ascii_lines = vec![String::new(); 11];
    for ch in time_str.chars() {
        let ascii_art = match ch {
            ':' => SEP,
            d if d.is_digit(15) => &number_to_ascii(d),
            _ => "",
        };

        let lines: Vec<&str> = ascii_art.lines().collect();
        for i in 0..11 {
            ascii_lines[i].push_str(lines[i]);
            ascii_lines[i].push(' ');
        }
    }

    for line in ascii_lines {
        println!("{}", line);
    }
}

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    let mut nosec: bool = false;

    for arg in args.iter() {
        if arg == "--no-sec" || arg == "-n" {
            nosec = true;
        }
    }

    loop {
        let now = Local::now();

        let hour = now.hour();
        let min = now.minute();
        let sec = now.second();
        if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", "cls"]).status().unwrap();
        } else {
            Command::new("clear").status().unwrap();
        }
        write_time(hour, min, sec, nosec);
        sleep(Duration::from_secs(1));
    }
}
