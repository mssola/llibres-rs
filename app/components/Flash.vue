<template>
  <div class="flex justify-center">
    <div
      class="flash py-2 px-4 rounded cursor-pointer"
      :title="`${i18n.t('clickmetoclose')}...`"
      v-bind:class="flashClass"
    >
      {{ displayMessage }}
    </div>
  </div>
</template>

<script>
import i18n from "../utils/i18n";

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

  data() {
    return {
      i18n: i18n,
    };
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

<style>
.flash-info {
  @apply bg-white text-green-800 border border-green-700;
}

.flash-error {
  @apply bg-white text-red-800 border border-red-700;
}
</style>
