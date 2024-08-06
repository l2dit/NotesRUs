# Kubernetes
Manifests are complex if you want a visual representation of whats happening look [HERE](https://argocd.nzdev.org/applications/argocd/notes-r-us). Also i have exposed the necessary environment variables to the container for a data base connection and application configuration. This not to be used directly to deploy the appliction yourself becuse there are sevral sevices not inclued in this. However it is a good refrance for copying.

```bash title="Enviroment Vars"
# Application Enviroment Vars
PORT="3000"
ORIGNS="0.0.0.0"
DOMAIN="notesrus.nzdev.org"
HTTPS="true"

# Database Enviroment Vars
DATABASE_URL=<posgresql_uri>
```
