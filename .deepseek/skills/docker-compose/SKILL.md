---
name: docker-compose
description: Docker Compose stack management with docker-compose.yml. Use when working with multi-container applications, compose file definitions, service configuration, networks, volumes, and docker compose up/down/logs/build commands. Project context: Container Desktop has a compose screen for managing stacks.
---

# Docker Compose

## Commands

```bash
docker compose up              # Start all services (foreground)
docker compose up -d           # Start detached
docker compose down            # Stop and remove containers, networks
docker compose down -v         # Also remove volumes
docker compose ps              # List compose containers
docker compose logs            # View logs (all services)
docker compose logs -f         # Follow logs
docker compose logs <service>  # Logs for specific service
docker compose build           # Build or rebuild services
docker compose build --no-cache
docker compose pull            # Pull latest images
docker compose restart         # Restart services
docker compose stop            # Stop services (keep containers)
docker compose start           # Start stopped services
docker compose exec <svc> <cmd>
docker compose run <svc> <cmd> # One-off command
docker compose config          # Validate and view config
```

## docker-compose.yml Structure

```yaml
services:
  app:
    build: .
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgres://user:pass@db/app
    depends_on:
      - db
    volumes:
      - ./src:/app/src
    restart: unless-stopped

  db:
    image: postgres:16
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: pass
      POSTGRES_DB: app
    volumes:
      - pgdata:/var/lib/postgresql/data

volumes:
  pgdata:

networks:
  default:
    driver: bridge
```

## Key Fields

- **build**: Path to Dockerfile context or object with `context`/`dockerfile`
- **image**: Pre-built image to use
- **ports**: `"HOST:CONTAINER"` or `"HOST:CONTAINER/PROTOCOL"`
- **environment**: Map or list of `KEY=VALUE`
- **env_file**: Path to `.env` file
- **volumes**: `"HOST:CONTAINER"` or named volume reference
- **depends_on**: Startup order (does NOT wait for readiness)
- **restart**: `no`, `always`, `on-failure`, `unless-stopped`
- **networks**: Custom network assignment
- **profiles**: Conditional service activation (`--profile` flag)

## Troubleshooting

- **Service won't start**: `docker compose logs <service>`
- **Port conflict**: Check with `docker compose ps` and `lsof -i :PORT`
- **Build cache issues**: `docker compose build --no-cache`
- **Env vars not working**: Verify `.env` file location and `env_file` directive
