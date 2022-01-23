<template>
  <div>
    <Flash v-if="kind != ''" :message="sanitizedMessage" :kind="kind"></Flash>
    <Books @flash="flash"></Books>
  </div>
</template>

<script>
import Books from "./components/Books.vue";
import Flash from "./components/Flash.vue";

export default {
  name: "App",
  components: {
    Books,
    Flash,
  },

  data() {
    return {
      message: "",
      kind: "",
    };
  },

  methods: {
    flash(kind, message) {
      this.kind = kind;
      this.message = message;
    },
  },

  computed: {
    sanitizedMessage() {
      if (this.message === "") {
        return "Unknown error";
      } else {
        return this.message;
      }
    },
  },
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>
