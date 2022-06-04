const path = require("path");
const programDir = path.join(__dirname, "programs", "token3");
const idlDir = path.join(__dirname, "idl");
const sdkDir = path.join(__dirname, "src", "generated");
const binaryInstallDir = path.join(__dirname, "..", ".crates");

module.exports = {
  idlGenerator: "anchor",
  programName: "token3",
  programId: "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS",
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
