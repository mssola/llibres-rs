<template>
  <div>
    <Flash
      @click="kind = ''"
      v-if="kind != ''"
      :message="sanitizedMessage"
      :kind="kind"
    ></Flash>
    <Language></Language>
    <Books @flash="flash"></Books>
  </div>
</template>

<script>
import Books from "./components/Books.vue";
import Flash from "./components/Flash.vue";
import Language from "./components/Language.vue";

export default {
  name: "App",
  components: {
    Books,
    Flash,
    Language,
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
