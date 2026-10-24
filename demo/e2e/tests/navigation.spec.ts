import type { Locator, Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// NavList and NavLink on the demo's /navigation page (also mounted at
// /navigation/details).

/** The `nav` landmark named `name`. */
function nav(page: Page, name: string): Locator {
  return page.getByRole('navigation', { name, exact: true });
}

async function box(locator: Locator) {
  const b = await locator.boundingBox();
  if (!b) throw new Error('element has no box');
  return b;
}

/** Colour and image both: a theme's fill may be a gradient. */
async function background(locator: Locator): Promise<string> {
  return locator.evaluate((el) => {
    const style = getComputedStyle(el);
    return `${style.backgroundColor} ${style.backgroundImage}`;
  });
}

/** Long enough for a hover or press transition to finish. */
const SETTLE_MS = 300;

test.describe('NavList', () => {
  test.beforeEach(async ({ page }) => {
    await openApp(page, '/navigation');
  });

  test('is a labelled nav landmark holding a list of links', async ({ page }) => {
    const list = nav(page, 'Demo pages');
    await expect(list).toHaveClass(/\bnav-list\b/);
    await expect(list.getByRole('list')).toHaveCount(1);
    await expect(list.getByRole('listitem')).toHaveCount(8);
    await expect(list.getByRole('link')).toHaveCount(8);
    for (const link of await list.getByRole('link').all()) {
      await expect(link).toHaveClass(/\bnav-link\b/);
    }
  });
});

test.describe('NavLink', () => {
  test.beforeEach(async ({ page }) => {
    await openApp(page, '/navigation');
  });

  test('the current page is marked and tinted; the others are not', async ({ page }) => {
    const list = nav(page, 'Demo pages');
    const current = list.getByRole('link', { name: 'Navigation' });
    const other = list.getByRole('link', { name: 'Buttons' });
    await expect(current).toHaveAttribute('aria-current', 'page');
    await expect(other).not.toHaveAttribute('aria-current');
    expect(await background(current)).not.toEqual(await background(other));
  });

  test('"/" is not current on another page', async ({ page }) => {
    await expect(nav(page, 'Demo pages').getByRole('link', { name: 'Themes' })).not.toHaveAttribute('aria-current');
  });

  test('a link is current below its path unless it is exact', async ({ page }) => {
    const nested = nav(page, 'Nested');
    const below = nested.getByRole('link', { name: 'Navigation and below' });
    const only = nested.getByRole('link', { name: 'Navigation only' });
    const details = nested.getByRole('link', { name: 'Details' });

    await expect(below).toHaveAttribute('aria-current', 'page');
    await expect(only).toHaveAttribute('aria-current', 'page');
    await expect(details).not.toHaveAttribute('aria-current');

    await details.click();
    await expect(page).toHaveURL(/\/navigation\/details$/);
    await expect(below).toHaveAttribute('aria-current', 'page');
    await expect(only).not.toHaveAttribute('aria-current');
    await expect(details).toHaveAttribute('aria-current', 'page');
  });

  test('the icon comes before the words and is hidden from screen readers', async ({ page }) => {
    const link = nav(page, 'With icons').getByRole('link', { name: 'Help', exact: true });
    const icon = link.locator('svg');
    await expect(icon).toHaveClass(/\bnav-link__icon\b/);
    await expect(icon).toHaveAttribute('aria-hidden', 'true');
    await expect(icon).toHaveAttribute('data-icon', 'help');
    const iconBox = await box(icon);
    const linkBox = await box(link);
    expect(iconBox.x - linkBox.x).toBeLessThan(linkBox.width / 3);
  });

  test('Tab reaches each link, with the focus ring', async ({ page }) => {
    const list = nav(page, 'Demo pages');
    await list.getByRole('link', { name: 'Themes' }).focus();
    await page.keyboard.press('Tab');
    const buttons = list.getByRole('link', { name: 'Buttons' });
    await expect(buttons).toBeFocused();
    expect(await buttons.evaluate((el) => getComputedStyle(el).outlineStyle)).toBe('solid');
  });

  test('hovering or pressing a link does not move the next one', async ({ page }) => {
    const list = nav(page, 'Demo pages');
    const next = list.getByRole('link', { name: 'Icons' });
    const before = await box(next);

    await list.getByRole('link', { name: 'Buttons' }).hover();
    await page.waitForTimeout(SETTLE_MS);
    expect(await box(next)).toEqual(before);

    await page.mouse.down();
    await page.waitForTimeout(SETTLE_MS);
    expect(await box(next)).toEqual(before);
    await page.mouse.up();
  });

  test('a link is at least 44px tall on a phone', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    const link = nav(page, 'Demo pages').getByRole('link', { name: 'Buttons' });
    expect((await box(link)).height).toBeGreaterThanOrEqual(44);
  });
});
