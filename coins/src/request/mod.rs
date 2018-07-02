use curl::easy::Easy;
use std::string::String;
use std::str;

pub fn get(url: &String) -> String{
    let mut easy = Easy::new();
    easy.url(url).unwrap();

    let mut response: String = String::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            response.push_str(str::from_utf8(data).unwrap());
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    };

    response
}
