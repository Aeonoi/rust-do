FROM alpine:edge 
WORKDIR /rust-do/code 
RUN apk add --no-cache rust cargo bash vim 
COPY . .
ENV HOME=/rust-do
RUN rm -r target 
RUN cargo build --release 
ENV PATH=$PATH:/rust-do/code/target/release
