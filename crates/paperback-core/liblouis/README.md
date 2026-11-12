# liblouis tables

The braille tables Paperback back-translates Braille Ready Files, PEF books and eBraille books with, from the [liblouis](https://github.com/liblouis/liblouis) 3.39.0 release. The build packs them into the binary, so every platform has them without installing anything.

They are read by [louis-rs](https://github.com/liblouis/louis-rs), the pure Rust liblouis, rather than by liblouis itself.

They come from a release rather than from liblouis's repository because a release includes the tables its build generates, such as Dutch `nl-NL-g0.utb` and `nl-chardefs.uti`, which the repository keeps only as m4 templates. Left out are the hyphenation dictionaries (`*.dic`), which back-translation never reads, the templates, and the build files.

## Patches

`patches/` holds fixes made to the tables that liblouis hasn't released yet, each named after its pull request. The tables here already have them applied, and updating applies them again on top of the new release:

- `2181-ueb-curly-apostrophes-forward-only.patch`, [liblouis#2181](https://github.com/liblouis/liblouis/pull/2181): in `en-ueb-g2.ctb`, the curly apostrophe variants are forward-only, so back-translation gives the plain apostrophe rather than choosing between rules for the same cells.
- `2182-it-comp6-del-forward-only.patch`, [liblouis#2182](https://github.com/liblouis/liblouis/pull/2182): in `it-it-comp6.utb`, the DEL character is forward-only, so the blank cell back-translates to a space.

Once a release includes a fix, delete its patch.

## Updating

Unpack `tables/` and `COPYING.LESSER` from a newer release, delete the same files again, apply the patches that are still needed, and change the release above:

```sh
cd crates/paperback-core/liblouis
rm -r tables
tar -xzf liblouis-<version>.tar.gz --strip-components=1 liblouis-<version>/tables liblouis-<version>/COPYING.LESSER
rm tables/Makefile.am tables/Makefile.in tables/README tables/maketablelist.sh tables/*.in tables/*.dic
cd ../../..
for patch in crates/paperback-core/liblouis/patches/*.patch; do git apply --directory=crates/paperback-core/liblouis "$patch"; done
```

On Windows, give `tar` the `--force-local` option, or it reads `C:` in a path as a remote host.

## License

The tables are under the GNU Lesser General Public License, version 2.1 or later (`COPYING.LESSER`), except a few whose headers say version 3 or later: `Es-Es-G0.utb`, `et-g0.utb`, `is-chardefs6.cti`, `is-chardefs8.cti`, `pt-pt-g2.ctb` and the Serbian `sr-*` tables.
