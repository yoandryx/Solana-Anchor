// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@project-serum/anchor");
import { Provider } from "@coral-xyz/anchor";

// Explicitly define the type of provider as `Provider`
module.exports = async function (provider: Provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);
};
