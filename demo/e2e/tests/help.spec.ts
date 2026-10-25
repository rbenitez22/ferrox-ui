import type { Locator, Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// HelpButton on the demo's /help page: the `?` names its topic, opens it in
// a top-layer popover placed beside it (inside the viewport, above when
// there is no room, following scroll), renders the topic's blocks, closes on
// Esc and outside clicks, and can stay out of the Tab order.

/** The `?` for a topic in the "Help topics" section (outside any assistant). */
const help = (page: Page, title: string) =>
  page
    .locator('section', { has: page.getByRole('heading', { name: 'Help topics' }) })
    .getByRole('button', { name: `Help: ${title}` });
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

test.describe('only while in use (HelpReveal::WhileActive)', () => {
  const form = (page: Page) => page.locator('section', { has: page.getByRole('heading', { name: 'Only while in use' }) });
  const q = (page: Page, title: string) => form(page).getByRole('button', { name: `Help: ${title}` });

  test.beforeEach(async ({ page }) => {
    // The pointer away from the form, so hover reveals nothing.
    await page.mouse.move(0, 0);
  });

  test('hidden at rest but in place, unless the field says always', async ({ page }) => {
    await expect(q(page, 'Reference numbers')).toHaveCSS('opacity', '0');
    await expect(q(page, 'Reference numbers')).toHaveClass(/help-btn--reveal/);
    await expect(q(page, 'Reminders')).toHaveCSS('opacity', '0');
    await expect(q(page, 'Reviewed entries')).toHaveCSS('opacity', '0');
    await expect(q(page, 'Categories')).toHaveCSS('opacity', '1');
    await expect(q(page, 'Categories')).not.toHaveClass(/help-btn--reveal/);
    // A dialog's or toolbar's `?` is never hidden this way.
    await expect(page.getByRole('button', { name: 'Help: Accounts' })).not.toHaveClass(/help-btn--reveal/);
  });

  test("focusing a control shows its ? and no other's", async ({ page }) => {
    await form(page).getByRole('textbox', { name: 'Reference', exact: true }).focus();
    await expect(q(page, 'Reference numbers')).toHaveCSS('opacity', '1');
    await expect(q(page, 'Amounts on a split')).toHaveCSS('opacity', '0');
    await form(page).getByRole('checkbox', { name: 'Send a reminder' }).focus();
    await expect(q(page, 'Reminders')).toHaveCSS('opacity', '1');
    await expect(q(page, 'Reference numbers')).toHaveCSS('opacity', '0');
    await form(page).getByRole('switch', { name: 'Reviewed' }).focus();
    await expect(q(page, 'Reviewed entries')).toHaveCSS('opacity', '1');
  });

  test('pointing at a control shows its ?', async ({ page }) => {
    await form(page).getByRole('textbox', { name: 'Amount', exact: true }).hover();
    await expect(q(page, 'Amounts on a split')).toHaveCSS('opacity', '1');
  });

  test('the keyboard reaches it, and reaching it shows it', async ({ page }) => {
    await form(page).getByRole('textbox', { name: 'Reference', exact: true }).focus();
    await page.keyboard.press('Shift+Tab');
    await expect(q(page, 'Reference numbers')).toBeFocused();
    await expect(q(page, 'Reference numbers')).toHaveCSS('opacity', '1');
  });

  test('stays shown while its topic is open', async ({ page }) => {
    await q(page, 'Reference numbers').click();
    await expect(topic(page, 'Reference numbers')).toBeVisible();
    await page.mouse.move(0, 0);
    await expect(q(page, 'Reference numbers')).toHaveAttribute('aria-expanded', 'true');
    await expect(q(page, 'Reference numbers')).toHaveCSS('opacity', '1');
  });
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

test.describe('asking an assistant', () => {
  const asking = (page: Page) => page.locator('section', { has: page.getByRole('heading', { name: 'Asking an assistant' }) });
  const ask = (page: Page) => page.getByRole('button', { name: 'Ask about this' });
  const assistant = (page: Page) => page.getByRole('dialog', { name: 'Assistant' });
  const request = (page: Page, part: string) => assistant(page).locator(`[data-request="${part}"]`);

  async function askAbout(page: Page, title: string) {
    await asking(page).getByRole('button', { name: `Help: ${title}` }).click();
    await ask(page).click();
    await expect(assistant(page)).toBeVisible();
  }

  test('outside an assistant there is nothing to ask', async ({ page }) => {
    await help(page, 'Statement balance').first().click();
    await expect(topic(page, 'Statement balance')).toBeVisible();
    await expect(ask(page)).toHaveCount(0);
  });

  test('the request names the topic, the component and its label, and closes the popover', async ({ page }) => {
    await askAbout(page, 'Statement balance');
    await expect(topic(page, 'Statement balance')).toBeHidden();
    await expect(request(page, 'topic')).toHaveText('statement-balance');
    await expect(request(page, 'component')).toHaveText('field');
    await expect(request(page, 'label')).toHaveText('Statement balance');
    await expect(request(page, 'value')).toHaveText('(none)');
  });

  test('context comes from every enclosing scope, outermost first', async ({ page }) => {
    await askAbout(page, 'Statement balance');
    await expect(request(page, 'prompt')).toHaveText(
      'topic: statement-balance (Statement balance)\ncomponent: field\nlabel: Statement balance\nscreen: reconcile\naccount: checking\n',
    );
  });

  test('context that follows the page is read when asking', async ({ page }) => {
    await page.getByRole('button', { name: 'Next account' }).click();
    await expect(page.locator('[data-echo="account"]')).toHaveText('savings');
    await askAbout(page, 'Statement balance');
    await expect(request(page, 'prompt')).toContainText('account: savings');
  });

  test("a scope reaches only its own children, not its siblings'", async ({ page }) => {
    await askAbout(page, 'The five kinds of account');
    await expect(request(page, 'prompt')).toHaveText(
      'topic: account-kinds (The five kinds of account)\ncomponent: help\nlabel: The five kinds of account\nscreen: reconcile\n',
    );
  });

  test('a value is sent only where it was given', async ({ page }) => {
    await asking(page).getByPlaceholder('Amount (sent with the request)').fill('12.50');
    await askAbout(page, 'Amounts on a split');
    await expect(request(page, 'value')).toHaveText('12.50');
    await expect(request(page, 'prompt')).toContainText('value: 12.50');
  });
});

test.describe('help on components', () => {
  const area = (page: Page) => page.locator('section', { has: page.getByRole('heading', { name: 'Help on components' }) });
  const q = (page: Page, title: string) => page.getByRole('button', { name: `Help: ${title}` });
  const assistant = (page: Page) => page.getByRole('dialog', { name: 'Assistant' });
  const request = (page: Page, part: string) => assistant(page).locator(`[data-request="${part}"]`);

  async function askVia(page: Page, title: string) {
    await q(page, title).click();
    await page.getByRole('button', { name: 'Ask about this' }).click();
    await expect(assistant(page)).toBeVisible();
  }

  async function expectRequest(page: Page, component: string, label: string, value = '(none)') {
    await expect(request(page, 'component')).toHaveText(component);
    await expect(request(page, 'label')).toHaveText(label);
    await expect(request(page, 'value')).toHaveText(value);
    await expect(request(page, 'prompt')).toContainText('screen: components');
  }

  test('Field: the ? follows the label, outside it; the value only when shared', async ({ page }) => {
    const row = area(page).locator('.field__label-row').filter({ hasText: 'Memo' });
    await expect(row.locator('label').getByRole('button')).toHaveCount(0);
    await expect(row.getByRole('button', { name: 'Help: Memo' })).toBeVisible();

    await area(page).locator('input[name=help-memo]').fill('Weekly shop');
    await askVia(page, 'Memo');
    await expectRequest(page, 'field', 'Memo', 'Weekly shop');
  });

  test('Field without help_shares_value sends no value', async ({ page }) => {
    await area(page).locator('input[name=help-payee]').fill('Market');
    await askVia(page, 'Payee');
    await expectRequest(page, 'field', 'Payee');
  });

  test('a password is never sent, even with help_shares_value', async ({ page }) => {
    await area(page).locator('input[name=help-password]').fill('hunter2');
    await askVia(page, 'Password');
    await expectRequest(page, 'field', 'Password');
    await expect(request(page, 'prompt')).not.toContainText('hunter2');
  });

  test('Checkbox: the ? follows the words outside the label, and does not tick it', async ({ page }) => {
    const box = area(page).getByRole('checkbox', { name: 'Repeat every month' });
    await expect(area(page).locator('.checkbox__row').getByRole('button')).toHaveCount(0);
    await q(page, 'Repeating').click();
    await expect(box).not.toBeChecked();
    await page.keyboard.press('Escape');

    await box.check();
    await askVia(page, 'Repeating');
    await expectRequest(page, 'checkbox', 'Repeat every month', 'true');
  });

  test('Switch: the ? sits beside it, out of the Tab order', async ({ page }) => {
    const sw = area(page).getByRole('switch', { name: 'Show archived' });
    await expect(sw.getByRole('button')).toHaveCount(0);
    await expect(q(page, 'Archived accounts')).toHaveAttribute('tabindex', '-1');
    await askVia(page, 'Archived accounts');
    await expectRequest(page, 'switch', 'Show archived');
    await expect(sw).toHaveAttribute('aria-checked', 'false');
  });

  test('Toolbar: the ? sits by the title, not by the buttons', async ({ page }) => {
    const title = await box(area(page).locator('.toolbar__title'));
    const help = await box(q(page, 'Accounts'));
    const button = await box(area(page).getByRole('button', { name: 'New account' }));
    expect(help.x - (title.x + title.width)).toBeLessThan(12);
    expect(button.x - (help.x + help.width)).toBeGreaterThan(100);
    await askVia(page, 'Accounts');
    await expectRequest(page, 'toolbar', 'Accounts');
  });

  for (const [opener, title, topicTitle, role, component] of [
    ['Dialog with help…', 'Edit split', 'Editing a split', 'dialog', 'dialog'],
    ['Confirm with help…', 'Delete split?', 'Deleting a split', 'alertdialog', 'alertdialog'],
    ['Sheet with help…', 'Menu', 'The menu', 'dialog', 'sheet'],
  ] as const) {
    test(`${component}: the ? is in the title bar, just before the ×`, async ({ page }) => {
      await area(page).getByRole('button', { name: opener }).click();
      const modal = page.getByRole(role, { name: title });
      await expect(modal).toBeVisible();
      // A sheet slides in: measure once it has arrived.
      await expect.poll(() => modal.evaluate((el) => getComputedStyle(el).transform)).toBe('none');
      const help = await box(modal.getByRole('button', { name: `Help: ${topicTitle}` }));
      const close = await box(modal.getByRole('button', { name: 'Close' }));
      expect(close.x - (help.x + help.width)).toBeLessThan(20);
      expect(close.x).toBeGreaterThan(help.x);

      await askVia(page, topicTitle);
      await expectRequest(page, component, title);
    });
  }
});
