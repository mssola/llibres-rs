##
# Backend

FROM rust:1.58 as backend

WORKDIR /usr/src/llibres
COPY . .

RUN cargo install --path .

##
# Frontend

FROM node:12-alpine as frontend

WORKDIR /usr/src/llibres
COPY package.json yarn.lock ./
RUN yarn

COPY . ./
RUN yarn build

##
# Combining all of the above.

FROM alpine:3

RUN mkdir -p /srv/llibres/bin /srv/llibres/public
COPY --from=backend /usr/local/cargo/bin/llibres /srv/llibres/bin/llibres
COPY --from=frontend /usr/src/llibres/dist /srv/llibres/public

EXPOSE 8080

CMD ["/srv/llibres/bin/llibres"]
