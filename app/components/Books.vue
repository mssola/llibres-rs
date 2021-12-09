<template>
  <div>
    <div v-if="page == INDEX">
      <button @click="page = NEW">NEW</button>

      <table>
        <tr>
          <th>Title</th>
          <th>Author</th>
        </tr>

        <tr v-for="book in books" :key="book.id">
          <td>{{ title_plus_super(book) }}</td>
          <td>{{ nullable(book.author) }}</td>
        </tr>
      </table>
    </div>
    <div v-else-if="page == NEW">
      <NewBook
        @flash="flash"
        @created="created"
        :completion="completion"
      ></NewBook>
    </div>
  </div>
</template>

<script>
import axios from "axios";
import NewBook from "./NewBook.vue";
import types from "../utils/types";

const INDEX = 0;
const NEW = 1;

export default {
  name: "Books",

  components: {
    NewBook,
  },

  data() {
    return {
      page: INDEX,
      books: [],
      authors: new Set(),
      locations: new Set(),
      INDEX: INDEX,
      NEW: NEW,
      languages: {
        ca: "Catalan",
        es: "Spanish",
        en: "English",
        grc: "Ancient Greek",
        la: "Latin",
        it: "Italian",
      },
    };
  },

  mounted() {
    axios({
      method: "GET",
      url: "/books",
      headers: {
        "Content-Type": "application/json",
      },
    }).then((resp) => {
      resp.data.forEach((book) => {
        if (!types.isblank(book.author)) {
          this.authors.add(book.author.trim());
        }
        if (!types.isblank(book.location)) {
          this.locations.add(book.location.trim());
        }
      });

      this.books = resp.data;
    });
  },

  computed: {
    completion() {
      return {
        authors: Array.from(this.authors),
        locations: Array.from(this.locations),
      };
    },
  },

  methods: {
    flash(code, message) {
      this.$emit("flash", code, message);
    },

    created(book) {
      this.books.push(book);
      this.page = INDEX;
    },

    title_plus_super(book) {
      if (types.isblank(book.supertitle)) {
        return book.title;
      }
      return `${book.title} (inside of '${book.supertitle}')`;
    },

    nullable(str) {
      if (types.isblank(str)) {
        return "-";
      }
      return str;
    },

    time_formatted(ts) {
      if (ts === null) {
        return "-";
      }

      let t = new Date(ts);
      let day = this.two_digits(t.getDate());
      let month = this.two_digits(t.getMonth() + 1);
      let year = t.getFullYear();

      return `${day}-${month}-${year}`;
    },

    two_digits(n) {
      if (n < 10) {
        return `0${n}`;
      }
      return n;
    },

    present_languages(lang) {
      return lang
        .split(",")
        .map((l) => {
          let v = this.languages[l.trim()];

          if (types.isblank(v)) {
            return "";
          }
          return v;
        })
        .join(", ");
    },
  },
};
</script>

<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
