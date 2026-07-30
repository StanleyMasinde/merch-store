## Merch Store 
Merch store is a sample Rust web application demonstrating the usage of my Daraja Crate, Axum, and Toasty ORM

## Running
1. Clone this repo. 
2. Copy `config.toml.example` to `config.toml`
3. Update config.toml with your Daraja credentials. 
4. Run the app with `cargo run` it will start the server in <localhost:3000>
5. You can test it with this curl command
```shell
curl  "localhost:3000/pay" --json '{"phone_number": <phone number to prompt>, "amount": 1}'
```
That's it for now.
