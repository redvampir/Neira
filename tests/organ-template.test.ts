/* neira:meta
id: NEI-20250101-000001-organ-template-test
intent: test
summary: |
  Validate example organs against schema.
*/
import { readFileSync } from 'node:fs';
import Ajv from 'ajv/dist/2020.js';

describe('organ template examples', () => {
  const schema = JSON.parse(readFileSync('schemas/organ-template.schema.json', 'utf8'));
  const ajv = new Ajv({ strict: false });
  const validate = ajv.compile(schema);

  const example = JSON.parse(
    readFileSync('examples/factory/voice-v1/organ.voice.v1.json', 'utf8')
  );

  it('voice-v1 organ conforms to schema', () => {
    expect(validate(example)).toBe(true);
  });
});
