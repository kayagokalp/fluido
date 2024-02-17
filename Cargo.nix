# This file was @generated by cargo2nix 0.11.0.
# It is not intended to be manually edited.

args@{
  release ? true,
  rootFeatures ? [
    "fluido-saturate/default"
  ],
  rustPackages,
  buildRustPackages,
  hostPlatform,
  hostPlatformCpu ? null,
  hostPlatformFeatures ? [],
  target ? null,
  codegenOpts ? null,
  profileOpts ? null,
  cargoUnstableFlags ? null,
  rustcLinkFlags ? null,
  rustcBuildFlags ? null,
  mkRustCrate,
  rustLib,
  lib,
  workspaceSrc,
  ignoreLockHash,
}:
let
  nixifiedLockHash = "8e644c408cb0a371034362a5d2536a85e58a374bad1cb0388c624525f5a5c3c4";
  workspaceSrc = if args.workspaceSrc == null then ./. else args.workspaceSrc;
  currentLockHash = builtins.hashFile "sha256" (workspaceSrc + /Cargo.lock);
  lockHashIgnored = if ignoreLockHash
                  then builtins.trace "Ignoring lock hash" ignoreLockHash
                  else ignoreLockHash;
in if !lockHashIgnored && (nixifiedLockHash != currentLockHash) then
  throw ("Cargo.nix ${nixifiedLockHash} is out of sync with Cargo.lock ${currentLockHash}")
else let
  inherit (rustLib) fetchCratesIo fetchCrateLocal fetchCrateGit fetchCrateAlternativeRegistry expandFeatures decideProfile genDrvsByProfile;
  profilesByName = {
  };
  rootFeatures' = expandFeatures rootFeatures;
  overridableMkRustCrate = f:
    let
      drvs = genDrvsByProfile profilesByName ({ profile, profileName }: mkRustCrate ({ inherit release profile hostPlatformCpu hostPlatformFeatures target profileOpts codegenOpts cargoUnstableFlags rustcLinkFlags rustcBuildFlags; } // (f profileName)));
    in { compileMode ? null, profileName ? decideProfile compileMode release }:
      let drv = drvs.${profileName}; in if compileMode == null then drv else drv.override { inherit compileMode; };
in
{
  cargo2nixVersion = "0.11.0";
  workspace = {
    fluido-saturate = rustPackages.unknown.fluido-saturate."0.0.0";
  };
  "registry+https://github.com/rust-lang/crates.io-index".ahash."0.7.8" = overridableMkRustCrate (profileName: rec {
    name = "ahash";
    version = "0.7.8";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "891477e0c6a8957309ee5c45a6368af3ae14bb510732d2684ffa19af310920f9"; };
    dependencies = {
      ${ if hostPlatform.parsed.kernel.name == "linux" || hostPlatform.parsed.kernel.name == "android" || hostPlatform.parsed.kernel.name == "windows" || hostPlatform.parsed.kernel.name == "darwin" || hostPlatform.parsed.kernel.name == "ios" || hostPlatform.parsed.kernel.name == "freebsd" || hostPlatform.parsed.kernel.name == "openbsd" || hostPlatform.parsed.kernel.name == "netbsd" || hostPlatform.parsed.kernel.name == "dragonfly" || hostPlatform.parsed.kernel.name == "solaris" || hostPlatform.parsed.kernel.name == "illumos" || hostPlatform.parsed.kernel.name == "fuchsia" || hostPlatform.parsed.kernel.name == "redox" || hostPlatform.parsed.kernel.name == "cloudabi" || hostPlatform.parsed.kernel.name == "haiku" || hostPlatform.parsed.kernel.name == "vxworks" || hostPlatform.parsed.kernel.name == "emscripten" || hostPlatform.parsed.kernel.name == "wasi" then "getrandom" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".getrandom."0.2.12" { inherit profileName; }).out;
      ${ if !((hostPlatform.parsed.cpu.name == "armv6l" || hostPlatform.parsed.cpu.name == "armv7l") && hostPlatform.parsed.kernel.name == "none") then "once_cell" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".once_cell."1.19.0" { inherit profileName; }).out;
    };
    buildDependencies = {
      version_check = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".version_check."0.9.4" { profileName = "__noProfile"; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".anstream."0.6.11" = overridableMkRustCrate (profileName: rec {
    name = "anstream";
    version = "0.6.11";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "6e2e1ebcb11de5c03c67de28a7df593d32191b44939c482e97702baaaa6ab6a5"; };
    features = builtins.concatLists [
      [ "auto" ]
      [ "default" ]
      [ "wincon" ]
    ];
    dependencies = {
      anstyle = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".anstyle."1.0.6" { inherit profileName; }).out;
      anstyle_parse = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".anstyle-parse."0.2.3" { inherit profileName; }).out;
      anstyle_query = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".anstyle-query."1.0.2" { inherit profileName; }).out;
      ${ if hostPlatform.isWindows then "anstyle_wincon" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".anstyle-wincon."3.0.2" { inherit profileName; }).out;
      colorchoice = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".colorchoice."1.0.0" { inherit profileName; }).out;
      utf8parse = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".utf8parse."0.2.1" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".anstyle."1.0.6" = overridableMkRustCrate (profileName: rec {
    name = "anstyle";
    version = "1.0.6";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "8901269c6307e8d93993578286ac0edf7f195079ffff5ebdeea6a59ffb7e36bc"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".anstyle-parse."0.2.3" = overridableMkRustCrate (profileName: rec {
    name = "anstyle-parse";
    version = "0.2.3";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "c75ac65da39e5fe5ab759307499ddad880d724eed2f6ce5b5e8a26f4f387928c"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "utf8" ]
    ];
    dependencies = {
      utf8parse = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".utf8parse."0.2.1" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".anstyle-query."1.0.2" = overridableMkRustCrate (profileName: rec {
    name = "anstyle-query";
    version = "1.0.2";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "e28923312444cdd728e4738b3f9c9cac739500909bb3d3c94b43551b16517648"; };
    dependencies = {
      ${ if hostPlatform.isWindows then "windows_sys" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows-sys."0.52.0" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".anstyle-wincon."3.0.2" = overridableMkRustCrate (profileName: rec {
    name = "anstyle-wincon";
    version = "3.0.2";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "1cd54b81ec8d6180e24654d0b371ad22fc3dd083b6ff8ba325b72e00c87660a7"; };
    dependencies = {
      anstyle = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".anstyle."1.0.6" { inherit profileName; }).out;
      ${ if hostPlatform.isWindows then "windows_sys" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows-sys."0.52.0" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".anyhow."1.0.79" = overridableMkRustCrate (profileName: rec {
    name = "anyhow";
    version = "1.0.79";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "080e9890a082662b09c1ad45f567faeeb47f22b5fb23895fbe1e651e718e25ca"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".autocfg."1.1.0" = overridableMkRustCrate (profileName: rec {
    name = "autocfg";
    version = "1.1.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "d468802bab17cbc0cc575e9b053f41e72aa36bfa6b7f55e3529ffa43161b97fa"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".byteorder."1.5.0" = overridableMkRustCrate (profileName: rec {
    name = "byteorder";
    version = "1.5.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "1fd0f2584146f6f2ef48085050886acf353beff7305ebd1ae69500e27c67f64b"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".cfg-if."1.0.0" = overridableMkRustCrate (profileName: rec {
    name = "cfg-if";
    version = "1.0.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "baf1de4339761588bc0619e3cbc0120ee582ebb74b53b4efbf79117bd2da40fd"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".clap."4.5.1" = overridableMkRustCrate (profileName: rec {
    name = "clap";
    version = "4.5.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "c918d541ef2913577a0f9566e9ce27cb35b6df072075769e0b26cb5a554520da"; };
    features = builtins.concatLists [
      [ "color" ]
      [ "default" ]
      [ "derive" ]
      [ "error-context" ]
      [ "help" ]
      [ "std" ]
      [ "suggestions" ]
      [ "usage" ]
    ];
    dependencies = {
      clap_builder = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".clap_builder."4.5.1" { inherit profileName; }).out;
      clap_derive = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".clap_derive."4.5.0" { profileName = "__noProfile"; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".clap_builder."4.5.1" = overridableMkRustCrate (profileName: rec {
    name = "clap_builder";
    version = "4.5.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "9f3e7391dad68afb0c2ede1bf619f579a3dc9c2ec67f089baa397123a2f3d1eb"; };
    features = builtins.concatLists [
      [ "color" ]
      [ "error-context" ]
      [ "help" ]
      [ "std" ]
      [ "suggestions" ]
      [ "usage" ]
    ];
    dependencies = {
      anstream = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".anstream."0.6.11" { inherit profileName; }).out;
      anstyle = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".anstyle."1.0.6" { inherit profileName; }).out;
      clap_lex = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".clap_lex."0.7.0" { inherit profileName; }).out;
      strsim = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".strsim."0.11.0" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".clap_derive."4.5.0" = overridableMkRustCrate (profileName: rec {
    name = "clap_derive";
    version = "4.5.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "307bc0538d5f0f83b8248db3087aa92fe504e4691294d0c96c0eabc33f47ba47"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
    dependencies = {
      heck = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".heck."0.4.1" { inherit profileName; }).out;
      proc_macro2 = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.78" { inherit profileName; }).out;
      quote = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.35" { inherit profileName; }).out;
      syn = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".syn."2.0.49" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".clap_lex."0.7.0" = overridableMkRustCrate (profileName: rec {
    name = "clap_lex";
    version = "0.7.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "98cc8fbded0c607b7ba9dd60cd98df59af97e84d24e49c8557331cfc26d301ce"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".colorchoice."1.0.0" = overridableMkRustCrate (profileName: rec {
    name = "colorchoice";
    version = "1.0.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "acbf1af155f9b9ef647e42cdc158db4b64a1b61f743629225fde6f3e0be2a7c7"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".egg."0.9.5" = overridableMkRustCrate (profileName: rec {
    name = "egg";
    version = "0.9.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "96beaf9d35dbc4686bc86a4ecb851fd6a406f0bf32d9f646b1225a5c5cf5b5d7"; };
    dependencies = {
      env_logger = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".env_logger."0.9.3" { inherit profileName; }).out;
      fxhash = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".fxhash."0.2.1" { inherit profileName; }).out;
      hashbrown = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".hashbrown."0.12.3" { inherit profileName; }).out;
      indexmap = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".indexmap."1.9.3" { inherit profileName; }).out;
      instant = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".instant."0.1.12" { inherit profileName; }).out;
      log = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".log."0.4.20" { inherit profileName; }).out;
      smallvec = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".smallvec."1.13.1" { inherit profileName; }).out;
      symbol_table = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".symbol_table."0.2.0" { inherit profileName; }).out;
      symbolic_expressions = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".symbolic_expressions."5.0.3" { inherit profileName; }).out;
      thiserror = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".thiserror."1.0.57" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".env_logger."0.9.3" = overridableMkRustCrate (profileName: rec {
    name = "env_logger";
    version = "0.9.3";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "a12e6657c4c97ebab115a42dcee77225f7f482cdd841cf7088c657a42e9e00e7"; };
    dependencies = {
      log = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".log."0.4.20" { inherit profileName; }).out;
    };
  });
  
  "unknown".fluido-saturate."0.0.0" = overridableMkRustCrate (profileName: rec {
    name = "fluido-saturate";
    version = "0.0.0";
    registry = "unknown";
    src = fetchCrateLocal workspaceSrc;
    dependencies = {
      anyhow = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".anyhow."1.0.79" { inherit profileName; }).out;
      clap = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".clap."4.5.1" { inherit profileName; }).out;
      egg = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".egg."0.9.5" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".fxhash."0.2.1" = overridableMkRustCrate (profileName: rec {
    name = "fxhash";
    version = "0.2.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "c31b6d751ae2c7f11320402d34e41349dd1016f8d5d45e48c4312bc8625af50c"; };
    dependencies = {
      byteorder = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".byteorder."1.5.0" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".getrandom."0.2.12" = overridableMkRustCrate (profileName: rec {
    name = "getrandom";
    version = "0.2.12";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "190092ea657667030ac6a35e305e62fc4dd69fd98ac98631e5d3a2b1575a12b5"; };
    dependencies = {
      cfg_if = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".cfg-if."1.0.0" { inherit profileName; }).out;
      ${ if hostPlatform.isUnix then "libc" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.153" { inherit profileName; }).out;
      ${ if hostPlatform.parsed.kernel.name == "wasi" then "wasi" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".wasi."0.11.0+wasi-snapshot-preview1" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".hashbrown."0.12.3" = overridableMkRustCrate (profileName: rec {
    name = "hashbrown";
    version = "0.12.3";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "8a9ee70c43aaf417c914396645a0fa852624801b24ebb7ae78fe8272889ac888"; };
    features = builtins.concatLists [
      [ "ahash" ]
      [ "default" ]
      [ "inline-more" ]
      [ "raw" ]
    ];
    dependencies = {
      ahash = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".ahash."0.7.8" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".heck."0.4.1" = overridableMkRustCrate (profileName: rec {
    name = "heck";
    version = "0.4.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "95505c38b4572b2d910cecb0281560f54b440a19336cbbcb27bf6ce6adc6f5a8"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".indexmap."1.9.3" = overridableMkRustCrate (profileName: rec {
    name = "indexmap";
    version = "1.9.3";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "bd070e393353796e801d209ad339e89596eb4c8d430d18ede6a1cced8fafbd99"; };
    dependencies = {
      hashbrown = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".hashbrown."0.12.3" { inherit profileName; }).out;
    };
    buildDependencies = {
      autocfg = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".autocfg."1.1.0" { profileName = "__noProfile"; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".instant."0.1.12" = overridableMkRustCrate (profileName: rec {
    name = "instant";
    version = "0.1.12";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "7a5bbe824c507c5da5956355e86a746d82e0e1464f65d862cc5e71da70e94b2c"; };
    dependencies = {
      cfg_if = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".cfg-if."1.0.0" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".libc."0.2.153" = overridableMkRustCrate (profileName: rec {
    name = "libc";
    version = "0.2.153";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "9c198f91728a82281a64e1f4f9eeb25d82cb32a5de251c6bd1b5154d63a8e7bd"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".log."0.4.20" = overridableMkRustCrate (profileName: rec {
    name = "log";
    version = "0.4.20";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "b5e6163cb8c49088c2c36f57875e58ccd8c87c7427f7fbd50ea6710b2f3f2e8f"; };
    features = builtins.concatLists [
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".once_cell."1.19.0" = overridableMkRustCrate (profileName: rec {
    name = "once_cell";
    version = "1.19.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "3fdb12b2476b595f9358c5161aa467c2438859caa136dec86c26fdd2efe17b92"; };
    features = builtins.concatLists [
      [ "alloc" ]
      [ "race" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.78" = overridableMkRustCrate (profileName: rec {
    name = "proc-macro2";
    version = "1.0.78";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "e2422ad645d89c99f8f3e6b88a9fdeca7fabeac836b1002371c4367c8f984aae"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "proc-macro" ]
    ];
    dependencies = {
      unicode_ident = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-ident."1.0.12" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".quote."1.0.35" = overridableMkRustCrate (profileName: rec {
    name = "quote";
    version = "1.0.35";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "291ec9ab5efd934aaf503a6466c5d5251535d108ee747472c3977cc5acc868ef"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "proc-macro" ]
    ];
    dependencies = {
      proc_macro2 = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.78" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".smallvec."1.13.1" = overridableMkRustCrate (profileName: rec {
    name = "smallvec";
    version = "1.13.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "e6ecd384b10a64542d77071bd64bd7b231f4ed5940fba55e98c3de13824cf3d7"; };
    features = builtins.concatLists [
      [ "const_generics" ]
      [ "union" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".strsim."0.11.0" = overridableMkRustCrate (profileName: rec {
    name = "strsim";
    version = "0.11.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "5ee073c9e4cd00e28217186dbe12796d692868f432bf2e97ee73bed0c56dfa01"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".symbol_table."0.2.0" = overridableMkRustCrate (profileName: rec {
    name = "symbol_table";
    version = "0.2.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "32bf088d1d7df2b2b6711b06da3471bc86677383c57b27251e18c56df8deac14"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "global" ]
    ];
    dependencies = {
      ahash = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".ahash."0.7.8" { inherit profileName; }).out;
      hashbrown = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".hashbrown."0.12.3" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".symbolic_expressions."5.0.3" = overridableMkRustCrate (profileName: rec {
    name = "symbolic_expressions";
    version = "5.0.3";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "7c68d531d83ec6c531150584c42a4290911964d5f0d79132b193b67252a23b71"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".syn."2.0.49" = overridableMkRustCrate (profileName: rec {
    name = "syn";
    version = "2.0.49";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "915aea9e586f80826ee59f8453c1101f9d1c4b3964cd2460185ee8e299ada496"; };
    features = builtins.concatLists [
      [ "clone-impls" ]
      [ "default" ]
      [ "derive" ]
      [ "full" ]
      [ "parsing" ]
      [ "printing" ]
      [ "proc-macro" ]
      [ "quote" ]
    ];
    dependencies = {
      proc_macro2 = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.78" { inherit profileName; }).out;
      quote = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.35" { inherit profileName; }).out;
      unicode_ident = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".unicode-ident."1.0.12" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".thiserror."1.0.57" = overridableMkRustCrate (profileName: rec {
    name = "thiserror";
    version = "1.0.57";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "1e45bcbe8ed29775f228095caf2cd67af7a4ccf756ebff23a306bf3e8b47b24b"; };
    dependencies = {
      thiserror_impl = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".thiserror-impl."1.0.57" { profileName = "__noProfile"; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".thiserror-impl."1.0.57" = overridableMkRustCrate (profileName: rec {
    name = "thiserror-impl";
    version = "1.0.57";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "a953cb265bef375dae3de6663da4d3804eee9682ea80d8e2542529b73c531c81"; };
    dependencies = {
      proc_macro2 = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".proc-macro2."1.0.78" { inherit profileName; }).out;
      quote = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".quote."1.0.35" { inherit profileName; }).out;
      syn = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".syn."2.0.49" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".unicode-ident."1.0.12" = overridableMkRustCrate (profileName: rec {
    name = "unicode-ident";
    version = "1.0.12";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "3354b9ac3fae1ff6755cb6db53683adb661634f67557942dea4facebec0fee4b"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".utf8parse."0.2.1" = overridableMkRustCrate (profileName: rec {
    name = "utf8parse";
    version = "0.2.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "711b9620af191e0cdc7468a8d14e709c3dcdb115b36f838e601583af800a370a"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".version_check."0.9.4" = overridableMkRustCrate (profileName: rec {
    name = "version_check";
    version = "0.9.4";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "49874b5167b65d7193b8aba1567f5c7d93d001cafc34600cee003eda787e483f"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".wasi."0.11.0+wasi-snapshot-preview1" = overridableMkRustCrate (profileName: rec {
    name = "wasi";
    version = "0.11.0+wasi-snapshot-preview1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "9c8d87e72b64a3b4db28d11ce29237c246188f4f51057d65a7eab63b7987e423"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows-sys."0.52.0" = overridableMkRustCrate (profileName: rec {
    name = "windows-sys";
    version = "0.52.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "282be5f36a8ce781fad8c8ae18fa3f9beff57ec1b52cb3de0789201425d9a33d"; };
    features = builtins.concatLists [
      [ "Win32" ]
      [ "Win32_Foundation" ]
      [ "Win32_System" ]
      [ "Win32_System_Console" ]
      [ "default" ]
    ];
    dependencies = {
      windows_targets = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows-targets."0.52.0" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows-targets."0.52.0" = overridableMkRustCrate (profileName: rec {
    name = "windows-targets";
    version = "0.52.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "8a18201040b24831fbb9e4eb208f8892e1f50a37feb53cc7ff887feb8f50e7cd"; };
    dependencies = {
      ${ if hostPlatform.config == "aarch64-pc-windows-gnullvm" then "windows_aarch64_gnullvm" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_aarch64_gnullvm."0.52.0" { inherit profileName; }).out;
      ${ if hostPlatform.parsed.cpu.name == "aarch64" && hostPlatform.parsed.abi.name == "msvc" then "windows_aarch64_msvc" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_aarch64_msvc."0.52.0" { inherit profileName; }).out;
      ${ if hostPlatform.parsed.cpu.name == "i686" && hostPlatform.parsed.abi.name == "gnu" then "windows_i686_gnu" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_i686_gnu."0.52.0" { inherit profileName; }).out;
      ${ if hostPlatform.parsed.cpu.name == "i686" && hostPlatform.parsed.abi.name == "msvc" then "windows_i686_msvc" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_i686_msvc."0.52.0" { inherit profileName; }).out;
      ${ if hostPlatform.parsed.cpu.name == "x86_64" && hostPlatform.parsed.abi.name == "gnu" then "windows_x86_64_gnu" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_gnu."0.52.0" { inherit profileName; }).out;
      ${ if hostPlatform.config == "x86_64-pc-windows-gnullvm" then "windows_x86_64_gnullvm" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_gnullvm."0.52.0" { inherit profileName; }).out;
      ${ if hostPlatform.parsed.cpu.name == "x86_64" && hostPlatform.parsed.abi.name == "msvc" then "windows_x86_64_msvc" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_msvc."0.52.0" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_aarch64_gnullvm."0.52.0" = overridableMkRustCrate (profileName: rec {
    name = "windows_aarch64_gnullvm";
    version = "0.52.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "cb7764e35d4db8a7921e09562a0304bf2f93e0a51bfccee0bd0bb0b666b015ea"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_aarch64_msvc."0.52.0" = overridableMkRustCrate (profileName: rec {
    name = "windows_aarch64_msvc";
    version = "0.52.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "bbaa0368d4f1d2aaefc55b6fcfee13f41544ddf36801e793edbbfd7d7df075ef"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_i686_gnu."0.52.0" = overridableMkRustCrate (profileName: rec {
    name = "windows_i686_gnu";
    version = "0.52.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "a28637cb1fa3560a16915793afb20081aba2c92ee8af57b4d5f28e4b3e7df313"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_i686_msvc."0.52.0" = overridableMkRustCrate (profileName: rec {
    name = "windows_i686_msvc";
    version = "0.52.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "ffe5e8e31046ce6230cc7215707b816e339ff4d4d67c65dffa206fd0f7aa7b9a"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_gnu."0.52.0" = overridableMkRustCrate (profileName: rec {
    name = "windows_x86_64_gnu";
    version = "0.52.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "3d6fa32db2bc4a2f5abeacf2b69f7992cd09dca97498da74a151a3132c26befd"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_gnullvm."0.52.0" = overridableMkRustCrate (profileName: rec {
    name = "windows_x86_64_gnullvm";
    version = "0.52.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "1a657e1e9d3f514745a572a6846d3c7aa7dbe1658c056ed9c3344c4109a6949e"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".windows_x86_64_msvc."0.52.0" = overridableMkRustCrate (profileName: rec {
    name = "windows_x86_64_msvc";
    version = "0.52.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "dff9641d1cd4be8d1a070daf9e3773c5f67e78b4d9d42263020c057706765c04"; };
  });
  
}