import type { Locator, Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// Panel on the demo's /layout page.

/** The panel named `title` (a section labelled by its heading). */
function panel(page: Page, title: string): Locator {
  return page.getByRole('region', { name: title, exact: true });
}

async function box(locator: Locator) {
  const b = await locator.boundingBox();
  if (!b) throw new Error('element has no box');
  return b;
}

async function css(locator: Locator, property: string): Promise<string> {
  return locator.evaluate((el, p) => getComputedStyle(el).getPropertyValue(p), property);
}

test.beforeEach(async ({ page }) => {
  await openApp(page, '/layout');
});

test.describe('Panel', () => {
  test('is a section named by its heading, at the level asked for', async ({ page }) => {
    const lists = panel(page, 'Shopping Lists');
    await expect(lists).toHaveClass(/\bpanel\b/);
    await expect(lists.getByRole('heading', { level: 1, name: 'Shopping Lists' })).toBeVisible();
    await expect(panel(page, 'Items').getByRole('heading', { level: 2, name: 'Items' })).toBeVisible();
  });

  test('the title sits left, the actions right, the + last', async ({ page }) => {
    const items = panel(page, 'Items');
    const title = items.locator('.panel__title');
    const search = items.getByRole('button', { name: 'Search items' });
    const add = items.getByRole('button', { name: 'Add item' });
    const titleBox = await box(title);
    const searchBox = await box(search);
    const addBox = await box(add);
    const panelBox = await box(items);

    expect(titleBox.x - panelBox.x).toBeLessThan(40);
    expect(searchBox.x).toBeGreaterThan(titleBox.x + titleBox.width);
    expect(addBox.x).toBeGreaterThan(searchBox.x);
    expect(panelBox.x + panelBox.width - (addBox.x + addBox.width)).toBeLessThan(24);
    await expect(add.locator('svg')).toHaveAttribute('data-icon', 'plus');
  });

  test('the + runs on_add once per click, and the actions run their own', async ({ page }) => {
    const lists = panel(page, 'Shopping Lists');
    await lists.getByRole('button', { name: 'Add' }).click();
    await lists.getByRole('button', { name: 'Add' }).click();
    await expect(lists.getByText('Added 2 times')).toBeVisible();

    const items = panel(page, 'Items');
    await items.getByRole('button', { name: 'Search items' }).click();
    await expect(items.getByText('Searched 1 times')).toBeVisible();
  });

  test('can_add removes the + and brings it back', async ({ page }) => {
    const items = panel(page, 'Items');
    const add = items.getByRole('button', { name: 'Add item' });
    const mayAdd = page.getByRole('switch', { name: 'May add items' });
    await expect(add).toBeVisible();

    await mayAdd.click();
    await expect(add).toHaveCount(0);
    await expect(items.getByRole('button', { name: 'Search items' })).toBeVisible();

    await mayAdd.click();
    await expect(add).toBeVisible();
  });

  test('a panel with no buttons has a bar of the same height', async ({ page }) => {
    const bare = panel(page, 'Load Data');
    await expect(bare.getByRole('button')).toHaveCount(0);
    const bareHeight = (await box(bare.locator('.panel__header'))).height;
    const withAdd = (await box(panel(page, 'Shopping Lists').locator('.panel__header'))).height;
    expect(bareHeight).toBe(withAdd);
  });

  test('help puts a ? right after the title', async ({ page }) => {
    const items = panel(page, 'Items');
    const help = items.getByRole('button', { name: 'Help: Items' });
    const titleBox = await box(items.locator('.panel__title'));
    const helpBox = await box(help);
    expect(helpBox.x).toBeGreaterThanOrEqual(titleBox.x + titleBox.width);
    expect(helpBox.x - (titleBox.x + titleBox.width)).toBeLessThan(16);
  });

  test('compact slims the band and the body', async ({ page }) => {
    const compact = panel(page, 'Compact');
    const normal = panel(page, 'Load Data');
    await expect(compact).toHaveClass(/\bpanel--compact\b/);
    expect((await box(compact.locator('.panel__header'))).height).toBeLessThan((await box(normal.locator('.panel__header'))).height);
    expect(parseFloat(await css(compact.locator('.panel__body'), 'padding-top'))).toBeLessThan(
      parseFloat(await css(normal.locator('.panel__body'), 'padding-top')),
    );
  });

  test('flush drops the body padding', async ({ page }) => {
    expect(await css(panel(page, 'Flush').locator('.panel__body'), 'padding')).toBe('0px');
  });

  test('each tone tints the band and the border; danger colours the title', async ({ page }) => {
    const plain = panel(page, 'Load Data');
    const plainBand = await css(plain.locator('.panel__header'), 'background-color');
    const plainBorder = await css(plain, 'border-top-color');
    for (const tone of ['Warning', 'Danger']) {
      const toned = panel(page, tone);
      await expect(toned).toHaveClass(new RegExp(`\\bpanel--${tone.toLowerCase()}\\b`));
      expect(await css(toned.locator('.panel__header'), 'background-color')).not.toBe(plainBand);
      expect(await css(toned, 'border-top-color')).not.toBe(plainBorder);
    }
    expect(await css(panel(page, 'Danger').locator('.panel__title'), 'color')).not.toBe(
      await css(plain.locator('.panel__title'), 'color'),
    );
  });

  test('on a phone the title ends in an ellipsis beside the buttons, and the + is 44px', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    const long = panel(page, 'A very long panel title that cannot fit beside its buttons on a phone');
    const header = long.locator('.panel__header');
    const title = long.locator('.panel__title');
    const add = long.getByRole('button', { name: 'Add' });

    const titleBox = await box(title);
    const addBox = await box(add);
    expect(Math.abs(titleBox.y + titleBox.height / 2 - (addBox.y + addBox.height / 2))).toBeLessThan(4);
    expect(await css(title, 'text-overflow')).toBe('ellipsis');
    expect(await title.evaluate((el) => el.scrollWidth > el.clientWidth)).toBe(true);
    expect((await box(header)).width).toBeLessThanOrEqual(390);
    expect(addBox.width).toBe(44);
    expect(addBox.height).toBe(44);
  });
});
