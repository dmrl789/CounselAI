import { createCipheriv, createDecipheriv, randomBytes, pbkdf2Sync } from 'crypto';
import { promises as fs } from 'fs';
import { dirname } from 'path';
import { z } from 'zod';

// Validation schemas
const VaultRecordSchema = z.object({
  id: z.string().min(1).max(255),
  createdAt: z.string().datetime(),
  updatedAt: z.string().datetime(),
  value: z.unknown(),
});

export interface VaultRecord<T = unknown> {
  id: string;
  createdAt: string;
  updatedAt: string;
  value: T;
}

// Validation error class
export class VaultValidationError extends Error {
  constructor(message: string, public readonly details?: unknown) {
    super(message);
    this.name = 'VaultValidationError';
  }
}

export class EncryptedVault<T = unknown> {
  private readonly filePath: string;
  private readonly key: Buffer;
  private readonly iterations: number = 120_000;
  private readonly maxRecordSize: number = 10 * 1024 * 1024; // 10MB per record
  private readonly maxTotalSize: number = 100 * 1024 * 1024; // 100MB total

  constructor(filePath: string, passphrase: string) {
    if (!filePath || typeof filePath !== 'string') {
      throw new VaultValidationError('Invalid file path');
    }
    if (!passphrase || typeof passphrase !== 'string' || passphrase.length < 16) {
      throw new VaultValidationError('Passphrase must be at least 16 characters');
    }
    
    this.filePath = filePath;
    const salt = Buffer.from('counsel-ai-mcp-salt');
    this.key = pbkdf2Sync(passphrase, salt, this.iterations, 32, 'sha256');
  }

  private async ensureDir(): Promise<void> {
    await fs.mkdir(dirname(this.filePath), { recursive: true });
  }

  private encrypt(plaintext: Buffer): Buffer {
    const iv = randomBytes(12);
    const cipher = createCipheriv('aes-256-gcm', this.key, iv);
    const ciphertext = Buffer.concat([cipher.update(plaintext), cipher.final()]);
    const tag = cipher.getAuthTag();
    return Buffer.concat([iv, tag, ciphertext]);
  }

  private decrypt(blob: Buffer): Buffer {
    const iv = blob.subarray(0, 12);
    const tag = blob.subarray(12, 28);
    const ciphertext = blob.subarray(28);
    const decipher = createDecipheriv('aes-256-gcm', this.key, iv);
    decipher.setAuthTag(tag);
    return Buffer.concat([decipher.update(ciphertext), decipher.final()]);
  }

  private async readAll(): Promise<Record<string, VaultRecord<T>>> {
    try {
      const data = await fs.readFile(this.filePath);
      
      // Check file size
      if (data.length > this.maxTotalSize) {
        throw new VaultValidationError('Vault file too large');
      }
      
      const plaintext = this.decrypt(data);
      const jsonStr = plaintext.toString('utf8');
      
      // Validate JSON structure
      const parsed = JSON.parse(jsonStr);
      if (typeof parsed !== 'object' || parsed === null) {
        throw new VaultValidationError('Invalid vault data structure');
      }
      
      // Validate each record
      const records: Record<string, VaultRecord<T>> = {};
      for (const [key, value] of Object.entries(parsed)) {
        try {
          const validated = VaultRecordSchema.parse(value);
          records[key] = validated as VaultRecord<T>;
        } catch (error) {
          console.warn(`Skipping invalid record ${key}:`, error);
        }
      }
      
      return records;
    } catch (err: any) {
      if (err?.code === 'ENOENT') return {};
      if (err instanceof VaultValidationError) throw err;
      throw new Error(`Failed to read vault: ${err.message}`);
    }
  }

  private async writeAll(records: Record<string, VaultRecord<T>>): Promise<void> {
    await this.ensureDir();
    const json = Buffer.from(JSON.stringify(records, null, 2), 'utf8');
    const blob = this.encrypt(json);
    await fs.writeFile(this.filePath, blob);
  }

  async get(id: string): Promise<VaultRecord<T> | undefined> {
    if (!id || typeof id !== 'string') {
      throw new VaultValidationError('Invalid record ID');
    }
    
    const records = await this.readAll();
    return records[id];
  }

  async list(): Promise<VaultRecord<T>[]> {
    const records = await this.readAll();
    return Object.values(records);
  }

  async put(id: string, value: T): Promise<VaultRecord<T>> {
    if (!id || typeof id !== 'string' || id.length > 255) {
      throw new VaultValidationError('Invalid record ID');
    }
    
    // Check value size
    const valueStr = JSON.stringify(value);
    if (valueStr.length > this.maxRecordSize) {
      throw new VaultValidationError('Record value too large');
    }
    
    const records = await this.readAll();
    const now = new Date().toISOString();
    const existing = records[id];
    const record: VaultRecord<T> = {
      id,
      createdAt: existing?.createdAt ?? now,
      updatedAt: now,
      value,
    };
    
    // Validate the record
    try {
      VaultRecordSchema.parse(record);
    } catch (error) {
      throw new VaultValidationError('Invalid record data', error);
    }
    
    records[id] = record;
    await this.writeAll(records);
    return record;
  }

  async delete(id: string): Promise<boolean> {
    if (!id || typeof id !== 'string') {
      throw new VaultValidationError('Invalid record ID');
    }
    
    const records = await this.readAll();
    const existed = Boolean(records[id]);
    delete records[id];
    await this.writeAll(records);
    return existed;
  }
}
