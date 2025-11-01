import type * as Process from 'node:process'
const ALLOWED_REGISTRY_HOSTS = [
  'registry.npmjs.org',
  'registry.yarnpkg.com',
  // Add any additional trusted registry domains here
]

function isAllowedRegistryHostname(urlString: string): boolean {
  try {
    const url = new URL(urlString)
    // Compare against allow-list. Consider only exact domain match for safety.
    return ALLOWED_REGISTRY_HOSTS.includes(url.hostname)
  } catch {
    return false
  }
}

export function getRegistryUrl() {
  // Prefer npm's configured registry (works with Verdaccio and custom registries)
  // Fallback to REGISTRY_URL for tests/dev, then npmjs
  const candidate =
    process.env.npm_config_registry
    || process.env.REGISTRY_URL
    || 'https://registry.npmjs.org'
  if (!isAllowedRegistryHostname(candidate)) {
    throw new Error(
      `Refusing to use registry URL '${candidate}' not in allowed list: ${ALLOWED_REGISTRY_HOSTS.join(', ')}`
    )
  }
  return candidate
}

export type Architecture = Extract<(typeof Process)['arch'], 'arm64' | 'x64'>
export type Platform = Extract<
  (typeof Process)['platform'],
  'darwin' | 'linux' | 'win32'
>

// Note: we intentionally don't export a combined `Platform-Architecture` alias here,
// since only specific pairs are supported (see `BINARY_DISTRIBUTION_PACKAGES`).

export const BINARY_DISTRIBUTION_PACKAGES = {
  darwin: {
    x64: '@foundry-rs/forge-darwin-amd64',
    arm64: '@foundry-rs/forge-darwin-arm64'
  },
  linux: {
    x64: '@foundry-rs/forge-linux-amd64',
    arm64: '@foundry-rs/forge-linux-arm64'
  },
  win32: {
    x64: '@foundry-rs/forge-win32-amd64'
  }
} as const

export const BINARY_NAME = process.platform === 'win32' ? 'forge.exe' : 'forge'
// @ts-expect-error
export const PLATFORM_SPECIFIC_PACKAGE_NAME = BINARY_DISTRIBUTION_PACKAGES[process.platform][process.arch]

export const colors = {
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
  cyan: '\x1b[36m',
  white: '\x1b[37m',
  reset: '\x1b[0m'
}
