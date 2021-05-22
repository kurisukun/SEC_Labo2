use std::error;
use std::fmt;
use strum::EnumMessage;
use strum_macros;

#[derive(PartialEq, Debug, strum_macros::EnumMessage)]
pub enum Errors {
    #[strum(message = "Error: Username/password incorrect")]
    LoginError,
    #[strum(message = "Token incorrect")]
    TokenError,
    #[strum(message = "This username is already taken")]
    EmailUsedError,
    #[strum(message = "You did not enter a correct email")]
    EmailFormatError,
    #[strum(message = "You did not enter a correct password (read the help for more information)")]
    PasswordFormatError,
    #[strum(message = "Your password is not strong enough")]
    TooWeakPasswordError,
    #[strum(message = "Something went wrong with user creation")]
    CreateUserError,
    #[strum(message = "Something went wrong when getting user")]
    GetUserError,
    #[strum(message = "Something went wrong with user update")]
    UpdateUserError,
    #[strum(message = "Something went wrong when checking if user exists")]
    UserExistsError,
    #[strum(message = "Email for password reset has not been sent")]
    EmailNotSentError,
    #[strum(message = "Token for password reset is not valid anymore")]
    EmailTimeoutError,
    #[strum(message = "Token for password reset is wrong")]
    EmailWrongToken,
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_message().unwrap())
    }
}

impl error::Error for Errors {
    fn description(&self) -> &str {
        self.get_message().unwrap()
    }
}
