import type { Locator, Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// DataView on the demo's /data page: a keyed list in four layouts, with its
// loading, error and empty states beside it.

/** The demo section titled `title`. */
const section = (page: Page, title: string) =>
  page.locator('.demo-section').filter({ has: page.getByRole('heading', { name: title, exact: true }) });

const recipes = (page: Page) => page.getByRole('list', { name: 'Recipes' });
const items = (page: Page) => recipes(page).getByRole('listitem');
const press = (page: Page, name: string) => section(page, 'Data view').getByRole('button', { name, exact: true }).click();
const flip = (page: Page, name: string) => section(page, 'Data view').getByRole('switch', { name }).click();
const echo = (page: Page, name: string) => page.locator(`[data-echo="${name}"]`);

async function boxes(locator: Locator) {
  return locator.evaluateAll((els) => els.map((el) => el.getBoundingClientRect().toJSON() as DOMRect));
}

/** How many items share the first item's row. */
async function columns(page: Page): Promise<number> {
  const all = await boxes(items(page));
  return all.filter((b) => Math.abs(b.y - all[0].y) < 1).length;
}

test.beforeEach(async ({ page }) => {
  await page.setViewportSize({ width: 1200, height: 900 });
  await openApp(page, '/data');
});

test.describe('DataView', () => {
  test('is a list named by its label, one item per record', async ({ page }) => {
    await expect(items(page)).toHaveCount(8);
    await expect(items(page).first().getByRole('heading', { name: 'Shakshuka' })).toBeVisible();
    await expect(recipes(page)).not.toHaveAttribute('aria-busy');
  });

  test('keyed: an item keeps its node and typed text through update, append, re-order and layout', async ({ page }) => {
    const pierogi = items(page).filter({ hasText: 'Pierogi' });
    await pierogi.evaluate((el) => el.setAttribute('data-mark', 'kept'));
    await pierogi.getByRole('textbox').fill('with dill');

    await press(page, 'Update');
    await press(page, 'Append');
    await expect(items(page)).toHaveCount(9);
    await press(page, 'Reverse');
    await expect(items(page).first()).toContainText('New recipe 9');
    await press(page, 'List');
    await expect(echo(page, 'layout')).toHaveText('List');

    await expect(pierogi).toHaveAttribute('data-mark', 'kept');
    await expect(pierogi.getByRole('textbox')).toHaveValue('with dill');
  });

  test('Grid: as many columns as fit, fewer as the card size grows', async ({ page }) => {
    const counts: number[] = [];
    for (const [name, size, min] of [['Small grid', 'sm', 160], ['Grid', 'md', 224], ['Large grid', 'lg', 320]] as const) {
      await press(page, name);
      await expect(page.locator('.data-view').first()).toHaveClass(new RegExp(`data-view--grid-${size}`));
      counts.push(await columns(page));
      for (const b of await boxes(items(page))) expect(b.width).toBeGreaterThanOrEqual(min - 1);
    }
    expect(counts[0]).toBeGreaterThan(counts[1]);
    expect(counts[1]).toBeGreaterThan(counts[2]);
    expect(counts[2]).toBeGreaterThan(1);
  });

  test('List: one full-width row per item, a hairline between', async ({ page }) => {
    await press(page, 'List');
    const list = (await recipes(page).boundingBox())!;
    const all = await boxes(items(page));
    for (const b of all) {
      expect(Math.abs(b.x - list.x)).toBeLessThan(1);
      expect(Math.abs(b.width - list.width)).toBeLessThan(1);
    }
    for (let i = 1; i < all.length; i++) expect(all[i].y).toBeGreaterThan(all[i - 1].y);
    const border = await items(page).nth(1).evaluate((el) => getComputedStyle(el).borderTopWidth);
    expect(border).toBe('1px');
  });

  test('Row: one strip that scrolls sideways', async ({ page }) => {
    await press(page, 'Strip');
    const all = await boxes(items(page));
    for (const b of all) expect(Math.abs(b.y - all[0].y)).toBeLessThan(1);
    const sideways = await recipes(page).evaluate((el) => el.scrollWidth > el.clientWidth && getComputedStyle(el).overflowX === 'auto');
    expect(sideways).toBe(true);
  });

  test('Wrap: items sized to their content, wrapping onto new lines', async ({ page }) => {
    await page.setViewportSize({ width: 420, height: 900 });
    const tags = page.getByRole('list', { name: 'Tags' }).getByRole('listitem');
    await expect(tags).toHaveCount(10);
    const all = await boxes(tags);
    expect(new Set(all.map((b) => Math.round(b.width))).size).toBeGreaterThan(1);
    expect(new Set(all.map((b) => Math.round(b.y))).size).toBeGreaterThan(1);
  });

  test('first load: a spinner and no list; then the empty text', async ({ page }) => {
    await press(page, 'Empty');
    await flip(page, 'Loading');
    const spinner = section(page, 'Data view').getByRole('status');
    await expect(spinner).toHaveText('Loading recipes…');
    await expect(recipes(page)).toBeHidden();
    await expect(section(page, 'Data view').getByText('No recipes yet.')).toHaveCount(0);

    await flip(page, 'Loading');
    await expect(spinner).toHaveCount(0);
    await expect(section(page, 'Data view').getByText('No recipes yet.')).toBeVisible();
  });

  test('a refresh keeps the items and marks the list busy', async ({ page }) => {
    await flip(page, 'Loading');
    await expect(recipes(page)).toHaveAttribute('aria-busy', 'true');
    await expect(items(page)).toHaveCount(8);
    await expect(section(page, 'Data view').getByRole('status')).toHaveCount(0);
  });

  test('error: an alert above the items, with Retry', async ({ page }) => {
    await flip(page, 'Failing');
    const alert = section(page, 'Data view').locator('.data-view__error');
    await expect(alert).toContainText("Couldn't load the recipes.");
    await expect(items(page)).toHaveCount(8);
    expect((await alert.boundingBox())!.y).toBeLessThan((await recipes(page).boundingBox())!.y);
    await alert.getByRole('button', { name: 'Retry' }).click();
    await expect(echo(page, 'retries')).toHaveText('1');

    await press(page, 'Empty');
    await expect(section(page, 'Data view').getByText('No recipes yet.')).toHaveCount(0);
  });

  test('empty: the given view in place of the text, until there is an item', async ({ page }) => {
    const empty = section(page, 'Empty view');
    await expect(empty.getByText('No shopping lists yet.')).toBeVisible();
    await expect(empty.getByRole('list', { name: 'Shopping lists' })).toBeHidden();
    await empty.getByRole('button', { name: 'Add a list' }).click();
    await expect(empty.getByRole('list', { name: 'Shopping lists' }).getByRole('listitem')).toHaveText(['List 1']);
    await expect(empty.getByText('No shopping lists yet.')).toHaveCount(0);
  });
});

test.describe('DataLayoutSwitch', () => {
  const group = (page: Page, title: string) => section(page, title).getByRole('group', { name: 'Layout' });

  test('a labelled group of buttons, the current layout pressed', async ({ page }) => {
    const buttons = group(page, 'Data view').getByRole('button');
    await expect(buttons).toHaveCount(6);
    expect(await buttons.evaluateAll((els) => els.map((el) => el.getAttribute('aria-label')))).toEqual([
      'Small grid', 'Grid', 'Large grid', 'List', 'Strip', 'Cloud',
    ]);
    expect(await buttons.evaluateAll((els) => els.map((el) => el.querySelector('svg')?.getAttribute('data-icon')))).toEqual([
      'layout-grid', 'layout-grid', 'layout-grid', 'list', 'gallery-horizontal', 'tags',
    ]);
    await expect(group(page, 'Data view').locator('[aria-pressed=true]')).toHaveAttribute('aria-label', 'Grid');
    await expect(group(page, 'Data view').locator('[aria-pressed=false]')).toHaveCount(5);
  });

  test('choosing one re-lays the view and moves the press', async ({ page }) => {
    await group(page, 'Data view').getByRole('button', { name: 'Cloud' }).click();
    await expect(page.locator('.data-view').first()).toHaveClass(/data-view--wrap/);
    await expect(group(page, 'Data view').getByRole('button', { name: 'Cloud' })).toHaveAttribute('aria-pressed', 'true');
    await expect(group(page, 'Data view').getByRole('button', { name: 'Grid', exact: true })).toHaveAttribute('aria-pressed', 'false');
  });

  test('two-way: setting the layout from code moves the press', async ({ page }) => {
    await press(page, 'Choose List from code');
    await expect(group(page, 'Data view').getByRole('button', { name: 'List' })).toHaveAttribute('aria-pressed', 'true');
    await expect(echo(page, 'layout')).toHaveText('List');
  });

  test('by default it offers Grid and List', async ({ page }) => {
    const buttons = group(page, 'Feed in a box').getByRole('button');
    expect(await buttons.evaluateAll((els) => els.map((el) => el.getAttribute('aria-label')))).toEqual(['Grid', 'List']);
    await expect(buttons.nth(1)).toHaveAttribute('aria-pressed', 'true');
    await buttons.nth(0).click();
    await expect(section(page, 'Feed in a box').locator('.data-view')).toHaveClass(/data-view--grid-md/);
  });
});
