<template>
  <div class="hidden">
    <!-- filter -->
    <span id="msg.en.filter">Filter</span>
    <span id="msg.ca.filter">Filtra</span>

    <!-- read -->
    <span id="msg.en.read">Read</span>
    <span id="msg.ca.read">Llegits</span>

    <!-- notread -->
    <span id="msg.en.notread">Not read</span>
    <span id="msg.ca.notread">No llegits</span>

    <!-- tobuy -->
    <span id="msg.en.tobuy">To buy</span>
    <span id="msg.ca.tobuy">Per comprar</span>

    <!-- selected -->
    <span id="msg.en.selected">Selected</span>
    <span id="msg.ca.selected">Seleccionats</span>

    <!-- tobepublished -->
    <span id="msg.en.tobepublished">To be published</span>
    <span id="msg.ca.tobepublished">Per ser publicats</span>

    <!-- notspecified -->
    <span id="msg.en.notspecified">Not specified</span>
    <span id="msg.ca.notspecified">Sense especificar</span>

    <!-- none -->
    <span id="msg.en.none">Any</span>
    <span id="msg.ca.none">General</span>

    <!-- poetry -->
    <span id="msg.en.poetry">Poetry</span>
    <span id="msg.ca.poetry">Poesia</span>

    <!-- theater -->
    <span id="msg.en.theater">Theater</span>
    <span id="msg.ca.theater">Teatre</span>

    <!-- essay -->
    <span id="msg.en.essay">Essay</span>
    <span id="msg.ca.essay">Assaig</span>

    <!-- shorts -->
    <span id="msg.en.shorts">Shorts</span>
    <span id="msg.ca.shorts">Curts</span>

    <!-- sortbyinterest -->
    <span id="msg.en.sortbyinterest">Sort by interest</span>
    <span id="msg.ca.sortbyinterest">Ordernar per interès</span>

    <!-- sortbydate -->
    <span id="msg.en.sortbydate">Sort by date</span>
    <span id="msg.ca.sortbydate">Ordernar per data</span>

    <!-- title -->
    <span id="msg.en.title">Title</span>
    <span id="msg.ca.title">Títol</span>

    <!-- kind -->
    <span id="msg.en.kind">Kind</span>
    <span id="msg.ca.kind">Tipus</span>

    <!-- author -->
    <span id="msg.en.author">Author</span>
    <span id="msg.ca.author">Autor</span>

    <!-- insideof -->
    <span id="msg.en.insideof">inside of</span>
    <span id="msg.ca.insideof">dins de</span>

    <!-- insideofup -->
    <span id="msg.en.insideofup">Inside of</span>
    <span id="msg.ca.insideofup">Dins de</span>

    <!-- publisher -->
    <span id="msg.en.publisher">Publisher</span>
    <span id="msg.ca.publisher">Editorial</span>

    <!-- language -->
    <span id="msg.en.language">Language</span>
    <span id="msg.ca.language">Idioma</span>

    <!-- location -->
    <span id="msg.en.location">Location</span>
    <span id="msg.ca.location">Lloc</span>

    <!-- rate -->
    <span id="msg.en.rate">Rate</span>
    <span id="msg.ca.rate">Qualificació</span>

    <!-- status -->
    <span id="msg.en.status">Status</span>
    <span id="msg.ca.status">Estat</span>

    <!-- new -->
    <span id="msg.en.new">New</span>
    <span id="msg.ca.new">Nou</span>

    <!-- update -->
    <span id="msg.en.update">Update</span>
    <span id="msg.ca.update">Actualitza</span>

    <!-- edit -->
    <span id="msg.en.edit">Edit</span>
    <span id="msg.ca.edit">Edita</span>

    <!-- cancel -->
    <span id="msg.en.cancel">Cancel</span>
    <span id="msg.ca.cancel">Cancel·la</span>

    <!-- delete -->
    <span id="msg.en.delete">Delete</span>
    <span id="msg.ca.delete">Elimina</span>

    <!-- confirm -->
    <span id="msg.en.confirm">Are you sure about this?</span>
    <span id="msg.ca.confirm">Estàs segur d'això?</span>
  </div>
</template>

<script>
import i18n from "../utils/i18n";

export default {
  name: "Language",

  mounted() {
    i18n.setCurrentLanguage();
  },
};
</script>
