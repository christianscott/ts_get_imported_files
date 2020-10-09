import { lex } from "./lex";
import { parse } from "./parse";
import { Resolver } from "./resolver";
import { Source } from "./token";

const source = new Source("my_module", "import foo from 'bar';");
const tokens = lex(source);

const deps = parse(tokens);

const resolver = new Resolver("tsconfig.json", [".ts", ".ts"]);
console.log(deps.map((dep) => resolver.resolve(dep)));
