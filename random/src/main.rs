use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::iter::FromIterator;

const LOW_ARGS_ERROR : &str = "You have the wrong arguments. Try -h!";

//Made by Oscar Veldman
fn main() { 
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() > 1 {
        find(arguments);
    }
    else {
        error(&LOW_ARGS_ERROR);
    }
}

fn find(arguments: Vec<String>) {
    let action = &arguments[1];

    match action.as_ref() {
        "add" => add(&arguments),
        "bubblesort" => bubble_sort(&arguments),
        "empty" => empty(&arguments),
        "insertionsort" => insertion_sort(&arguments),
        "math" => math(&arguments),
        "mergesort" => start_merge_sort(&arguments),
        "selectionsort" => selection_sort(&arguments),
        "remove" => remove(&arguments),
        "replace" => replace(&arguments),
        "-h" => help(),
        _ => error(&LOW_ARGS_ERROR),
    }
}

fn add(arguments: &Vec<String>) {
    if arguments.len() < 4 {
        error(&LOW_ARGS_ERROR);
    }

    let file = &arguments[2];
    let to_add = &arguments[3];

    let content = read(&file);
    let content = add_to_content(content, to_add);
    let finish_code = save(&file, content);
    done(finish_code);
}

fn empty(arguments: &Vec<String>) {
    if arguments.len() < 3 {
        error(&LOW_ARGS_ERROR);
    }

    let file = &arguments[2];
    let content = String::from("");
    let finish_code = save(&file, content);
    done(finish_code);
}

fn math(arguments: &Vec<String>){
    if arguments.len() < 2 {
        error(&LOW_ARGS_ERROR);
    }

    let formule = &arguments[2];
    calculate(&formule);
}

fn remove(arguments: &Vec<String>) {
    if arguments.len() < 4 {
        error(&LOW_ARGS_ERROR);
    }

    let file = &arguments[2];
    let to_remove = &arguments[3];

    let content = read(&file);
    let content = remove_from_content(content, to_remove);
    let finish_code = save(&file, content);
    done(finish_code);
}

fn replace(arguments: &Vec<String>) {
    if arguments.len() < 5 {
        error(&LOW_ARGS_ERROR);
    }

    let file = &arguments[2];
    let previous = &arguments[3];
    let replacement = &arguments[4];

    let content = read(&file);
    let content = copy(content, previous, replacement);
    let finish_code = save(file, content);
    done(finish_code);
}   

fn read(file: &String) -> String {
    let open_file = File::open(file);
    let mut open_file = match open_file {
        Ok(file) => file,
        Err(e) => {
            let error_message = String::from("The file doesn't exists!");
            error(&error_message);
            panic!("Problem: {:?}", e)
        },
    };

    let mut content = String::new();
    open_file.read_to_string(&mut content).expect("something went wrong reading the file");
    content
}

fn add_to_content(content: String, to_add: &String) -> String {
    let mut temp_content = content.clone();
    let mut temp_add = String::from("\n");
    temp_add.push_str(to_add);
    temp_content.push_str(&temp_add);
    temp_content
}

fn calculate(formule: &String) {
    let mut formule = split_math(&formule, true);
    let mut orde_math = get_order_math(&formule);
    let max_loop = (formule.len() - 1) / 2;
    let  mut current_loop = 0;
    let mut answer = 0;

    while current_loop < max_loop {
        let current_orde = orde_math[0];
        let min_orde = current_orde - 1;
        let max_orde = current_orde + 2;
        let small_formule = Vec::from_iter(formule[min_orde..max_orde].iter().cloned());
        answer = run_math(&small_formule);
        formule.remove(current_orde - 1); 
        formule.remove(current_orde - 1);
        formule[current_orde - 1] = answer.to_string();
        orde_math = set_smaller_orde(orde_math);
        current_loop = current_loop + 1;
    }

    println!("Answer: {}", answer);
}

fn set_smaller_orde(orde: Vec<usize>) -> Vec<usize>{
    let mut temp_orde = orde.clone();
    let last_orde = temp_orde[0];
    temp_orde.remove(0);
    let mut current_loop = 0;

    while current_loop < temp_orde.len() {
        if last_orde < temp_orde[current_loop] {
            temp_orde[current_loop] = temp_orde[current_loop] - 2;
        }
        current_loop = current_loop + 1;
    }

    temp_orde
}

fn get_order_math(formule: &Vec<String>) -> Vec<usize> {
    let mut orde: Vec<usize> = Vec::new();

    let mut place = 0;
    for check_sign in formule {
       match check_sign.as_ref() {
           "*" => orde.push(place),
           "/" => orde.push(place),
           _ => {},
       };

       place = place + 1;
    }

    let mut place = 0;
    for check_sign in formule {
       match check_sign.as_ref() {
           "+" => orde.push(place),
           "-" => orde.push(place),
           _ => {},
       };

       place = place + 1;
    }
    orde
}

fn run_math(formule: &Vec<String>) -> i32 {
    let value_one = formule[0].parse::<i32>().unwrap();
    let value_two = formule[2].parse::<i32>().unwrap();
    let check_sign = &formule[1];

    let answer = match check_sign.as_ref() {
        "+" =>  value_one + value_two,
        "-" =>  value_one - value_two,
        "*" =>  value_one * value_two,
        "/" =>  value_one / value_two,
        _ => {
            let message = String::from("Doesn't understand the math sign");
            error(&message);
            0
        },
    };
    answer
}

fn remove_from_content(content: String, to_remove: &String) -> String {
    let replacement = String::from("");
    let content = copy(content, to_remove, &replacement);
    content
}

fn copy(content: String, previous: &String, replacement: &String) -> String {
    content.replace(previous, replacement)
}

fn save(file: &String, content: String) -> bool {
    let mut open_file = File::create(file).expect("file couldn't created");
    open_file.write(content.as_bytes()).expect("something went wrong writing the file");
    true
}

fn split_math(formule: &String, is_math: bool) -> Vec<String>{
    let mut splitted_formule : Vec<String> = Vec::new();
    let mut temp_value: String = String::new();
    let mut last_integer: bool = false;

    for temp_char in formule.chars(){
        match temp_char.to_string().parse::<i32>() {
            Ok(..) => {
                temp_value.push_str(&temp_char.to_string());
                last_integer = true;
            },
            Err(..) => {
                if last_integer {
                    splitted_formule.push(temp_value);

                    if is_math {
                        splitted_formule.push(temp_char.to_string());
                    }

                    temp_value = String::new();
                }else{
                    let error_message = String::from("The Math is wrong!");
                    error(&error_message);
                }
                last_integer = false;
            },
        }
    }
    if last_integer {
        splitted_formule.push(temp_value);
    }else{
        let error_message = String::from("The math is wrong!");
        error(&error_message);
    }


    splitted_formule 
}

fn done(no_error: bool){
    if no_error {
        println!("Done!");
        process::exit(0x0100);
    } else {
        let error_message = String::from("Something happend!");
        error(&error_message);
    }
}

fn error(message: &str) {
    println!("You did something wrong! {}", message);
    process::exit(0x0100);
}

fn help() {
    let file = String::from("/home/rodero/selfmade/help.txt");
    let content = read(&file);
    println!("{}", content);
}

fn start_merge_sort(arguments: &Vec<String>){
    if arguments.len() < 3 {
        error(LOW_ARGS_ERROR);
    }

    let list = split_math(&arguments[2], false);
    let list = create_list_from_arguments(&list);
    println!("{:?}", list);
    let length_list = list.len();
    let new_array = copy_list(&list, 0, length_list);
    let array_sorted = merge_split(new_array, 1, length_list);
    println!("{:?}", array_sorted)
}

fn create_list_from_arguments(integer_list: &Vec<String>) -> Vec<i32> {
    let mut int_list: Vec<i32> = Vec::new();
    let temp_list = integer_list.clone();
    let max_loop = temp_list.len();
    let mut follow = 0;

    while follow < max_loop {
        match temp_list[follow].to_string().parse::<i32>() {
            Ok(..) => {
                let temp_int = temp_list[follow].parse::<i32>().unwrap();
                int_list.push(temp_int);
            },
            Err(..) => {
                let message = "This is not a number";
                error(message);
            },
        }

        follow = follow + 1;
    }

    int_list
}

fn merge_split(new_list: Vec<i32>, begin: usize, end: usize) -> Vec<i32> {
    let mut new_list = new_list; 
    if begin < end {
        let middle: usize = (begin + end) / 2;
        new_list = merge_split(new_list, begin, middle);
        new_list = merge_split(new_list, middle + 1, end);
        new_list = top_down_merge(new_list, begin, middle, end);
    }
    new_list
}

fn top_down_merge(new_list: Vec<i32>, begin: usize, middle: usize, end: usize) -> Vec<i32>{
    let mut new_list = new_list; 
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut left_number = 0;

    let get_left = middle - begin + 1;
    let get_right = end - middle;    

    while left_number < get_left {
        let orginal_left = begin + left_number - 1;
        left.push(new_list[orginal_left]);
        left_number = left_number + 1;
    }

    let mut right_number = 0;
    while right_number < get_right {
        let orginal_right = middle + right_number;
        right.push(new_list[orginal_right]);
        right_number = right_number + 1;
    }

    let mut k: usize = begin - 1;

    let mut i: usize = 0;
    let mut j: usize = 0;

    while k < end {

        if j == right.len() {
            new_list[k] = left[i];
            i = i + 1;
        } else if i == left.len() {
            new_list[k] = right[j];
            j = j + 1;
        }

        if i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                new_list[k] = left[i];
                i = i + 1;
            }
            else
            {
                new_list[k] = right[j];
                j = j + 1;
            }
        }

        k = k + 1;
    }

    new_list
}

fn copy_list(list: &Vec<i32>, begin: usize, end: usize) -> Vec<i32> { 
    let mut new_array: Vec<i32> = Vec::new();

    let mut follow: usize = begin;
    while follow < end {
        new_array.push(list[follow]);
        follow = follow + 1;
    }

    new_array
}

fn bubble_sort(arguments: &Vec<String>) {
    if arguments.len() < 3 {
        error(LOW_ARGS_ERROR);
    }

    let list = split_math(&arguments[2], false);
    let mut list = create_list_from_arguments(&list);
    println!("{:?}", list);

    let mut length_list = list.len() - 1;
    let mut current_item: usize = 0;
    while length_list > 0 {
        if length_list != current_item {
            if list[current_item] > list[current_item + 1] {
                let temp_integer = list[current_item];
                list[current_item] = list[current_item + 1];
                list[current_item + 1] = temp_integer;
            }
        }

        if current_item == length_list {
            current_item = 0;
            length_list = length_list - 1;
        }else{
            current_item = current_item + 1;
        }
    }
    println!("{:?}", list);
}

fn insertion_sort(arguments: &Vec<String>){
    if arguments.len() < 3 {
        error(LOW_ARGS_ERROR);
    }

    let list = split_math(&arguments[2], false);
    let mut list = create_list_from_arguments(&list);
    println!("{:?}", list);

    let mut far_item = 1;
    let length_list = list.len();
    let mut current_item = 1;
    let mut reset = false;

    while far_item < length_list {
        if current_item < 1 || reset {
            far_item = far_item + 1;
            current_item = far_item;
            reset = false;
            continue;
        }

        if list[current_item] < list[current_item - 1] {
            let temp_item = list[current_item];
            list[current_item] = list[current_item - 1];
            list[current_item - 1] = temp_item;
            current_item = current_item - 1;
        }else{
            reset = true;
        }
    }
    println!("{:?}", list);
}

fn selection_sort(arguments: &Vec<String>){
    if arguments.len() < 3 {
        error(LOW_ARGS_ERROR);
    }

    let list = split_math(&arguments[2], false);
    let mut list = create_list_from_arguments(&list);
    println!("{:?}", list);

    let max_length = list.len() - 1;
    let mut current_item = 0;
    let mut lowest_item = 0;
    let mut sorted_item = 0;
    
    while sorted_item != max_length {
        if list[lowest_item] > list[current_item] {
            lowest_item = current_item;
        } 

        if max_length == current_item {
            current_item = sorted_item + 1;
            let temp_int = list[sorted_item];
            list[sorted_item] = list[lowest_item];
            list[lowest_item] = temp_int;
            sorted_item = current_item;
            lowest_item = current_item;
        }else{
            current_item = current_item + 1;
        }
    }
    println!("{:?}", list);
}

