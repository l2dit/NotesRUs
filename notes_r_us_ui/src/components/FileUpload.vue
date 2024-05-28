<template>
  <div>
    <form @submit.prevent="handle_submit">
      <label>Upload markdown file.</label>
      <br />
      <input type="file" @change="handle_file_change" accept=".md, .markdown" />
    </form>
    <p v-if="file_content">{{ file_content }}</p>
  </div>
</template>

<script>
import axios from 'axios'
let post_route = window.location.hostname.concat("/file/upload");
export default {
  data() {
    return {
      file_content: ''
    };
  },
  methods: {
    handle_file_change(event) {
      const file = event.target.files[0];
      const fileForm = new FormData();
      fileForm.append("file", file)
      if (file) {

      axios.post(post_route, {
        file: fileForm
      }, {
        headers: {
          'Content-Type': 'text/md'
        }
      })

        const file_reader = new FileReader();
        file_reader.onload = (e) => {
          this.file_content = e.target.result;
        };
        file_reader.readAsText(file);
      }
    },
    handle_submit() {
      // put other form submit stuff here
      console.log('form submitted succesfully');
    }
  }
};
</script>

<style scoped>
* {
  font-family: "Poppins", sans-serif;
  padding-left: 5%;
}
</style>
