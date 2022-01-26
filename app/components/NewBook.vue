<template>
  <div class="flex justify-center">
    <div class="w-full max-w-xs">
      <h1 class="text-2xl my-6">{{ i18n.t("newbook") }}</h1>

      <div class="field">
        <label class="block text-gray-700 text-sm font-bold mb-1" for="title">{{
          i18n.t("title")
        }}</label>

        <input
          id="title"
          class="rounded border border-gray-400 w-full"
          type="text"
          autocomplete="off"
          v-model="title"
        />
      </div>

      <div class="field">
        <label
          class="block text-gray-700 text-sm font-bold mb-1"
          for="author"
          >{{ i18n.t("author") }}</label
        >
        <v-select
          id="author"
          v-model="author"
          class="bg-white w-full"
          :options="completion.authors"
          label="text"
          taggable
        ></v-select>
      </div>

      <div class="field">
        <label
          class="block text-gray-700 text-sm font-bold mb-1"
          for="publisher"
          >{{ i18n.t("publisher") }}</label
        >
        <v-select
          id="publisher"
          v-model="publisher"
          class="bg-white w-full"
          :options="completion.publishers"
          label="text"
          taggable
        ></v-select>
      </div>

      <div class="field">
        <label
          class="block text-gray-700 text-sm font-bold mb-1"
          for="language"
          >{{ i18n.t("language") }}</label
        >

        <input
          id="language"
          class="rounded border border-gray-400 w-full"
          type="text"
          autocomplete="off"
          v-model="language"
        />
      </div>

      <div class="field">
        <label
          class="block text-gray-700 text-sm font-bold mb-1"
          for="location"
          >{{ i18n.t("location") }}</label
        >
        <v-select
          id="location"
          v-model="location"
          class="bg-white w-full"
          :options="completion.locations"
          label="text"
          taggable
        ></v-select>
      </div>

      <div class="field">
        <label class="block text-gray-700 text-sm font-bold mb-1" for="rate">{{
          i18n.t("rate")
        }}</label>
        <input
          id="rate"
          class="rounded border border-gray-400 w-full"
          type="number"
          autocomplete="off"
          v-model="rate"
          min="0"
          max="10"
        />
      </div>

      <div class="field">
        <label
          class="block text-gray-700 text-sm font-bold mb-1"
          for="status"
          >{{ i18n.t("status") }}</label
        >
        <select
          id="status"
          class="rounded border border-gray-400 w-full"
          v-model="status"
        >
          <option
            v-for="(val, key) in statuses"
            v-bind:key="val"
            v-bind:value="val"
          >
            {{ i18n.t(key) }}
          </option>
        </select>
      </div>

      <div class="field">
        <label class="block text-gray-700 text-sm font-bold mb-1" for="notes">{{
          i18n.t("notes")
        }}</label>
        <textarea
          id="notes"
          class="rounded border border-gray-400 w-full resize-y"
          autocomplete="off"
          v-model="notes"
        />
      </div>

      <div class="field">
        <label class="block text-gray-700 text-sm font-bold mb-1" for="kind">{{
          i18n.t("kind")
        }}</label>
        <select
          id="kind"
          class="rounded border border-gray-400 w-full"
          v-model="kind"
        >
          <option
            v-for="(val, key) in kinds"
            v-bind:key="val"
            v-bind:value="val"
          >
            {{ i18n.t(key) }}
          </option>
        </select>
      </div>

      <div class="field">
        <label class="block text-gray-700 text-sm font-bold mb-1" for="date">{{
          i18n.t("boughtat")
        }}</label>
        <input
          id="date"
          class="rounded border border-gray-400 w-full"
          type="date"
          v-model="date"
        />
      </div>

      <div class="field">
        <button
          id="create"
          class="font-semibold py-2 px-4 rounded cursor-pointer bg-green-500 text-white border border-green-700 hover:bg-green-700 w-full mt-2 mb-2"
          @click="createBook"
        >
          {{ i18n.t("create") }}
        </button>

        <button
          class="font-semibold py-2 px-4 rounded cursor-pointer bg-white text-red-800 border border-red-700 hover:bg-red-700 hover:border-red-700 hover:text-white w-full"
          @click="$emit('close')"
        >
          {{ i18n.t("cancel") }}
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import "vue-select/dist/vue-select.css";
import axios from "axios";
import vSelect from "vue-select";
import types from "../utils/types";
import i18n from "../utils/i18n";
import books from "../utils/books";

export default {
  name: "NewBook",

  components: {
    vSelect,
  },

  props: {
    completion: Object,
  },

  data() {
    return {
      title: "",
      rate: 0,
      location: "",
      author: "",
      notes: "",
      publisher: "",
      language: "",
      kind: 0,
      status: 0,
      date: new Date().toISOString().substr(0, 10),
      statuses: books.STATUSES,
      kinds: books.KINDS,
      i18n: i18n,
    };
  },

  methods: {
    createBook() {
      let book = {
        title: this.title.trim(),
        rate: this.rate,
        status: this.status,
        location: types.isblank(this.location) ? "" : this.location.trim(),
        author: types.isblank(this.author) ? "" : this.author.trim(),
        notes: this.notes.trim(),
        kind: this.kind,
        publisher: this.publisher,
        language: this.language,
        bought_at: new Date(this.date).toISOString(),
      };
      axios({
        method: "POST",
        url: `/books`,
        headers: {
          "Accept-Language": i18n.getLanguage(),
          "Content-Type": "application/json",
          Accept: "application/json",
        },
        data: book,
      })
        .then((resp) => {
          book.id = resp.data;
          this.$emit("created", book);
        })
        .catch((e) => {
          if (e.response.status === 400) {
            this.$emit(
              "flash",
              "error",
              `${i18n.t("bad")}: ${e.response.data.message}.`
            );
          } else {
            this.$emit("flash", "error", i18n.t("error"));
          }
        });
    },
  },
};
</script>

<style>
.field {
  @apply mb-4;
}

.vs__open-indicator {
  fill: rgba(0, 0, 0, 1);
  transform: scale(0.6);
}

.vs__search {
  @apply py-1 px-1;
}

#status,
#kind,
textarea,
select,
input[type="text"],
input[type="date"],
input[type="number"] {
  @apply rounded border border-gray-400 py-2 px-2 text-gray-800;
}

input[type="number"] {
  -moz-appearance: textfield;
  appearance: textfield;
  margin: 0;
}

input[type="number"]::-webkit-inner-spin-button,
input[type="number"]::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
</style>
