const rust = import("./pkg");
const convex_hulls = require("./convex_hulls.json");

rust
  .then((m) => {
    m.draw_convex_hulls(400, 300, 5, convex_hulls);
  })
  .catch(console.error);
