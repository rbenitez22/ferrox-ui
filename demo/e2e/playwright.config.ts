import { defineConfig, devices } from '@playwright/test';

// Two ways to run:
// - Developing (default): port 3200, same as demo/Trunk.toml. A `trunk serve`
//   already running there is reused, so tests run against what you see.
// - Release check (`E2E_FRESH=1`, scripts/release-check.sh): Playwright starts
//   its own `trunk serve` on port 3299 with its own dist folder, so it tests a
//   fresh build of the working tree and never touches a running dev server.
const FRESH = !!process.env.E2E_FRESH;
const PORT = FRESH ? 3299 : 3200;
/** The starter app's port (tests/starter.spec.ts). */
export const STARTER_PORT = 3399;
const SERVE = FRESH
  ? `trunk serve --port ${PORT} --dist ../target/e2e-dist --no-autoreload`
  : 'trunk serve';

export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 1 : 0,
  reporter: [['list'], ['html', { open: 'never' }]],
  use: {
    baseURL: `http://127.0.0.1:${PORT}`,
    trace: 'on-first-retry',
  },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
    { name: 'firefox', use: { ...devices['Desktop Firefox'] } },
  ],
  webServer: [
    {
      command: SERVE,
      cwd: '..',
      url: `http://127.0.0.1:${PORT}`,
      reuseExistingServer: !FRESH,
      // The first Trunk build compiles the whole workspace to WASM.
      timeout: 300_000,
    },
    {
      // The starter app (tests/starter.spec.ts), built against this working
      // tree by scripts/starter-local.sh.
      command: `bash ../scripts/starter-local.sh serve --port ${STARTER_PORT}`,
      cwd: '..',
      url: `http://127.0.0.1:${STARTER_PORT}`,
      reuseExistingServer: !FRESH,
      timeout: 300_000,
    },
  ],
});
