# Python

```python
# @neyra:visual_block id="sum" display="Add" i18n.ru="Сложение" category="math" translator="Иван"
def add(a, b):
    # @neyra:var id="a" display="First number" i18n.ru="Первое число"
    # @neyra:var id="b" display="Second number" i18n.ru="Второе число"
    result = a + b
    # @neyra:connection from="a" to="sum" category="data" source=ai model="gpt-4" confidence=0.9
    # @neyra:connection from="b" to="sum" category="data" source=ai model="gpt-4" confidence=0.9
    return result
```
