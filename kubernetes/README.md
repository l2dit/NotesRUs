# Kubernetes
Manifests are complex if you want a high level look of whats happening look [HERE](https://argocd.nzdev.org/applications/argocd/notes-r-us). Also i have exposed the necessary environment variables to the container for a data base connection and application configuration.

```bash title="Enviroment Vars"
# Application Enviroment Vars
PORT="3000"
ORIGNS="0.0.0.0"
DOMAIN="notesrus.nzdev.org"

# PostgreSQL Enviroment Vars
POSTGRESQL_USERNAME=<username_secret>
POSTGRESQL_PASSWORD=<password_secret>
POSGRESQL_IP=<postgresql_ip_address>
POSGRESQL_PORT=<posgresql_port>
```
