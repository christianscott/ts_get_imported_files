import * as tsConfigPaths from "tsconfig-paths";
import { Dependency } from "./parse";
import { Preconditions } from "./preconditions";

export class Resolver {
  private matchPath: tsConfigPaths.MatchPath;

  constructor(
    tsConfigFilePath: string,
    private readonly resolvableExtensions: readonly string[]
  ) {
    const loadResult = tsConfigPaths.loadConfig(tsConfigFilePath);
    if (loadResult.resultType === "failed") {
      throw new Error(loadResult.message);
    }

    this.matchPath = tsConfigPaths.createMatchPath(
      loadResult.absoluteBaseUrl,
      loadResult.paths
    );
  }

  resolve(dep: Dependency): string | undefined {
    const path = Preconditions.checkExists(dep.path.lexeme);
    return this.matchPath(
      path,
      undefined,
      undefined,
      this.resolvableExtensions
    );
  }
}
