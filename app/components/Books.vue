<template>
  <div>
    <div v-if="page == INDEX">
      <div class="grid justify-center">
        <div class="max-w-sm md:max-w-5xl">
          <div
            class="filter grid grid-flow-col auto-cols-max gap-2 justify-left"
          >
            <div>
              <input
                type="text"
                class="rounded border border-gray-400 px-2 py-1"
                v-model="filter"
                :placeholder="`${i18n.t('filter')}...`"
              />
            </div>

            <div>
              <select v-model="status">
                <option
                  v-for="(val, key) in statuses"
                  v-bind:key="val"
                  v-bind:value="val"
                >
                  {{ i18n.t(key) }}
                </option>
              </select>
            </div>

            <div>
              <select v-model="kind">
                <option :value="-1">{{ i18n.t("notspecified") }}</option>
                <option
                  v-for="(val, key) in kinds"
                  v-bind:key="val"
                  v-bind:value="val"
                >
                  {{ i18n.t(key) }}
                </option>
              </select>
            </div>

            <div>
              <select v-model="sortingAlgorithm">
                <option value="0">{{ i18n.t("sortbyinterest") }}</option>
                <option value="1">{{ i18n.t("sortbydate") }}</option>
              </select>
            </div>

            <div>
              <button
                class="font-semibold py-2 px-4 rounded cursor-pointer bg-green-500 text-white border border-green-700 hover:bg-green-700"
                @click="page = NEW"
              >
                {{ i18n.t("new") }}
              </button>
            </div>
          </div>
        </div>

        <div class="max-w-sm md:max-w-5xl">
          <table>
            <tr class="border-b border-gray-400">
              <th>{{ i18n.t("kind") }}</th>
              <th>{{ i18n.t("title") }}</th>
              <th>{{ i18n.t("author") }}</th>
            </tr>

            <tr v-for="book in filterList()" :key="book.id">
              <td>
                <div class="flex items-center justify-center">
                  <div
                    v-if="book.kind == kinds.none"
                    class="w-2 h-2 border border-gray-400 rounded-full"
                    :title="`${i18n.t('none')}`"
                  ></div>
                  <div
                    v-else-if="book.kind == kinds.poetry"
                    class="w-2 h-2 border border-gray-400 bg-yellow-400 rounded-full"
                    :title="`${i18n.t('poetry')}`"
                  ></div>
                  <div
                    v-else-if="book.kind == kinds.theater"
                    class="w-2 h-2 border border-gray-400 bg-purple-400 rounded-full"
                    :title="`${i18n.t('theater')}`"
                  ></div>
                  <div
                    v-else-if="book.kind == kinds.essay"
                    class="w-2 h-2 border border-gray-400 bg-red-400 rounded-full"
                    :title="`${i18n.t('essay')}`"
                  ></div>
                  <div
                    v-else-if="book.kind == kinds.shorts"
                    class="w-2 h-2 border border-gray-400 bg-blue-400 rounded-full"
                    :title="`${i18n.t('shorts')}`"
                  ></div>
                </div>
              </td>
              <td
                class="px-4 py-1 text-left cursor-pointer"
                @click="showDescription(book)"
              >
                {{ title_plus_super(book) }}
              </td>
              <td
                class="px-2 py-1 text-left cursor-pointer"
                @click="showDescription(book)"
              >
                {{ nullable(book.author) }}
              </td>
            </tr>
          </table>
        </div>
      </div>

      <Modal
        v-if="isModalVisible"
        :book="this.selectedBook"
        @close="isModalVisible = false"
        @updateBook="updateBook"
        @deleteBook="deleteBook"
      ></Modal>
    </div>
    <div v-else-if="page == NEW">
      <NewBook
        @flash="flash"
        @created="created"
        @close="page = INDEX"
        :completion="completion"
      ></NewBook>
    </div>
  </div>
</template>

<script>
import axios from "axios";
import NewBook from "./NewBook.vue";
import Modal from "./Modal.vue";
import types from "../utils/types";
import ubooks from "../utils/books";
import i18n from "../utils/i18n";

const INDEX = 0;
const NEW = 1;

export default {
  name: "Books",

  components: {
    NewBook,
    Modal,
  },

  data() {
    return {
      page: INDEX,
      books: [],
      authors: new Set(),
      locations: new Set(),
      INDEX: INDEX,
      NEW: NEW,
      statuses: ubooks.STATUSES,
      kinds: ubooks.KINDS,
      sortingAlgorithm: 0,
      i18n: i18n,
      filter: "",
      status: ubooks.STATUSES.notread,
      kind: -1,
      isModalVisible: false,
      selectedBook: null,
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
      return `${book.title} (${i18n.t("insideof")} '${book.supertitle}')`;
    },

    bookSort(b1, b2) {
      if (this.sortingAlgorithm === 0) {
        if (b1.rate === b2.rate) {
          return b1.title
            .toLocaleLowerCase()
            .localeCompare(b2.title.toLocaleLowerCase());
        }
        return b1.rate - b2.rate;
      }

      if (b1.bought_at === b2.bought_at && !types.isnull(b1)) {
        return b1.title
          .toLocaleLowerCase()
          .localeCompare(b2.title.toLocaleLowerCase());
      }
      return b1.bought_at > b2.bought_at;
    },

    filterList() {
      return this.books
        .filter((book) => {
          if (this.status !== book.status) {
            return false;
          }
          if (this.kind >= 0 && this.kind !== book.kind) {
            return false;
          }

          if (types.isblank(this.filter)) {
            return true;
          }
          return (
            book.title
              .toLocaleLowerCase()
              .includes(this.filter.toLocaleLowerCase()) ||
            book.author
              .toLocaleLowerCase()
              .includes(this.filter.toLocaleLowerCase())
          );
        })
        .sort(this.bookSort);
    },

    filterBy(status) {
      return this.books
        .filter((book) => {
          return book.status === status;
        })
        .sort(this.bookSort);
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

    showDescription(book) {
      this.selectedBook = book;
      this.isModalVisible = true;
    },

    updateBook(book) {
      // TODO: handle error
      axios({
        method: "PUT",
        url: `/books/${book.id}`,
        headers: {
          "Content-Type": "application/json",
        },
        data: {
          title: book.title,
          supertitle: book.supertitle,
          rate: book.rate,
          status: book.status,
          location: book.location,
          author: book.author,
          publisher: book.publisher,
          language: book.language,
          notes: book.notes,
          kind: book.kind,
          bought_at: book.bought_at,
        },
      }).then(() => {
        for (let i = 0; i < this.books.length; i++) {
          if (this.books[i].id === book.id) {
            this.books[i] = { ...book };
          }
        }

        this.isModalVisible = false;
      });
    },

    deleteBook(book) {
      console.log(book);

      this.isModalVisible = false;
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
