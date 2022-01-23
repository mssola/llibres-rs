<template>
  <div>
    <div class="field">
      <label for="title">Title</label>
      <input id="title" type="text" autocomplete="off" v-model="title" />
    </div>

    <div class="field">
      <label for="rate">Rate</label>
      <input
        id="rate"
        type="number"
        autocomplete="off"
        v-model="rate"
        min="0"
        max="10"
      />
    </div>

    <div class="field">
      <label for="status">Status</label>
      <select v-model="status">
        <option v-for="st in statuses" v-bind:key="st.key">
          {{ st.text }}
        </option>
      </select>
    </div>

    <div class="field">
      <label for="author">Author</label>
      <v-select
        id="author"
        v-model="author"
        :options="completion.authors"
        label="text"
        taggable
      ></v-select>
    </div>

    <div class="field">
      <label for="location">Location</label>
      <v-select
        id="location"
        v-model="location"
        :options="completion.locations"
        label="text"
        taggable
      ></v-select>
    </div>

    <div class="field">
      <label for="notes">Notes</label>
      <input id="notes" type="textarea" autocomplete="off" v-model="notes" />
    </div>

    <div class="field">
      <label for="kind">Kind</label>
      <select v-model="kind">
        <option v-for="k in kinds" v-bind:key="k.key" v-bind:value="k.value">
          {{ k.key }}
        </option>
      </select>
    </div>

    <div class="field">
      <label for="date">When did you buy it?</label>
      <input id="date" type="date" v-model="date" />
    </div>

    <div class="field">
      <button @click="createBook">Create</button>
    </div>

    <div class="field">
      <button @click="$emit('close')">Cancel</button>
    </div>
  </div>
</template>

<script>
import "vue-select/dist/vue-select.css";
import axios from "axios";
import vSelect from "vue-select";
import types from "../utils/types";

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
      kind: 0,
      status: 0,
      date: new Date().toISOString().substr(0, 10),
      statuses: [
        { key: 0, text: "Read" },
        { key: 1, text: "Not read" },
        { key: 2, text: "To buy" },
        { key: 3, text: "Selected" },
        { key: 4, text: "To be published" },
        { key: 5, text: "To reread" },
      ],
      kinds: [
        { key: "None", value: 0 },
        { key: "Poetry", value: 1 },
        { key: "Theater", value: 2 },
        { key: "Essay", value: 3 },
        { key: "Shorts", value: 4 },
      ],
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
        bought_at: new Date(this.date).toISOString(),
      };
      axios({
        method: "POST",
        url: `/books`,
        headers: {
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
          this.$emit("flash", "error", e.response.data);
        });
    },
  },
};
</script>
