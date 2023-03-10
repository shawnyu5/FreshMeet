FROM golang:1.19.1-alpine3.16 AS build

WORKDIR /bot

COPY . .
RUN go build -o bot

FROM golang:1.19.1-alpine3.16

WORKDIR /bot

COPY --from=build /bot/config.json .
COPY --from=build /bot/bot .

CMD ["./bot"]
