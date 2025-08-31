/* neira:meta
id: NEI-20250101-000002-validate-examples
intent: utility
summary: |
  Validate all example organ specs against OrganTemplate schema.
  Supports --watch mode to revalidate on change.
*/
/* global console, process */
import { promises as fs } from 'node:fs';
import path from 'node:path';
import Ajv from 'ajv/dist/2020.js';
import chokidar from 'chokidar';

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

async function validateFile(validate, file) {
  const data = JSON.parse(await fs.readFile(file, 'utf8'));
  if (!validate(data)) {
    console.error(`\u274c  ${file}`);
    console.error(validate.errors);
    return false;
  }
  console.log(`\u2705  ${file}`);
  return true;
}

async function main() {
  const schema = JSON.parse(
    await fs.readFile('schemas/organ-template.schema.json', 'utf8')
  );
  const ajv = new Ajv({ strict: false });
  const validate = ajv.compile(schema);

  async function runAll() {
    const files = await collectJsonFiles('examples');
    let ok = true;
    for (const file of files) {
      if (!(await validateFile(validate, file))) ok = false;
    }
    if (!ok) process.exit(1);
  }

  if (process.argv.includes('--watch')) {
    await runAll();
    console.log('Watching example organs...');
    chokidar
      .watch('examples', { ignoreInitial: true })
      .on('add', (file) => {
        if (path.basename(file).startsWith('organ.') && file.endsWith('.json'))
          validateFile(validate, file);
      })
      .on('change', (file) => {
        if (path.basename(file).startsWith('organ.') && file.endsWith('.json'))
          validateFile(validate, file);
      });
  } else {
    await runAll();
  }
}

main();
