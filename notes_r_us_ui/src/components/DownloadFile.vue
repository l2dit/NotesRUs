<template>
  <div class="center-content">
    <form @submit.prevent="download_file">
      <label id="button-label">Download markdown.</label>
      <br />
      <input type="text" id="file-id" placeholder="File id" />
      <button type="submit" id="file-input-button">Download</button>
    </form>
    <p id="error-message" class="error-message-hide">{{ error_msg }}</p>
  </div>
</template>

<script>
import axios from 'axios';
let route = process.env.NODE_ENV == "production" ? "https://notesrus.nzdev.org/api" : `http://127.0.0.1:3000/api`;

export default {
  data() {
    return {
      /* Error is just a placeholder so that the p tag when text added does not move the whole upload section upwards */
      error_msg: 'Error'
    };
  },
  methods: {
    async download_file() {
      /* gets the id of the file the user wants to fetch */
      const file_id = document.getElementById("file-id").value;
      /* route to make requests to */
      let get_route = `${route}/api/file/download/${file_id}`;
      /* make get request to the route */
      axios.get(get_route)
        .then(response => {
          console.log(response)
          /* create obj url with the blob recieved in response */
          const url = window.URL.createObjectURL(new Blob([response.data]));
          /*  create new anchor tag */
          const link = document.createElement('a');
          /* set the location of the link to the url and set it to download with filename markdown */
          link.href = url;
          link.setAttribute('download', "markdown.md");
          /* add the link tag to the body */
          document.body.appendChild(link);
          /* click the link to trigger download */
          link.click();
          /* remove the link from page */
          document.body.removeChild(link);
          window.URL.revokeObjectURL(url);
        })
        .catch(error => {
          this.error_msg = `Error file failed to download: ${error.message}`;
          document.getElementById("error-message").classList.add("error-message-show")
          setTimeout(() => { document.getElementById("error-message").classList.remove("error-message-show") }, 3000);
          console.error("Error:", error);
        });
    }
  }
};
</script>

<style scoped>
* {
  font-family: "Poppins", sans-serif;
}

.error-message-hide {
  transition: all 0.5s ease-in-out;
  opacity: 0;
}

.error-message-show {
  opacity: 1;
}

#file-input-button {
  transition: background-color 0.5s ease-in-out;
  border: 1px solid #ccc;
  background-color: #ffff;
  display: inline-block;
  padding: 6px 12px;
  cursor: pointer;
  margin-top: 10px;
}

#file-input-button:hover {
  border: 1px solid #000000;
}

input[type=text] {
  margin-right: 1rem;
}

#button-label {
  font-size: 300%;
}

.center-content {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
}
</style>
