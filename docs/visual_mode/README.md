# Visual Mode Workflow

Neira's visual mode extracts structured graphs from annotated source code. The
workflow is:

1. **Annotate** code with `@neyra:` comments describing blocks, variables and
   their connections.
2. **Parse** the source using the visual-mode parser to build metadata.
3. **Edit** the resulting graph in the visual editor.
4. **Localize** display names with `i18n.<locale>` fields when translations are
   needed.

## Annotation Syntax
Annotations are normal comments beginning with `@neyra:` followed by a type and
space separated `key=value` pairs.

Common annotations:

- `@neyra:visual_block` – marks a block that appears in the editor.
- `@neyra:var` – declares a variable or parameter.
- `@neyra:connection` – links two identifiers.

Strings containing spaces may be quoted with `"`. Additional attributes are
permitted and ignored by parsers that do not understand them.

## Supported Languages
The parser currently ships modules for:
C, C++, C#, Dart, Go, Haskell, Java, JavaScript, Kotlin, MATLAB, Objective‑C,
PHP, Python, R, Ruby, Rust, Scala, Swift and TypeScript. Any language that
supports comments can carry annotations.

## Guidelines for Translators

- Add localized names using `i18n.<locale>="Translation"`.
- Leave `id` values untouched and keep code semantics unchanged.
- Record authorship with attributes such as `translator="Name"` and
  `reviewer="Name"`.

## AI Metadata Usage

When annotations or translations are produced by automation, include
`source=ai` and optional details such as `model="gpt-4"` or
`confidence=0.9`. Human curated data may use `source=human` and name the
`translator`.

Language specific examples are available in the [examples](examples/) folder.
