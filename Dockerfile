FROM rust
ARG mango_api_key
ARG mango_client_id
ARG sentry_dsn
ARG database_url
ENV MANGO_API_KEY=$mango_api_key
ENV MANGO_CLIENT_ID=$mango_client_id
ENV ROCKET_SENTRY_DSN=$sentry_dsn
ENV DATABASE_URL=$database_url
WORKDIR /usr/src/mecen_backend
COPY . .
RUN cargo build
CMD ["cargo", "run"]
