<template>
  <div class="center-content">
    <form>
      <label id="button-label">Upload markdown.</label>
      <br />
      <label id="file-input-button" for="file-input">Upload</label>
      <input id="file-input" type="file" @change="handle_file_change" accept=".md, .markdown" />
    </form>
    <p id="error-message" class="error-message-hide">{{ error_msg }}</p>
  </div>
</template>

<script>
import axios from 'axios';
let post_route = "https://notesrus.nzdev.org/api/file/upload";

export default {
  data() {
    return {
      /* Error is just a placeholder so that the p tag when text added does not move the whole upload section upwards */
      error_msg: 'Error'
    };
  },
  methods: {
    handle_file_change(event) {
      /* Get file from upload buttom */
      const file = event.target.files[0];
      if (file) {
        /* create form to put file in */
        const fileForm = new FormData();
        /* add file to form */
        fileForm.append("file", file);
        /* make a post request to the api with the file form */
        axios.post(post_route, fileForm, {
          headers: {
            'Content-Type': 'multipart/form-data',
          }
        })
          .then(response => {
            this.set_background_success();
            this.error_msg = `File uploaded successfully with File Id: ${response.data}`;
            console.log('File uploaded successfully:', response.data);
          })
          .catch(error => {
            this.set_background_error();
            this.error_msg = `File failed to upload: ${error.message}`;
            console.log('Error uploading file:', error.message);

          });
      }
    },
    /* change button background to green */
    set_background_success() {
      const button = document.getElementById("file-input-button");
      document.getElementById('error-message').classList.toggle('error-message-show');
      button.classList.remove("error-background");
      button.classList.add("success-background");
      setTimeout(this.remove_background, 5000)
    },
    /* reset button to normal colour */
    remove_background() {
      document.getElementById('error-message').classList.toggle('error-message-show');
      const button = document.getElementById("file-input-button");
      button.classList.remove("success-background");
      button.classList.remove("error-background");

    },
    /* set button background to red */
    set_background_error() {
      const button = document.getElementById("file-input-button");
      document.getElementById('error-message').classList.toggle('error-message-show');
      button.classList.remove("success-background");
      button.classList.add("error-background");
      setTimeout(this.remove_background, 3000)
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

#file-input-button {
  transition: background-color 0.5s ease-in-out;
  border: 1px solid #ccc;
  display: inline-block;
  padding: 6px 12px;
  cursor: pointer;
  margin-top: 10px;
}

#file-input-button:hover {
  border: 1px solid #000000;
}

#button-label {
  font-size: 300%;
}

div form input[type=file] {
  display: none;
}

.success-background {
  background-color: #28a745;
}

.error-background {
  background-color: #dc3545;
}

.center-content {
  position: absolute;
  top: 53%;
  left: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
}
</style>
