import type { Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// HelpGuide on the demo's /help-guide page: every registry topic in reading
// order under a contents list, sections addressed by topic id, links that
// move in place, related links, and "Ask about this" per topic.

const guide = (page: Page) => page.locator('.help-guide');
const section = (page: Page, title: string) => page.getByRole('region', { name: title, exact: true });

test('a contents list, then each area and its topics in reading order', async ({ page }) => {
  await openApp(page, '/help-guide');
  const contents = page.getByRole('navigation', { name: 'Contents' });
  await expect(contents.getByRole('link')).toHaveText(['Statement balance', 'The five kinds of account', 'Amounts on a split']);
  await expect(guide(page).getByRole('heading', { level: 2 })).toHaveText(['Accounts', 'Entries']);
  await expect(guide(page).getByRole('heading', { level: 3 })).toHaveText(['Statement balance', 'The five kinds of account', 'Amounts on a split']);
  await expect(section(page, 'Accounts').locator('.help-guide__intro')).toHaveText('What an account is, and the kinds there are.');
  await expect(section(page, 'The five kinds of account').locator('table tbody tr')).toHaveCount(5);
  await expect(section(page, 'Statement balance')).toHaveAttribute('id', 'statement-balance');
});

test('arriving with a hash opens at that topic, its title focused', async ({ page }) => {
  await page.setViewportSize({ width: 800, height: 400 });
  await openApp(page, '/help-guide#split-amount');
  const title = section(page, 'Amounts on a split').getByRole('heading');
  await expect(title).toBeFocused();
  await expect(title).toBeInViewport();
});

test('a contents link moves in place: the hash changes, focus moves, no page load', async ({ page }) => {
  await page.setViewportSize({ width: 800, height: 400 });
  await openApp(page, '/help-guide');
  await page.evaluate(() => ((window as unknown as { marker: number }).marker = 1));
  await page.getByRole('navigation', { name: 'Contents' }).getByRole('link', { name: 'Amounts on a split' }).click();
  await expect(page).toHaveURL(/\/help-guide#split-amount$/);
  const title = section(page, 'Amounts on a split').getByRole('heading');
  await expect(title).toBeFocused();
  await expect(title).toBeInViewport();
  expect(await page.evaluate(() => (window as unknown as { marker?: number }).marker)).toBe(1);

  await page.goBack();
  await expect(page).toHaveURL(/\/help-guide$/);
});

test('related topics are links to their sections', async ({ page }) => {
  await openApp(page, '/help-guide');
  const related = section(page, 'Statement balance').locator('.help-guide__related');
  await expect(related.getByRole('link')).toHaveText(['Amounts on a split', 'The five kinds of account']);
  await related.getByRole('link', { name: 'The five kinds of account' }).click();
  await expect(page).toHaveURL(/#account-kinds$/);
  await expect(section(page, 'The five kinds of account').getByRole('heading')).toBeFocused();
  await expect(section(page, 'The five kinds of account').locator('.help-guide__related')).toHaveCount(0);
});

test('"Ask about this" sends the topic, from the guide, with the context around it', async ({ page }) => {
  await openApp(page, '/help-guide');
  await section(page, 'The five kinds of account').getByRole('button', { name: 'Ask about this' }).click();
  await expect(page.locator('[data-request="topic"]')).toHaveText('account-kinds');
  await expect(page.locator('[data-request="component"]')).toHaveText('guide');
  await expect(page.locator('[data-request="label"]')).toHaveText('The five kinds of account');
  await expect(page.locator('[data-request="prompt"]')).toContainText('screen: help');
});

test('Ask is a chat icon beside each topic\'s title, its tooltip naming the topic', async ({ page }) => {
  await openApp(page, '/help-guide');
  for (const title of ['Statement balance', 'The five kinds of account', 'Amounts on a split']) {
    const header = section(page, title).locator('.help-guide__topic-header');
    const ask = header.getByRole('button', { name: 'Ask about this' });
    await expect(ask).toHaveCount(1);
    await expect(ask.locator('svg.icon')).toHaveAttribute('data-icon', 'message-circle');
    await expect(ask).toHaveAttribute('title', `Ask about this: ${title}`);
    const [heading, icon] = [await header.locator('.help-guide__topic-title').boundingBox(), await ask.boundingBox()];
    expect(icon!.x).toBeGreaterThan(heading!.x);
    expect(Math.abs(icon!.y - heading!.y)).toBeLessThan(heading!.height);
  }
});

test('a topic has one Ask, in its header', async ({ page }) => {
  await openApp(page, '/help-guide');
  const topic = section(page, 'Statement balance');
  await expect(topic.getByRole('button', { name: 'Ask about this' })).toHaveCount(1);
  await expect(topic.locator(':scope > .help-guide__ask')).toHaveCount(0);
});
