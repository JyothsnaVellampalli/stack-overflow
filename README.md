#Postgres Container

```
docker run --name stack_overflow_postgres_container   --network my_network   -e POSTGRES_USER=postgres   -e POSTGRES_PASSWORD=1234   -e POSTGRES_DB=mydatabase   -p 5432:5432   -d postgres:latest
```

#PgAdmin
```
docker run --name my_pgadmin_container   --network my_network   -e PGADMIN_DEFAULT_EMAIL=jyothsna@example.com   -e PGADMIN_DEFAULT_PASSWORD=1234   -p 5050:80   -d dpage/pgadmin4
```

#Postgres container address
```
docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' stack_overflow_postgres_container
```

#Install sqlx-cli
```cargo install sqlx-cli```

#Run SQL migrations
```sqlx migrate run```
