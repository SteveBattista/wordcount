FROM rust:latest as builder

RUN USER=root git clone https://github.com/SteveBattista/wordcount.git
WORKDIR ./wordcount
RUN cargo build --release
RUN rm src/*.rs

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt update \
    && apt -y upgrade\
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p ${APP}

COPY --from=builder /wordcount/target/release/wordcount ${APP}/wordcount

WORKDIR ${APP}

CMD ["./wordcount"]