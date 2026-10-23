import { test as base, expect, type Page } from '@playwright/test';

/** Opens a demo route and waits until the WASM app has mounted. */
export async function openApp(page: Page, path: string): Promise<void> {
  await page.goto(path);
  // demo/src/main.rs sets this right after mount_to_body.
  await expect(page.locator('body')).toHaveAttribute('data-ready', 'true');
}

type Fixtures = {
  /**
   * Errors a test expects the page to log (e.g. a failed login logs the 401),
   * as one pattern: `test.use({ expectedErrors: /401|Login failed/ })`.
   * Not an array: `test.use` would read `[a, b]` as a value + options pair.
   */
  expectedErrors: RegExp | null;
  pageErrors: string[];
};

/**
 * `test` with page-error checking: an uncaught exception or a console error
 * (a Rust panic arrives as one) fails the test after it ran, unless it matches
 * `expectedErrors`.
 */
export const test = base.extend<Fixtures>({
  expectedErrors: [null, { option: true }],
  pageErrors: [
    async ({ page, expectedErrors }, use) => {
      const errors: string[] = [];
      page.on('pageerror', (err) => errors.push(`pageerror: ${err.name}: ${err.message}`));
      page.on('console', (msg) => {
        if (msg.type() === 'error') errors.push(`console.error: ${msg.text()}`);
      });
      await use(errors);
      const unexpected = errors.filter((e) => !expectedErrors?.test(e));
      expect(unexpected, 'the page logged errors').toEqual([]);
    },
    { auto: true },
  ],
});

export { expect };
