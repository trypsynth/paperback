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

        # Vendor cargo dependencies for offline builds
        cargoVendorDir = pkgs.rustPlatform.importCargoLock {
          lockFile = ./lib/Cargo.lock;
        };

        # chmlib tarball for offline libchm build
        chmlibTarball = pkgs.fetchurl {
          url = "http://www.jedrea.com/chmlib/chmlib-0.40.tar.bz2";
          sha256 = "3449d64b0cf71578b2c7e3ddc048d4af3661f44a83941ea074a7813f3a59ffa3";
        };

        # Extract version from CMakeLists.txt to keep single source of truth
        cmakeContent = builtins.readFile ./CMakeLists.txt;
        versionMatch = builtins.match ".*project\\(paperback VERSION ([0-9.]+) LANGUAGES.*" (
          builtins.replaceStrings [ "\n" ] [ " " ] cmakeContent
        );
        paperbackVersion = builtins.head versionMatch;

        # flatpak-cargo-generator for generating cargo sources for Flatpak builds
        flatpak-cargo-generator = pkgs.python3Packages.buildPythonApplication {
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
        };

        # Main Paperback package
        paperback = pkgs.clangStdenv.mkDerivation {
          pname = "paperback";
          version = paperbackVersion;

          src = ./.;

          nativeBuildInputs = with pkgs; [
            cmake
            ninja
            pkg-config
            gettext
            pandoc
            cargo
            rustc
          ];

          buildInputs = with pkgs; [
            lerc
            libdatrie
            libdeflate
            libepoxy
            libselinux
            libsepol
            libsysprof-capture
            libthai
            libwebp
            libxdmcp
            libxkbcommon
            xorg.libXtst
            pcre2
            pdfium-binaries
            util-linux
            wxGTK32
            xz
            zstd
            gtk3
          ];

          # Configure cargo to use vendored dependencies (no network in Nix build)
          preConfigure = ''
            # Copy vendor dir to writable location so we can patch libchm
            # Use -L to follow symlinks, --no-preserve=mode for write access
            cp -rL --no-preserve=mode ${cargoVendorDir} cargo-vendor

            # Create pdfium directory structure that lib/build.rs expects
            mkdir -p build/lib/pdfium/linux-x64/lib
            ln -s ${pkgs.pdfium-binaries}/lib/libpdfium.so build/lib/pdfium/linux-x64/lib/

            # Patch libchm build.rs to use local chmlib tarball instead of downloading
            cat > cargo-vendor/libchm-0.1.0/build.rs << 'BUILDRS'
            use std::{env, fs, io::Cursor, path::{Path, PathBuf}};
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
            }
            BUILDRS

            mkdir -p lib/.cargo
            cat > lib/.cargo/config.toml << EOF
            [source.crates-io]
            replace-with = "vendored-sources"

            [source.vendored-sources]
            directory = "$PWD/cargo-vendor"
            EOF
          '';

          CHMLIB_TARBALL = chmlibTarball;

          # Enable system libraries mode (instead of vcpkg)
          cmakeFlags = [
            "-DCMAKE_BUILD_TYPE=Release"
            "-DUSE_SYSTEM_LIBS=ON"
          ];

          meta = with pkgs.lib; {
            description = "A lightweight, fast, and accessible ebook and document reader";
            homepage = "https://github.com/trypsynth/paperback";
            license = licenses.mit;
            platforms = platforms.linux;
            mainProgram = "paperback";
          };
        };

      in
      {
        packages = {
          default = paperback;
          inherit paperback;
        };

        devShells.default = pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
          inputsFrom = [ paperback ];
          packages = with pkgs; [
            nil
            nixfmt-rfc-style
            clang-tools
            gdb
            lldb
            flatpak-cargo-generator
          ];
        };
      }
    );
}
