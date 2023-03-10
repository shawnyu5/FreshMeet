FROM golang:1.19.1-alpine3.16 AS build

WORKDIR /bot

COPY . .
COPY ./.env .
RUN go build -o bot

FROM golang:1.19.1-alpine3.16

WORKDIR /bot

COPY --from=build /bot/config.json .
COPY --from=build /bot/bot .
COPY --from=build /bot/.env .

CMD ["./bot"]
