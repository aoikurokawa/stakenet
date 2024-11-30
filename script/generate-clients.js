const codama = require("codama");
const anchorIdl = require("@codama/nodes-from-anchor");
const path = require("path");
const renderers = require('@codama/renderers');

// Paths.
const projectRoot = path.join(__dirname, "..");

const stewardIdlDir = path.join(projectRoot, "programs", "steward", "idl");
const validatorHistoryIdlDir = path.join(projectRoot, "programs", "validator-history", "idl");

const rustClientsDir = path.join(__dirname, "..", "clients", "rust");

const traitOptions = {
    baseDefaults: [
        'borsh::BorshSerialize',
        'borsh::BorshDeserialize',
        'serde::Serialize',
        'serde::Deserialize',
        'Clone',
        'Debug',
    ],
    dataEnumDefaults: [],
    scalarEnumDefaults: ['Copy', 'Hash', 'num_derive::FromPrimitive'],
    structDefaults: [],
};

// Generate the steward client in Rust.
const rustStewardClientDir = path.join(rustClientsDir, "steward");
const stewardRootNode = anchorIdl.rootNodeFromAnchor(require(path.join(stewardIdlDir, "steward.json")));
const stewardCodama = codama.createFromRoot(stewardRootNode);
stewardCodama.accept(renderers.renderRustVisitor(path.join(rustStewardClientDir, "src", "generated"), {
	formatCode: true,
	crateFolder: rustStewardClientDir,
	deleteFolderBeforeRendering: true,
	toolchain: "+nightly-2024-07-25",
	traitOptions,
}));

// Generate the validator-history client in Rust.
const rustValidatorHistoryClientDir = path.join(rustClientsDir, "validator_history");
const validatorHistoryRootNode = anchorIdl.rootNodeFromAnchor(require(path.join(validatorHistoryIdlDir, "validator_history.json")));
const validatorHistoryCodama = codama.createFromRoot(validatorHistoryRootNode);
validatorHistoryCodama.accept(renderers.renderRustVisitor(path.join(rustValidatorHistoryClientDir, "src", "generated"), {
	formatCode: true,
	crateFolder: rustValidatorHistoryClientDir,
	deleteFolderBeforeRendering: true,
	toolchain: "+nightly-2024-07-25",
}));
