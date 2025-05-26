use std::io;
pub fn read_inputs (name : &str, var : &mut String) -> io::Result<()> {
    println!("Enter the {}: " ,name); 
    io::stdin().read_line( var)?;
    *var = var.trim()
              .replace(" ", "")
              .replace("\t", ""); 
    Ok(())
} 

pub fn freq_analyse (key : &String , plain : &String) {
    let mut i = 0 ; 
     
    while i < key.len() {
        let mut j = 0 ;
        let mut freq : Vec <char>  = Vec::new(); 
        loop {
            if (i + key.len()*j) >= plain.len() {
                 break ; 
            }
            else {
                freq.push(plain.chars().nth(i + key.len()*j).unwrap());
                j+=1;
            }
        }
        println!("The letter {} encrypt the letters {:#?}",key.chars().nth(i).unwrap(),freq);
        i+=1 ;   
    } 
}

pub fn extend_key (key : &mut String , plain : &mut String) { // maybe i will use it later
    let mut i = key.len();
    if key.len() < plain.len() {
        let mut j = 0;
        while i < plain.len() {
            let char_j = key.chars().nth(j).unwrap();
            key.push(char_j);
            j += 1;
            if j == key.len() {
                j = 0;
            }
            i += 1;
        }
    }
}

