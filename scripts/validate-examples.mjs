/* neira:meta
id: NEI-20250101-000002-validate-examples
intent: utility
summary: |
  Validate all example organ specs against OrganTemplate schema.
*/
/* global console, process */
import { promises as fs } from 'node:fs';
import path from 'node:path';
import Ajv from 'ajv/dist/2020.js';

async function collectJsonFiles(dir) {
  const dirents = await fs.readdir(dir, { withFileTypes: true });
  const files = await Promise.all(
    dirents.map((dirent) => {
      const res = path.resolve(dir, dirent.name);
      if (dirent.isDirectory()) return collectJsonFiles(res);
      if (dirent.isFile() && dirent.name.startsWith('organ.') && dirent.name.endsWith('.json'))
        return [res];
      return [];
    })
  );
  return files.flat();
}

async function main() {
  const schema = JSON.parse(
    await fs.readFile('schemas/organ-template.schema.json', 'utf8')
  );
  const ajv = new Ajv({ strict: false });
  const validate = ajv.compile(schema);
  const files = await collectJsonFiles('examples');
  let ok = true;
  for (const file of files) {
    const data = JSON.parse(await fs.readFile(file, 'utf8'));
    if (!validate(data)) {
      console.error(`\u274c  ${file}`);
      console.error(validate.errors);
      ok = false;
    } else {
      console.log(`\u2705  ${file}`);
    }
  }
  if (!ok) process.exit(1);
}

main();
