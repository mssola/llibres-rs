# Contributing

## Backend

The backend resides in the `src` directory and is written in
[Rust](https://www.rust-lang.org/) (no special reason, I just wanted to do
something with Rust). In order to run it, you have to make sure that the
database is ready for it. Thus, if it's your first time, you will need to instal
`diesel` like so:

```sh
$ cargo install diesel_cli
```

After that, setup the database with `diesel`:

```sh
$ diesel setup
```

Notice that the database url is set on the `.env` file. Otherwise, you can pass
an alternative `DATABASE_URL` value. Finally, you can just run the code with
`cargo`:

```sh
$ cargon run
```

This should take care of everything and the server will be running at port
`8080`. If you really want to use another port for whatever reason, you can set
the `LLIBRES_PORT` environment variable beforehand.

You can then seed the database with test data by running:

```sh
# Or wherever you backend server is running.
$ ruby tests/fixtures/seed.rb tests/fixtures/test.yml http://localhost:8080
```

## Frontend

The frontend resides in the `app` directory and is basically a
[Vue.js](https://vuejs.org/) application. In order to run it, first make sure
that the server is running, and then run from the root directory:

```sh
$ yarn serve
```

This will run the dev server, which will automatically build whatever changes
you are doing and re-render the application. After that, you can just open up a
web browser and visit `localhost:5000`.

## Running tests

I am basically only running end-to-end tests, since this application is small
and easy enough and I just need to test a couple of workflows. In order to
achieve this, you need to start the backend and fill the database with some test
data. One easy way to achieve this is to create a `test` database and feed the
test data that I have in `tests/fixtures`. This can be done with:

```sh
# Or whatever name you want to give to the database.
$ DATABASE_URL=postgres://localhost/llibres-test diesel setup
$ DATABASE_URL=postgres://localhost/llibres-test cargo run
```

And then you can start the [cypress](https://www.cypress.io/) process from
another terminal:

```sh
$ yarn test:e2e
```

From there you should be able to run tests from the web GUI.

These tests will be automatically run for every pull request and commit in the
Github actions set up for this project..

## Issue reporting

I'm using Github in order to host the code. Thus, in order to report issues you
can do it on its [issue tracker](https://github.com/mssola/mihi/issues). A
couple of notes on reports:

- Check that the issue has not already been reported or fixed in the `main`
  branch.
- Try to be concise and precise in your description of the problem.
- Provide a step by step guide on how to reproduce this problem.
- Provide the version you are using (git commit SHA).

## Pull requests

- Write a [good commit message](https://chris.beams.io/posts/git-commit/).
- Run the tests.
- Update the [changelog](./CHANGELOG.md) file (if relevant).
- Open a pull request with _only_ one subject and a clear title and description.
  Refrain from submitting pull requests with tons of different unrelated
  commits.

## Deployment

To do.
