import type { Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';
import { STARTER_PORT } from '../playwright.config';

// The starter app (starter/, served by scripts/starter-local.sh): it mounts
// without errors, themes its first paint, and its one page works end to end.

test.use({ baseURL: `http://127.0.0.1:${STARTER_PORT}` });

const cards = (page: Page) => page.getByRole('list', { name: 'Items' }).getByRole('listitem');

test.beforeEach(async ({ page }) => {
  await openApp(page, '/');
});

test('mounts: a page panel, then the loaded items', async ({ page }) => {
  await expect(page.getByRole('heading', { level: 1, name: 'Items' })).toBeVisible();
  await expect(cards(page)).toHaveCount(2);
  await expect(cards(page).first().getByRole('heading')).toHaveText('First item');
});

test('the page is a padded <main>, its width capped', async ({ page }) => {
  const main = page.getByRole('main');
  await expect(main.getByRole('heading', { level: 1, name: 'Items' })).toBeVisible();
  // The page's padding inside the grid, and no browser margin around it
  // (.ferrox-app): the grid starts at the window's edge on a narrow window.
  await page.setViewportSize({ width: 800, height: 900 });
  const grid = (await page.locator('.page-grid').boundingBox())!;
  const panel = (await page.locator('.panel').first().boundingBox())!;
  expect(grid.x).toBe(0);
  expect(Math.round(panel.x - grid.x)).toBe(24);
  await page.setViewportSize({ width: 1600, height: 900 });
  expect((await page.locator('.page-grid').boundingBox())!.width).toBeLessThanOrEqual(64 * 16 + 1);
});

test('adds an item through the dialog', async ({ page }) => {
  await expect(cards(page)).toHaveCount(2);
  await page.getByRole('button', { name: 'Add item' }).click();
  const dialog = page.getByRole('dialog', { name: 'Add item' });
  await dialog.getByLabel('Name').fill('Third item');
  await dialog.getByRole('button', { name: 'Save' }).click();
  await expect(dialog).toBeHidden();
  await expect(cards(page)).toHaveCount(3);
  await expect(cards(page).last().getByRole('heading')).toHaveText('Third item');
});

test('renames from the row menu, and the card shows the new name', async ({ page }) => {
  await cards(page).first().getByRole('button', { name: 'More actions' }).click();
  await page.getByRole('menuitem', { name: 'Rename' }).click();
  const dialog = page.getByRole('dialog', { name: 'Rename item' });
  await expect(dialog.getByLabel('Name')).toHaveValue('First item');
  await dialog.getByLabel('Name').fill('Renamed');
  await dialog.getByRole('button', { name: 'Save' }).click();
  await expect(cards(page).first().getByRole('heading')).toHaveText('Renamed');
});

test('deletes after a confirmation naming the item; then the empty view', async ({ page }) => {
  for (const name of ['First item', 'Second item']) {
    await cards(page).first().getByRole('button', { name: 'More actions' }).click();
    await page.getByRole('menuitem', { name: 'Delete' }).click();
    const confirm = page.getByRole('alertdialog', { name: 'Delete item?' });
    await expect(confirm).toContainText(`This deletes “${name}”.`);
    await confirm.getByRole('button', { name: 'Delete' }).click();
    await expect(confirm).toBeHidden();
  }
  await expect(page.getByText('No items yet.')).toBeVisible();
  await page.getByRole('button', { name: 'Add an item' }).click();
  await expect(page.getByRole('dialog', { name: 'Add item' })).toBeVisible();
});

test.describe('before the app loads', () => {
  // The WASM is blocked, so only the inlined pre-paint script can set the theme.
  test.use({ expectedErrors: /wasm|WebAssembly|ERR_FAILED|NS_ERROR|Failed to fetch|NetworkError|import/i });

  test('the saved theme is applied by the pre-paint script alone', async ({ page }) => {
    await page.evaluate(() => localStorage.setItem('ferrox.theme', 'classic:dark:classic:classic-dark'));
    await page.route('**/*.wasm', (route) => route.abort());
    await page.goto('/');
    await expect(page.locator('html')).toHaveAttribute('data-theme', 'classic-dark');
    await expect(page.locator('.ferrox-loading')).toBeVisible();
    expect(await page.evaluate(() => document.body.getAttribute('data-ready'))).toBeNull();
  });
});

test('the page itself is in the theme: background, text colour, body font', async ({ page }) => {
  const style = await page.evaluate(() => {
    const body = getComputedStyle(document.body);
    const root = getComputedStyle(document.documentElement);
    const probe = document.createElement('div');
    probe.style.background = root.getPropertyValue('--background');
    document.body.append(probe);
    const background = getComputedStyle(probe).backgroundColor;
    probe.remove();
    return { body: body.backgroundColor, background, font: body.fontFamily, fontToken: root.getPropertyValue('--font-body').trim() };
  });
  expect(style.body).toBe(style.background);
  expect(style.font.replace(/["']/g, '')).toBe(style.fontToken.replace(/["']/g, ''));
});
