import { promises as fs } from "fs";
import { lex } from "./lex";
import { parse } from "./parse";
import { Resolver } from "./resolver";
import { Source } from "./token";

export type GetImportedFilesOpts = { filename: string } & (
  | { resolver: Resolver }
  | {
      tsconfigFilePath: string;
      resolvableExtensions: string[];
    }
);

export async function getImportedFiles(
  opts: GetImportedFilesOpts
): Promise<string[]> {
  const source = new Source(
    opts.filename,
    await fs.readFile(opts.filename, { encoding: "utf8" })
  );
  const tokens = lex(source);
  const deps = parse(tokens);

  const resolver =
    "resolver" in opts
      ? opts.resolver
      : new Resolver(opts.tsconfigFilePath, opts.resolvableExtensions);
  return deps.map((dep) => resolver.resolve(dep));
}
