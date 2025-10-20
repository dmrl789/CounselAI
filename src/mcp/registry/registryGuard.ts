import { createHash } from 'crypto';
import { promises as fs } from 'fs';
import path from 'path';
import { promisify } from 'util';
import { execFile } from 'child_process';

const execFileAsync = promisify(execFile);

const REGISTRY_PATH = path.resolve('services/mcp-gateway/models/trusted_models.json');
const SIGNATURE_PATH = `${REGISTRY_PATH}.asc`;
const HASH_PATH = `${REGISTRY_PATH}.sha256`;
const PUBLIC_KEY_PATH = path.resolve('keys/current.asc');
const MAX_SIGNATURE_AGE_DAYS = 180;

class RegistryValidationError extends Error {
  constructor(message: string, cause?: unknown) {
    super(message);
    this.name = 'RegistryValidationError';
    if (cause instanceof Error && cause.stack) {
      this.stack = `${this.name}: ${message}\nCaused by: ${cause.stack}`;
    }
  }
}

async function ensureFileExists(filePath: string, description: string) {
  try {
    await fs.access(filePath);
  } catch (error) {
    throw new RegistryValidationError(`${description} is missing at ${filePath}`, error);
  }
}

async function verifySignatureFreshness() {
  const stats = await fs.stat(SIGNATURE_PATH);
  const ageMs = Date.now() - stats.mtimeMs;
  const ageDays = ageMs / (1000 * 60 * 60 * 24);
  if (ageDays > MAX_SIGNATURE_AGE_DAYS) {
    throw new RegistryValidationError(
      `Model registry signature is ${ageDays.toFixed(1)} days old (max ${MAX_SIGNATURE_AGE_DAYS})`
    );
  }
}

async function verifySha256() {
  const hashContent = await fs.readFile(HASH_PATH, 'utf8');
  const expectedHash = hashContent.trim().split(/\s+/)[0];
  if (!expectedHash) {
    throw new RegistryValidationError('SHA256 file is empty or malformed');
  }

  const registryBytes = await fs.readFile(REGISTRY_PATH);
  const actualHash = createHash('sha256').update(registryBytes).digest('hex');
  if (expectedHash !== actualHash) {
    throw new RegistryValidationError('Registry hash does not match recorded SHA256');
  }
}

async function verifyWithGpg() {
  try {
    await execFileAsync('gpg', ['--verify', SIGNATURE_PATH, REGISTRY_PATH]);
  } catch (error) {
    throw new RegistryValidationError('PGP signature verification failed', error);
  }
}

export async function validateModelRegistry() {
  await ensureFileExists(REGISTRY_PATH, 'Model registry');
  await ensureFileExists(SIGNATURE_PATH, 'Model registry signature');
  await ensureFileExists(HASH_PATH, 'Model registry SHA256 digest');
  await ensureFileExists(PUBLIC_KEY_PATH, 'Current PGP public key');

  await verifyWithGpg();
  await verifySha256();
  await verifySignatureFreshness();
}

export { RegistryValidationError };
