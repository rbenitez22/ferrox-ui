import type { Locator, Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// HelpButton on the demo's /help page: the `?` names its topic, opens it in
// a top-layer popover placed beside it (inside the viewport, above when
// there is no room, following scroll), renders the topic's blocks, closes on
// Esc and outside clicks, and can stay out of the Tab order.

const help = (page: Page, title: string) => page.getByRole('button', { name: `Help: ${title}` });
const topic = (page: Page, title: string) => page.getByRole('dialog', { name: title });

async function box(locator: Locator) {
  const b = await locator.boundingBox();
  if (!b) throw new Error('element has no box');
  return b;
}

const focusedLabel = (page: Page) => page.evaluate(() => document.activeElement?.getAttribute('aria-label') ?? null);

test.beforeEach(async ({ page }) => {
  await openApp(page, '/help');
});

test.describe('the ?', () => {
  test('names what it explains, as its label and its tooltip', async ({ page }) => {
    const button = help(page, 'Statement balance');
    await expect(button).toHaveAttribute('title', 'Help: Statement balance');
    await expect(button).toHaveAttribute('aria-expanded', 'false');
    await expect(button.locator('svg')).toHaveAttribute('data-icon', 'help');
  });

  test('opens its topic in the top layer, below it', async ({ page }) => {
    const button = help(page, 'Statement balance');
    await button.click();
    const popover = topic(page, 'Statement balance');
    await expect(popover).toBeVisible();
    expect(await popover.evaluate((el) => el.matches(':popover-open'))).toBe(true);
    await expect(button).toHaveAttribute('aria-expanded', 'true');
    await expect(popover).toHaveAttribute('data-side', 'below');

    const b = await box(button);
    const p = await box(popover);
    expect(p.y - (b.y + b.height)).toBeCloseTo(4, 0);
    expect(p.x).toBeCloseTo(b.x, 0);
  });

  test('renders paragraphs, lists, tables and the guide link', async ({ page }) => {
    await help(page, 'Statement balance').click();
    await expect(topic(page, 'Statement balance').locator('.help-popover__paragraph')).toHaveCount(2);
    await page.keyboard.press('Escape');

    await help(page, 'Amounts on a split').first().click();
    await expect(topic(page, 'Amounts on a split').locator('li')).toHaveText([
      'Positive: money comes in',
      'Negative: money goes out',
      'The lines of an entry add up to zero',
    ]);
    await page.keyboard.press('Escape');

    await help(page, 'The five kinds of account').first().click();
    const kinds = topic(page, 'The five kinds of account');
    await expect(kinds.locator('th')).toHaveText(['Kind', 'Holds', 'Grows with']);
    await expect(kinds.locator('tbody tr')).toHaveCount(5);
    const guide = kinds.getByRole('link', { name: 'Full guide' });
    await expect(guide).toHaveAttribute('href', '#account-kinds');
  });
});

test.describe('closing', () => {
  test('Esc closes it and gives focus back to the ?', async ({ page }) => {
    const button = help(page, 'Statement balance');
    await button.click();
    await expect(topic(page, 'Statement balance')).toBeVisible();
    await page.keyboard.press('Escape');
    await expect(topic(page, 'Statement balance')).toBeHidden();
    await expect(button).toHaveAttribute('aria-expanded', 'false');
    await expect.poll(() => focusedLabel(page)).toBe('Help: Statement balance');
  });

  test('a click elsewhere closes it', async ({ page }) => {
    await help(page, 'Statement balance').click();
    await expect(topic(page, 'Statement balance')).toBeVisible();
    await page.getByRole('heading', { name: 'Help topics' }).click();
    await expect(topic(page, 'Statement balance')).toBeHidden();
  });

  test('opening another ? closes the first', async ({ page }) => {
    await help(page, 'Statement balance').click();
    await help(page, 'Amounts on a split').first().click();
    await expect(topic(page, 'Statement balance')).toBeHidden();
    await expect(topic(page, 'Amounts on a split')).toBeVisible();
  });
});

test('skip_tab: Tab goes field to field, past the ?; a click still opens it', async ({ page }) => {
  const row = page.locator('section', { has: page.getByRole('heading', { name: 'Row of fields' }) });
  const button = row.getByRole('button', { name: 'Help: Amounts on a split' });
  await expect(button).toHaveAttribute('tabindex', '-1');

  await row.getByPlaceholder('Memo').focus();
  await page.keyboard.press('Tab');
  await expect(row.getByPlaceholder('Amount')).toBeFocused();
  await page.keyboard.press('Tab');
  await expect(button).not.toBeFocused();

  await button.click();
  await expect(topic(page, 'Amounts on a split')).toBeVisible();
});

test.describe('placement', () => {
  test('at the right edge, it stays inside the viewport', async ({ page }) => {
    const button = page.locator('.demo-help-right');
    await button.click();
    const p = await box(topic(page, 'The five kinds of account'));
    expect(p.x + p.width).toBeLessThanOrEqual(page.viewportSize()!.width - 8 + 0.5);
  });

  test('with no room below, it opens above', async ({ page }) => {
    const button = page.locator('.demo-help-bottom');
    await button.evaluate((el) => el.scrollIntoView({ block: 'end' }));
    await button.click();
    const popover = topic(page, 'Amounts on a split');
    await expect(popover).toHaveAttribute('data-side', 'above');
    const b = await box(button);
    const p = await box(popover);
    expect(b.y - (p.y + p.height)).toBeCloseTo(4, 0);
  });

  test('it follows the ? when the page scrolls', async ({ page }) => {
    await page.setViewportSize({ width: 1000, height: 500 });
    const button = help(page, 'Statement balance');
    await button.click();
    const popover = topic(page, 'Statement balance');
    await expect(popover).toBeVisible();
    await page.evaluate(() => window.scrollBy(0, 60));
    await expect.poll(async () => {
      const b = await box(button);
      const p = await box(popover);
      return Math.round(p.y - (b.y + b.height));
    }).toBe(4);
    expect((await box(button)).y).toBeLessThan(200);
  });
});
