# Example [warp](https://github.com/seanmonstar/warp) project

An example full stack project using the warp framework, [Diesel ORM](http://diesel.rs/) and a frontend app built with JS.



## Setup

You need the Rust compiler and the cargo package manager installed. The most convenient way to get them is with Rustup:

```bash
curl https://sh.rustup.rs -sSf | sh
```

### Database & Diesel setup

You'll need a PostgreSQL database running and accessible on your network, as well as the PostgreSQL C headers the Rust compiler will link with, available on Debian-based distros inside the `libpq-dev` package:

```bash
sudo apt install postgresql libpq-dev ## or postgresql-10
```

The Diesel CLI (just the PostgreSQL features) is required, install it with:

```bash
cargo install diesel_cli --no-default-features --features postgres  
```

### .env file

You need to setup the following environment variables inside of a `.env` file:

```env
DATABASE_URL
```

## Frontend

```bash
cd app/
```

The frontend was built using [VueJS](https://vuejs.org) and [Parcel](https://parceljs.org) following [this tutorial](https://alligator.io/vuejs/vue-parceljs/). We consume the Rust API using `axios`.

Install the dependencies:

```bash
npm i -g parcel-bundler ## to get the parcel CLI
npm i
```

Launch the development server with hot module reloading:

    parcel index.html
