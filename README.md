<h1 align="center">ASMDA</h1>

ASMDA (automatic social media data archiver) is a [Rust](https://www.rust-lang.org) program written to automate backing up your personal data from mainstream platforms.

<!-- TODO: add more description and details here later -->

## Installation

<details>

<summary>Docker</summary>

This program has been containerized using [Docker](https://www.docker.com), so you can easily run it in different environments. The image can be found here: `ghcr.io/khenziii/asmda:latest`.

<!-- TODO: add an example of a full working `Dockerfile` here -->

</details>

<details>

<summary>Native</summary>

If you're using x86-64 Linux, you can easily download this app without any virtualization layers using the installer scripts. Here's how to use them:

```shell
$ wget https://raw.githubusercontent.com/khenziii/asmda/master/scripts/bootstrap.sh && sudo chmod +x bootstrap.sh && sudo ./bootstrap.sh && rm -f bootstrap.sh
```

> TIP: You can pass `"y"` as the first parameter when calling this script to skip any "Are you sure?" questions. Here's how that would look like:
> ```shell
> $ wget https://raw.githubusercontent.com/khenziii/asmda/master/scripts/bootstrap.sh && sudo chmod +x bootstrap.sh && sudo ./bootstrap.sh y && rm -f bootstrap.sh
> ```

> CAUTION: Downloading scripts like this and running them as root without any double checking is extremely dangerous. As my script isn't too long, I advise reading it beforehand.
>
> <https://raw.githubusercontent.com/khenziii/asmda/master/scripts/bootstrap.sh>

After running the above command (and filling out a couple of inputs), you'll have to define ASMDA's configuration options in `~/asmda-secrets.env`. 

> TIP: If you want to change the path of this file, you can edit `/usr/bin/asmda`.

The easiest way to do so is by copying them from this repository and later modifying them as needed. To do so run:

```shell
$ wget https://raw.githubusercontent.com/khenziii/asmda/master/.env.example
$ mv .env.example asmda-secrets.env
$ mv asmda-secrets.env ~
```

</details>

## Development Environment

First you'll need to define your environment variables. Do so by copying `.env.example`, renaming it as `.env` and filling out the fields.

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
