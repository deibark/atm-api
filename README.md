## How to run it

Install rust language

```
$ curl https://sh.rustup.rs -sSf | sh
```

Switch to a nightly branch

```
$ rustup default nightly
$ rustup update && cargo update
```


To start the api server, make sure you are in the project directory and run

```
$ cargo run
```

When server is started, there should be an endpoint available at

```
http://localhost:8000/atm/withdraw/<amount>
```

Replace the <amount> with the amount you want to withdraw.

To run the tests. make sure you are in project directory and run

```
$ cargo test
```
