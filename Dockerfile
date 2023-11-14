# Render static files
FROM ghcr.io/quarto-dev/quarto:1.3.340 AS quartofiles

COPY ./paywall_blog ./paywall_blog
RUN cd paywall_blog && quarto install --no-prompt extension shafayetShafee/bsicons && quarto render

# Build Rust server binary
FROM rust:1.69 AS server

COPY ./rust_server .
RUN cargo build --release

# Target image
FROM debian:trixie-slim

ENV JWT_SECRET=TEST_SECRET

COPY --from=quartofiles ./paywall_blog/_site ./paywall_blog/_site
COPY --from=server ./target/release/rust_server ./rust_server/rust_server
COPY --from=server ./templates ./rust_server/templates 

WORKDIR rust_server
EXPOSE 5001
CMD ["./rust_server"]
