use lazy_static::lazy_static;
use regex::Regex;

pub fn validate_username(username: &str) -> bool{
    lazy_static!{
        //found here: 
        static ref REGEX_USERNAME: Regex = Regex::new(r"^(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}$").unwrap(); 
    }

    REGEX_USERNAME.is_match(username)
}


#[cfg(test)]
mod test{

    use super::*;

    
    #[test]
    fn valid_usernames(){
        assert!(validate_username("chris.barroshenriques.heig@gmail.com"));
        assert!(validate_username("chris.barroshenriques@heig-vd.ch"));
        assert!(validate_username("test333@hotmail.fr"));
        assert!(validate_username("test._-+%0123456789@hotmail.com"));
    } 

    #[test]
    fn invalid_usernames_wrong_format(){
        assert!(!validate_username("chris.barroshenriques@gmail.t"));
        assert!(!validate_username("chris.barroshenriques@gmail.testt"));
        assert!(!validate_username("chris.barroshenriques@gmail.testtest"));
    }

    #[test]
    fn invalid_usernames_missing_parts(){
        assert!(!validate_username("chris.barroshenriques.heig@gmail"));
        assert!(!validate_username("@gmail.com"));
        assert!(!validate_username("chris.barroshenriques.heig.com"));
    } 
}