# domain_changer
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
