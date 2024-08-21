<template>
</template>

<script>
import introJs from 'intro.js';

export default {
  name: 'IntroTour',
  mounted() {
    this.startTour();
  },
  methods: {
    startTour() {
      const intro = introJs();
      intro.setOption("dontShowAgain", true);
      intro.setOptions({
        steps: [
          {
            title: "Notesrus",
            intro: "Welcome to NotesRus 🗒️",
          },
          {
            element: document.querySelector('#app > .center-content'),
            title: "Notesrus",
            intro: "You can upload your <a href='https://www.google.com/url?sa=t&source=web&rct=j&opi=89978449&url=https://www.markdownguide.org/&ved=2ahUKEwju582U_4aIAxU51jgGHR9FJLAQFnoECAYQAQ&usg=AOvVaw1fohdJEEbL6kohiJ-Pimbe'>.md<a/> files here",
            position: "Right",
          },
          {
            element: document.querySelector('#app > nav:nth-child(1)'),
            intro: "Here's your navigation buttons",
            position: "Right",
          },
          {
            title: "How it Works",
            intro: "After uploading a file you will recieve an ID"
          },
          {
            title: "How it Works",
            intro: "This ID is used to access the file in the view/remove/download pages"
          },
          {
            title: "Sharing Notes",
            intro: "If you would like to share your note, just send them the ID and they will be able to access it"
          }
        ]
      });

      intro.start();
    }
  }
}
</script>
