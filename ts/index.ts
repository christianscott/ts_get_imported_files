import { lex } from "./lex";
import { parse } from "./parse";
import { Source } from "./token";

(() => {
  const deps = parse(lex(new Source("my_module", "import foo from 'bar';")));
  console.log(deps);
})();
