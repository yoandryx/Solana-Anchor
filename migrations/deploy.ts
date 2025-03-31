import * as anchor from "@coral-xyz/anchor";

module.exports = async function (provider: anchor.Provider) {
  // Configure the client to use the provided provider.
  anchor.setProvider(provider);
};
