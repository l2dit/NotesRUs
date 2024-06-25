# NotesRus Frontend
the frontend is made with Vue and Vite



## Installing
Ensure that npm is installed and run below to start the frontend.  
`npm i; npm run dev`


# Use with production and localhost
automatically sets route for the api usage based on the NODE_ENV variable. 

When using vite build the variable will be set to production
when using vite run the variable is set to development

Whilst the variable is set to prod it will use the notesrus.nzdev.org api 
however using development will search for the api running on port 3000 on the localmachine

you can manually change this by adding NODE_ENV=production or development before vite run
example for using server instead of localhost
`NODE_ENV=production vite run`

for more information look at modes in [env_and_mode](https://vitejs.dev/guide/env-and-mode)