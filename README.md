<h1 align="center">ASMDA</h1>

ASMDA (automatic social media data archiver) is a [Rust](https://www.rust-lang.org) program written to automate backing up your personal data from mainstream platforms.

<!-- TODO: add more description and details here later -->

## Development Environment

First you'll need to fill in your environment variables. Do so by copying `.env.example`, renaming it as `.env` and filling out the fields.

```shell
$ cp .env.example .env
```

Now, to execute the program run those commands:

```shell
$ docker compose up --detach
$ cargo run
```

After running the commands those ports will become available:

- [localhost:3000](http://localhost:3000) a [MinIO](https://www.min.io) console, so that you can check what files the program has already archived,
- [localhost:3001](http://localhost:3001/?autoconnect=1&resize=scale&password=secret) head of the WebDriver, so that you can observe the data export process of some platforms.
