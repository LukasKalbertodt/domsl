# `domsl`


TODO.


## Contributing

TODO

**Running the tests**

As the tests require a full JS environment with DOM, you unfortunately can't run the tests as you would run other tests.
But it's not too difficult, either!
You need `wasm-pack` and either Firefox, Chrome or Safari.
You can then simply execute this:

```
wasm-pack test --headless --firefox  # or --chrome or --safari
```

This command should install missing tools, prepare everything and finally run all tests in a headless browser.



---

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
