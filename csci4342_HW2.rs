//Description: This program takes in a binary file (which lists fixed-length values each on a separate line) 
// and passes it to load_diagnostics, which reads it and passes it to check_life_support. check_life_support
// takes in the list of binary numbers and O2 generator rate (most common digit in that that numerical place,
// removing numbers that don't match the rate), CO2 scrubber rate (least common digit in that numerical place,
// removing numbers that don't match the rate), and overall life support rate (O2 generator rate multipled by 
// CO2 scrubber rate). The life support rate is returned and printed to terminal.

use std::env::args; //import for command line arguments
use std::fs; //import for reading files
use std::convert::TryInto; //import for converting data types

fn main(){
    let input_file: String = args().nth(1).unwrap(); //takes command line argument
    println!("Loading diagnostics...");
    let file_content = load_diagnostics(input_file);
    check_life_support(file_content);
    //println!("{:?}",load_diagnostics(input_file)); //:? is format for printing vectors
}

fn load_diagnostics(input_file: String)-> Vec<String>{
    let file_content = fs::read_to_string(input_file).unwrap().lines().map(String::from).collect();
        //reads each line of the file into an element of a vector
    return file_content;
}

fn check_life_support(file_content: Vec<String>){
    let mut o2_generator_rate = "".to_string();//.to_string allows push_str later on
    let mut co2_scrubber_rate = "".to_string();//.to_string allows push_str later on
    let life_support_rate;
    let mut count = 0.0; //decimal type to allow proper division and comparison
    let mut number_length: i32 = file_content[0].len().try_into().unwrap();

	//o2 generator rate is number comprised of most common bit in each place (assume 1 if equal)
    //disregard strings if they do not have previously determined most common bit
    while number_length > 0{ //iterates through string
        let mut included_strings = 0.0; //decimal type to allow proper division and comparison
        for element in file_content.iter(){
            let element_as_int: i32 = element.parse().unwrap();
            let base: i32 = 10;
            let bit_shift: i32 = base.pow((number_length-1).try_into().unwrap());
            if element.starts_with(&o2_generator_rate){
                included_strings = included_strings + 1.0;
                if (element_as_int/bit_shift).to_string().ends_with('1'){
                    count = count + 1.0;
                }
            }
        }
        if count >= (included_strings/2.0){
            let new_character = "1".to_string();
            o2_generator_rate.push_str(&new_character);
        }
        else{
            let new_character = "0".to_string();
            o2_generator_rate.push_str(&new_character);
        }
        count = 0.0;
        number_length = number_length - 1;
    }
    //println!("{} o2", o2_generator_rate);
    println!("O2 Generator rate computed...");

    number_length = file_content[0].len().try_into().unwrap();

	//co2 scrubber rate is number comprised of least common bit in each place (assume 1 if equal)
    //disregard strings if they do not have previously determined least common bit
    let mut reduced_file_content: Vec<String> = file_content.clone(); //holds contents of file that meet criteria
    while number_length > 0{ //iterates through string
        let mut included_strings = 0.0; //decimal type to allow proper division and comparison
        let mut final_number = false;
        for element in file_content.iter(){
            let element_as_int: i32 = element.parse().unwrap();
            let base: i32 = 10;
            let bit_shift: i32 = base.pow((number_length-1).try_into().unwrap());

            //if only one element is remaining in reduced_file_content, copy the rest of the digits into co2 scrubber rate
            if reduced_file_content.len() == 1{
                final_number = true;
                let final_element_as_int:i32 = reduced_file_content[0].parse().unwrap();
                if (final_element_as_int/bit_shift).to_string().ends_with('1'){
                    let new_character = "1".to_string();
                    co2_scrubber_rate.push_str(&new_character);
                }
                if (final_element_as_int/bit_shift).to_string().ends_with('0'){
                    let new_character = "0".to_string();
                    co2_scrubber_rate.push_str(&new_character);
                }
                break;
            }

            if reduced_file_content.contains(element){
                included_strings = included_strings + 1.0;
                if (element_as_int/bit_shift).to_string().ends_with('1'){
                    count = count + 1.0;
                }
            }
        }

        if !final_number && count == included_strings{
            let new_character = "1".to_string();
            co2_scrubber_rate.push_str(&new_character);
        }
        
        else if !final_number && count == 0.0{
            let new_character = "0".to_string();
            co2_scrubber_rate.push_str(&new_character);
        }

        else if !final_number && count < included_strings/2.0{
            let new_character = "1".to_string();
            co2_scrubber_rate.push_str(&new_character);
        }

        else if !final_number && count >= included_strings/2.0{
            let new_character = "0".to_string();
            co2_scrubber_rate.push_str(&new_character);
        }

        for element in file_content.iter(){
            if !final_number && element.starts_with(&co2_scrubber_rate) == false{
                if reduced_file_content.contains(element){
                    let index = reduced_file_content.iter().position(|r| *r == element.to_string()).unwrap();
                    reduced_file_content.remove(index);
                }
            }
        }
        count = 0.0;
        number_length = number_length - 1;
    }
    //println!("{} co2", co2_scrubber_rate);
    println!("CO2 Scrubber rate computed...");

    let o2_generator_rate_as_bin = isize::from_str_radix(&o2_generator_rate, 2).unwrap(); //converts string to binary
    let co2_scrubber_rate_as_bin = isize::from_str_radix(&co2_scrubber_rate, 2).unwrap(); //converts string to binary
    //life support rate is o2 generator rate multiplied by co2 scrubber rate
    life_support_rate = o2_generator_rate_as_bin * co2_scrubber_rate_as_bin;
    println!("Life support rate: {}", life_support_rate);

}