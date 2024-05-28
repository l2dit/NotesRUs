<template>
  <div class="center-content">
    <form @submit.prevent="download_file">
      <label id="button-label">Download markdown.</label>
      <br />
      <input type="text" id="file-id" placeholder="File id" />
      <button type="submit" id="file-input-button">Download</button>
    </form>
    <p id="error-message" :class="{ 'error-message-show': error_msg, 'error-message-hide': !error_msg }">
      {{ error_msg }}
    </p>
  </div>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      error_msg: ''
    };
  },
  methods: {
    download_file() {
      const file_id = document.getElementById("file-id").value;
      const get_route = `https://notesrus.nzdev.org/api/file/download/${file_id}`;
      axios.get(get_route, { responseType: 'blob' })
        .then(response => {
          const url = window.URL.createObjectURL(new Blob([response.data]));
          const link = document.createElement('a');
          link.href = url;
          link.setAttribute('download', "markdown.md");
          document.body.appendChild(link);
          link.click();
          document.body.removeChild(link);
          window.URL.revokeObjectURL(url);
        })
        .catch(error => {
          this.error_msg = 'Error: Unable to download the file.';
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
