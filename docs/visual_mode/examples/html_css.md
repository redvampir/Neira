# HTML & CSS

```html
<!-- @neyra:visual_block id="container" display="Container" category="layout" -->
<div class="container">
  <!-- @neyra:visual_block id="title" display="Title" -->
  <h1>Hello</h1>
</div>
```

```css
/* @neyra:visual_block id="container-style" display="Container style" category="style" */
.container {
  /* @neyra:var id="color" display="Text color" */
  color: blue;
  /* @neyra:connection from="title" to="container-style" category="style" source=ai model="gpt-4" */
}
```
