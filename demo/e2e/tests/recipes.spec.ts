import type { Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// The recipes (ferrox-ui/docs/recipes, compiled into the demo): one small app, "Cookbook", at /recipes,
// outside the demo's frame. Its store answers after 400ms; `?offline` makes
// the first load fail.

const cards = (page: Page) => page.getByRole('list', { name: 'Recipes' }).getByRole('listitem');
const card = (page: Page, name: string) => cards(page).filter({ has: page.getByRole('heading', { name, exact: true }) });
const rowAction = async (page: Page, name: string, action: string) => {
  await card(page, name).getByRole('button', { name: 'More actions' }).click();
  await page.getByRole('menuitem', { name: action }).click();
};

test.beforeEach(async ({ page }) => {
  await page.setViewportSize({ width: 1200, height: 900 });
});

test.describe('app shell', () => {
  test('an app bar (the banner), sticky; the name links home; a user menu with a header', async ({ page }) => {
    await openApp(page, '/recipes');
    const bar = page.getByRole('banner');
    await expect(bar).toHaveClass(/app-bar--sticky/);
    await expect(bar.getByRole('link', { name: 'Cookbook' })).toHaveAttribute('href', '/recipes');
    await bar.getByRole('button', { name: 'Account' }).click();
    await expect(page.locator('.menu__panel:popover-open .menu__header')).toHaveText('Signed in as ada@example.org');
    await page.getByRole('menuitem', { name: 'Settings' }).click();
    await expect(page).toHaveURL(/\/recipes\/settings$/);
  });

  test('the side menu has the app\'s pages, and closes on a link', async ({ page }) => {
    await openApp(page, '/recipes');
    await page.getByRole('button', { name: 'Menu' }).click();
    const nav = page.getByRole('navigation', { name: 'Sections' });
    await nav.getByRole('link', { name: 'Settings' }).click();
    await expect(page).toHaveURL(/\/recipes\/settings$/);
    await expect(page.getByRole('dialog', { name: 'Cookbook' })).toBeHidden();
  });

  test('the page is the main landmark; the demo\'s frame is not there', async ({ page }) => {
    await openApp(page, '/recipes');
    // One <main>: the shell's, with the page's grid inside it.
    await expect(page.getByRole('main')).toHaveCount(1);
    await expect(page.getByRole('main').locator('.page-grid')).toHaveCount(1);
    await expect(page.locator('.demo-header')).toHaveCount(0);
  });

  test('a skip link is the first Tab stop, shown on focus, and moves focus to the page', async ({ page }) => {
    await openApp(page, '/recipes');
    const skip = page.getByRole('link', { name: 'Skip to content' });
    const hidden = await skip.boundingBox();
    expect(hidden!.y + hidden!.height).toBeLessThanOrEqual(0);
    await page.keyboard.press('Tab');
    await expect(skip).toBeFocused();
    const shown = await skip.boundingBox();
    expect(shown!.y).toBeGreaterThanOrEqual(0);
    await page.keyboard.press('Enter');
    await expect(page.getByRole('main')).toBeFocused();
    // Tab goes on from the page, not back to the bar.
    await page.keyboard.press('Tab');
    expect(await page.evaluate(() => !!document.activeElement?.closest('main'))).toBe(true);
  });

  test("the side menu's footer, below its links", async ({ page }) => {
    await openApp(page, '/recipes');
    await page.getByRole('button', { name: 'Menu' }).click();
    const menu = page.getByRole('dialog', { name: 'Cookbook' });
    await expect(menu.locator('.side-menu__footer').getByRole('link', { name: 'Back to the ferrox-ui demo' })).toBeVisible();
  });
});

test.describe('Page: the tab title and focus', () => {
  test("the tab's title is the page's, then the app's, and follows a title that arrives late", async ({ page }) => {
    await openApp(page, '/recipes');
    await expect(page).toHaveTitle('Recipes · Cookbook');
    // A deep link: the recipe's name arrives with the data.
    await openApp(page, '/recipes/2');
    await expect(page).toHaveTitle('Ribollita · Cookbook');
  });

  test('on the first load, focus stays where the browser puts it', async ({ page }) => {
    await openApp(page, '/recipes');
    await expect(page.getByRole('heading', { level: 1, name: 'Recipes' })).toBeVisible();
    await page.waitForTimeout(200);
    expect(await page.evaluate(() => document.activeElement === document.body)).toBe(true);
  });

  test("after following a link, focus is on the new page's title", async ({ page }) => {
    await openApp(page, '/recipes');
    await page.getByRole('button', { name: 'Menu' }).click();
    await page.getByRole('navigation', { name: 'Sections' }).getByRole('link', { name: 'Settings' }).click();
    const title = page.getByRole('heading', { level: 1, name: 'Settings' });
    await expect(title).toBeFocused();
    await expect(page).toHaveTitle('Settings · Cookbook');
    // Tab goes on from the page's title.
    await page.keyboard.press('Tab');
    expect(await page.evaluate(() => !!document.activeElement?.closest('main'))).toBe(true);
  });

  test("one h1 per page; a settings page's further panels follow its own", async ({ page }) => {
    await openApp(page, '/recipes/settings');
    await expect(page.getByRole('heading', { level: 1 })).toHaveCount(1);
    const headings = await page.locator('main .panel').evaluateAll((panels) =>
      panels.map((p) => p.querySelector('h1, h2')?.textContent?.trim()),
    );
    // A collapsible panel's heading also says its summary, after its title.
    expect(headings[0]).toBe('Settings');
    expect(headings[1]).toMatch(/^Advanced/);
    expect(headings[2]).toMatch(/^Danger zone/);
  });
});

test.describe('list page', () => {
  test('loads the recipes as cards; a card is a link to its page', async ({ page }) => {
    await openApp(page, '/recipes');
    await expect(cards(page)).toHaveCount(3);
    await expect(card(page, 'Shakshuka')).toContainText('30 min');
    await card(page, 'Shakshuka').getByRole('link', { name: 'Shakshuka' }).click();
    await expect(page).toHaveURL(/\/recipes\/1$/);
    await expect(page.getByRole('heading', { level: 1, name: 'Shakshuka' })).toBeVisible();
  });

  test('adds a recipe: errors only after trying, then saving, then the new card', async ({ page }) => {
    await openApp(page, '/recipes');
    await expect(cards(page)).toHaveCount(3);
    await page.getByRole('button', { name: 'Add recipe' }).click();
    const dialog = page.getByRole('dialog', { name: 'New recipe' });
    await expect(dialog.getByText('Give the recipe a name.')).toHaveCount(0);
    await dialog.getByRole('button', { name: 'Save' }).click();
    await expect(dialog.getByText('Give the recipe a name.')).toBeVisible();
    await expect(dialog.getByText('Minutes: a whole number above 0.')).toBeVisible();
    await dialog.getByLabel('Name').fill('Dal makhani');
    await dialog.getByLabel('Minutes').fill('45');
    await dialog.getByRole('button', { name: 'Save' }).click();
    await expect(dialog.getByRole('button', { name: 'Save' })).toHaveClass(/is-saving/);
    await expect(dialog).toBeHidden();
    await expect(cards(page)).toHaveCount(4);
    await expect(card(page, 'Dal makhani')).toContainText('45 min');
  });

  test('edits from the row menu: the dialog is filled, the card shows the change', async ({ page }) => {
    await openApp(page, '/recipes');
    await rowAction(page, 'Ribollita', 'Edit');
    const dialog = page.getByRole('dialog', { name: 'Edit recipe' });
    await expect(dialog.getByLabel('Name')).toHaveValue('Ribollita');
    await dialog.getByLabel('Name').fill('Ribollita, the next day');
    await dialog.getByRole('button', { name: 'Save' }).click();
    await expect(card(page, 'Ribollita, the next day')).toHaveCount(1);
    await expect(cards(page)).toHaveCount(3);
  });

  test('deletes after a confirmation naming the recipe', async ({ page }) => {
    await openApp(page, '/recipes');
    await rowAction(page, 'Pierogi', 'Delete');
    const confirm = page.getByRole('alertdialog', { name: 'Delete recipe?' });
    await expect(confirm).toContainText('This deletes “Pierogi”.');
    await confirm.getByRole('button', { name: 'Delete' }).click();
    await expect(confirm).toBeHidden();
    await expect(cards(page)).toHaveCount(2);
  });

  test('the layout switch re-lays the cards', async ({ page }) => {
    await openApp(page, '/recipes');
    await page.getByRole('group', { name: 'Layout' }).getByRole('button', { name: 'List' }).click();
    await expect(page.locator('.data-view')).toHaveClass(/data-view--list/);
  });

  test('offline: the error with Retry, then the recipes', async ({ page }) => {
    await openApp(page, '/recipes?offline');
    const alert = page.locator('.data-view__error');
    await expect(alert).toContainText("Couldn't load the recipes.");
    await alert.getByRole('button', { name: 'Retry' }).click();
    await expect(cards(page)).toHaveCount(3);
    await expect(alert).toHaveCount(0);
  });
});

test.describe('detail page', () => {
  test('its name as the page title, its facts in the side column', async ({ page }) => {
    await openApp(page, '/recipes/2');
    await expect(page.getByRole('heading', { level: 1, name: 'Ribollita' })).toBeVisible();
    const side = page.getByRole('complementary');
    await expect(side.locator('dt', { hasText: 'Time' })).toBeVisible();
    await expect(side.locator('dd', { hasText: '90 minutes' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'All recipes' })).toHaveAttribute('href', '/recipes');
  });

  test('Edit, and Duplicate behind the chevron, which opens the copy\'s page once saved', async ({ page }) => {
    await openApp(page, '/recipes/1');
    await page.getByRole('button', { name: 'Edit', exact: true }).click();
    await expect(page.getByRole('dialog', { name: 'Edit recipe' }).getByLabel('Name')).toHaveValue('Shakshuka');
    await page.keyboard.press('Escape');
    await page.getByRole('button', { name: 'More recipe actions' }).click();
    await page.getByRole('menuitem', { name: 'Duplicate' }).click();
    const dialog = page.getByRole('dialog', { name: 'New recipe' });
    await expect(dialog.getByLabel('Name')).toHaveValue('Shakshuka (copy)');
    await dialog.getByRole('button', { name: 'Save' }).click();
    await expect(page).toHaveURL(/\/recipes\/4$/);
    await expect(page.getByRole('heading', { level: 1, name: 'Shakshuka (copy)' })).toBeVisible();
  });

  test('Delete asks, then goes back to the list without it', async ({ page }) => {
    await openApp(page, '/recipes/3');
    await page.getByRole('button', { name: 'More recipe actions' }).click();
    await page.getByRole('menuitem', { name: 'Delete' }).click();
    await page.getByRole('alertdialog', { name: 'Delete recipe?' }).getByRole('button', { name: 'Delete' }).click();
    await expect(page).toHaveURL(/\/recipes$/);
    await expect(cards(page)).toHaveCount(2);
  });

  test('no such recipe: says so, once the recipes have loaded', async ({ page }) => {
    await openApp(page, '/recipes/99');
    await expect(page.getByRole('heading', { level: 1, name: 'Not found' })).toBeVisible();
  });
});

test.describe('settings page', () => {
  test('cooking times on the cards follow the switch', async ({ page }) => {
    await openApp(page, '/recipes/settings');
    await page.getByRole('switch', { name: 'Show cooking times on cards' }).click();
    await page.getByRole('banner').getByRole('link', { name: 'Cookbook' }).click();
    await expect(cards(page)).toHaveCount(3);
    await expect(card(page, 'Shakshuka')).not.toContainText('min');
  });

  test('units show on a recipe\'s page', async ({ page }) => {
    await openApp(page, '/recipes/settings');
    await page.getByLabel('Units').selectOption('us');
    await page.getByRole('banner').getByRole('link', { name: 'Cookbook' }).click();
    await card(page, 'Shakshuka').getByRole('link', { name: 'Shakshuka' }).click();
    await expect(page.getByRole('complementary').locator('dd', { hasText: 'US cups and ounces' })).toBeVisible();
  });

  test('without "Ask before deleting", a delete happens at once', async ({ page }) => {
    await openApp(page, '/recipes/settings');
    await page.getByRole('button', { name: /Advanced/ }).click();
    await page.getByRole('switch', { name: 'Ask before deleting a recipe' }).click();
    await page.getByRole('banner').getByRole('link', { name: 'Cookbook' }).click();
    await expect(cards(page)).toHaveCount(3);
    await rowAction(page, 'Pierogi', 'Delete');
    await expect(page.getByRole('alertdialog')).toHaveCount(0);
    await expect(cards(page)).toHaveCount(2);
  });

  test('the danger zone asks, then the list is empty with its way to add one', async ({ page }) => {
    await openApp(page, '/recipes/settings');
    await page.getByRole('button', { name: /Danger zone/ }).click();
    await page.getByRole('button', { name: 'Delete all recipes…' }).click();
    await page.getByRole('alertdialog', { name: 'Delete every recipe?' }).getByRole('button', { name: 'Delete all' }).click();
    await page.getByRole('banner').getByRole('link', { name: 'Cookbook' }).click();
    await expect(page.getByText('No recipes yet.')).toBeVisible();
    await page.getByRole('button', { name: 'Add a recipe' }).click();
    await expect(page.getByRole('dialog', { name: 'New recipe' })).toBeVisible();
  });
});
