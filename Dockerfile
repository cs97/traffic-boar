FROM debian:12-slim

RUN <<EOF
apt-get update
apt-get install -y certbot
EOF

WORKDIR /app

COPY target/release/traffic-boar  /app/traffic-boar 

COPY configuration.yaml /app/configuration.yaml

COPY key.pem /app/key.pem

COPY cert.pem /app/cert.pem

RUN chmod 755 server

EXPOSE 8000

CMD ["/app/traffic-boar "]
