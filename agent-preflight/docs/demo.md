# Local demo

Copy `fixtures/demo/openai_before` to a disposable directory.

```text
agent-preflight scan <before-copy>
agent-preflight review <before-copy>
agent-preflight approve <before-copy> claude-query-permission-mode
agent-preflight task <before-copy> claude-query-permission-mode
agent-preflight verify <before-copy> --ci
```

The last command exits `1` because the decorated OpenAI function lacks its structural approval control. The repair packet does not edit source.

Manually add `needs_approval=True`, then rerun `scan`, `approve`, and `verify --ci`. The repaired fixture exits `0`. The demo is local and needs no credentials.
