import { createCipheriv, createDecipheriv, randomBytes, pbkdf2Sync } from 'crypto';
import { promises as fs } from 'fs';
import { dirname } from 'path';

export interface VaultRecord<T = unknown> {
  id: string;
  createdAt: string;
  updatedAt: string;
  value: T;
}

export class EncryptedVault<T = unknown> {
  private readonly filePath: string;
  private readonly key: Buffer;
  private readonly iterations: number = 120_000;

  constructor(filePath: string, passphrase: string) {
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
      const plaintext = this.decrypt(data);
      return JSON.parse(plaintext.toString('utf8')) as Record<string, VaultRecord<T>>;
    } catch (err: any) {
      if (err?.code === 'ENOENT') return {};
      throw err;
    }
  }

  private async writeAll(records: Record<string, VaultRecord<T>>): Promise<void> {
    await this.ensureDir();
    const json = Buffer.from(JSON.stringify(records, null, 2), 'utf8');
    const blob = this.encrypt(json);
    await fs.writeFile(this.filePath, blob);
  }

  async get(id: string): Promise<VaultRecord<T> | undefined> {
    const records = await this.readAll();
    return records[id];
  }

  async list(): Promise<VaultRecord<T>[]> {
    const records = await this.readAll();
    return Object.values(records);
  }

  async put(id: string, value: T): Promise<VaultRecord<T>> {
    const records = await this.readAll();
    const now = new Date().toISOString();
    const existing = records[id];
    const record: VaultRecord<T> = {
      id,
      createdAt: existing?.createdAt ?? now,
      updatedAt: now,
      value,
    };
    records[id] = record;
    await this.writeAll(records);
    return record;
  }

  async delete(id: string): Promise<boolean> {
    const records = await this.readAll();
    const existed = Boolean(records[id]);
    delete records[id];
    await this.writeAll(records);
    return existed;
  }
}
