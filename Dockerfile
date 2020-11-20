FROM debian:buster-slim
WORKDIR /usr/local/bin
COPY ./target/release/document_microservice /usr/local/bin/document_microservice
RUN apt-get update && apt-get install -y
RUN apt-get install curl -y
# Set server timezone to Budapest timezone
RUN sudo timedatectl set-timezone Europe/Budapest
STOPSIGNAL SIGINT
ENV RUST_LOG=trace
ENTRYPOINT ["invoice_microservice"]