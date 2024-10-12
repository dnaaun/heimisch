# How I produced these types

First, I obtained a list of Github webhook categories by copying the HTML (specifically, the `<ul />` element) on [Github's docs](https://docs.github.com/en/webhooks/webhook-events-and-payloads#about-webhook-events-and-payloads) listing the categories, and then downloaded Github's bespoke JSON description of the webhook format via:

```bash
cat categories_ul.html  | pandoc -f html -t org | rg '^\s*[a-z]' | sd '^\s*' '' | gxargs -P4 -I{} curl "https://docs.github.com/api/webhooks/v1?version=free-pro-team%40latest&category={}" -o "webhook_descriptions/{}.json"
```

And then, simply do


```bash
cargo run --release -p meta_tools webhook-desc-to-rust webhook_descriptions/
```
