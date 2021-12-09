<template>
  <div class="flash" v-bind:class="flashClass">{{ displayMessage }}</div>
</template>

<script>
export default {
  props: {
    kind: {
      type: String,
      required: true,
    },
    message: {
      type: String,
      required: true,
    },
  },

  computed: {
    displayMessage() {
      return this.message.trim();
    },

    flashClass() {
      if (this.kind === "info") {
        return "flash-info";
      } else {
        return "flash-error";
      }
    },
  },
};
</script>

<style></style>
