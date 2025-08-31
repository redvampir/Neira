/* neira:meta
id: NEI-20250101-000001-organ-template-test
intent: test
summary: |
  Validate example organs against schema.
*/
import { readFileSync, readdirSync, statSync } from "node:fs";
import path from "node:path";
import Ajv from "ajv/dist/2020.js";

describe("organ template examples", () => {
  const schema = JSON.parse(
    readFileSync("schemas/organ-template.schema.json", "utf8"),
  );
  const ajv = new Ajv({ strict: false });
  const validate = ajv.compile(schema);

  function collectFiles(dir: string): string[] {
    return readdirSync(dir).flatMap((name) => {
      const res = path.join(dir, name);
      if (statSync(res).isDirectory()) return collectFiles(res);
      if (name.startsWith("organ.") && name.endsWith(".json")) return [res];
      return [];
    });
  }

  const files = collectFiles("examples");

  for (const file of files) {
    it(`${file} conforms to schema`, () => {
      const data = JSON.parse(readFileSync(file, "utf8"));
      expect(validate(data)).toBe(true);
    });
  }
});

