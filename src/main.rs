use std::io;

fn main() {
    let mut temp = String::new(); //new empty string
    
    println!("Input ur temp bro");
    
    io::stdin()
        .read_line(&mut temp).expect("wrong, too bad!"); //typical standard input series of commands to get input from the user and modify a variable
                                                         //notice we had to initliaze the temp variable
    
    let temp: f64 = temp.trim().parse() //here we trim the variable from the whitespaces
                                        //we parse it to be sure its a f64 number
        .expect("i cant lmao"); //this is a safety measure in case
    
    println!("Input ur target temp lmaoooooooo");
    let mut tt = String::new(); //new empty string
    io::stdin()//input shit
        .read_line(&mut tt).expect("again, too bad!");//error handling and initliazing the borrowed variable

    let tt = tt.trim(); //trimming the 'tt' variable

    let mut result = 0.0; //new variable to put the final result in

    if tt == "c" {
        result = f2c(temp); //new functions to modify temp and give result a value
    } else if tt == "f" {
        result = c2f(temp);
    } else {
        println!("bruto");
    }
    println!("temp is {}{}", result, tt);
}

fn f2c(x: f64) -> f64 { //x is the variable we use in this function but when we call it in the main function whatever goes in the () when we call f2c will replace 'x' here
    ((x - 32.0)*5.0) / 9.0
}

fn c2f(x: f64) -> f64 {
    ((x / 5.0) * 9.0) + 32.0
}

//something important to note is that we had to use the 'temp' variable in the same body of the other variables or whatever
//because if we tried to use temp in the body of another operation it would have to be initiliazed