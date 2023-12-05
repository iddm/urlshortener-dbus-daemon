# urlshortener-dbus-daemon
[![](https://meritbadge.herokuapp.com/urlshortener-dbus-daemon)](https://crates.io/crates/urlshortener-dbus-daemon) [![](https://travis-ci.org/iddm/urlshortener-dbus-daemon.svg?branch=master)](https://travis-ci.org/iddm/urlshortener-dbus-daemon) [![](https://docs.rs/urlshortener-dbus-daemon/badge.svg)](https://docs.rs/urlshortener-dbus-daemon)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


A very simple urlshortener dbus daemon written in Rust. Uses [urlshortener library](https://github.com/iddm/urlshortener-rs).

## Usage (dbus-send)

```bash
$ dbus-send --session --dest=io.crates.urlshortener --type=method_call --print-reply / io.crates.urlshortener.Shorten string:"http://google.ru"
method return time=1533217676.442292 sender=:1.2237 -> destination=:1.2238 serial=3 reply_serial=2
   string "https://is.gd/h5kR5r"
```

## License

This project is [licensed under the MIT license](https://github.com/iddm/urlshortener-dbus-daemon/blob/master/LICENSE).
