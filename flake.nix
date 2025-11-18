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

        # Extract version from CMakeLists.txt to keep single source of truth
        cmakeContent = builtins.readFile ./CMakeLists.txt;
        versionMatch = builtins.match ".*project\\(paperback VERSION ([0-9.]+) LANGUAGES.*"
          (builtins.replaceStrings ["\n"] [" "] cmakeContent);
        paperbackVersion = builtins.head versionMatch;

        # Create a package for maddy (header-only markdown parser)
        # Since it's not in nixpkgs, we fetch it from GitHub
        maddy = pkgs.stdenv.mkDerivation rec {
          pname = "maddy";
          version = "1.3.0";

          src = pkgs.fetchFromGitHub {
            owner = "progsource";
            repo = "maddy";
            rev = version;
            sha256 = "sha256-sVUXACT94PSPcohnOyIp7KK8baCBuf6ZNMIyk6Cfdjg=";
          };

          nativeBuildInputs = [ pkgs.cmake ];

          cmakeFlags = [
            "-DMADDY_BUILD_EXAMPLES=OFF"
          ];

          # Header-only library, so we just need to install headers
          installPhase = ''
            mkdir -p $out/include
            cp -r $src/include/maddy $out/include/
            mkdir -p $out/lib/cmake/maddy
            cat > $out/lib/cmake/maddy/maddyConfig.cmake << EOF
            # maddy CMake config file
            if(NOT TARGET maddy::maddy)
              add_library(maddy::maddy INTERFACE IMPORTED)
              set_target_properties(maddy::maddy PROPERTIES
                INTERFACE_INCLUDE_DIRECTORIES "$out/include"
              )
            endif()
            EOF
          '';
        };

        # Main Paperback package
        paperback = pkgs.stdenv.mkDerivation rec {
          pname = "paperback";
          version = paperbackVersion;

          src = ./.;

          nativeBuildInputs = with pkgs; [
            cmake
            ninja
            pkg-config
            gettext
            pandoc
          ];

          buildInputs = with pkgs; [
            chmlib
            lerc
            lexbor
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
            maddy
            mbedtls
            nlohmann_json
            pcre2
            pdfium-binaries
            pugixml
            util-linux
            wxGTK32
            xz
            zstd
            gtk3
          ];

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
          inherit paperback maddy;
        };

        apps.default = {
          type = "app";
          program = "${paperback}/bin/paperback";
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ paperback ];
          packages = with pkgs; [
            nil
            nixfmt-rfc-style
            clang-tools
            gdb
            lldb
          ];
        };
      }
    );
}
