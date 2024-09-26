FROM busybox
WORKDIR /
COPY ./target/site /site
COPY ./target/x86_64-unknown-linux-musl/release/kotiboksi /server

ENV RUST_LOG="info"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
ENV DATABASE_URL="sqlite://guestbook.sqlite?mode=rwc"
EXPOSE 8080

CMD ["/server"]