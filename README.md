## SEC Labo2



**Author:** Chris Barros Henriques



### Setup

No particular setup is needed, the file db.sqlite is automatically created when we launch the app. Obviously, you will first need to register at least one time to entirely test it.

One thing, if you want to use the reset password feature with the email, you need to put your credentials in reset_password.rs (SMTP_USER and SMTP_PASS). 



### Architecture

Every time an user is added to the database, his password is hashed using argon2. 

The lab is uses on [StructOpt](https://github.com/TeXitoi/structopt) to have a CLI app. 



### Usage



#### Registration

The registration is done using the flag `-r`. By default, the account has not the two factors activated, so a login is needed to activate it.

**With long flags:**

```bash
./sec_labo2 --username chris.barroshenriques.heig@gmail.com --password aA1_lpa23B --register
```

**With short flags:**

```bash
./sec_labo2 -u chris.barroshenriques.heig@gmail.com -p aA1_lpa23B -r
```



#### Authentication

There is two different types of authentication:

- Simple authentication with username and password
- Two factors authentication with username, password and a google token produced with Google Authenticator. When enabled, the app will ask for the google token to be entered to validate the login.



**Simple login:**

```bash
./sec_labo2 --username chris.barroshenriques.heig@gmail.com --password aA1_lpa23B
```

```bash
./sec_labo2 -u chris.barroshenriques.heig@gmail.com -p aA1_lpa23B
```



**Login with two factors activation/desactivation:**

This will output a link which will display a QR code which can be used and scanned. It will then be added to the Google Authentificator App.

```bash
./sec_labo2 --username chris.barroshenriques.heig@gmail.com --password aA1_lpa23B --two_factors
```

```bash
./sec_labo2 -u chris.barroshenriques.heig@gmail.com -p aA1_lpa23B -t
```



**Login with reset of the password:**

```bash
./sec_labo2 --username chris.barroshenriques.heig@gmail.com --password aA1_lpa23B --reset_password
```

```bash
./sec_labo2 -u chris.barroshenriques.heig@gmail.com -p aA1_lpa23B --reset_password
```



**Help:**

```
./sec_labo2 -h
sec_labo2 0.1.0
SEC Labo2: two-factor authentication program using Google Authenticator

USAGE:
    sec_labo2 [FLAGS] [OPTIONS]

FLAGS:
    -h, --help              Prints help information
    -r, --register          Argument to proceed a registration
        --reset-password    Argument to reset the password
    -t, --two-factors       Argument to enable/disable the two factors authentication
    -V, --version           Prints version information

OPTIONS:
    -p, --password <password>    Verifies the syntax of password and given criterium:
                                   -the size of the password is between 10-20 chars
                                   -has at least one lowercase char
                                   -has at least one uppercase char
                                   -has at least one special char in the given list: .?!@_-#$%^&*+
                                  [default: ]
    -u, --username <username>     [default: ]
```



### Tests

It happends that some tests that have use of the database fail, it seems the tests are done too quickly for the database to handle it correctly. So to launch the tests, it's advisable to run them one by one.



Every tests using Google Authenticator has not been done since we would have to mock the functions needed. Same for the ones that test the email feature.



**Bonus done:** Database SQLite , QR Code with Google Authenticator, Command Line Interface and sending an email for the password reset