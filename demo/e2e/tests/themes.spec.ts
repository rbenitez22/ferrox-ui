import type { Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// The theme runtime: choosing, persisting, pre-paint, URL, OS mode, fallbacks.
// Families: patina (default, light half on :root), classic, foundry (no dark
// half), jamaica, pl-ex, and harbor (a demo-only app theme).

const html = (page: Page) => page.locator('html');

async function choose(page: Page, family: string, mode: 'system' | 'light' | 'dark') {
  await page.getByLabel('Theme').selectOption(family);
  await page.getByLabel('Mode').selectOption(mode);
}

/** The value of a CSS custom property on <html>, as the page computes it. */
function token(page: Page, name: string): Promise<string> {
  return page.evaluate((n) => getComputedStyle(document.documentElement).getPropertyValue(n).trim(), name);
}

test.describe('choosing a theme', () => {
  test.use({ colorScheme: 'light' });

  test('the default is Patina on :root, with no attribute', async ({ page }) => {
    await openApp(page, '/');
    await expect(html(page)).not.toHaveAttribute('data-theme');
    await expect(page.getByText('(none, :root)')).toBeVisible();
  });

  test('family and mode set data-theme, and the theme CSS applies', async ({ page }) => {
    await openApp(page, '/');
    const patinaBackground = await token(page, '--background');

    await choose(page, 'jamaica', 'light');
    await expect(html(page)).toHaveAttribute('data-theme', 'jamaica');

    await choose(page, 'jamaica', 'dark');
    await expect(html(page)).toHaveAttribute('data-theme', 'jamaica-dark');
    expect(await token(page, '--background')).not.toBe(patinaBackground);
  });

  test("Patina's dark half is a file; its light half is :root again", async ({ page }) => {
    await openApp(page, '/');
    await choose(page, 'patina', 'dark');
    await expect(html(page)).toHaveAttribute('data-theme', 'patina-dark');
    await choose(page, 'patina', 'light');
    await expect(html(page)).not.toHaveAttribute('data-theme');
  });

  test('a family without a dark half stays light in dark mode', async ({ page }) => {
    await openApp(page, '/');
    await choose(page, 'foundry', 'dark');
    await expect(html(page)).toHaveAttribute('data-theme', 'foundry');
  });

  test("an app's own theme file is offered and applies", async ({ page }) => {
    await openApp(page, '/');
    await choose(page, 'harbor', 'light');
    await expect(html(page)).toHaveAttribute('data-theme', 'harbor');
  });
});

test.describe('remembering the choice', () => {
  test('it survives a reload, stored as family:mode:light:dark', async ({ page }) => {
    await openApp(page, '/');
    await choose(page, 'jamaica', 'dark');

    const stored = await page.evaluate(() => localStorage.getItem('ferrox.theme'));
    expect(stored).toBe('jamaica:dark:jamaica:jamaica-dark');

    await page.reload();
    await expect(page.locator('body')).toHaveAttribute('data-ready', 'true');
    await expect(html(page)).toHaveAttribute('data-theme', 'jamaica-dark');
    await expect(page.getByLabel('Theme')).toHaveValue('jamaica');
    await expect(page.getByLabel('Mode')).toHaveValue('dark');
  });

  test('the pre-paint script sets the theme before the app mounts', async ({ page }) => {
    await openApp(page, '/');
    await choose(page, 'jamaica', 'dark');

    // Record the first time data-theme is set, and whether the app had
    // mounted (body[data-ready]) by then. The pre-paint script runs in <head>,
    // before <body> even exists.
    await page.addInitScript(() => {
      const w = window as unknown as { firstTheme?: { value: string | null; appMounted: boolean } };
      new MutationObserver((_, observer) => {
        w.firstTheme = {
          value: document.documentElement.getAttribute('data-theme'),
          appMounted: document.body?.dataset.ready === 'true',
        };
        observer.disconnect();
        // `document`, not documentElement: in Chromium, init scripts run
        // before <html> exists.
      }).observe(document, { subtree: true, attributes: true, attributeFilter: ['data-theme'] });
    });
    await page.reload();
    await expect(page.locator('body')).toHaveAttribute('data-ready', 'true');

    const first = await page.evaluate(() => (window as unknown as { firstTheme?: unknown }).firstTheme);
    expect(first).toEqual({ value: 'jamaica-dark', appMounted: false });
  });
});

test.describe('before the app mounts', () => {
  // The stylesheet is linked in index.html (written by build.rs), not injected
  // by the WASM, so the theme, fonts and loading screen work before mount.

  test('the page is already styled in the stored theme', async ({ page }) => {
    await openApp(page, '/');
    await choose(page, 'jamaica', 'dark');
    const mountedBackground = await token(page, '--background');

    // At DOMContentLoaded the stylesheet has loaded (scripts wait for it) but
    // the WASM app has not mounted yet.
    await page.addInitScript(() => {
      document.addEventListener('DOMContentLoaded', () => {
        const loading = document.querySelector('.ferrox-loading');
        (window as unknown as { beforeMount?: unknown }).beforeMount = {
          theme: document.documentElement.getAttribute('data-theme'),
          background: getComputedStyle(document.documentElement).getPropertyValue('--background').trim(),
          loadingDisplay: loading ? getComputedStyle(loading).display : null,
          appMounted: document.body.dataset.ready === 'true',
        };
      });
    });
    await page.reload();
    await expect(page.locator('body')).toHaveAttribute('data-ready', 'true');

    const before = await page.evaluate(() => (window as unknown as { beforeMount?: unknown }).beforeMount);
    expect(before).toEqual({
      theme: 'jamaica-dark',
      background: mountedBackground,
      loadingDisplay: 'flex',
      appMounted: false,
    });
  });

  test('the loading screen hides once the app is mounted', async ({ page }) => {
    await openApp(page, '/');
    await expect(page.locator('.ferrox-loading')).toBeHidden();
  });

  test('the shipped fonts load from the copied fonts folder', async ({ page }) => {
    await openApp(page, '/');
    const loaded = await page.evaluate(async () => {
      const faces = await Promise.all([
        document.fonts.load('16px "Cormorant Garamond"'),
        document.fonts.load('italic 16px "Cormorant Garamond"'),
        document.fonts.load('400 16px "DM Mono"'),
      ]);
      return faces.map((f) => f.length);
    });
    expect(loaded.every((n) => n > 0), `font faces loaded: ${loaded}`).toBe(true);
  });

  test('a choice stored by webapp-lib is kept, and moved to the new key', async ({ page }) => {
    await page.addInitScript(() => {
      if (!localStorage.getItem('ferrox.theme')) {
        localStorage.setItem('webapp-lib.theme', 'jamaica:dark:jamaica:jamaica-dark');
      }
    });
    await openApp(page, '/');
    await expect(html(page)).toHaveAttribute('data-theme', 'jamaica-dark');
    expect(await page.evaluate(() => localStorage.getItem('ferrox.theme'))).toBe('jamaica:dark:jamaica:jamaica-dark');
  });
});

test.describe('the theme from the URL', () => {
  test('?theme=family:mode picks it on load', async ({ page }) => {
    await openApp(page, '/buttons?theme=jamaica:dark');
    await expect(html(page)).toHaveAttribute('data-theme', 'jamaica-dark');
  });

  test('an unknown family falls back to the default, keeping the mode', async ({ page }) => {
    await openApp(page, '/?theme=nosuch:dark');
    await expect(html(page)).toHaveAttribute('data-theme', 'patina-dark');
  });
});

test.describe('system mode', () => {
  test('follows the OS setting, live', async ({ page }) => {
    await page.emulateMedia({ colorScheme: 'light' });
    await openApp(page, '/');
    await choose(page, 'jamaica', 'system');
    await expect(html(page)).toHaveAttribute('data-theme', 'jamaica');

    await page.emulateMedia({ colorScheme: 'dark' });
    await expect(html(page)).toHaveAttribute('data-theme', 'jamaica-dark');

    await page.emulateMedia({ colorScheme: 'light' });
    await expect(html(page)).toHaveAttribute('data-theme', 'jamaica');
  });
});
