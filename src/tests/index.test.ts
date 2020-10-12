import * as path from "path";
import { getImportedFiles } from "../index";

const fixture = (file: string) => path.join(__dirname, "fixtures", file);

describe("findTreeForFiles", () => {
  test("works", async () => {
    const importedFiles = getImportedFiles({
      filename: fixture("one/src/mod2/index.ts"),
      resolvableExtensions: [".ts"],
      tsconfigFilePath: fixture("one/tsconfig.json"),
    });
    await expect(importedFiles).resolves.toEqual([fixture("one/src/mod1")]);
  });
});
