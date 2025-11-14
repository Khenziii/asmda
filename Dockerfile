FROM rust:1.90.0 AS builder

WORKDIR /app

RUN cargo init && rm Cargo.toml

COPY Cargo.toml Cargo.lock .

# Download the dependencies (this is seperated to make this step cacheable).
RUN cargo build --release

# Copy the rest of program's source code.
RUN rm -r src target
COPY src ./src

# Build the code. 
RUN cargo build --release

# Remove debug symbols.
RUN strip target/release/asmda

FROM debian:12-slim AS runtime

RUN apt-get update && apt-get install -y libssl3

COPY --from=builder /app/target/release/asmda /bin/asmda

CMD ["/bin/asmda"]
