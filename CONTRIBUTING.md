# Contributing

## Backend

The backend resides in the `src` directory and is written in
[Rust](https://www.rust-lang.org/) (no special reason, I just wanted to do
something with Rust). In order to run it, just get into that directory and run:

```sh
$ cargon run
```

This should take care of everything and the server will be running at port
`8080`. If you really want to use another port for whatever reason, you can set
the `LLIBRES_PORT` environment variable beforehand.

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

To do.

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
