<template>
  <div>
    <form @submit.prevent="handle_submit">
      <label>Upload markdown.</label>
      <br />
      <label id="file-input-button"for="file-input" >Upload</label>
      <input id="file-input"type="file" @change="handle_file_change" accept=".md, .markdown"/>
    </form>
    <p>{{ success }}</p>
  </div>
</template>

<script>
import axios from 'axios';

let post_route = "https://notesrus.nzdev.org/api/file/upload";

export default {
  data() {
    return {
      success: ''
    };
  },
  methods: {
    handle_file_change(event) {
      const file = event.target.files[0];
      if (file) {
        const fileForm = new FormData();
        fileForm.append("file", file);

        axios.post(post_route, fileForm, {
          headers: {
            'Content-Type': 'text/markdown'
          }
        })
        .then(response => {
          this.success = "File uploaded successfully";
          console.log('File uploaded successfully:', response.data);
        })
        .catch(error => {
          this.success = `File failed to upload ${error}`;
          console.error('Error uploading file:', error);
        });
      }
    },
    handle_submit() {
      console.log('Form submitted successfully');
    }
  }
};
</script>

<style scoped>
* {
  font-family: "Poppins", sans-serif;
  padding-left: 5%;
}
div form input[type=file] {
  display: none;
}
#file-input-button {
  transition: all 0.1s;
  border: 1px solid #ccc;
  display: inline-block;
  padding: 6px 12px;
  cursor: pointer;
}

#file-input-button:hover {
  transition: all 0.3s;
  border: 1px solid #000000;
  display: inline-block;
  padding: 6px 12px;
  cursor: pointer;
}
</style>

