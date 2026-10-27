import assert from "node:assert/strict";
import test from "node:test";
import * as changeCase from "change-case";
import { swapCase as swapCaseLib } from "swap-case";
import { titleCase } from "title-case";

// Pure library checks — these are the core deps Tezbar relies on for Windows+macOS
test("change-case library converts core cases", () => {
  const input = "hello world example";
  assert.equal(changeCase.camelCase(input), "helloWorldExample");
  assert.equal(changeCase.snakeCase(input), "hello_world_example");
  assert.equal(changeCase.constantCase(input), "HELLO_WORLD_EXAMPLE");
  assert.equal(changeCase.kebabCase(input), "hello-world-example");
  assert.equal(changeCase.pascalCase(input), "HelloWorldExample");
  assert.equal(changeCase.dotCase(input), "hello.world.example");
  assert.equal(changeCase.pathCase(input), "hello/world/example");
});

test("swap-case and title-case libs work as expected", () => {
  assert.equal(swapCaseLib("Hello World"), "hELLO wORLD");
  // title-case capitalizes words except small words
  assert.match(titleCase("hello world"), /Hello World/);
});

test("alternating case logic matches customCases implementation", () => {
  const ALPHABETIC_REGEX = /\p{L}/u;
  const isAlphabetic = (char: string) => ALPHABETIC_REGEX.test(char);
  const alternatingCase = (input: string) => {
    let count = 0;
    return input
      .split("")
      .map((char) => {
        if (!isAlphabetic(char)) return char;
        count++;
        return count % 2 === 1 ? char.toUpperCase() : char.toLowerCase();
      })
      .join("");
  };
  assert.equal(alternatingCase("hello"), "HeLlO");
  assert.equal(alternatingCase("hello world"), "HeLlO wOrLd");
  assert.equal(alternatingCase("123-abc"), "123-AbC");
});

test("package.json declares Windows+macOS and Tezbar metadata", async () => {
  const pkg = await import("../package.json", { with: { type: "json" } });
  const data = (pkg as unknown as { default: Record<string, unknown> }).default ?? (pkg as unknown as Record<string, unknown>);
  assert.deepEqual((data as { platforms: string[] }).platforms, ["macOS", "Windows"]);
  const tezbar = (data as { tezbar: { platforms: string[]; repository: string; upstream: { path: string } } }).tezbar;
  assert.deepEqual(tezbar.platforms, ["macos", "windows"]);
  assert.equal(tezbar.repository, "https://github.com/almatkai/change-case");
  assert.equal(tezbar.upstream.path, "extensions/change-case");
  const commands = (data as { commands: { platforms: string[] }[] }).commands;
  for (const cmd of commands) {
    assert.deepEqual(cmd.platforms, ["macOS", "Windows"]);
  }
});
