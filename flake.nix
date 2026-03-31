{
  description = "Paperback - A lightweight, fast, and accessible ebook and document reader";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        pdfiumLib = "${pkgs.pdfium-binaries}/lib";

        commonNativeBuildInputs = with pkgs; [
          cmake
          ninja
          pkg-config
          gettext
          pandoc
          cargo
          rustc
          makeBinaryWrapper
          wrapGAppsHook3
          python3
          llvmPackages.libclang
          gcc
        ];

        commonBuildInputs = with pkgs; [
          openssl
          gtk3
          webkitgtk_4_1
          pdfium-binaries
          libxkbcommon
          libxtst
          wayland
          wayland-scanner
          wayland-protocols
        ];

        commonEnv = {
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          BINDGEN_EXTRA_CLANG_ARGS = "-isystem ${pkgs.stdenv.cc.libc.dev}/include";
          CHMLIB_TARBALL = pkgs.fetchurl {
            url = "http://www.jedrea.com/chmlib/chmlib-0.40.tar.bz2";
            sha256 = "3449d64b0cf71578b2c7e3ddc048d4af3661f44a83941ea074a7813f3a59ffa3";
          };
        };

      in
      {
        packages.default = pkgs.stdenv.mkDerivation (
          commonEnv
          // {
            pname = "paperback";
            version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;

            src = ./.;

            nativeBuildInputs = commonNativeBuildInputs;
            buildInputs = commonBuildInputs;

            preConfigure = ''
                # Extract wxWidgets source so build.rs skips downloading
                mkdir -p wxWidgets-extracted
                ${pkgs.unzip}/bin/unzip -qo ${pkgs.fetchurl {
                  url = "https://github.com/wxWidgets/wxWidgets/releases/download/v3.3.2/wxWidgets-3.3.2.zip";
                  sha256 = "sha256-9qVt5tj7VTFyMPuk72T4GmRq1vjEOdJxDZh1BJOopWk=";
                }} -d wxWidgets-extracted
                export WXWIDGETS_DIR="$PWD/wxWidgets-extracted"

                # Copy vendor dir to writable location so we can patch dependencies
                cp -rL --no-preserve=mode ${pkgs.rustPlatform.importCargoLock {
                  lockFile = ./Cargo.lock;
                  outputHashes = {
                    "pdfium-0.1.1" = "sha256-J+BXxorzHJmC5JotofsN8AQLDGHOb6EZbIdJOPHZ/CY=";
                  };
                }} cargo-vendor

                # Patch libchm build.rs to use local chmlib tarball
                echo 'use std::{env, fs, io::Cursor, path::PathBuf};
                use bzip2::read::BzDecoder;
                use cc::Build;
                use tar::Archive;

                fn main() {
                    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
                    let chmlib_dir = out_dir.join("chmlib-0.40");
                    let src_dir = chmlib_dir.join("src");
                    if !chmlib_dir.exists() {
                        let tarball_path = env::var("CHMLIB_TARBALL").expect("CHMLIB_TARBALL must be set");
                        let buf = fs::read(&tarball_path).expect("Failed to read chmlib tarball");
                        let mut archive = Archive::new(BzDecoder::new(Cursor::new(buf)));
                        archive.unpack(&out_dir).expect("Failed to extract chmlib");
                    }
                    let chm_lib_path = src_dir.join("chm_lib.c");
                    let mut contents = fs::read_to_string(&chm_lib_path).expect("Failed to read chm_lib.c");
                    contents = contents.replace(
                        "/* yielding an error is preferable to yielding incorrect behavior */\n#error \"Please define the sized types for your platform in chm_lib.c\"",
                        "typedef unsigned char UChar;\ntypedef int16_t Int16;\ntypedef uint16_t UInt16;\ntypedef int32_t Int32;\ntypedef uint32_t UInt32;\ntypedef int64_t Int64;\ntypedef uint64_t UInt64;"
                    );
                    contents = contents.replace("#if __sun || __sgi\n#include <strings.h>", "#ifdef CHMLIB_HAVE_STRINGS_H\n#include <strings.h>");
                    fs::write(&chm_lib_path, contents).expect("Failed to write patched chm_lib.c");
                    Build::new()
                        .file(src_dir.join("chm_lib.c"))
                        .file(src_dir.join("lzx.c"))
                        .include(&src_dir)
                        .warnings(false)
                        .define("CHMLIB_HAVE_STRINGS_H", None)
                        .compile("chm");
                    println!("cargo:rustc-link-lib=static=chm");
                }' > cargo-vendor/libchm-0.1.0/build.rs

                # Create cargo config for vendored sources
                mkdir -p .cargo
                echo '[source.crates-io]
                replace-with = "vendored-sources"

                [source."git+https://github.com/AllenDang/wxDragon"]
                git = "https://github.com/AllenDang/wxDragon"
                replace-with = "vendored-sources"

                [source."git+https://github.com/aryanchoudharypro/PDFium-rs?branch=feature/tagged-pdf-support"]
                git = "https://github.com/aryanchoudharypro/PDFium-rs"
                branch = "feature/tagged-pdf-support"
                replace-with = "vendored-sources"

                [source.vendored-sources]' > .cargo/config.toml
                echo "directory = \"$PWD/cargo-vendor\"" >> .cargo/config.toml
              '';

            dontUseCmakeConfigure = true;

            configurePhase = ''
              runHook preConfigure
              runHook postConfigure
            '';

            buildPhase = ''
              runHook preBuild
              cargo build --release --offline
              runHook postBuild
            '';

            installPhase = ''
              runHook preInstall

              install -Dm755 target/release/paperback $out/bin/.paperback-unwrapped
              makeWrapper $out/bin/.paperback-unwrapped $out/bin/paperback \
                --prefix LD_LIBRARY_PATH : ${pdfiumLib}

              if [ -d target/release/langs ]; then
                mkdir -p $out/share
                cp -r target/release/langs $out/share/locale
              fi

              install -Dm644 paperback.desktop $out/share/applications/paperback.desktop
              for size in 16 32 48 64 128 256; do
                if [ -f icons/hicolor/''${size}x''${size}/apps/paperback.png ]; then
                  install -Dm644 icons/hicolor/''${size}x''${size}/apps/paperback.png \
                    $out/share/icons/hicolor/''${size}x''${size}/apps/paperback.png
                fi
              done

              runHook postInstall
            '';

            meta = with pkgs.lib; {
              description = "A lightweight, fast, and accessible ebook and document reader";
              homepage = "https://github.com/trypsynth/paperback";
              license = licenses.mit;
              platforms = platforms.linux;
              mainProgram = "paperback";
            };
          }
        );

        devShells.default = pkgs.mkShell (
          commonEnv
          // {
            nativeBuildInputs = commonNativeBuildInputs;
            buildInputs = commonBuildInputs;

            packages = with pkgs; [
              nil
              nixfmt
              clang-tools
              gdb
              lldb
              (pkgs.python3Packages.buildPythonApplication {
                pname = "flatpak-cargo-generator";
                version = "unstable-2024-01-01";
                format = "other";

                src = pkgs.fetchFromGitHub {
                  owner = "flatpak";
                  repo = "flatpak-builder-tools";
                  rev = "db39dc0f75a3b24cfb09906f3aba2c13b0c48afe";
                  hash = "sha256-TnGkivHjVbOCqcowWgCw+v2MIgHz+2zU5AU2PO/prFo=";
                };

                propagatedBuildInputs = with pkgs.python3Packages; [
                  aiohttp
                  tomlkit
                ];

                installPhase = ''
                  install -Dm755 cargo/flatpak-cargo-generator.py $out/bin/flatpak-cargo-generator
                '';
              })
            ];

            LD_LIBRARY_PATH = pdfiumLib;
          }
        );
      }
    );
}
