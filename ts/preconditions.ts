export module Preconditions {
  export function checkExists<T>(value: T | undefined | null): T {
    if (value != null) {
      return value;
    }
    throw new Error(`expected value to be non-null`);
  }
}
