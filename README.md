<div align="center" width="100%">
    <img src="./logo.svg" height="196" alt="" />
</div>

<div align="center" width="100%">
    <p>
        <a href="https://github.com/BigBoot/AutoKuma/actions"><img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/BigBoot/AutoKuma/docker-build-push.yml?style=flat&logo=rust&link=https%3A%2F%2Fgithub.com%2FBigBoot%2FAutoKuma%2Factions"></a>
        <a href="https://github.com/BigBoot/AutoKuma/releases/latest"><img alt="GitHub Tag" src="https://img.shields.io/github/v/tag/BigBoot/AutoKuma?logo=github&label=latest"></a>
        <a href="https://ghcr.io/bigboot/autokuma"><img alt="GHCR Tag" src="https://img.shields.io/github/v/tag/BigBoot/AutoKuma?logo=docker&logoColor=white&label=GHCR"></a>
    </p>
    <p>
        <b>
            <a href="#autokuma">AutoKuma</a>
            &nbsp&nbsp
            <a href="#kuma-cli">Kuma CLI</a> 
            &nbsp&nbsp
            <a href="#kuma-client">Kuma Client</a> 
        </b>
    </p>
</div>


# AutoKuma

AutoKuma is a utility that automates the creation of Uptime Kuma monitors based on Docker container labels. With AutoKuma, you can eliminate the need for manual monitor creation in the Uptime Kuma UI.

## 🔧 How to Install

Binaries for windows linux and mac are provided for [GitHub Releases](https://github.com/BigBoot/AutoKuma/releases/latest), additionally AutoKuma is available as a Docker container on [GitHub Container Registry (GHCR)](https://github.com/BigBoot/AutoKuma/pkgs/container/autokuma). To install, simply pull the container using:

```bash
docker pull ghcr.io/bigboot/autokuma:latest
```

## Example Docker Compose

Here's an example `docker-compose.yml`:

```yaml
version: '3'

services:
  autokuma:
    image: ghcr.io/bigboot/autokuma:latest
    restart: unless-stopped
    environment:
      AUTOKUMA__KUMA__URL: http://localhost:3001
      # AUTOKUMA__KUMA__USERNAME: <username> 
      # AUTOKUMA__KUMA__PASSWORD: <password>
      # AUTOKUMA__KUMA__MFA_TOKEN: <token>
      # AUTOKUMA__KUMA__HEADERS: "<header1_key>=<header1_value>,<header2_key>=<header2_value>,..."
      # AUTOKUMA__KUMA__TAG_NAME: AutoKuma
      # AUTOKUMA__KUMA__TAG_COLOR: "#42C0FB"
      # AUTOKUMA__KUMA__CALL_TIMEOUT: 5s
      # AUTOKUMA__KUMA__CONNECT_TIMEOUT: 5s
      # AUTOKUMA__KUMA__DEFAULT_SETTINGS: >- 
      #    docker.docker_container: {{container_name}}
      #    http.max_redirects: 10
      #    *.max_retries: 3
      # AUTOKUMA__DOCKER__SOCKET: /var/run/docker.sock
      # AUTOKUMA__DOCKER__LABEL_PREFIX: kuma
      
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
```

## Configuration

AutoKuma can be configured using the following environment variables:

| Variable                          | Description                                                                              |
|-----------------------------------|------------------------------------------------------------------------------------------|
| `AUTOKUMA__STATIC_MONITORS`       | The path to the folder in which AutoKuma will search for static Monitor definitions      |
| `AUTOKUMA__TAG_NAME`              | The name of the AutoKuma tag, used to track managed containers                           |
| `AUTOKUMA__TAG_COLOR`             | The color of the AutoKuma tag                                                            |
| `AUTOKUMA__DEFAULT_SETTINGS`      | Default settings applied to all generated Monitors, see the example above for the syntax |
| `AUTOKUMA__KUMA__URL`             | The URL AutoKuma should use to connect to Uptime Kuma                                    |
| `AUTOKUMA__KUMA__USERNAME`        | The username for logging into Uptime Kuma (required unless auth is disabled)             |
| `AUTOKUMA__KUMA__PASSWORD`        | The password for logging into Uptime Kuma (required unless auth is disabled)             |
| `AUTOKUMA__KUMA__MFA_TOKEN`       | The MFA token for logging into Uptime Kuma (required if MFA is enabled)                  |
| `AUTOKUMA__KUMA__HEADERS`         | List of HTTP headers to send when connecting to Uptime Kuma                              |
| `AUTOKUMA__KUMA__CONNECT_TIMEOUT` | The timeout for the initial connection to Uptime Kuma                                    |
| `AUTOKUMA__KUMA__CALL_TIMEOUT`    | The timeout for executing calls to the Uptime Kuma server                                |
| `AUTOKUMA__DOCKER__SOCKET`        | Path to the Docker socket                                                                |
| `AUTOKUMA__DOCKER__LABEL_PREFIX`  | Prefix used when scanning for container labels                                           |



## Usage

AutoKuma interprets Docker container labels with the following format:

```plaintext
<prefix>.<id>.<type>.<setting>: <value>
```

- `<prefix>`: Default is `kuma` unless changed using the `DOCKER__LABEL_PREFIX` env variable.
- `<id>`: A unique identifier for the monitor (ensure it's unique between all monitors).
- `<type>`: The type of the monitor as configured in Uptime Kuma.
- `<setting>`: The key of the value to be set.
- `<value>`: The value for the option.

Labels are grouped by `<id>` into a single monitor. For example, to create a simple HTTP monitor, use the following labels:

```plaintext
kuma.example.http.name: "Example"
kuma.example.http.url: "https://example.com"
```

Take a look at [all available monitor types](MONITOR_TYPES.md) and the corresponding settings.


AutoKuma also provides support for creating and assigning groups:

```plaintext
kuma.mygroup.group.name: "This is a Group"
kuma.mymonitor.http.name: "This is a Monitor assigned to a Group"
kuma.mymonitor.http.parent_name: "mygroup"
kuma.mymonitor.http.url: "https://example.com"
```

There are also some text replacements available which will be replaced by details about the corresponding docker container:
| Template             | Description                   | Example Value                                                           |
|----------------------|-------------------------------|-------------------------------------------------------------------------|
| `{{container_id}}`   | The container id              | 92366941fb1f211c573c56d261f3b3e5302f354941f2aa295ae56d5781e97221        |
| `{{image_id}}`       | Sha256 of the container image | sha256:c2e38600b252f147de1df1a5ca7964f9c8e8bace97111e56471a4a431639287a |
| `{{image}}`          | Name of the container image   | ghcr.io/immich-app/immich-server:release                                |
| `{{container_name}}` | Name of the container         | /immich-immich-1                                                        |


### Static Monitors
In addition to reading Monitors from Docker labels, AutoKuma can create Monitors from files, this can be usefull if you have want AutoKuma to manage monitors which aren't directly related to a container. 

To create static Monitors just a a .json or .toml file in the directory specified by `AUTOKUMA__STATIC_MONITORS`, take a look at [the examples here](monitors).

In case of static Monitors the id is determined by the filename (without the extension).


# Kuma CLI

Kuma CLI is a Command-Line Interface (CLI) tool for managing and interacting with [Uptime Kuma](https://uptime.kuma.pet/). With Kuma CLI, you can easily configure, monitor, and manage your applications from the command line. 

## Features
- [x] Commands: `kuma monitor`
    - [x] `add`
    - [x] `delete`
    - [x] `edit`
    - [x] `list`
    - [x] `get`
    - [x] `pause`
    - [x] `resume`
- [x] Commands : `kuma tag`
    - [x] `add`
    - [x] `delete`
    - [x] `edit`
    - [x] `ls`
    - [x] `get`
- [x] Commands : `kuma notification`
    - [x] `add`
    - [x] `delete`
    - [x] `edit`
    - [x] `ls`
    - [x] `get`
- [x] Commands : `kuma maintenance`
    - [x] `add`
    - [x] `delete`
    - [x] `edit`
    - [x] `ls`
    - [x] `get`
    - [x] `pause`
    - [x] `resume`
- [x] Commands : `kuma status-page`
    - [x] `add`
    - [x] `delete`
    - [x] `edit`
    - [x] `ls`
    - [x] `get`

## 🔧 How to Install

Binaries for windows linux and mac are provided for [GitHub Releases](https://github.com/BigBoot/AutoKuma/releases/latest), additionally Kuma CLI can be installed using `cargo`:

```bash
cargo install --git https://github.com/BigBoot/AutoKuma.git kuma-cli
```

## Usage

```bash
Usage: kuma [OPTIONS] [COMMAND]

Commands:
  monitor       Manage Monitors
  notification  Manage Notifications
  tag           Manage Tags
  maintenanc    Manage Maintenances
  help          Print this message or the help of the given subcommand(s)

Options:
      --url <URL>
          The URL AutoKuma should use to connect to Uptime Kuma
      --username <USERNAME>
          The username for logging into Uptime Kuma (required unless auth is disabled)
      --password <PASSWORD>
          The password for logging into Uptime Kuma (required unless auth is disabled)
      --mfa-token <MFA_TOKEN>
          The MFA token for logging into Uptime Kuma (required if MFA is enabled)
      --header <KEY=VALUE>
          Add a HTTP header when connecting to Uptime Kuma
      --connect-timeout <CONNECT_TIMEOUT>
          The timeout for the initial connection to Uptime Kuma [default: 30.0]
      --call-timeout <CALL_TIMEOUT>
          The timeout for executing calls to the Uptime Kuma server [default: 30.0]
      --format <OUTPUT_FORMAT>
          The output format [default: json] [possible values: json, toml, yaml]
      --pretty
          Wether the output should be pretty printed or condensed
  -h, --help
          Print help
  -V, --version
          Print version
```


## Configuration

All configuration options can also be specified as environment variables:
```
KUMA__URL="http://localhost:3001/"
KUMA__USERNAME="<username>"
KUMA__PASSWORD="<password>"
...
```

Additionally Kuma CLI will read configuration from a file named `kuma.{toml,yaml,json}` in the current directory and in the following locations:
| Platform | Value                                                            | Example                                                   |
|----------|------------------------------------------------------------------|-----------------------------------------------------------|
| Linux    | `$XDG_CONFIG_HOME`/kuma/config.{toml,yaml,json}                  | /home/alice/.config/kuma/config.toml                      |
| macOS    | `$HOME`/Library/Application Support/kuma/config.{toml,yaml,json} | /Users/Alice/Library/Application Support/kuma/config.toml |
| Windows  | `%LocalAppData%`\kuma\config.{toml,yaml,json}                    | C:\Users\Alice\AppData\Local\kuma\config.toml             |

An example `.toml` config could look like the following:
```toml
url = "http://localhost:3001/"
username = "<username>"
password = "<password>"
```

# Kuma Client

TODO

# Contributing

Contributions to AutoKuma are welcome! Feel free to open issues, submit pull requests, or provide feedback.

# License

AutoKuma is released under the [MIT License](LICENSE).
