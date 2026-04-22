# ripe

## Rust pipe editor (`vipe` alternative)

```shell
 ✗  # building
 ✗ cargo b --release && strip ./target/release/ripe && \
 mv ./target/release/ripe ~/.local/bin
~...~
 ✗  # example of usage (with single `my_rules.ripe` template arg, which is optional)
 ✗ more -l webext/{{*.json,*.pug,*.js},panel/{panel.js,panel.css,baza.js}} \
 ../server/{bindings/types.d.ts,swagger/openapi.json} | \
 ripe my_rules.ripe | nvidia-chat
~...~
 ✗
```

> [!NOTE]
> *Rust* 🦀 inside!

---

## License

MIT license ([LICENSE](https://github.com/vitali2y/ripe/blob/main/LICENSE) or <http://opensource.org/licenses/MIT>)
