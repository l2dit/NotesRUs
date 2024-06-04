<template>
  <div class="center-content">
    <form @submit.prevent="delete_file">
      <label id="button-label">Delete markdown.</label>
      <br />
      <div class="delete_input_container">
        <input type="text" id="file-id" placeholder="File id" />
        <button type="submit" id="file-input-button">Delete</button>
      </div>
    </form>
    <p id="error-message" class="error-message-hide">{{ error_msg }}</p>
  </div>
</template>

<script>
import axios from 'axios';
let route;
if (window.location.origin == "http://localhost:5173") {
  route = "http://localhost:3000";
} else {
  route = "https://notesrus.nzdev.org";
}

export default {
  data() {
    return {
      /* Error is just a placeholder so that the p tag when text added does not move the whole upload section upwards */
      error_msg: 'Error'
    };
  },
  methods: {
    async delete_file() {
      /* gets the id of the file the user wants to delete */
      const file_id = document.getElementById("file-id").value;
      /* route to make requests to */
      const delete_route = `${route}/api/file/delete/${file_id}`;
      /* make delete request to the route */
      axios.delete(delete_route)
        .then(response => {
          this.error_msg = `File with id ${file_id} has been deleted.`;
          document.getElementById("error-message").classList.add("error-message-show")
          setTimeout(() => { document.getElementById("error-message").classList.remove("error-message-show") }, 3000);
          console.log(response)
        })
        .catch(error => {
          this.error_msg = `Error file failed to Delete: ${error.message}`;
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

#file-id {
  transition: all 0.3s ease-in-out;
  height: 31px;
  border: 2px solid gray;
  border-right: none;
  margin-right: 0px !important;
}

#file-id:focus {
  border: 2.1px solid rgb(239, 222, 164);
  border-right: none;
  box-shadow: 0 0 11px rgba(33, 33, 33, .2);
  outline: none;
}

#file-id:focus+#file-input-button {
  border: 2.375px solid rgb(239, 222, 164);
  border-left: none;
  box-shadow: 0 0 11px rgba(33, 33, 33, .2);
  outline: none;
}

#file-input-button {
  transition: all 0.3s ease-in-out;
  color: rgba(0, 0, 0, 0.7);
  display: inline-block;
  padding: 6px 12px;
  cursor: pointer;
  margin-top: 10px;
  background-color: #ffffff00;
  border: 2px solid gray;
  border-left: none;
  background-color: rgb(255, 174, 35);
}

#file-input-button:hover {
  color: rgb(255, 255, 255);
  background-color: rgb(255, 102, 0);
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
