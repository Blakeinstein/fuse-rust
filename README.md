<h1 align="center">Fuse-rust</h1>
<p align="center">
    <a href="https://crates.io/crates/fuse-rust"><img src="https://img.shields.io/crates/v/fuse-rust.svg"/></a>
    <img src="https://github.com/Blakeinstein/fuse-rust/workflows/CI/badge.svg" />
</p>

## What is Fuse?
Fuse is a super lightweight library which provides a simple way to do fuzzy searching.

Fuse-RS is a port of https://github.com/krisk/fuse-swift written purely in rust.

## Usage

<img src="/.github/Demo.gif" align="right" width="400px"/>

An example of a real use case, a search bar made using [iced](https://github.com/iced-rs/iced) is also available.

Try it using 
```shell
cargo run --package search_bar
```
> Check all available examples and their source code [here.](/examples/)


### Async
Use the feature flag "async" to also be able to use async functions.
```toml
fuse-rust = { version = ..., features = ["async"]}
```

#### Initializing

The first step is to create a fuse object, with the necessary parameters. Fuse::default, returns the following parameters.
```rust
Fuse::default() = Fuse{
    location: 0, // Approx where to start looking for the pattern
    distance: 100, // Maximum distance the score should scale to
    threshold: 0.6, // A threshold for guess work
    max_pattern_length: 32, // max valid pattern length
    is_case_sensitive: false,
    tokenize: false, // the input search text should be tokenized
}
```
For how to implement individual searching operations, check the [examples.](/examples/)

## Options

As given above, Fuse takes the following options

- `location`: Approximately where in the text is the pattern expected to be found. Defaults to `0`
- `distance`: Determines how close the match must be to the fuzzy `location` (specified above). An exact letter match which is `distance` characters away from the fuzzy location would score as a complete mismatch. A distance of `0` requires the match be at the exact `location` specified, a `distance` of `1000` would require a perfect match to be within `800` characters of the fuzzy location to be found using a 0.8 threshold. Defaults to `100`
- `threshold`: At what point does the match algorithm give up. A threshold of `0.0` requires a perfect match (of both letters and location), a threshold of `1.0` would match anything. Defaults to `0.6`
- `maxPatternLength`: The maximum valid pattern length. The longer the pattern, the more intensive the search operation will be. If the pattern exceeds the `maxPatternLength`, the `search` operation will return `nil`. Why is this important? [Read this](https://en.wikipedia.org/wiki/Word_(computer_architecture)#Word_size_choice). Defaults to `32`
- `isCaseSensitive`: Indicates whether comparisons should be case sensitive. Defaults to `false`

<br clear="right"/>