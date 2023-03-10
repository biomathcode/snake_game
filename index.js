const compression = require("compression");

const express = require("express");

const path = require("path");

const app = express();
const port = process.env.PORT || 3000;

const public = path.join(__dirname, "public");

app.use(compression());

app.use(express.static(public));

app.get("*", (req, res) => {
  res.sendFile("index.html", { root: public });
});

app.listen(port, () => {
  console.log("server is running");
});

module.exports = app;
