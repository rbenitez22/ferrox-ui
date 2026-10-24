import type { Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// DataView's infinite scroll on the demo's /data page: "Feed" scrolls with
// the window (6 a page, 24 in all), "Feed in a box" inside an overflow box
// (2 a page, 20 in all). Each page lands 300ms after it is asked for.

const entries = (page: Page, name: string) => page.getByRole('list', { name, exact: true }).getByRole('listitem');
const calls = (page: Page, feed: string) => page.locator(`[data-echo="${feed}-calls"]`);
const toBottom = (page: Page) => page.evaluate(() => window.scrollTo(0, document.documentElement.scrollHeight));

test.beforeEach(async ({ page }) => {
  await page.setViewportSize({ width: 1200, height: 900 });
  await openApp(page, '/data');
});

test('reaching the end asks once; the page appends', async ({ page }) => {
  await expect(entries(page, 'Feed')).toHaveCount(6);
  await expect(calls(page, 'page')).toHaveText('0');
  await toBottom(page);
  await expect(calls(page, 'page')).toHaveText('1');
  await expect(entries(page, 'Feed')).toHaveCount(12);
  await page.waitForTimeout(600);
  await expect(calls(page, 'page')).toHaveText('1');
  await expect(entries(page, 'Feed').nth(11)).toContainText('Entry 12');
});

test('while a page is on its way: a spinner, and no second call', async ({ page }) => {
  await toBottom(page);
  const spinner = page.locator('[data-feed=page]').getByRole('status');
  await expect(spinner).toHaveText('Loading more…');
  await page.mouse.wheel(0, 200);
  await toBottom(page);
  await expect(spinner).toHaveCount(0);
  await expect(entries(page, 'Feed')).toHaveCount(12);
  await expect(calls(page, 'page')).toHaveText('1');
});

test('nothing more once everything is in', async ({ page }) => {
  await expect.poll(async () => {
    await toBottom(page);
    return entries(page, 'Feed').count();
  }, { timeout: 10_000 }).toBe(24);
  await expect(calls(page, 'page')).toHaveText('3');
  await toBottom(page);
  await page.waitForTimeout(600);
  await expect(calls(page, 'page')).toHaveText('3');
});

test('in an overflow box: a short page pulls the next until the box is full', async ({ page }) => {
  const box = page.locator('[data-feed=box]');
  await box.scrollIntoViewIfNeeded();
  await expect.poll(() => box.evaluate((el) => el.scrollHeight > el.clientHeight)).toBe(true);
  await page.waitForTimeout(700);
  const settled = Number(await calls(page, 'box').textContent());
  expect(settled).toBeGreaterThanOrEqual(1);
  expect(await entries(page, 'Feed in a box').count()).toBe(2 + 2 * settled);
  await page.waitForTimeout(600);
  await expect(calls(page, 'box')).toHaveText(String(settled));
});

test('in an overflow box: scrolling the box, not the window, asks for more', async ({ page }) => {
  const box = page.locator('[data-feed=box]');
  await box.scrollIntoViewIfNeeded();
  await page.waitForTimeout(700);
  const before = await entries(page, 'Feed in a box').count();
  const windowY = await page.evaluate(() => window.scrollY);
  await box.evaluate((el) => el.scrollTo(0, el.scrollHeight));
  await expect.poll(() => entries(page, 'Feed in a box').count()).toBeGreaterThan(before);
  expect(await page.evaluate(() => window.scrollY)).toBe(windowY);
  await expect.poll(async () => {
    await box.evaluate((el) => el.scrollTo(0, el.scrollHeight));
    return entries(page, 'Feed in a box').count();
  }, { timeout: 15_000 }).toBe(20);
});

test('the marker is not a list item', async ({ page }) => {
  await expect(page.getByRole('list', { name: 'Recipes' }).getByRole('listitem')).toHaveCount(8);
  await expect(page.locator('[data-feed=page] .data-view__more')).toHaveAttribute('role', 'none');
});
