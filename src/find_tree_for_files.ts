import * as fs from "fs";
import { lex } from "./lex";
import { parse } from "./parse";
import { Resolver } from "./resolver";
import { Source } from "./token";

export function findTreeForFiles(
  files: readonly string[],
  resolver: Resolver
): readonly string[] {
  const resolved = new Set<string>();
  for (const file of files) {
    const contents = fs.readFileSync(file, { encoding: "utf-8" });
    const deps = parse(lex(new Source(file, contents)));
    for (const dep of deps) {
      resolved.add(resolver.resolve(dep));
    }
  }
  return [...resolved];
}
