# liblouis tables

The braille tables Paperback back-translates Braille Ready Files and PEF books with, from [liblouis](https://github.com/liblouis/liblouis) at commit `011cfb3b5868f9e0a63092d5cfa39b0caf57fe21` (master, 2026-09-18). The build packs them into the binary, so every platform has them without installing anything.

They are read by [louis-rs](https://github.com/liblouis/louis-rs), the pure Rust liblouis, rather than by liblouis itself.

Left out are the hyphenation dictionaries (`*.dic`), which back-translation never reads, the two `.in` templates liblouis's build turns into the Dutch tables, and the build files.

## Updating

Copy `tables/` from a newer liblouis commit over this one, delete the same files again, and change the commit above:

```sh
git -C ../liblouis archive <commit> tables COPYING.LESSER | tar -x -C crates/paperback-core/liblouis
cd crates/paperback-core/liblouis
rm tables/Makefile.am tables/README tables/maketablelist.sh tables/*.in tables/*.dic
```

## License

The tables are under the GNU Lesser General Public License, version 2.1 or later (`COPYING.LESSER`), except a few whose headers say version 3 or later: `Es-Es-G0.utb`, `et-g0.utb`, `is-chardefs6.cti`, `is-chardefs8.cti`, `pt-pt-g2.ctb` and the Serbian `sr-*` tables.
