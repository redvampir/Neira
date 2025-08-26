import { hello } from '../src/index';

test('hello returns greeting', () => {
  expect(hello()).toBe('Hello, Neira!');
});

test('hello returns greeting of correct length', () => {
  expect(hello()).toHaveLength(13);
});
