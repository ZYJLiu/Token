const path = require("path");
const programDir = path.join(__dirname, "programs", "token");
const idlDir = path.join(__dirname, "idl");
const sdkDir = path.join(__dirname, "src", "generated");
const binaryInstallDir = path.join(__dirname, "..", ".crates");

module.exports = {
  idlGenerator: "anchor",
  programName: "tokens",
  programId: "53pUyMnFNBEbpncA5sKZHjmf2bexs2Rk7s7d8no4vVd8",
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
