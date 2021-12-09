const path = require("path");

module.exports = {
  // Small trick so to tell Vue and Webpack that our JS application is in `app`
  // and not in `src` which is where the Rust backend resides.
  //
  // Taken from:
  // https://vuejsdevelopers.com/2019/03/18/vue-cli-3-rename-src-folder/
  chainWebpack: (config) => {
    config.entry("app").clear().add("./app/main.js").end();
    config.resolve.alias.set("@", path.join(__dirname, "./app"));
  },

  // Here it is where the Rust server will be running on. If that's not the case
  // for you, then change it. We need this just so we don't get CORS
  // restrictions and stuff like that.
  devServer: {
    proxy: "http://localhost:8080",
  },
};
