# Visual Mode Comment Annotations

Neira's visual mode reads metadata embedded in code comments. Each annotation
starts with the `@neyra:` prefix so the base language parser can ignore them.

## General Rules
- Annotations must live inside the host language's comment syntax (e.g. `#`,
  `//`, `/* */`).
- Place each annotation on its own line or at the end of a line of code.
- Attributes are written as `key=value` pairs separated by spaces. Strings may
  be quoted with `"` if they contain whitespace.

Additional annotations can be defined using the same `@neyra:` prefix.

## `@neyra:visual_block`
Marks a code region that represents a block in the visual editor.

Example:
```python
# @neyra:visual_block id="sum" display="Add" category="math"
```

**Fields**
- `id` (required) – unique identifier for the block.
- `display` – base display name shown in the UI.
- `i18n.<locale>` – localized display names, e.g. `i18n.es="Suma"`.
- `category` – classification string for grouping blocks.
- `range` – `start_line:start_col-end_line:end_col` describing covered code.

## `@neyra:var`
Declares a variable or parameter used in the visual graph.

```
# @neyra:var id="x" display="X value"
```

Fields mirror `@neyra:visual_block` and may include `category`, `display`,
`i18n.<locale>` and `range`.

## `@neyra:connection`
Defines a connection between blocks or variables.

```
# @neyra:connection from="sum" to="result" category="data"
```

**Fields**
- `from` and `to` (required) – identifiers of the source and target nodes.
- `display`, `i18n.<locale>`, and `category` – optional metadata.

## Position Encoding
Positions and ranges use 1-based line and column numbers.
- `position=line:column` marks a single point.
- `range=start_line:start_col-end_line:end_col` marks a span.

## Display Names & Localization
- `display` provides a default name.
- `i18n.<locale>` overrides the name for a locale (e.g. `i18n.fr`).

## Categories
The `category` field groups blocks, variables, or connections under an
arbitrary string.

## Editor Integration
Since annotations are ordinary comments, editors that do not understand them
should ignore them entirely. Visual-mode aware tools may parse `@neyra:` lines
but must not alter program semantics when annotations are added or removed.
