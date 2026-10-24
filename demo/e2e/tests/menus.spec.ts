import type { Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// Menu on the demo's /menus page: "More actions" (an icon, end-aligned:
// Edit, Duplicate, Archive | Delete), "Export" (a button, start-aligned),
// "Account" (a custom trigger), "Locked" (disabled), a menu in a dialog,
// and "Low menu" low on the page. `chosen` logs every action.

const trigger = (page: Page, name: string) => page.getByRole('button', { name, exact: true });
const menu = (page: Page, name: string) => page.getByRole('menu', { name });
const items = (page: Page, name: string) => menu(page, name).getByRole('menuitem');
const chosen = (page: Page) => page.locator('[data-echo="chosen"]');
const focusedText = (page: Page) => page.evaluate(() => document.activeElement?.textContent?.trim() ?? '');

test.beforeEach(async ({ page }) => {
  await page.setViewportSize({ width: 1200, height: 900 });
  await openApp(page, '/menus');
});

test.describe('Menu', () => {
  test('the trigger says it opens a menu; the menu is named and holds menuitems', async ({ page }) => {
    const more = trigger(page, 'More actions');
    await expect(more).toHaveAttribute('aria-haspopup', 'menu');
    await expect(more).toHaveAttribute('aria-expanded', 'false');
    await more.click();
    await expect(more).toHaveAttribute('aria-expanded', 'true');
    await expect(menu(page, 'Recipe actions')).toBeVisible();
    expect(await more.getAttribute('aria-controls')).toBe(await menu(page, 'Recipe actions').getAttribute('id'));
    await expect(items(page, 'Recipe actions')).toHaveText(['Edit', 'Duplicate', 'Archive', 'Delete']);
    await expect(menu(page, 'Recipe actions').getByRole('separator')).toHaveCount(1);
  });

  test('a click, Enter or Space opens it on the first item', async ({ page }) => {
    await trigger(page, 'More actions').click();
    await expect(items(page, 'Recipe actions').first()).toBeFocused();
    await page.keyboard.press('Escape');
    for (const key of ['Enter', ' ']) {
      await trigger(page, 'More actions').focus();
      await page.keyboard.press(key);
      await expect(items(page, 'Recipe actions').first()).toBeFocused();
      await page.keyboard.press('Escape');
    }
  });

  test('↓ opens on the first item, ↑ on the last', async ({ page }) => {
    await trigger(page, 'More actions').focus();
    await page.keyboard.press('ArrowDown');
    await expect(items(page, 'Recipe actions').first()).toBeFocused();
    await page.keyboard.press('Escape');
    await page.keyboard.press('ArrowUp');
    await expect(items(page, 'Recipe actions').last()).toBeFocused();
  });

  test('↑ / ↓ wrap; Home / End; a letter finds the next item starting with it', async ({ page }) => {
    await trigger(page, 'More actions').click();
    await expect(items(page, 'Recipe actions').first()).toBeFocused();
    await page.keyboard.press('ArrowUp');
    expect(await focusedText(page)).toBe('Delete');
    await page.keyboard.press('ArrowDown');
    expect(await focusedText(page)).toBe('Edit');
    await page.keyboard.press('End');
    expect(await focusedText(page)).toBe('Delete');
    await page.keyboard.press('Home');
    expect(await focusedText(page)).toBe('Edit');
    await page.keyboard.press('d');
    expect(await focusedText(page)).toBe('Duplicate');
    await page.keyboard.press('d');
    expect(await focusedText(page)).toBe('Delete');
    await page.keyboard.press('a');
    expect(await focusedText(page)).toBe('Archive');
  });

  test('choosing closes the menu, returns focus, runs the action once', async ({ page }) => {
    const more = trigger(page, 'More actions');
    await more.click();
    await items(page, 'Recipe actions').nth(1).click();
    await expect(chosen(page)).toHaveText('duplicate');
    await expect(menu(page, 'Recipe actions')).toBeHidden();
    await expect(more).toBeFocused();
    await expect(more).toHaveAttribute('aria-expanded', 'false');

    await page.keyboard.press('Enter');
    await expect(items(page, 'Recipe actions').first()).toBeFocused();
    await page.keyboard.press('ArrowDown');
    await page.keyboard.press('Enter');
    await expect(chosen(page)).toHaveText('duplicate, duplicate');
    await expect(more).toBeFocused();
    await page.keyboard.press('ArrowUp');
    await expect(items(page, 'Recipe actions').last()).toBeFocused();
    await page.keyboard.press(' ');
    await expect(chosen(page)).toHaveText('duplicate, duplicate, delete');
    await expect(more).toBeFocused();
  });

  test('Esc closes and returns focus; Tab closes and moves on', async ({ page }) => {
    const more = trigger(page, 'More actions');
    await more.click();
    await page.keyboard.press('Escape');
    await expect(menu(page, 'Recipe actions')).toBeHidden();
    await expect(more).toBeFocused();

    await page.keyboard.press('Enter');
    await expect(items(page, 'Recipe actions').first()).toBeFocused();
    await page.keyboard.press('Tab');
    await expect(menu(page, 'Recipe actions')).toBeHidden();
    await expect(trigger(page, 'Export')).toBeFocused();
    await expect(chosen(page)).toHaveText('');
  });

  test('a press outside closes it without taking focus back', async ({ page }) => {
    await trigger(page, 'More actions').click();
    await page.getByRole('heading', { name: 'Low on the page' }).click();
    await expect(menu(page, 'Recipe actions')).toBeHidden();
    await expect(trigger(page, 'More actions')).not.toBeFocused();
    await expect(chosen(page)).toHaveText('');
  });

  test('placement: under the trigger, end- or start-aligned', async ({ page }) => {
    const more = trigger(page, 'More actions');
    await more.click();
    await expect(items(page, 'Recipe actions').first()).toBeFocused();
    const t = (await more.boundingBox())!;
    const p = (await page.locator('.menu__panel:popover-open').boundingBox())!;
    expect(p.y).toBeGreaterThanOrEqual(t.y + t.height);
    expect(Math.abs(p.x + p.width - (t.x + t.width))).toBeLessThan(1.5);
    await page.keyboard.press('Escape');

    const exp = trigger(page, 'Export');
    await exp.click();
    await expect(items(page, 'Export').first()).toBeFocused();
    const e = (await exp.boundingBox())!;
    const q = (await page.locator('.menu__panel:popover-open').boundingBox())!;
    expect(Math.abs(q.x - e.x)).toBeLessThan(1.5);
    expect(q.y).toBeGreaterThanOrEqual(e.y + e.height);
  });

  test('near the bottom of the window it opens above; it follows the trigger on scroll', async ({ page }) => {
    const low = trigger(page, 'Low menu');
    const at = (await low.boundingBox())!;
    // The window ends just under the trigger: no room below for four items.
    await page.setViewportSize({ width: 1200, height: Math.ceil(at.y + at.height + 40) });
    await low.click();
    await expect(items(page, 'Low actions').first()).toBeFocused();
    const panel = page.locator('.menu__panel:popover-open');
    await expect(panel).toHaveAttribute('data-side', 'above');
    const t = (await low.boundingBox())!;
    const p = (await panel.boundingBox())!;
    expect(p.y + p.height).toBeLessThanOrEqual(t.y);

    await page.setViewportSize({ width: 1200, height: Math.ceil(at.y + at.height + 40) - 1 });
    await page.mouse.wheel(0, 30);
    await expect.poll(async () => {
      const t2 = (await low.boundingBox())!;
      const p2 = (await panel.boundingBox())!;
      return Math.round(t2.y - (p2.y + p2.height));
    }).toBe(Math.round(t.y - (p.y + p.height)));
  });

  test('on a phone it stays inside the viewport', async ({ page }) => {
    await page.setViewportSize({ width: 360, height: 700 });
    await trigger(page, 'Account').click();
    await expect(items(page, 'Account').first()).toBeFocused();
    const p = (await page.locator('.menu__panel:popover-open').boundingBox())!;
    expect(p.x).toBeGreaterThanOrEqual(0);
    expect(p.x + p.width).toBeLessThanOrEqual(360);
  });

  test('each kind of trigger', async ({ page }) => {
    const exp = trigger(page, 'Export');
    await expect(exp).toHaveClass(/\bbtn\b/);
    await expect(exp.locator('svg.icon')).toHaveAttribute('data-icon', 'chevron-down');
    await expect(trigger(page, 'More actions').locator('svg.icon')).toHaveAttribute('data-icon', 'more-vertical');
    const account = trigger(page, 'Account');
    await expect(account).toContainText('Ada Lovelace');
    await account.click();
    await expect(items(page, 'Account')).toHaveText(['Settings', 'Help', 'Sign out']);
  });

  test('a disabled menu does not open', async ({ page }) => {
    await expect(trigger(page, 'Locked')).toBeDisabled();
    await expect(menu(page, 'Locked actions')).toBeHidden();
  });

  test('inside a dialog: not clipped, Esc closes only the menu, choosing keeps the dialog', async ({ page }) => {
    await page.getByRole('button', { name: 'Menu in a dialog…' }).click();
    const dialog = page.getByRole('dialog', { name: 'Move a recipe' });
    await dialog.getByRole('button', { name: 'Destination' }).click();
    const first = items(page, 'Destinations').first();
    await expect(first).toBeFocused();
    const b = (await first.boundingBox())!;
    const hit = await page.evaluate(([x, y]) => document.elementFromPoint(x, y)?.textContent?.trim(), [b.x + b.width / 2, b.y + b.height / 2]);
    expect(hit).toBe('Breakfast');
    await page.keyboard.press('Escape');
    await expect(menu(page, 'Destinations')).toBeHidden();
    await expect(dialog).toBeVisible();
    await expect(dialog.getByRole('button', { name: 'Destination' })).toBeFocused();

    await page.keyboard.press('Enter');
    await expect(items(page, 'Destinations').first()).toBeFocused();
    await page.keyboard.press('ArrowDown');
    await page.keyboard.press('Enter');
    await expect(chosen(page)).toHaveText('to dinner');
    await expect(dialog).toBeVisible();
  });
});

test.describe('Menu items', () => {
  test('an icon before the text', async ({ page }) => {
    await trigger(page, 'More actions').click();
    await expect(items(page, 'Recipe actions').first().locator('svg.icon')).toHaveAttribute('data-icon', 'edit');
    await expect(items(page, 'Recipe actions').last().locator('svg.icon')).toHaveAttribute('data-icon', 'delete');
    await expect(items(page, 'Recipe actions').nth(1).locator('svg.icon')).toHaveCount(0);
  });

  test('danger: its own colour', async ({ page }) => {
    await trigger(page, 'More actions').click();
    const color = (n: number) => items(page, 'Recipe actions').nth(n).evaluate((el) => getComputedStyle(el).color);
    expect(await color(3)).not.toBe(await color(0));
    await expect(items(page, 'Recipe actions').nth(3)).toHaveClass(/menu__item--danger/);
  });

  test('disabled: reachable, announced, does nothing, keeps the menu open', async ({ page }) => {
    await trigger(page, 'More actions').click();
    const archive = items(page, 'Recipe actions').nth(2);
    await expect(archive).toHaveAttribute('aria-disabled', 'true');
    await expect(items(page, 'Recipe actions').first()).toBeFocused();
    await page.keyboard.press('a');
    await expect(archive).toBeFocused();
    await page.keyboard.press('Enter');
    // Playwright won't click an aria-disabled element unless forced; a user can.
    await archive.click({ force: true });
    await expect(menu(page, 'Recipe actions')).toBeVisible();
    await expect(chosen(page)).toHaveText('');

    await page.keyboard.press('Escape');
    await page.getByRole('switch', { name: 'Archiving allowed' }).click();
    await trigger(page, 'More actions').click();
    await expect(archive).not.toHaveAttribute('aria-disabled');
    await archive.click();
    await expect(chosen(page)).toHaveText('archive');
  });

  test('current: marked with a check, moves when another is chosen, still choosable', async ({ page }) => {
    await trigger(page, 'Book').click();
    const household = items(page, 'Books').first();
    await expect(household).toHaveAttribute('aria-current', 'true');
    await expect(household.locator('svg[data-icon=check]')).toHaveCount(1);
    await expect(items(page, 'Books').locator('[aria-current]')).toHaveCount(0);
    await expect(menu(page, 'Books').locator('[aria-current=true]')).toHaveCount(1);
    await items(page, 'Books').nth(1).click();
    await expect(page.locator('[data-echo="book"]')).toHaveText('Business');
    await trigger(page, 'Book').click();
    await expect(items(page, 'Books').nth(1)).toHaveAttribute('aria-current', 'true');
    await expect(household).not.toHaveAttribute('aria-current');
    await items(page, 'Books').nth(1).click();
    await expect(chosen(page)).toHaveText('book Business, book Business');
  });

  test('a link: a menuitem that the router follows (click, Enter, Space)', async ({ page }) => {
    const mark = () => page.evaluate(() => ((window as any).__kept = true));
    const kept = () => page.evaluate(() => (window as any).__kept === true);
    await mark();
    await trigger(page, 'Go to').click();
    const layout = items(page, 'Go to').first();
    await expect(layout).toHaveAttribute('href', '/layout');
    await layout.click();
    await expect(page).toHaveURL(/\/layout$/);
    expect(await kept()).toBe(true);

    for (const key of ['Enter', ' ']) {
      await page.goBack();
      await expect(page).toHaveURL(/\/menus$/);
      await trigger(page, 'Go to').click();
      await expect(items(page, 'Go to').first()).toBeFocused();
      await page.keyboard.press('ArrowDown');
      await page.keyboard.press(key);
      await expect(page).toHaveURL(/\/data$/);
      expect(await kept()).toBe(true);
    }
  });

  test('a header above the items, outside the menu', async ({ page }) => {
    await trigger(page, 'Account').click();
    const header = page.locator('.menu__panel:popover-open .menu__header');
    await expect(header).toHaveText('Signed in as ada@example.org');
    await expect(menu(page, 'Account').getByText('Signed in as')).toHaveCount(0);
    await expect(items(page, 'Account').first()).toBeFocused();
  });

  test('on a phone, items are 44px tall', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await trigger(page, 'More actions').click();
    for (const b of await items(page, 'Recipe actions').evaluateAll((els) => els.map((el) => el.getBoundingClientRect().height))) {
      expect(b).toBeGreaterThanOrEqual(44);
    }
  });
});

test.describe('SplitButton', () => {
  const group = (page: Page, text: string) => page.locator('.split-button').filter({ has: page.getByRole('button', { name: text, exact: true }) });

  test('two buttons with their own names; the chevron opens a menu', async ({ page }) => {
    const save = group(page, 'Save');
    await expect(save.getByRole('button')).toHaveCount(2);
    await expect(save.getByRole('button', { name: 'Save', exact: true })).not.toHaveAttribute('aria-haspopup');
    const chevron = save.getByRole('button', { name: 'More save options' });
    await expect(chevron).toHaveAttribute('aria-haspopup', 'menu');
    await expect(chevron.locator('svg.icon')).toHaveAttribute('data-icon', 'chevron-down');
    await chevron.click();
    await expect(items(page, 'More save options')).toHaveText(['Save as draft', 'Save and close']);
    await expect(items(page, 'More save options').first()).toBeFocused();
    await items(page, 'More save options').nth(1).click();
    await expect(chosen(page)).toHaveText('save and close');
    await expect(chevron).toBeFocused();
  });

  test('the main button runs its own action', async ({ page }) => {
    await group(page, 'Save').getByRole('button', { name: 'Save', exact: true }).click();
    await expect(page.locator('[data-echo="saves"]')).toHaveText('1');
    await expect(menu(page, 'More save options')).toBeHidden();
  });

  test('the menu hangs from the group’s end', async ({ page }) => {
    const save = group(page, 'Save');
    await save.getByRole('button', { name: 'More save options' }).click();
    await expect(items(page, 'More save options').first()).toBeFocused();
    const g = (await save.boundingBox())!;
    const p = (await menu(page, 'More save options').locator('..').boundingBox())!;
    // End-aligned with the group, unless that would cross the viewport's
    // 8px margin, which wins.
    expect(Math.abs(p.x - Math.max(8, g.x + g.width - p.width))).toBeLessThan(1.5);
    expect(p.y).toBeGreaterThanOrEqual(g.y + g.height);
    await page.keyboard.press('Escape');

    // Publish sits far enough right for the right edges to meet.
    const publish = group(page, 'Publish');
    await publish.getByRole('button', { name: 'More publish options' }).click();
    await expect(items(page, 'More publish options').first()).toBeFocused();
    const h = (await publish.boundingBox())!;
    const q = (await menu(page, 'More publish options').locator('..').boundingBox())!;
    expect(h.x + h.width - q.width).toBeGreaterThan(8);
    expect(Math.abs(q.x + q.width - (h.x + h.width))).toBeLessThan(1.5);
  });

  test('the halves join: no gap, one height, inner corners square', async ({ page }) => {
    const save = group(page, 'Save');
    const main = (await save.getByRole('button', { name: 'Save', exact: true }).boundingBox())!;
    const chevron = (await save.getByRole('button', { name: 'More save options' }).boundingBox())!;
    expect(Math.abs(chevron.x - (main.x + main.width))).toBeLessThan(0.5);
    expect(Math.abs(chevron.y - main.y)).toBeLessThan(0.5);
    expect(Math.abs(chevron.height - main.height)).toBeLessThan(0.5);
    const radius = await save.getByRole('button', { name: 'Save', exact: true }).evaluate((el) => getComputedStyle(el).borderTopRightRadius);
    expect(radius).toBe('0px');
  });

  test('family reaches both halves', async ({ page }) => {
    const publish = group(page, 'Publish');
    await expect(publish.getByRole('button', { name: 'Publish', exact: true })).toHaveClass(/btn--secondary/);
    await expect(publish.getByRole('button', { name: 'More publish options' })).toHaveClass(/btn--secondary/);
  });

  test('saving: the main button sweeps and refuses clicks, the menu can’t open', async ({ page }) => {
    const save = group(page, 'Save');
    const main = save.getByRole('button', { name: 'Save', exact: true });
    await main.click();
    await expect(main).toHaveClass(/is-saving/);
    await expect(save.getByRole('button', { name: 'More save options' })).toBeDisabled();
    await expect(main).not.toHaveClass(/is-saving/);
    await expect(save.getByRole('button', { name: 'More save options' })).toBeEnabled();
    await expect(page.locator('[data-echo="saves"]')).toHaveText('1');
  });

  test('disabled: both halves', async ({ page }) => {
    await page.getByRole('switch', { name: 'Saving locked' }).click();
    const save = group(page, 'Save');
    await expect(save.getByRole('button', { name: 'Save', exact: true })).toBeDisabled();
    await expect(save.getByRole('button', { name: 'More save options' })).toBeDisabled();
  });
});
