const proxy = require("http-proxy-middleware");
module.exports = function(app) {
  app.use(
    "/api/graphql",
    proxy({
      target: "http://dev.fly.me",
      changeOrigin: true
    })
  );
};
