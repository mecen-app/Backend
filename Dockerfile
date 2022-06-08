FROM rust
ARG mango-api-key
ARG mango-client-id
ARG sentry-dsn
ENV MANGO_API_KEY=$mango-api-key
ENV MANGO_CLIENT_ID=$mango-client-id
ENV ROCKET_SENTRY_DSN=$sentry-dsn
WORKDIR /usr/src/mecen_backend
COPY . .
RUN cargo build
CMD ["cargo", "run"]
