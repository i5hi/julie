FROM rust:buster as builder

#ENV RUSTFLAGS="-Ctarget-feature=-crt-static"
RUN useradd -m julie
USER julie
WORKDIR /home/julie

COPY ../. /home/julie/
RUN mkdir -p /home/julie/.julie/.keys && \
    cp /home/julie/art/hound.ascii ~/.julie/banner.ascii && \
    cp /home/julie/config.json ~/.julie/config.json && \
    cp /home/julie/email.html ~/.julie/email.html

RUN cargo build --release && \
    cd target/release/ && \
    strip jd && \
    strip jc

CMD ["/home/julie/target/release/jd"]