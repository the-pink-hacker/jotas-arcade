// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import("../games/demo/pkg");
import $ from "jquery";

rust.then(m => {
  m.setup_game();
}).catch(reason => console.error("Game possible failed\n" + reason));
