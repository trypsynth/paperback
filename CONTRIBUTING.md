# Contributing to Paperback

Thanks for helping with Paperback. Bug reports, translations, and code are all welcome. This guide explains what makes each kind of contribution easy to accept.

To set up a build, see the [README](README.md).

## Report a bug

Open an issue on GitHub. Include the following:

- Your platform and its version, and the Paperback version shown in the About dialog.
- Your screen reader, if you use one, and its version.
- The steps that cause the problem, what you expected, and what happened instead.
- For a problem with one document, the document itself, or a smaller one that shows the same problem. Most reading bugs depend on how a particular file is built, and can't be fixed without it. To attach a file that GitHub doesn't accept, put it in a `.zip` archive.

For a crash or a problem that's hard to reproduce, also attach the log, `paperback.log`. It's in the same folder as Paperback's settings:

- Windows, installed: `%APPDATA%\Paperback`
- macOS: `~/Library/Application Support/Paperback`
- Linux AppImage: `~/.config/Paperback`
- A portable copy on Windows or Linux: the folder Paperback runs from

## Suggest a feature

Open an issue that describes the problem you want solved, not only the feature you have in mind. Knowing what you're trying to do helps find a solution that fits the rest of the app, and that works on every platform it applies to.

## Translations

Each language's interface strings are in `po/<lang>.po`, and its user guide is in `doc/readme-<lang>.md`. To translate, edit those files with any gettext editor, such as Poedit, or a text editor, and then open a pull request.

Strings that nobody has translated yet are filled in by machine translation, which a GitHub workflow proposes as a pull request after changes reach `master`. If you maintain a language yourself, you can turn that off for it:

- To stop machine translation for your language entirely, add its code to `po/human-maintained-locales.txt`.
- To keep machine translation but guide it, write your language's conventions in `po/style/<lang>.md`. For an example, see `po/style/nl.md`.

## Contribute code

### Before you start

For anything bigger than a small fix, open an issue first, or comment on an existing one, so that we can agree on the approach before you spend time on it.

### Code style

Format Rust code with the nightly toolchain that CI uses:

```
cargo +nightly fmt --all
```

The [pre-commit hooks](README.md#pre-commit-hooks) do this for you. Beyond what the formatter enforces, Paperback's code follows these conventions in every language it uses:

- Comments explain why the code does something, never what it does. Leave out comments that restate a name or narrate the steps.
- Lines aren't wrapped by hand. Keep a call, a condition, or a shell command on one line, up to the formatter's 120 columns.
- Functions have no blank lines inside them.

Run Clippy before you open a pull request, and fix any warnings your change adds:

```
cargo clippy --release --workspace --all-targets
```

### Strings

Every string a user sees goes through `t()`, so that it can be translated. When a string could be read more than one way, put a comment directly above it that starts with `TRANSLATORS:` and explains where the string appears.

### Tests

Run the tests before you open a pull request:

```
cargo test --workspace
```

Add a test for a bug you fix, where you can. For a parsing bug, the test usually takes the shape of the document that triggered it.

### Pull requests

In your pull request, explain what changed and why, and say how you tested it, including which platforms you ran it on. Paperback runs on five platforms, and few people can test on all of them, so saying what you didn't test helps as much as saying what you did.

Keep each pull request to one change. CI must pass before a pull request is merged.

## AI-assisted contributions

You can use AI tools to write a contribution, as long as you're the one driving it. That means the following:

- You understand the change and can answer questions about it yourself.
- You've tested it as thoroughly as you can, on the platforms you have, and say how in the pull request.
- You've read the code and the pull request description before you submit them, and they describe what the change actually does.

A pull request that the person submitting it hasn't tested or can't explain will be closed.

## License

By contributing, you agree that your contribution is licensed under the [MIT license](LICENSE.md), like the rest of Paperback.
