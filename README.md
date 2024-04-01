# Rust URL Shortener

This is a simple URL shortener pet project implemented in Rust.

## How to run
Docker is required to run this project. 
Docker-compose will create a PostgreSQL container and a Rust container. The Rust container right now is set as a development container.

To run the project, execute the following command:
```bash
docker-compose up -d
# To enter the Rust container
docker exec -it rust_shortener-app-1 /bin/bash
# To run the server
cargo run
```
The server will be available at `http://127.0.0.1:8081`.

To test the server, you can use the following command, it will create a short URL for `https://www.google.com` and return the short URL. You can then use the short URL to redirect to the original URL
```bash
curl -X POST -d 'https://www.google.com' http://127.0.0.1:8001/
```


## Notes:
- Uncomment the line indicated in the docker-compose.yml file to persist the PostgreSQL data.
