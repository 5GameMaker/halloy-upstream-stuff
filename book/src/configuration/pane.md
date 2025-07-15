# `[pane]`

Pane settings for Halloy. A pane contains a [buffer](../configuration//buffer.md).

## `scrollbar`

Scrollbar configuration.

### width

Width of the scrollbar.

```toml
# Type: integer
# Values: any positive integer
# Default: 5

[pane.scrollbar]
width = 5
```

### width

Width of the scrollbar scroller.

```toml
# Type: integer
# Values: any positive integer
# Default: 5

[pane.scrollbar]
scroller_width = 5
```

## `split_axis`

Default axis used when splitting a pane (i.e. default orientation of the divider between panes).

```toml
# Type: string
# Values: "horizontal", "vertical"
# Default: "horizontal"

[pane]
split_axis = "vertical"
```
