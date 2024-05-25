# NotesRUs
Notes Aplication

# What You Need
1. Built [The Front End](./notes_r_us_ui) to ./notes_r_us_ui/dist/.
2. Rust

# Build/Run

```zsh
cargo build // builds to the target directory `./target/debug/notes_r_us`

cargo run // Runs The Application On `0.0.0.0:3000`

./target/debug/notes_r_us // Runs The Application On `0.0.0.0:3000`
```
# Deployment
## Docker
We have docker images both on [Docker Hub](https://hub.docker.com/r/asskit/notesrus) and GitHub's [ghcr.io](https://github.com/l2dit/NotesRUs/pkgs/container/notesrus).

```bash
docker pull ghcr.io/l2dit/notesrus:latest

docker pull asskit/notesrus:latest
```

## Deployment
This is slightly harder to recreate as curenetly we are using kubernetes info [HERE](./kubernetes).
