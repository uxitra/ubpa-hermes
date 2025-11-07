# Setup Guide

## 1. Install Docker

Make sure [Docker](https://docs.docker.com/get-started/get-docker/) is installed on the system

You can check if docker is already installed by running:

```
sudo docker ps
```

## 2. Clone the repository

```bash
git clone https://github.com/uxitra/ubpa-hermes.git
```

# 3. Build the Docker image

build the image using the included `Dockerfile`

```bash
sudo docker build -t ubpa-hermes:latest .
```

If the build completes successfully, you’re ready to start the service.

## 4. Run the application

Use Docker Compose to start the required container:

```bash
sudo docker compose up
```

Most directories and files will be automatically created during the first build or run.

## Configuration

One important exception is the configuration file, config.json.
This file contains essential settings such as the sender’s email address and password — required for the Rust backend to send emails.

An example configuration file (`config.example.json`) is provided in the project root.
Copy it to `config.json` and adjust the values for your environment.

**Example**:

```json
{
  "email": "your-email@example.com",
  "password": "password",
  "submit_subject": "",
  "submit": "",
  "removed_subject": "",
  "removed": "",
  "changed_state_subject": "",
  "changed_state": ""
}
```

once configured, your environment is ready — the application should now be running through Docker!
