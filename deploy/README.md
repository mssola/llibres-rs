# Deploying this

This directory contains files that might help you deploy this on production.

## Docker image

The Docker image is to be built from the project root as follows:

```bash
$ docker build -f deploy/Dockerfile --tag <name>:<version> .
```

You shouldn't have to do this manually, it's recommended to have this built as
part of your CI workflow. Take a look at the `.github/workflows/ci.yml` pipeline
for inspiration.

## Docker compose

You can have a simple deployment up and running by running the
`deploy/docker-compose.yml` file. This file expects a couple of secrets to be
stored as files:

- `deploy/secrets/db_password.txt`: the password for the database.
- `deploy/secrets/db_url.txt`: the database url as expected by
  [diesel.rs](https://diesel.rs/).

Other than that, this `docker-compose.yml` file will create two directories:

- `data`: where the database data will be stored.
- `public`: where the public static files will be populated upon starting the
  `web` container. Point to this directory as the root when using
  NGinx/Apache/whatever.

After this, simply:

```bash
$ docker-compose up -d
```

And you should have the website available at the port `8080`. At this point you
may want to put NGinx or any other load balancer in front so the site can be
exposed with the proper domain name and with all the right SSL certificates. You
can find countless tutorials on the internet about this,
