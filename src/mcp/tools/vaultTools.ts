import { z } from 'zod';
import { EncryptedVault } from '../../crypto/vault.js';
import { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js';

const VaultItemSchema = z.object({
  id: z.string().min(1),
  value: z.any(),
});

export function registerVaultTools(mcp: McpServer, vault: EncryptedVault<any>): void {
  // No-arg tool: use overload without params schema to avoid ambiguity
  mcp.tool('vault.list', 'List all records in the encrypted vault', async (_extra) => {
    const items = await vault.list();
    return {
      content: [{ type: 'text', text: `Found ${items.length} record(s)` }],
      structuredContent: { items },
    };
  });

  mcp.tool(
    'vault.get',
    'Get a record by id from the encrypted vault',
    { id: z.string().min(1) },
    async ({ id }, _extra) => {
      const item = await vault.get(id);
      return {
        content: [{ type: 'text', text: item ? 'ok' : 'not_found' }],
        structuredContent: { item: item ?? null },
      };
    },
  );

  mcp.tool(
    'vault.put',
    'Create/update a record in the encrypted vault',
    { id: z.string().min(1), value: z.any() },
    async ({ id, value }, _extra) => {
      const item = await vault.put(id, value);
      return {
        content: [{ type: 'text', text: 'ok' }],
        structuredContent: { item },
      };
    },
  );

  mcp.tool(
    'vault.delete',
    'Delete a record by id from the encrypted vault',
    { id: z.string().min(1) },
    async ({ id }, _extra) => {
      const deleted = await vault.delete(id);
      return { content: [{ type: 'text', text: deleted ? 'deleted' : 'not_found' }] };
    },
  );
}
