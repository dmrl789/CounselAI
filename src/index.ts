import { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import { z } from 'zod';
import { EncryptedVault } from './crypto/vault.js';
import { registerVaultTools } from './mcp/tools/vaultTools.js';
import { validateModelRegistry, RegistryValidationError } from './mcp/registry/registryGuard.js';

const EnvSchema = z.object({
  MCP_VAULT_PATH: z.string().default('.data/vault.bin'),
  MCP_VAULT_PASSPHRASE: z.string().min(16, 'MCP_VAULT_PASSPHRASE must be at least 16 chars'),
});

function loadEnv() {
  const env = EnvSchema.safeParse({
    MCP_VAULT_PATH: process.env.MCP_VAULT_PATH,
    MCP_VAULT_PASSPHRASE: process.env.MCP_VAULT_PASSPHRASE,
  });
  if (!env.success) {
    console.error('Invalid environment:', env.error.flatten().fieldErrors);
    process.exit(1);
  }
  return env.data;
}

async function main() {
  const { MCP_VAULT_PATH, MCP_VAULT_PASSPHRASE } = loadEnv();

  try {
    await validateModelRegistry();
  } catch (error) {
    if (error instanceof RegistryValidationError) {
      console.error('Model registry validation failed:', error.message);
      process.exit(1);
    }
    throw error;
  }

  const vault = new EncryptedVault(MCP_VAULT_PATH, MCP_VAULT_PASSPHRASE);

  const mcp = new McpServer({
    name: 'counsel-ai-mcp',
    version: '0.1.0',
  });

  // Register tools
  registerVaultTools(mcp, vault);

  const transport = new StdioServerTransport();
  await mcp.connect(transport);
}

main().catch((err) => {
  console.error('MCP server failed:', err);
  process.exit(1);
});
