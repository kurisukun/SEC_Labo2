use lazy_static::lazy_static;
use regex::Regex;
use regex::RegexSet;


/// Verifies the syntax of username (email)
///
/// Returns true if the email respectes the given regex, false otherwise
pub fn syntatic_validation_username(username: &str) -> bool{
    lazy_static!{
        //found here: 
        static ref REGEX_USERNAME: Regex = Regex::new(r"^(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}$").unwrap(); 
    }

    REGEX_USERNAME.is_match(username)
}

/// Verifies the syntax of password and given criterium:
///     -the size of the password is between 8-20 chars
///     -has at least one lowercase char
///     -has at least one uppercase char
///     -has at least one special char in the given list: .?!@_\-#$%^&*+
///
/// Returns true if all criterium are respected, false otherwise
pub fn syntatic_validation_password(password: &str) -> bool{
    lazy_static!{
        static  ref REGEX_PASSWORD: RegexSet = RegexSet::new(&[
            r"[a-z]+",
            r"[A-Z]+",
            r"[0-9]+",
            r"[.?!@_\-#$%^&*+]+",
            r"^[[a-zA-Z0-9].?!@_\-#$%^&*]{8,20}$"
        ]).unwrap();
    }

    let matches : Vec<_> = REGEX_PASSWORD.matches(password).into_iter().collect();

    matches.len() == REGEX_PASSWORD.len()
}


#[cfg(test)]
mod test{

    use super::*;

    
    #[test]
    fn valid_usernames(){
        assert!(syntatic_validation_username("chris.barroshenriques.heig@gmail.com"));
        assert!(syntatic_validation_username("chris.barroshenriques@heig-vd.ch"));
        assert!(syntatic_validation_username("test333@hotmail.fr"));
        assert!(syntatic_validation_username("test._-+%0123456789@hotmail.com"));
    } 

    #[test]
    fn invalid_usernames_wrong_format(){
        assert!(!syntatic_validation_username("chris.barroshenriques@gmail.t"));
        assert!(!syntatic_validation_username("chris.barroshenriques@gmail.testt"));
        assert!(!syntatic_validation_username("chris.barroshenriques@gmail.testtest"));
    }

    #[test]
    fn invalid_usernames_missing_parts(){
        assert!(!syntatic_validation_username("chris.barroshenriques.heig@gmail"));
        assert!(!syntatic_validation_username("@gmail.com"));
        assert!(!syntatic_validation_username("chris.barroshenriques.heig.com"));
    } 

    #[test]
    fn valid_passwords(){
        assert!(syntatic_validation_password("aA1_lpa2"));
        assert!(syntatic_validation_password("BaAA1$._lpa2"));
        assert!(syntatic_validation_password("aA1_lpa222$_-^aaaaaa"));
    }

    #[test]
    fn invalid_passwords_length(){
        //has not 8 chars minimum
        assert!(!syntatic_validation_password("aA1_lpa"));
        //has more than 20 chars
        assert!(!syntatic_validation_password("aA1_lpa222$_-^aaaaaaA"));
    }

    #[test]
    fn invalid_passwords_miss_one_criterium(){
        //miss at least one special char
        assert!(!syntatic_validation_password("aA1lpaA"));
        //miss at least one number
        assert!(!syntatic_validation_password("aA_lpaA"));
        //miss at least on lowercase char
        assert!(!syntatic_validation_password("AA1lPAA"));
        //miss at least one uppercase char
        assert!(!syntatic_validation_password("aa1lpaa"));
    }
}