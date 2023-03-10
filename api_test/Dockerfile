FROM bash:latest
WORKDIR /app
COPY ./script.sh ./
RUN chmod +x ./script.sh
RUN apk add curl
CMD ["bash", "script.sh"]
