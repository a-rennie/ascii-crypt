# ascii-crypt
really basic encryption program to figure out how rust works
## how to use
Calling either the `encode(&str)` or the `decode(&str)` function returns a `Result<String, String>`.
Calling `encode()` should return an `Ok(String)` with each char of the input `&str` replaced with a three digit ascii value,
and `decode` returns the inverse (each three digit segment replaced with an ascii character).\
\
`main.rs` also includes a basic demonstration of the program
## examples
```rust
ascii_crypt::encode("example input");
```
returns `Ok("049050051010101120097109112108101032105110112117116")`
```rust
ascii_crypt::encode("049050051010101120097109112108101032111117116112117116");
```
returns `Ok("example output")`
### error examples
```rust
ascii_crypt::encode("emojis don't work > 🦀") {
```
returns `Err(EncodeError::CharNotAscii('🦀'))`
```rust
ascii_crypt::decode("i am definitely a number")
```
returns `Err(DecodeError::UnexpectedCharIn("i a"))` (note that in order for this error to occur, `input.len()` must be divisible by 3, otherwise the error below occurs)
```rust
ascii_crypt::decode("07334")
```
returns `Err(InvalidLength)`