const path = require("path");
const programDir = path.join(__dirname, "programs", "token");
const idlDir = path.join(__dirname, "idl");
const sdkDir = path.join(__dirname, "src", "generated");
const binaryInstallDir = path.join(__dirname, "..", ".crates");

module.exports = {
  idlGenerator: "anchor",
  programName: "tokens",
  programId: "DwY4TVmavvs63Znj28ccT1ryw8u3Y2Ctz88JfdY4UP7i",
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
