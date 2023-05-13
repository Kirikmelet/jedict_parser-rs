# DOM Implementation Notes

## Query Child
The function `DictDOMItem::query_child` should be able to query through children nodes.
The question arrives however wether or not this function should be recursive or not. Function should work similarly to JavaScript DOM's `Document.QuerySelectorAll`.

Resources to look at:
- [](https://medium.com/@lancelyao/implement-queryselectorall-in-javascript-baf948eadc1f)
