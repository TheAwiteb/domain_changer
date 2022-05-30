# domain_changer

[![Wagmi](https://shields.io/badge/Wagmi-Awiteb-blueviolet?logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPCEtLSBHZW5lcmF0b3I6IEFkb2JlIElsbHVzdHJhdG9yIDI0LjAuMCwgU1ZHIEV4cG9ydCBQbHVnLUluIC4gU1ZHIFZlcnNpb246IDYuMDAgQnVpbGQgMCkgIC0tPgo8c3ZnIHZlcnNpb249IjEuMSIgaWQ9IkxheWVyXzEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiIHg9IjBweCIgeT0iMHB4IgoJIHZpZXdCb3g9IjAgMCAzOTcuNyAzMTEuNyIgc3R5bGU9ImVuYWJsZS1iYWNrZ3JvdW5kOm5ldyAwIDAgMzk3LjcgMzExLjc7IiB4bWw6c3BhY2U9InByZXNlcnZlIj4KPHN0eWxlIHR5cGU9InRleHQvY3NzIj4KCS5zdDB7ZmlsbDp1cmwoI1NWR0lEXzFfKTt9Cgkuc3Qxe2ZpbGw6dXJsKCNTVkdJRF8yXyk7fQoJLnN0MntmaWxsOnVybCgjU1ZHSURfM18pO30KPC9zdHlsZT4KPGxpbmVhckdyYWRpZW50IGlkPSJTVkdJRF8xXyIgZ3JhZGllbnRVbml0cz0idXNlclNwYWNlT25Vc2UiIHgxPSIzNjAuODc5MSIgeTE9IjM1MS40NTUzIiB4Mj0iMTQxLjIxMyIgeTI9Ii02OS4yOTM2IiBncmFkaWVudFRyYW5zZm9ybT0ibWF0cml4KDEgMCAwIC0xIDAgMzE0KSI+Cgk8c3RvcCAgb2Zmc2V0PSIwIiBzdHlsZT0ic3RvcC1jb2xvcjojMDBGRkEzIi8+Cgk8c3RvcCAgb2Zmc2V0PSIxIiBzdHlsZT0ic3RvcC1jb2xvcjojREMxRkZGIi8+CjwvbGluZWFyR3JhZGllbnQ+CjxwYXRoIGNsYXNzPSJzdDAiIGQ9Ik02NC42LDIzNy45YzIuNC0yLjQsNS43LTMuOCw5LjItMy44aDMxNy40YzUuOCwwLDguNyw3LDQuNiwxMS4xbC02Mi43LDYyLjdjLTIuNCwyLjQtNS43LDMuOC05LjIsMy44SDYuNQoJYy01LjgsMC04LjctNy00LjYtMTEuMUw2NC42LDIzNy45eiIvPgo8bGluZWFyR3JhZGllbnQgaWQ9IlNWR0lEXzJfIiBncmFkaWVudFVuaXRzPSJ1c2VyU3BhY2VPblVzZSIgeDE9IjI2NC44MjkxIiB5MT0iNDAxLjYwMTQiIHgyPSI0NS4xNjMiIHkyPSItMTkuMTQ3NSIgZ3JhZGllbnRUcmFuc2Zvcm09Im1hdHJpeCgxIDAgMCAtMSAwIDMxNCkiPgoJPHN0b3AgIG9mZnNldD0iMCIgc3R5bGU9InN0b3AtY29sb3I6IzAwRkZBMyIvPgoJPHN0b3AgIG9mZnNldD0iMSIgc3R5bGU9InN0b3AtY29sb3I6I0RDMUZGRiIvPgo8L2xpbmVhckdyYWRpZW50Pgo8cGF0aCBjbGFzcz0ic3QxIiBkPSJNNjQuNiwzLjhDNjcuMSwxLjQsNzAuNCwwLDczLjgsMGgzMTcuNGM1LjgsMCw4LjcsNyw0LjYsMTEuMWwtNjIuNyw2Mi43Yy0yLjQsMi40LTUuNywzLjgtOS4yLDMuOEg2LjUKCWMtNS44LDAtOC43LTctNC42LTExLjFMNjQuNiwzLjh6Ii8+CjxsaW5lYXJHcmFkaWVudCBpZD0iU1ZHSURfM18iIGdyYWRpZW50VW5pdHM9InVzZXJTcGFjZU9uVXNlIiB4MT0iMzEyLjU0ODQiIHkxPSIzNzYuNjg4IiB4Mj0iOTIuODgyMiIgeTI9Ii00NC4wNjEiIGdyYWRpZW50VHJhbnNmb3JtPSJtYXRyaXgoMSAwIDAgLTEgMCAzMTQpIj4KCTxzdG9wICBvZmZzZXQ9IjAiIHN0eWxlPSJzdG9wLWNvbG9yOiMwMEZGQTMiLz4KCTxzdG9wICBvZmZzZXQ9IjEiIHN0eWxlPSJzdG9wLWNvbG9yOiNEQzFGRkYiLz4KPC9saW5lYXJHcmFkaWVudD4KPHBhdGggY2xhc3M9InN0MiIgZD0iTTMzMy4xLDEyMC4xYy0yLjQtMi40LTUuNy0zLjgtOS4yLTMuOEg2LjVjLTUuOCwwLTguNyw3LTQuNiwxMS4xbDYyLjcsNjIuN2MyLjQsMi40LDUuNywzLjgsOS4yLDMuOGgzMTcuNAoJYzUuOCwwLDguNy03LDQuNi0xMS4xTDMzMy4xLDEyMC4xeiIvPgo8L3N2Zz4K&style=flat)](https://wagmi.bio/Awiteb)

  <a href="https://github.com/TheAwiteb/domain_changer/actions">
    <img src="https://github.com/TheAwiteb/domain_changer/workflows/Continuous%20integration/badge.svg">
  </a>
  <a href="https://docs.rs/domain_changer/latest/domain_changer/">
    <img src="https://img.shields.io/badge/docs-docs.rs-orange">
  </a>
  <a href="https://crates.io/crates/domain_changer">
    <img src="https://img.shields.io/crates/v/domain_changer.svg">
  </a>
 <a href="https://rust-reportcard.xuri.me/report/github.com/TheAwiteb/domain_changer">
    <img src="https://rust-reportcard.xuri.me/badge/github.com/TheAwiteb/domain_changer" alt="Rust report">
  </a>

Rust library that helps you to **change the domain of the link to another domain**, the library helps with privacy.
It can be used to change the domain of sites that do not care about privacy to another that does.


## Examples
### Parse String
Parse the string and convert the old urls to new one
> Note: You can change the domains as you want
```rust
use domain_changer::parse_string;
use domain_changer::types::Config;

let text: String = "Wellcome to my youtube channel: https://www.youtube.com/channel/UCeRbJsc8cl7xBwT3jIxaAdg And my twitter is: twitter.com/Awiteb".to_string();
let config: Config = Config::default();
assert_eq!(parse_string(&config, text),
           "Wellcome to my youtube channel: https://piped.kavin.rocks/channel/UCeRbJsc8cl7xBwT3jIxaAdg And my twitter is: https://nitter.net/Awiteb".to_string()
);
```

### Extract Domains
You can extract domains from string if any (Just the domain you add it to config)
```rust
use domain_changer::extract_old_domains;
use domain_changer::types::{Config, Domain};

let config: Config = Config::default();
let text: String = String::from(
    "Hi i hate youtube.com and https://twitter.com what about you?"
);
assert_eq!(
    extract_old_domains(&config, text),
    vec![
        &Domain::try_from(("https://youtube.com/", "https://piped.kavin.rocks/")).unwrap(),
        &Domain::try_from(("https://twitter.com/", "https://nitter.net/")).unwrap()
    ]
);
```

### Serialize and Deserialize (json feature)
Serialize and deserialize from/to json in Domain struct and Config
> Note: Need `json` feature
#### Domain
```rust
use domain_changer::types::{Domain, ToFromJson};

let domain: Domain = Domain::try_from(("https://twitter.com/", "https://nitter.net/")).unwrap();
assert_eq!(domain.to_json().unwrap(), "{\"old\":\"https://twitter.com/\",\"new\":\"https://nitter.net/\"}");
assert_eq!(Domain::from_json("{\"old\":\"https://twitter.com/\",\"new\":\"https://nitter.net/\"}").unwrap(), domain);
assert!(Domain::from_json("{\"old\":\"twitter.com/\",\"new\":\"nitter.net/\"}").is_err());
```
#### Config
```rust
use domain_changer::types::{Config, Domain, ToFromJson};

let config: Config = Config::new(vec![Domain::try_from(("https://twitter.com/", "https://nitter.net/")).unwrap()]);
assert_eq!(
    config.to_json().unwrap(),
    "{\"domains\":[{\"old\":\"https://twitter.com/\",\"new\":\"https://nitter.net/\"}]}".to_string()
);

assert_eq!(
    Config::from_json("{\"domains\":[{\"old\":\"https://twitter.com/\",\"new\":\"https://nitter.net/\"}]}").unwrap(),
    config
);
```

## Donating

| Currency                | Address                                          |
|-------------------------|--------------------------------------------------|
| Binance **BNB BEP20**   | ```0xD89c146B03B72191be91064D313610981dCAF6d4``` |
| USD Coin **USDC BEP20** | ```0xD89c146B03B72191be91064D313610981dCAF6d4``` |
| Bitcoin **BTC**         | ```bc1q0ltmqmsc4qs740ssyf9k9jq99nwxtqu8aupmdj``` |
| Bitcoin Cash **BCH**    | ```qrpm6zyte3d4z2u9r24l04m3havc2wd9vgqlz8sjgr``` |

## Contributors
<a href="https://github.com/TheAwiteb/domain_changer/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=TheAwiteb/domain_changer" />
</a>

## License
The [GNU Affero General Public](https://www.gnu.org/licenses/agpl-3.0.en.html) License is a free, 
copyleft license for software and other kinds of works, specifically designed to ensure cooperation with the community in the case of network server software.
