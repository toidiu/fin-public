# fin

 Financial porfolio manager.

![alt text](./screenshot.png)

---

### Website
Vue, Typescript, parceljs

#### Requirements
- npm 6.7.0

#### Setup
```
cd website
npm start
```

---

### Postgres Database

#### Requirements
- https://github.com/diesel-rs/diesel/tree/master/diesel_cli
- local postgres instance (https://postgresapp.com)

#### Setup
```
cd service
make run
```

---

### Server
Rust

#### Requirements
- cargo 1.32.0

#### Setup
```
cd service
cargo run
```

---

### IEX
Switch mode by replacing dependency in file `/fin/service/fin/Cargo.toml`.

- *fake data (default)*: the `iex-rs` crate is compiles as `source_debug` by default
which generates deterministic fake prices for stocks. It is a good way to quickly get
started and work without an internet connection.

- *real data*: for real data register for a free account at [iex cloud](https://iexcloud.io)
and replace token in `fin/service/iex-rs/src/lib.rs`.

---

### Running the Application
```
http://localhost:1234/
username: apoorv@toidu.com
password: 123456
```
