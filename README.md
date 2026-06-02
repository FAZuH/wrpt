<h1 align="center">WRPT</h1>

<h4 align="center">A minimal <a href="https://portainer.io/" target="_blank">Portainer</a> cli built with <a href="https://www.rust-lang.org" target="_blank">Rust</a></h4>

<p align="center">
    <a href="https://crates.io/crates/wrpt"><img src="https://img.shields.io/crates/v/wrpt.svg" alt=""/></a>
    <a href="https://hub.docker.com/repository/docker/wahl/wrpt"><img src="https://img.shields.io/docker/v/wahl/wrpt?sort=semver&label=dockerhub&color=blue" alt=""/></a>
    <a href="https://github.com/wahl-dev/wrpt/actions?query=workflow:Tests"><img src="https://github.com/wahl-dev/wrpt/workflows/Tests/badge.svg" alt=""/></a>
    <a href="./LICENSE"><img src="https://img.shields.io/crates/l/wrpt.svg" alt=""/></a>
</p>

<p align="center">
    <a href="#about">About</a> •
    <a href="#roadmap">Roadmap</a> •
    <a href="#installation">Installation</a> •
    <a href="#quick-start">Quick Start</a> •
    <a href="#available-commands">Available Commands</a> •
    <a href="#docker">Docker</a> •
    <a href="#cicd-integration">CI/CD Integration</a> •
    <a href="#changelog">Changelog</a> •
    <a href="#license">License</a>
</p>

---

## About

WRPT is a lightweight command-line interface designed to streamline the deployment of Docker-Compose stacks on Portainer.  

While its primary focus is on stack deployment, it also provides additional features such as stack/endpoint listing and access control management (wip). WRPT is designed not only for manual usage but also for integration into CI/CD pipelines, making it a versatile tool for automating deployment workflows.

This project draws inspiration from <a href="https://gitlab.com/tortuetorche" target="_blank">@tortuetorche</a>'s work on <a href="https://gitlab.com/psuapp/psu" target="_blank">psuapp/psy</a>.  

It is also my first project written in Rust and is under **active development**, so contributions and feedback are welcome! Stay tuned for new features and improvements.

---

## Roadmap

Here are the planned enhancements and features for WRPT:  

- 🚧 **Access Control Management:** Enable stack deployments with fine-grained access control, allowing assignment to specific users and/or groups.  
- ✅ **Comprehensive Documentation:** Write detailed usage guides, including setup instructions for integration into CI/CD pipelines on GitLab and GitHub.
- ✅ **Automated Testing:** Write tests to ensure the reliability and stability of the tool.
- 💭 **Kubernetes Compatibility:** Extend the tool to support Portainer deployments on Kubernetes environment.
- ✅ **Automated Release Process:** Implement CI pipelines to generate changelogs and releases automatically based on versioning and commit history.
- ✅ **Docker Image:** Create a Docker image.

### Legend
- ✅ : Completed  
- 🚧 : In progress  
- ⏳ : Pending  
- 💭 : In reflection
- ❌ : Abandoned 

---

## Installation

### From crates.io

```bash
cargo install wrpt
```

### Docker

```bash
docker pull wahl/wrpt:latest
```

### From source

```bash
git clone https://github.com/wahl-dev/wrpt.git
cd wrpt
cargo build --release
# Binary available at ./target/release/wrpt
```

---

## Quick Start

### 1. Generate a Portainer access token

In your Portainer instance, go to **My Account** > **Access tokens** > **Add access token**.

See the [Portainer documentation](https://docs.portainer.io/api/access#creating-an-access-token) for more details.

### 2. Set your environment variables

```bash
export PORTAINER_URL="https://portainer.example.com"
export PORTAINER_ACCESS_TOKEN="your-access-token"
```

### 3. List your endpoints

```bash
wrpt endpoint list
```

### 4. List your stacks

```bash
wrpt stack list
```

### 5. Deploy a stack

```bash
wrpt stack deploy my-stack \
  --endpoint 1 \
  --compose-file docker-compose.yml
```

You can also pass environment variables to the stack, or select endpoint by name:

```bash
wrpt stack deploy my-stack \
  --endpoint-name my-docker-endpoint \
  --compose-file docker-compose.yml \
  --env-file .env
```

Or pass a directory containing your compose file and .env:

```bash
# Loads my/stack/docker-compose.yml with env my/stack/.env
wrpt stack deploy my-stack \
  --endpoint-name my-docker-endpoint \
  --stack-dir my/stack/

# Skip .env with --no-env
# Or choose a different env with --env-file
```

---

## Available Commands

| Name                                                | Description                                               |
|-----------------------------------------------------|-----------------------------------------------------------|
| [`stack deploy`](#stack-deploy)                     | Deploy/update a stack.                                    |
| [`stack remove`](#stack-remove)                     | Remove a stack.                                           |
| [`stack list`](#stack-list)                         | List all stacks based on the current user authorizations. |
| [`stack resource-control`](#stack-resource-control) | Display the ResourceControl details of a specific stack.  |
| [`stack start`](#stack-start)                       | Starts a stack.                                           |
| [`stack stop`](#stack-stop)                         | Stops a stack.                                            |
| [`endpoint list`](#endpoint-list)                   | List endpoints.                                           |
| [`team list`](#team-list)                           | List teams.                                               |
| [`user list`](#user-list)                           | List users.                                               |
| `help`                                              | Display help message.                                     |

### Global options

| Flag  | Option                          | Description                                                                                                                                          |
|-------|---------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------|
| -l    | `--url <URL>`                   | URL of the Portainer instance.                                                                                                                       |
| -A    | `--access-token <ACCESS_TOKEN>` | Access token of the Portainer instance. Learn how to generate an access token [here](https://docs.portainer.io/api/access#creating-an-access-token). |
|       | `--color <COLOR>`               | When to use terminal colours [default: auto] [possible values: auto, always, never].                                                                 |
|       | `--insecure`                    | Skip the host's SSL certificate verification, use at your own risk.                                                                                  |
| -v... |                                 | Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace.                                     |
| -q    | `--quiet`                       | Do not output any message.                                                                                                                           |
| -h    | `--help`                        | Print help.                                                                                                                                          |

### Available environment variables

| Environment variable           | Description                             |
|--------------------------------|-----------------------------------------|
| `PORTAINER_URL=URL`            | URL of the Portainer instance.          |
| `PORTAINER_ACCESS_TOKEN=TOKEN` | Access token of the Portainer instance. |

### Commands in details

#### Stack

##### Stack deploy

```
Deploy a stack

Usage: wrpt stack deploy [OPTIONS] <--endpoint <ENDPOINT>|--endpoint-name <ENDPOINT_NAME>> <--compose-file <COMPOSE_FILE>|--stack-dir <STACK_DIR>> <STACK_NAME>

Arguments:
  <STACK_NAME>  Name of the stack

Options:
  -E, --endpoint <ENDPOINT>            Id of the environment (endpoint) that will be used
  -l, --url <URL>                      URL of the Portainer instance
  -A, --access-token <ACCESS_TOKEN>    Access token of the Portainer instance
  -n, --endpoint-name <ENDPOINT_NAME>  Name of the environment (endpoint) that will be used
  -c, --compose-file <COMPOSE_FILE>    Path to docker compose/stack file
      --insecure                       Skip the host's SSL certificate verification, use at your own risk
  -s, --stack-dir <STACK_DIR>          Path to directory containing docker compose and optional .env file
  -v...                                Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
      --no-env                         When used with --stack-dir, skip loading the .env file from that directory
  -q, --quiet                          Do not output any message
      --color <COLOR>                  When to use terminal colours [default: auto] [possible values: auto, always, never]
  -e, --env-file <ENV_FILE>            Path to a file of environment variables, to be used by the stack
      --prune                          Whether to prune unused containers or not
      --pull-image                     Force a pulling to current image with the original tag though the image is already the latest
  -h, --help                           Print help
```

##### Stack remove

```
Remove a stack

Usage: wrpt stack remove [OPTIONS] <--endpoint <ENDPOINT>|--endpoint-name <ENDPOINT_NAME>> <STACK_NAME>

Arguments:
  <STACK_NAME>  Name of the stack

Options:
  -E, --endpoint <ENDPOINT>            Id of the environment (endpoint) that will be used
  -l, --url <URL>                      URL of the Portainer instance
  -A, --access-token <ACCESS_TOKEN>    Access token of the Portainer instance
  -n, --endpoint-name <ENDPOINT_NAME>  Name of the environment (endpoint) that will be used
      --insecure                       Skip the host's SSL certificate verification, use at your own risk
  -v...                                Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
  -q, --quiet                          Do not output any message
      --color <COLOR>                  When to use terminal colours [default: auto] [possible values: auto, always, never]
  -h, --help                           Print help
```

##### Stack resource-control

```
Display the ResourceControl details of a specific stack

Usage: wrpt stack resource-control [OPTIONS] <--endpoint <ENDPOINT>|--endpoint-name <ENDPOINT_NAME>> <STACK_NAME>

Arguments:
  <STACK_NAME>  Name of the stack

Options:
  -E, --endpoint <ENDPOINT>            Id of the environment (endpoint) that will be used
  -l, --url <URL>                      URL of the Portainer instance
  -A, --access-token <ACCESS_TOKEN>    Access token of the Portainer instance
  -n, --endpoint-name <ENDPOINT_NAME>  Name of the environment (endpoint) that will be used
      --insecure                       Skip the host's SSL certificate verification, use at your own risk
  -v...                                Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
  -q, --quiet                          Do not output any message
      --color <COLOR>                  When to use terminal colours [default: auto] [possible values: auto, always, never]
  -h, --help                           Print help
```

##### Stack start

```
Starts a Stack

Usage: wrpt stack start [OPTIONS] <--endpoint <ENDPOINT>|--endpoint-name <ENDPOINT_NAME>> <STACK_NAME>

Arguments:
  <STACK_NAME>  Name of the stack

Options:
  -E, --endpoint <ENDPOINT>            Id of the environment (endpoint) that will be used
  -l, --url <URL>                      URL of the Portainer instance
  -A, --access-token <ACCESS_TOKEN>    Access token of the Portainer instance
  -n, --endpoint-name <ENDPOINT_NAME>  Name of the environment (endpoint) that will be used
      --insecure                       Skip the host's SSL certificate verification, use at your own risk
  -v...                                Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
  -q, --quiet                          Do not output any message
      --color <COLOR>                  When to use terminal colours [default: auto] [possible values: auto, always, never]
  -h, --help                           Print help
```

##### Stack stop

```
Stops a Stack

Usage: wrpt stack stop [OPTIONS] <--endpoint <ENDPOINT>|--endpoint-name <ENDPOINT_NAME>> <STACK_NAME>

Arguments:
  <STACK_NAME>  Name of the stack

Options:
  -E, --endpoint <ENDPOINT>            Id of the environment (endpoint) that will be used
  -l, --url <URL>                      URL of the Portainer instance
  -A, --access-token <ACCESS_TOKEN>    Access token of the Portainer instance
  -n, --endpoint-name <ENDPOINT_NAME>  Name of the environment (endpoint) that will be used
      --insecure                       Skip the host's SSL certificate verification, use at your own risk
  -v...                                Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
  -q, --quiet                          Do not output any message
      --color <COLOR>                  When to use terminal colours [default: auto] [possible values: auto, always, never]
  -h, --help                           Print help
```

##### Stack list

```
List all stacks based on the current user authorizations

Usage: wrpt stack list [OPTIONS]

Options:
  -l, --url <URL>                    URL of the Portainer instance
  -A, --access-token <ACCESS_TOKEN>  Access token of the Portainer instance
      --insecure                     Skip the host's SSL certificate verification, use at your own risk
  -v...                              Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
  -q, --quiet                        Do not output any message
      --color <COLOR>                When to use terminal colours [default: auto] [possible values: auto, always, never]
  -h, --help                         Print help
```

#### Endpoint

##### Endpoint list

```
List endpoints

Usage: wrpt endpoint list [OPTIONS]

Options:
  -l, --url <URL>                    URL of the Portainer instance
  -A, --access-token <ACCESS_TOKEN>  Access token of the Portainer instance
  -v...                              Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
  -q, --quiet                        Do not output any message
      --color <COLOR>                When to use terminal colours [default: auto] [possible values: auto, always, never]
  -h, --help                         Print help
```

#### Team

##### Team list

```
List teams

Usage: wrpt team list [OPTIONS]

Options:
  -l, --url <URL>                    URL of the Portainer instance
  -A, --access-token <ACCESS_TOKEN>  Access token of the Portainer instance
  -v...                              Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
  -q, --quiet                        Do not output any message
      --color <COLOR>                When to use terminal colours [default: auto] [possible values: auto, always, never]
  -h, --help                         Print help
```

#### User

##### User list

```
List users

Usage: wrpt user list [OPTIONS]

Options:
  -l, --url <URL>                    URL of the Portainer instance
  -A, --access-token <ACCESS_TOKEN>  Access token of the Portainer instance
  -v...                              Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
  -q, --quiet                        Do not output any message
      --color <COLOR>                When to use terminal colours [default: auto] [possible values: auto, always, never]
  -h, --help                         Print help
```

---

## Docker

WRPT is also available as a Docker image for easier usage and integration. The image is hosted on Docker Hub: [wahl/wrpt](https://hub.docker.com/repository/docker/wahl/wrpt).

### Available Tags

The available tags for the Docker image can be found [here](https://hub.docker.com/repository/docker/wahl/wrpt/tags).

### Pull the Docker image

```bash
docker pull wahl/wrpt:latest
```

### Example usage

Below is an example of using the Docker image to list stacks:

```bash
docker run -it --rm \
  -e PORTAINER_URL="$PORTAINER_URL" \
  -e PORTAINER_ACCESS_TOKEN="$PORTAINER_ACCESS_TOKEN" \
  wahl/wrpt:latest stack list
```

### Notes
- Replace `$PORTAINER_URL` and `$PORTAINER_ACCESS_TOKEN` with your Portainer instance details.

---

## CI/CD Integration

WRPT's Docker image makes it easy to integrate stack deployments into your CI/CD pipelines.

### GitHub Actions

Add this workflow to `.github/workflows/deploy.yml`:

```yaml
name: Deploy Stack

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    container:
      image: wahl/wrpt:latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Deploy stack
        env:
          PORTAINER_URL: ${{ secrets.PORTAINER_URL }}
          PORTAINER_ACCESS_TOKEN: ${{ secrets.PORTAINER_ACCESS_TOKEN }}
        run: |
          wrpt stack deploy my-stack \
            --endpoint ${{ vars.PORTAINER_ENDPOINT }} \
            --compose-file docker-compose.yml
```

**Required secrets** (Settings > Secrets and variables > Actions):

| Secret | Description |
|--------|-------------|
| `PORTAINER_URL` | URL of your Portainer instance (e.g. `https://portainer.example.com`) |
| `PORTAINER_ACCESS_TOKEN` | Portainer API access token |

**Required variables** (Settings > Secrets and variables > Actions > Variables):

| Variable | Description |
|----------|-------------|
| `PORTAINER_ENDPOINT` | ID of the Portainer endpoint to deploy to |

### GitLab CI

Add this to your `.gitlab-ci.yml`:

```yaml
stages:
  - deploy

deploy-stack:
  stage: deploy
  image: wahl/wrpt:latest
  only:
    - main
  script:
    - wrpt stack deploy my-stack
        --endpoint $PORTAINER_ENDPOINT
        --compose-file docker-compose.yml
  variables:
    PORTAINER_URL: $PORTAINER_URL
    PORTAINER_ACCESS_TOKEN: $PORTAINER_ACCESS_TOKEN
```

**Required CI/CD variables** (Settings > CI/CD > Variables):

| Variable | Protected | Masked | Description |
|----------|-----------|--------|-------------|
| `PORTAINER_URL` | Yes | No | URL of your Portainer instance |
| `PORTAINER_ACCESS_TOKEN` | Yes | Yes | Portainer API access token |
| `PORTAINER_ENDPOINT` | Yes | No | ID of the Portainer endpoint |

### CI/CD Best Practices

- **Never hardcode tokens** in your pipeline files. Always use secrets/protected variables.
- **Use `--insecure` only if necessary** (e.g. self-signed certificates in internal environments). Prefer proper SSL certificates.
- **Use `-vv` for debugging** pipeline failures — it enables verbose output to help diagnose issues.
- **Verify your endpoint first** by running `wrpt endpoint list` as a preliminary step to confirm connectivity.
- **Pin the Docker image tag** to a specific version (e.g. `wahl/wrpt:0.6.3`) in production pipelines for reproducible deployments.

---

## Changelog

The changelog is available in the [CHANGELOG.md](./CHANGELOG.md) file.

---

## License

The source code of this project is licensed under the [MIT license](https://opensource.org/license/MIT). 

See [LICENSE](LICENSE) file for reference.
