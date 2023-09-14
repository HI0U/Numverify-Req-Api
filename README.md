> Numverify

```
NumVerify offers a complete yet simple RESTful JSON API for the validation of national and international phone numbers, as well as information lookup for a total of 232 countries worldwide.

Requested numbers are processed in real-time, cross-checked with the latest international numbering plan databases, and returned in a convenient JSON format enriched with useful data about the carrier, geographical location, and line type.

Integrating the numverify API into your application will allow you to verify the validity of phone numbers at the point of entry, protecting you from fraud and increasing the opportunities for quality contact.
```
I was practicing Rust and decided to upload this script.


> Run Code
```Bash
1 - cd /Code-projects/Numverify-Req-Api/src 
2 - cargo clean
3 - cargo run main.rs
``` 

Make sure to review and understand the script before running it, and also to change the fields like the Number, etc., to the ones you want !!!!

```rust
let api_key = "Key";
let phone_number = "Number"; // Ex 787
let country_code = "Code"; // Ex US
```
