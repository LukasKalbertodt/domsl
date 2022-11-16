# `domsl`: DOM(ain) Specific Language

> Not developed anymore!

This was a fun experiment, but I gave up on it. It's not really useful in it's current form.

---

# Old readme:


This is an experimental crate created for usage in another project of mine.
Its goal is to provide a simple and concise syntax to create DOM nodes via `web-sys`.
There are a couple of full blown WASM Rust UI frameworks out there, and a couple of crates that work like this one but create a virtual DOM instead (e.g. [`typed-html`](https://github.com/bodil/typed-html)).
This crate is not a full UI framework and does not use a virtual or shadow DOM; the macro expands to calls to `web-sys` functions only.

The crate is super unstable still.
I haven't even released the first version yet.
If you are interested in this crate, please come back later.
I will hopefully release (and officially announce) the first version in 2019 still.



## Contributing

*I am currently not yet accepting PRs!* As soon as I have released the first version, I welcome all kinds of contributions.

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
