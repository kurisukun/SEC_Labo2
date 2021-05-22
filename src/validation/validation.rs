use lazy_static::lazy_static;
use regex::Regex;
use regex::RegexSet;
use zxcvbn::zxcvbn;

/// Verifies the syntax of username (email)
///
/// Returns true if the email respectes the given regex, false otherwise
pub fn syntatic_validation_username(username: &str) -> bool {
    lazy_static! {
        static ref REGEX_USERNAME: Regex =
            Regex::new(r"^(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}$").unwrap();
    }
    REGEX_USERNAME.is_match(username)
}

/// Verifies the syntax of password and given criterium:
///     -the size of the password is between 10-20 chars
///     -has at least one lowercase char
///     -has at least one uppercase char
///     -has at least one special char in the given list: .?!@_\-#$%^&*+
///
/// Returns true if all criterium are respected, false otherwise
pub fn syntatic_validation_password(password: &str) -> bool {
    lazy_static! {
        static ref REGEX_PASSWORD: RegexSet = RegexSet::new(&[
            r"[a-z]+",
            r"[A-Z]+",
            r"[0-9]+",
            r"[.?!@_\-#$%^&*+]+",
            r"^[[a-zA-Z0-9].?!@_\-#$%^&*]{10,20}$"
        ])
        .unwrap();
    }

    let matches: Vec<_> = REGEX_PASSWORD.matches(password).into_iter().collect();
    matches.len() == REGEX_PASSWORD.len()
}

/// Verifies the syntax of google token 
///
/// Returns true if the token respectes the given regex, false otherwise
pub fn syntatic_validation_google_token(token: &str) -> bool {
    lazy_static! {
        static ref REGEX_TOKEN: Regex = Regex::new(r"^(\d){6}$").unwrap();
    }
    REGEX_TOKEN.is_match(token)
}

/// Verifies the syntax of uuid
///
/// Returns true if the uuid respectes the given regex, false otherwise
pub fn syntatic_validation_uuid(uuid: &str) -> bool {
    lazy_static! {
        static ref REGEX_UUID: Regex = Regex::new(
            r"^[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}$"
        )
        .unwrap();
    }
    REGEX_UUID.is_match(uuid)
}

/// Verfies given password is secure enough based on zxcvbn library. Any password with a score less than 3 is considered weak
///
/// Returns true if the password is secure enough, false otherwise
pub fn is_secure_password(password: &str) -> bool {
    let estimate = zxcvbn(password, &[]).unwrap();
    estimate.score() >= 3
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn valid_usernames() {
        assert!(syntatic_validation_username(
            "chris.barroshenriques.heig@gmail.com"
        ));
        assert!(syntatic_validation_username(
            "chris.barroshenriques@heig-vd.ch"
        ));
        assert!(syntatic_validation_username("test333@hotmail.fr"));
        assert!(syntatic_validation_username(
            "test._-+%0123456789@hotmail.com"
        ));
    }

    #[test]
    fn invalid_usernames_wrong_format() {
        assert!(!syntatic_validation_username(
            "chris.barroshenriques@gmail.t"
        ));
        assert!(!syntatic_validation_username(
            "chris.barroshenriques@gmail.testt"
        ));
        assert!(!syntatic_validation_username(
            "chris.barroshenriques@gmail.testtest"
        ));
    }

    #[test]
    fn invalid_usernames_missing_parts() {
        assert!(!syntatic_validation_username(
            "chris.barroshenriques.heig@gmail"
        ));
        assert!(!syntatic_validation_username("@gmail.com"));
        assert!(!syntatic_validation_username(
            "chris.barroshenriques.heig.com"
        ));
    }

    #[test]
    fn valid_passwords() {
        assert!(syntatic_validation_password("aA1_lpa23B"));
        assert!(syntatic_validation_password("BaAA1$._lpa2"));
        assert!(syntatic_validation_password("aA1_lpa222$_-^aaaaaa"));
    }

    #[test]
    fn invalid_passwords_length() {
        //has not 10 chars minimum
        assert!(!syntatic_validation_password("aA1_lpaaa"));
        //has more than 20 chars
        assert!(!syntatic_validation_password("aA1_lpa222$_-^aaaaaaA"));
    }

    #[test]
    fn invalid_passwords_miss_one_criterium() {
        //miss at least one special char
        assert!(!syntatic_validation_password("aA1lpaAaA1"));
        //miss at least one number
        assert!(!syntatic_validation_password("a_A_lpaA.a"));
        //miss at least on lowercase char
        assert!(!syntatic_validation_password("AA1LPAA?AA"));
        //miss at least one uppercase char
        assert!(!syntatic_validation_password("a_a?1lpaaa"));
        //miss almost every critera
        assert!(!syntatic_validation_password("1234"));
    }

    #[test]
    fn valid_google_token(){
        assert!(syntatic_validation_google_token("123456"));
        assert!(syntatic_validation_google_token("111111"));
    }

    #[test]
    fn invalid_google_token_length(){
        assert!(!syntatic_validation_google_token(""));
        assert!(!syntatic_validation_google_token("12345"));
        assert!(!syntatic_validation_google_token("1234567"));
    }

    #[test]
    fn invalid_google_token_not_only_numbers(){
        assert!(!syntatic_validation_google_token("1234S6"));
        assert!(!syntatic_validation_google_token("test"));
    }

    #[test]
    fn secured_passwords() {
        assert!(is_secure_password("aA1_lpa222$_-^aaaaaa"));
        assert!(is_secure_password("aZ$#3^'la"));
        assert!(is_secure_password("MyNameIsBob123"));
    }

    #[test]
    fn not_secured_password() {
        //This password respects all criterium for syntax but is not a good password
        assert!(!is_secure_password("Pa$$w0rD"));
        assert!(!is_secure_password("test1234"));
        assert!(!is_secure_password("password123456789"));
        //Too short
        assert!(!is_secure_password("5_abcPq2"));
    }
}
