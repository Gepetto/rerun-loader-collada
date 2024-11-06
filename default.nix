{
  lib,
  rustPlatform,
}:

rustPlatform.buildRustPackage {
  pname = "rerun-loader-collada";
  version = "0.1.0";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.lock
      ./Cargo.toml
      ./src
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    description = "Rerun external mesh loader for collada files";
    homepage = "https://github.com/gepetto/rerun-loader-collada";
    license = with lib.licenses; [
      asl20
      mit
    ];
    maintainers = with lib.maintainers; [ nim65s ];
    mainProgram = "rerun-loader-collada";
  };
}
