[![argocd badge](https://argocd.nzdev.org/api/badge?name=notes-r-us&revision=true)](https://argocd.nzdev.org/applications/argocd/notes-r-us) [![Build Docker Image](https://github.com/l2dit/NotesRUs/actions/workflows/Docker_Build.yaml/badge.svg)](https://github.com/l2dit/NotesRUs/actions/workflows/Docker_Build.yaml)
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
