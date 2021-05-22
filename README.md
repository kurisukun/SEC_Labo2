## SEC Labo2



**Author:** Chris Barros Henriques



### Setup

No particular setup is needed, the file db.sqlite is automatically created when we launch the app.



### Usage

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





