# Hugin
 
## What is Hugin ?
Hugin is a REST API server to easy to run that I use for various projects.

## Build Hugin
To build Hugin you can use the **cargo** command from the Rust env.

```sh
  # Build the project
  $ cargo build

  # Build and run the project
  $ cargo run --port <port> --dir <dir_path>

  # Clean executable
  $ cargo clean
```

## Run inside docker
Hugin is in the dockerhub. You can use it as follow:
```sh
docker run --rm -d --name hugin \
       -p <port>:8080 \
       -v <path_to_template>:/app/content \
       maiste/hugin:latest
```

You can then ask the server to send a json with `localhost:<port>/json/<name>.json`.

By default it run on [localhost:8080](http://localhost:8080/test)

