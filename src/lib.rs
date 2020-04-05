///Format the two strings together
///  # Examples
/// 
/// ```
/// extern crate string_format;
/// use string_format::*;
/// let hello = String::from("hello {}");
/// let world = String::from("world");
/// assert_eq!("hello world".to_string(), string_format(hello, world));
/// ```
pub fn string_format(string : String, text : String) -> String {
    let mut result = string.clone();
    //loop throught the String to find the first pair of brackets
    match string.find("{}") {
        Some(offset) => {
            result.replace_range(offset..offset+2, &text);
            result
        }
        None => { string }
    }
}

///work like the format! macro but with a String
/// # Example
/// ```
///  # #[macro_use] extern crate string_format;
///  # fn main(){
/// use string_format::*;
/// 
/// let hello = String::from("hello {} {}");
/// let cruel = String::from("cruel");
/// let world = String::from("world");
/// assert_eq!("hello cruel world".to_string(), string_format!(hello, cruel, world));
/// # }
/// ```
#[macro_export]
macro_rules! string_format {
    ($($arg:expr),*) => {{
        let text = String::from("{}");
        $(
            let text = string_format(text, $arg);
        )*
        text
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_format() {
        assert_eq!("hello world".to_string(), string_format("hello {}".to_string(), "world".to_string()));
    }

    #[test]
    fn test_string_format_macro(){
        let hello = String::from("hello {} {}");
        let cruel = String::from("cruel");
        let world = String::from("world");
        assert_eq!("hello cruel world".to_string(), string_format!(hello, cruel, world));
    }
}