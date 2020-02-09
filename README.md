# barrido

![Crates.io](https://img.shields.io/crates/v/barrido)
![Crates.io](https://img.shields.io/crates/l/barrido/0.1.0)


Console utility to find web application paths.

Still in alpha version.

## Examples

Multiple URLs bruteforcing:

```
$ cat urls.txt
https://target.a.com
https://target.b.com
https://target.c.com
$ barrido urls.txt wordlists/quickhits.txt -se | tee curious_paths.txt
https://target.c.com/manager/ 401
https://target.a.com/.git 200
https://target.b.com/phpinfo 200
```

Single URL bruteforcing with scraper:
```
$ barrido https://scrapabble.com wordlists/raft-small-directories-lowercase.txt --scraper
/index
/help
/customers/faqs
/webmaster/login
/api/js/retrieve_session_token
```

## Usage

```
$ ./target/release/barrido -h
barrido 0.1
Discover them all!

USAGE:
    barrido [FLAGS] [OPTIONS] <url> <wordlist>

FLAGS:
    -l, --body-length         Show the discovered paths with the response code
    -e, --expand-path         Return paths with the complete url
        --follow-redirects    Follow HTTP redirections
    -h, --help                Prints help information
    -k, --insecure            Allow insecure connections when using SSL
    -p, --progress            Show the progress of requests
        --scraper             Scrap for new paths in responses
    -s, --status              Show the discovered paths with the response code
    -V, --version             Prints version information
    -v                        Verbosity

OPTIONS:
    -H, --header <header>...               Headers to send in request
        --invalid-codes <invalid-codes>    Response codes which are invalid
        --invalid-regex <invalid-regex>    Regex to match invalid responses
        --exact-length <length>            Exact length of responses
        --max-length <max-length>          Maximum length in responses
        --min-length <min-length>          Minimum length in responses
        --no-exact-length <no-length>      Exact length of invalid responses
    -o, --out-file <out-file>              File to write results (json format)
    -x, --proxy <proxy>                    Specify proxy in format: http[s]://<host>[:<port>]
    -t, --threads <threads>                Number of threads [default: 10]
        --timeout <timeout>                HTTP requests timeout [default: 10]
    -A, --user-agent <user-agent>          Set custom User-Agent [default: barrido]
        --valid-codes <valid-codes>        Response codes which are valid [default: 200,204,301,302,307,401,403]

ARGS:
    <url>         url to load
    <wordlist>    list of paths
```


## Installation

From crates.io:
```
cargo install barrido
barrido -h
```

From source:
```
git clone https://gitlab.com/Zer1t0/barrido.git
cd ./barrido
cargo build --release
./target/release/barrido -h
```

## Features

* Single URL bruteforcing
* Multiple URLs bruteforcing
* Scraper discover
* Response filter based on:
    + Status code
    + Regex
    + Length
* No disturbing banner displayed at execution init