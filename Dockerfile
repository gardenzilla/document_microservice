FROM fedora:34
WORKDIR /usr/local/bin
COPY ./target/release/document_microservice /usr/local/bin/document_microservice
RUN dnf install curl -y && dnf clean all -y
# Set server timezone to Budapest timezone
RUN sudo timedatectl set-timezone Europe/Budapest
STOPSIGNAL SIGINT
ENV RUST_LOG=trace
ENTRYPOINT ["document_microservice"]
