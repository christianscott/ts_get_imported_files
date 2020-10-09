import * as path from "path";
import { findTreeForFiles } from "../find_tree_for_files";
import { Resolver } from "../resolver";

const fixture = (file: string) => path.join(__dirname, "fixtures", file);

describe("findTreeForFiles", () => {
  test("works", () => {
    const resolver = new Resolver(fixture("one/tsconfig.json"), [".ts"]);
    expect(
      findTreeForFiles([fixture("one/src/mod2/index.ts")], resolver)
    ).toEqual([fixture("one/src/mod1")]);
  });
});
