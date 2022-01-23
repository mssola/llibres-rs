<template>
  <transition name="modal-fade">
    <div class="modal-backdrop">
      <div
        class="modal"
        role="dialog"
        aria-labelledby="modalTitle"
        aria-describedby="modalDescription"
      >
        <header class="modal-header" id="modalTitle">
          <button
            type="button"
            class="btn-close"
            @click="close"
            aria-label="Close modal"
          >
            x
          </button>
        </header>

        <section class="modal-body" id="modalDescription">
          <table class="min-w-full table-auto">
            <tr>
              <td class="w-fit"><b>Title</b>:</td>
              <td class="w-full" v-if="editing">
                <input v-model="b.title" class="min-w-full" type="text" />
              </td>
              <td class="w-full pl-2" v-else>{{ b.title }}</td>
            </tr>

            <tr>
              <td class="w-fit"><b>Inside of</b>:</td>
              <td class="w-full" v-if="editing">
                <input v-model="b.supertitle" class="min-w-full" type="text" />
              </td>
              <td class="w-full pl-2" v-else>
                {{ nullable_text(b.supertitle) }}
              </td>
            </tr>

            <tr>
              <td class="w-fit"><b>Publisher</b>:</td>
              <td class="w-full" v-if="editing">
                <input v-model="b.publisher" class="min-w-full" type="text" />
              </td>
              <td class="w-full pl-2" v-else>{{ b.publisher }}</td>
            </tr>

            <tr>
              <td class="w-fit"><b>Language</b>:</td>
              <td v-if="editing">
                <input v-model="b.language" type="text" />
              </td>
              <td class="pl-2" v-else>{{ b.language }}</td>
            </tr>

            <tr>
              <td class="w-fit"><b>Location</b>:</td>
              <td v-if="editing">
                <input v-model="b.location" type="text" />
              </td>
              <td class="pl-2" v-else>{{ b.location }}</td>
            </tr>

            <tr>
              <td class="w-fit"><b>Rate</b>:</td>
              <td v-if="editing">
                <input
                  v-model="b.rate"
                  type="number"
                  autocomplete="off"
                  min="0"
                  max="10"
                />
              </td>
              <td class="pl-2" v-else>{{ b.rate }}</td>
            </tr>

            <tr>
              <td class="w-fit"><b>Status</b>:</td>
              <td v-if="editing">
                <select
                  v-model="b.status"
                  class="rounded border border-gray-400 py-2 px-4 text-gray-800"
                >
                  <option
                    v-for="(val, key) in statuses"
                    v-bind:key="val"
                    v-bind:value="val"
                  >
                    {{ key }}
                  </option>
                </select>
              </td>
              <td class="pl-2" v-else>{{ text_from(b.status, statuses) }}</td>
            </tr>

            <tr>
              <td class="w-fit"><b>Kind</b>:</td>
              <td v-if="editing">
                <select
                  v-model="b.kind"
                  class="rounded border border-gray-400 py-2 px-4 text-gray-800"
                >
                  <option
                    v-for="(val, key) in kinds"
                    v-bind:key="val"
                    v-bind:value="val"
                  >
                    {{ key }}
                  </option>
                </select>
              </td>
              <td class="pl-2" v-else>{{ text_from(b.kind, kinds) }}</td>
            </tr>
          </table>
        </section>

        <footer class="modal-footer">
          <button
            v-if="editing"
            class="font-semibold py-2 px-4 rounded cursor-pointer bg-blue-500 text-white border border-blue-700 hover:bg-blue-700"
            @click="$emit('updateBook', b)"
          >
            Update
          </button>

          <button
            v-else
            class="font-semibold py-2 px-4 rounded bg-white text-gray-800 border border-gray-400 hover:bg-gray-100"
            @click="editing = true"
          >
            Edit
          </button>

          <button
            v-if="editing"
            class="font-semibold py-2 px-4 rounded cursor-pointer bg-white text-red-800 border border-red-700 hover:bg-red-700 hover:border-red-700 hover:text-white"
            @click="editing = false"
          >
            Cancel
          </button>

          <button
            v-else
            class="font-semibold py-2 px-4 rounded cursor-pointer bg-white text-red-800 border border-red-700 hover:bg-red-700 hover:border-red-700 hover:text-white"
            @click="$emit('deleteBook', book)"
          >
            Delete
          </button>
        </footer>
      </div>
    </div>
  </transition>
</template>

<script>
import books from "../utils/books";
import types from "../utils/types";

export default {
  name: "Modal",
  props: {
    book: Object,
  },

  data() {
    return {
      statuses: books.STATUSES,
      kinds: books.KINDS,
      editing: false,
      b: { ...this.book },
    };
  },

  mounted() {
    this.b = { ...this.book };
  },

  methods: {
    close() {
      this.$emit("close");
    },

    text_from(value, ary) {
      let k = Object.keys(ary).find((key) => ary[key] === value);

      return k ? k : "-";
    },

    nullable_text(value) {
      return types.isblank(value) ? "-" : value;
    },
  },
};
</script>

<style>
.modal-backdrop {
  position: fixed;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  background-color: rgba(0, 0, 0, 0.3);
  display: flex;
  justify-content: center;
  align-items: center;
}

.modal {
  background: #ffffff;
  box-shadow: 2px 2px 20px 1px;
  overflow-x: auto;
  display: flex;
  flex-direction: column;
  min-width: 500px;
  text-overflow: ellipsis;
}

.modal-header,
.modal-footer {
  display: flex;
}

.modal-header {
  position: relative;
  padding: 15px;
  justify-content: space-between;
}

.modal-footer {
  padding: 20px 10px;
  flex-direction: column;
}

.modal-body {
  position: relative;
  padding: 20px 10px;
}

.btn-close {
  position: absolute;
  top: 0;
  right: 0;
  border: none;
  font-size: 20px;
  padding: 10px;
  cursor: pointer;
  font-weight: bold;
  background: transparent;
}

.modal-fade-enter,
.modal-fade-leave-to {
  opacity: 0;
}

.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.5s ease;
}

input[type="text"],
input[type="number"] {
  @apply rounded border border-gray-400 py-2 px-4 text-gray-800;
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
