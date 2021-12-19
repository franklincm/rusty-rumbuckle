# Rusty Rumbuckle

A _very_ simple discord bot to expose dice rolling via [Dicer](https://github.com/gnullByte/dicer)

Running:

```sh
DISCORD_TOKEN=XXXX cargo run
```

Docker:

```sh
docker build -t rusty .
echo "DISCORD_TOKEN=XXXX" > .env
docker run -d --env-file=.env rusty:latest
```
