<template>
  <div id="centered-parent-div" class="center-content">
    <div id="form-file-preview-selector">
        <form @submit.prevent="preview_file">
        <label id="button-label">Preview markdown.</label>
        <br />
        <input type="text" id="file-id" placeholder="File id" />
        <button type="submit" id="file-download-button">Preview</button>
        </form>
        <p id="error-message" class="error-message-hide">{{ error_msg }}</p>
    </div>
    <html id="preview-contents"></html>
  </div>
</template>

<script>
import { marked } from 'marked';
import axios from 'axios';
import DOMPurify from 'dompurify';
export default {
  data() {
    return {
      /* Error is just a placeholder so that the p tag when text added does not move the whole upload section upwards */
      error_msg: 'Error',
      computedHeight: "95vh"
    };
  },
  methods: {
    async preview_file() {
        /* gets the id of the file the user wants to fetch */
      const file_id = document.getElementById("file-id").value;
      /* route to make requests to */
      const get_route = `https://notesrus.nzdev.org/api/file/download/${file_id}`;
      /* make get request to the route */
      axios.get(get_route)
        .then(response => {
          document.getElementById("form-file-preview-selector").style.visibility = "hidden";
          document.getElementById("preview-contents").innerHTML = DOMPurify.sanitize(marked.parse(response.data));
          console.log(response.data)
          document.getElementById("centered-parent-div").classList.remove("center-content");
          document.getElementById("preview-contents").classList.add("center-content-markdown");
        })
        .catch(error => {
          this.error_msg = 'Error: Unable to download the file.';
          document.getElementById("error-message").classList.add("error-message-show")
          setTimeout(() => {document.getElementById("error-message").classList.remove("error-message-show")}, 3000);
          console.error("Error:", error);
        });
    },
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

#file-download-button {
  transition: background-color 0.5s ease-in-out;
  border: 1px solid #ccc;
  background-color: #ffff;
  display: inline-block;
  padding: 6px 12px;
  cursor: pointer;
  margin-top: 10px;
}

#file-download-button:hover {
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
.center-content-markdown {
    background-color: #FFFFFF;
    display: flex;
    justify-content: center;
    /* text-align: center; */
    margin: 0 10% 10% 10%;
    
}
</style>
