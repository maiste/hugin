
# Stage to build the file
FROM ekidd/rust-musl-builder:stable as builder
RUN USER=root cargo new hugin
WORKDIR ./hugin
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm -rf src/*
ADD . ./
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/hugin*
RUN cargo build --release

# Create light image with exec only
FROM alpine:3.12
ARG APP=/app
LABEL maintainer="Étienne \"maiste\" Marais <etiennemarais@maiste.fr>"
ENV APP_USER=app
RUN addgroup -S $APP_USER\
    && adduser -S -g $APP_USER $APP_USER
RUN apk update
COPY --from=builder /home/rust/src/hugin/target/x86_64-unknown-linux-musl/release/hugin ${APP}/hugin
RUN chown -R $APP_USER:$APP_USER $APP
USER $APP_USER
WORKDIR ${APP}
RUN mkdir content
EXPOSE 8080
CMD ["/app/hugin", "run", "--port",  "8080" ,"--dir",  "content"]

