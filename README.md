# Example [warp](https://github.com/seanmonstar/warp) project

An example full stack project using the warp framework, Diesel ORM and a JS frontend.

## Setup .env file

You need to setup the following environment variables inside of a `.env` file:

```
DATABASE_URL
```

## Frontend

The frontend was built using [VueJS](https://vuejs.org) and [Parcel](https://parceljs.org) following [this tutorial](https://alligator.io/vuejs/vue-parceljs/). We consume the Rust API using `fetch`.

Install the dependencies:

    npm i


Launch the development server with hot module reloading:

    parcel index.html
