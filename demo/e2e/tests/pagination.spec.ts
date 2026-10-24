import type { Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// Pagination on the demo's /data page: 20 pages of 10 items under a
// DataView; buttons switch to 5 pages or 1, and set the page from code.

const nav = (page: Page) => page.getByRole('navigation', { name: 'Item pages' });
const slots = (page: Page) => nav(page).locator('.pagination__slot');
const pageButton = (page: Page, n: number) => nav(page).getByRole('button', { name: `Page ${n}`, exact: true });
const previous = (page: Page) => nav(page).getByRole('button', { name: 'Previous page' });
const next = (page: Page) => nav(page).getByRole('button', { name: 'Next page' });
const items = (page: Page) => page.getByRole('list', { name: 'Paged items' }).getByRole('listitem');
const echo = (page: Page) => page.locator('[data-echo="page"]');
const press = (page: Page, name: string) => page.getByRole('button', { name, exact: true }).click();

test.beforeEach(async ({ page }) => {
  await page.setViewportSize({ width: 1200, height: 900 });
  await openApp(page, '/data');
});

test('numbers and gaps: seven slots wherever you are', async ({ page }) => {
  await expect(slots(page)).toHaveText(['1', '2', '3', '4', '5', '…', '20']);
  await pageButton(page, 5).click();
  await expect(slots(page)).toHaveText(['1', '…', '4', '5', '6', '…', '20']);
  await press(page, 'Go to page 20 from code');
  await expect(slots(page)).toHaveText(['1', '…', '16', '17', '18', '19', '20']);
  await expect(nav(page).locator('.pagination__gap').first()).toHaveAttribute('aria-hidden', 'true');
});

test('the current page is aria-current; the list shows its items', async ({ page }) => {
  await expect(pageButton(page, 1)).toHaveAttribute('aria-current', 'page');
  await pageButton(page, 3).click();
  await expect(pageButton(page, 3)).toHaveAttribute('aria-current', 'page');
  await expect(pageButton(page, 1)).not.toHaveAttribute('aria-current');
  await expect(nav(page).locator('[aria-current]')).toHaveCount(1);
  await expect(items(page).first()).toHaveText('Item 21');
  await expect(items(page)).toHaveCount(10);
});

test('Previous and Next step, and are disabled at the ends', async ({ page }) => {
  await expect(previous(page)).toBeDisabled();
  await next(page).click();
  await expect(echo(page)).toHaveText('2');
  await previous(page).click();
  await expect(echo(page)).toHaveText('1');
  await press(page, 'Go to page 20 from code');
  await expect(next(page)).toBeDisabled();
  await expect(previous(page)).toBeEnabled();
});

test('two-way: setting the page from code moves the current page', async ({ page }) => {
  await press(page, 'Go to page 20 from code');
  await expect(pageButton(page, 20)).toHaveAttribute('aria-current', 'page');
  await expect(items(page).first()).toHaveText('Item 191');
});

test('the pressed number keeps focus as the window moves', async ({ page }) => {
  await pageButton(page, 5).click();
  await expect(pageButton(page, 5)).toBeFocused();
  await page.keyboard.press('Tab');
  await expect(pageButton(page, 6)).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(pageButton(page, 6)).toBeFocused();
  await expect(echo(page)).toHaveText('6');
});

test('paging never moves Next', async ({ page }) => {
  const x = async () => (await next(page).boundingBox())!.x;
  const at1 = await x();
  await pageButton(page, 5).click();
  expect(await x()).toBe(at1);
  await pageButton(page, 6).click();
  expect(await x()).toBe(at1);
  await press(page, 'Go to page 20 from code');
  expect(await x()).toBe(at1);
});

test('few pages: every number, no gaps; one page: nothing', async ({ page }) => {
  await press(page, '5 pages');
  await expect(slots(page)).toHaveText(['1', '2', '3', '4', '5']);
  await press(page, '1 page');
  await expect(nav(page)).toHaveCount(0);
});

test('on a phone: Previous, "6 / 20", Next, as 44px targets', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await press(page, 'Go to page 20 from code');
  await previous(page).click();
  await expect(echo(page)).toHaveText('19');
  await expect(slots(page).first()).toBeHidden();
  const position = nav(page).locator('.pagination__position');
  await expect(position).toBeVisible();
  await expect(position.locator('[aria-hidden=true]')).toHaveText('19 / 20');
  await expect(position.locator('.sr-only')).toHaveText('Page 19 of 20');
  for (const step of [previous(page), next(page)]) {
    const b = (await step.boundingBox())!;
    expect(Math.round(b.width)).toBe(44);
    expect(Math.round(b.height)).toBe(44);
  }
});

test('on a wide screen the compact position is hidden', async ({ page }) => {
  await expect(nav(page).locator('.pagination__position')).toBeHidden();
});
