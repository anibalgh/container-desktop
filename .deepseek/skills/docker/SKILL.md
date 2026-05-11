---
name: docker
description: Docker container and image management via CLI and API. Use when working with Dockerfiles, docker run/exec/logs/ps, managing images/containers/volumes/networks, troubleshooting Docker daemon connection issues, or interacting with the Docker socket. Project context: Container Desktop is a Docker management GUI using bollard (Rust Docker API client).
---

# Docker

## Connection Check

```bash
docker info                   # Verify daemon is running
docker ps                     # List running containers
docker images                 # List images
```

## Socket Permissions (Linux)

```bash
ls -la /var/run/docker.sock   # Check permissions
sudo usermod -aG docker $USER # Add user to docker group (requires re-login)
```

## Container Management

```bash
docker ps -a                  # All containers (including stopped)
docker run <image>            # Run container
docker start <id>             # Start stopped container
docker stop <id>              # Stop container
docker rm <id>                # Remove container
docker exec -it <id> <shell>  # Exec into container
docker exec -u root <id> <cmd># Exec as root
docker logs <id>              # View logs
docker logs --tail 100 <id>   # Last 100 lines
docker logs --since 1h <id>   # Logs from last hour
docker inspect <id>           # Full container metadata
```

## Image Management

```bash
docker pull <image>:<tag>     # Pull image
docker images                 # List images
docker rmi <id>               # Remove image
docker tag <src> <dst>        # Tag image
docker build -t <name> .      # Build from Dockerfile
docker push <image>           # Push to registry
```

## Volume Management

```bash
docker volume ls              # List volumes
docker volume create <name>   # Create volume
docker volume rm <name>       # Remove volume
docker volume prune           # Remove unused volumes
```

## Network Management

```bash
docker network ls             # List networks
docker network create <name>  # Create network
docker network rm <name>      # Remove network
docker network connect <net> <container>
docker network disconnect <net> <container>
```

## Dockerfile Patterns

```dockerfile
FROM rust:1.86-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/app /usr/local/bin/app
CMD ["/usr/local/bin/app"]
```

## Troubleshooting

- **Connection refused**: Docker daemon not running → `sudo systemctl start docker`
- **Permission denied**: User not in docker group → `sudo usermod -aG docker $USER`
- **Image pull failures**: Check network/proxy, verify registry credentials
- **Container won't start**: Check logs with `docker logs <id>`, verify port conflicts
