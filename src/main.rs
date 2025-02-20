use std::fs::read_to_string;
use std::io::stdin;
use std::fs;
use std::collections::LinkedList;

fn greeeting(){
    /*
    Provides a message of welcome to the user starting to use the program, it is oriented to show the options available
    to use the program.
    */
    println!("Main Menu");
    println!("1: Search a text file");
    println!("2: Create a text file");
    println!("3: Delete a text file");
}

fn get_user_text(prompt_message: String) -> String{
    /*
    Prompts a message and requests to enter text, that text is returned
    */
    println!("{prompt_message}");
    let mut mytext = String::new();
    stdin().read_line(&mut mytext).expect("Please enter some text");
    return mytext;
}

fn list_files(){
    /*
    Shows all the files in the current folders.
    */
    println!("\nThe list of files is the following:");
    let mut _item = String::new();
    for file in fs::read_dir(".").unwrap(){
        _item = file.unwrap().path().display().to_string().replace(".\\", "");
        println!("{}", _item);
    }
    println!("");
}

fn read_file(file_name: String) -> std::io::Result<()> {
    /*
    Prints the contents of an entire file, and limits them according to the configuration specified in the function
    */
    let mut contents = String::new();
    for line in read_to_string(file_name.trim()).unwrap().lines() {
        contents.push_str(line);
        contents.push_str("\n");
    }

    let output_character_limit = 200;
    let complete_content = String::from(contents);
    let content_extracted = &complete_content[0..output_character_limit];

    println!("\nLimiting output to {output_character_limit} characters");
    println!("\n{content_extracted}\n");
    Ok(())
}

fn create_file(file_name: String, content: String){
    /*
    Creataes a new text file based on user input
    */
    let _ = fs::write(file_name.trim(), content);
    println!("File created\n");
}

fn delete_file(file_name: String){
    /*
    Deletes a file based on the name the user provided.
    */
    fs::remove_file(file_name.trim()).expect("Error");
    println!("File deleted\n");
}

const FIRST_OPTION: &str = "1";
const SECOND_OPTION: &str = "2";
const THIRD_OPTION: &str = "3";

struct FileConfiguration {
    first_option: String,
    second_option: String,
    third_option: String
}

fn main() -> std::io::Result<()> {
    /* Basic File I/O: Write a program that reads from and writes to text files, demonstrating file input/output operations. 
    The way in which this will satisfy the requirements of this module, is because I will add slicing as one of the additional requirements, 
    in addition I will use Variables (mutable and immutable), Expressions, Conditionals, Loops and Functions, for instance, when we read from 
    a text file, we will have to store momentarily that text into a variable, and verify that it is not empty, then we will use variables and 
    conditionals, and we can create functions to avoid repeating certain parts of the code.
    */

    let file_configuration = FileConfiguration {
        first_option: FIRST_OPTION.to_string(),
        second_option: SECOND_OPTION.to_string(),
        third_option: THIRD_OPTION.to_string()
    };

    let mut messages: LinkedList<&str> = LinkedList::new();
    messages.push_back("Enter the name of the file you want to read:");
    messages.push_back("\nEnter the name of the file you want to create:");
    messages.push_back("\nPlease enter the content of the file:");
    messages.push_back("\nEnter the name of the file you want to delete: ");
    
    let mut msg = messages.iter_mut();
    let _message_first = msg.next().unwrap();
    let _message_second = msg.next().unwrap();
    let _message_third = msg.next().unwrap();
    let _message_fourth = msg.next().unwrap();

    loop {
        
        greeeting();
        
        let selected_option = get_user_text("\nChoose an option:".to_string());
        if selected_option.trim() == file_configuration.first_option {
            list_files();

            let file_name = get_user_text(_message_first.to_string());

            match read_file(file_name.to_string()){
                Ok(()) => (),
                Err(_) => {
                    println!("\nThe file name you entered does not exist");
                }
            }
            
            continue;
        }

        if selected_option.trim() == file_configuration.second_option {
            let file_name = get_user_text(_message_second.to_string());
            let content = get_user_text(_message_third.to_string());

            create_file(file_name.to_string(), content.to_string());
            continue;
        }

        if selected_option.trim() == file_configuration.third_option {
            list_files();

            let file_name = get_user_text(_message_fourth.to_string());
            
            delete_file(file_name.to_string());
            continue;
        
        } else {
            println!("Please choose a valid option\n");
            continue;
        }
    }
}
