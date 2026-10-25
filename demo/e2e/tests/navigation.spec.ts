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

test.describe('NavGroup', () => {
  test.beforeEach(async ({ page }) => {
    await openApp(page, '/navigation');
  });

  /** A collapsible group's toggle, in the list named `list`. */
  function toggle(page: Page, list: string, group: string): Locator {
    return nav(page, list).getByRole('button', { name: group });
  }

  /** The list a toggle controls (found by id, so also while it's hidden). */
  async function controlled(page: Page, button: Locator): Promise<Locator> {
    const id = await button.getAttribute('aria-controls');
    if (!id) throw new Error('toggle has no aria-controls');
    return page.locator(`[id="${id}"]`);
  }

  test('a plain group: its heading names its list, a hairline splits it off, the heading is not focusable', async ({ page }) => {
    const grouped = nav(page, 'Grouped');
    const account = grouped.getByRole('list', { name: 'Account' });
    await expect(account.getByRole('link')).toHaveCount(2);

    const group = account.locator('xpath=..');
    await expect(group).toHaveClass(/\bnav-group\b/);
    expect(await group.evaluate((el) => getComputedStyle(el).borderTopWidth)).toBe('1px');

    const heading = group.locator('.nav-group__heading');
    await expect(heading).toHaveText(/account/i);
    expect(await heading.evaluate((el) => (el as HTMLElement).tabIndex)).toBe(-1);
    await expect(grouped.getByRole('button', { name: 'Account' })).toHaveCount(0);
  });

  test('a collapsible group starts collapsed; its toggle shows and hides the links', async ({ page }) => {
    const button = toggle(page, 'Grouped', 'Product database');
    const list = await controlled(page, button);
    await expect(button).toHaveAttribute('aria-expanded', 'false');
    await expect(list).toBeHidden();

    await button.click();
    await expect(button).toHaveAttribute('aria-expanded', 'true');
    await expect(list).toBeVisible();
    await expect(nav(page, 'Grouped').getByRole('list', { name: 'Product database' }).getByRole('link')).toHaveCount(3);

    await button.click();
    await expect(button).toHaveAttribute('aria-expanded', 'false');
    await expect(list).toBeHidden();
  });

  test('Enter and Space toggle it', async ({ page }) => {
    const button = toggle(page, 'Grouped', 'Product database');
    await button.focus();
    await page.keyboard.press('Enter');
    await expect(button).toHaveAttribute('aria-expanded', 'true');
    await page.keyboard.press('Space');
    await expect(button).toHaveAttribute('aria-expanded', 'false');
  });

  test('the chevron turns while expanded', async ({ page }) => {
    const button = toggle(page, 'Grouped', 'Product database');
    const chevron = button.locator('svg.nav-group__chevron');
    await expect(chevron).toHaveAttribute('aria-hidden', 'true');
    await expect(chevron).toHaveCSS('transform', 'none');
    await button.click();
    await expect(chevron).not.toHaveCSS('transform', 'none');
  });

  test('Tab skips the links of a collapsed group', async ({ page }) => {
    await toggle(page, 'Grouped', 'Product database').focus();
    await page.keyboard.press('Tab');
    await expect(nav(page, 'Grouped').getByRole('link', { name: 'Help' })).toBeFocused();
  });

  test('start_expanded starts it expanded', async ({ page }) => {
    const button = toggle(page, 'Group states', 'Starts expanded');
    await expect(button).toHaveAttribute('aria-expanded', 'true');
    await expect(await controlled(page, button)).toBeVisible();
  });

  test('an owned state is two-way', async ({ page }) => {
    const button = toggle(page, 'Group states', 'Owned by the page');
    await expect(button).toHaveAttribute('aria-expanded', 'false');

    await page.getByRole('button', { name: 'Toggle from code' }).click();
    await expect(button).toHaveAttribute('aria-expanded', 'true');

    await button.click();
    await expect(page.getByText('Owned group: collapsed')).toBeVisible();
  });

  test('a collapsed group expands itself when the route moves to one of its links', async ({ page }) => {
    const button = toggle(page, 'Grouped', 'Product database');
    await expect(button).toHaveAttribute('aria-expanded', 'false');

    await nav(page, 'Nested').getByRole('link', { name: 'Details' }).click();
    await expect(page).toHaveURL(/\/navigation\/details$/);
    await expect(button).toHaveAttribute('aria-expanded', 'true');
    await expect(nav(page, 'Grouped').getByRole('link', { name: 'Details' })).toHaveAttribute('aria-current', 'page');
  });

  test('a toggle is at least 44px tall on a phone', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    expect((await box(toggle(page, 'Grouped', 'Product database'))).height).toBeGreaterThanOrEqual(44);
  });
});

// Opened straight on a page inside a group: its own describe, so the page is
// opened once (a second navigation aborts font loads, an error in Firefox).
test.describe('NavGroup, opened inside it', () => {
  test.beforeEach(async ({ page }) => {
    await openApp(page, '/navigation/details');
  });

  test('a group holding the page it opens on starts expanded, and can still be collapsed', async ({ page }) => {
    const button = nav(page, 'Grouped').getByRole('button', { name: 'Product database' });
    await expect(button).toHaveAttribute('aria-expanded', 'true');

    await button.click();
    await expect(button).toHaveAttribute('aria-expanded', 'false');
  });
});

// SideMenu and SideMenuButton: the demo's own menu, in its header (src/menu.rs).
test.describe('SideMenu', () => {
  test.beforeEach(async ({ page }) => {
    await openApp(page, '/navigation');
  });

  function menuButton(page: Page): Locator {
    return page.getByRole('button', { name: 'Menu', exact: true });
  }

  function menu(page: Page): Locator {
    return page.getByRole('dialog', { name: 'ferrox-ui demo' });
  }

  async function openMenu(page: Page): Promise<Locator> {
    await menuButton(page).click();
    await expect(menu(page)).toBeVisible();
    return menu(page);
  }

  test('the button says it opens a dialog, and whether it is open', async ({ page }) => {
    const button = menuButton(page);
    await expect(button).toHaveAttribute('aria-haspopup', 'dialog');
    await expect(button).toHaveAttribute('aria-expanded', 'false');
    await expect(button.locator('svg')).toHaveAttribute('data-icon', 'menu');

    await openMenu(page);
    await expect(button).toHaveAttribute('aria-expanded', 'true');
  });

  test('it opens as a modal sheet named by its title, holding a labelled nav', async ({ page }) => {
    const sheet = await openMenu(page);
    await expect(sheet).toHaveClass(/\bsheet\b/);
    await expect(sheet).toHaveClass(/\bside-menu\b/);
    expect(await sheet.evaluate((el) => el.matches(':modal'))).toBe(true);
    await expect(sheet.getByRole('navigation', { name: 'Sections' }).getByRole('link', { name: 'Buttons' })).toBeVisible();
  });

  test('following a link navigates and closes it', async ({ page }) => {
    const sheet = await openMenu(page);
    await sheet.getByRole('link', { name: 'Buttons' }).click();
    await expect(page).toHaveURL(/\/buttons$/);
    await expect(sheet).toBeHidden();
    await expect(menuButton(page)).toHaveAttribute('aria-expanded', 'false');
  });

  test('a plain router link closes it too', async ({ page }) => {
    const sheet = await openMenu(page);
    await sheet.getByRole('link', { name: 'About help' }).click();
    await expect(page).toHaveURL(/\/help$/);
    await expect(sheet).toBeHidden();
  });

  test("the current page's link closes it, though the route stays", async ({ page }) => {
    const sheet = await openMenu(page);
    const current = sheet.getByRole('link', { name: 'Navigation', exact: true });
    await expect(current).toHaveAttribute('aria-current', 'page');
    await current.click();
    await expect(sheet).toBeHidden();
    await expect(page).toHaveURL(/\/navigation$/);
  });

  test('the back button closes it', async ({ page }) => {
    await nav(page, 'Nested').getByRole('link', { name: 'Details' }).click();
    await expect(page).toHaveURL(/\/navigation\/details$/);
    const sheet = await openMenu(page);
    await page.goBack();
    await expect(page).toHaveURL(/\/navigation$/);
    await expect(sheet).toBeHidden();
  });

  for (const [way, close] of [
    ['Esc', async (page: Page) => page.keyboard.press('Escape')],
    ['the ×', async (page: Page) => menu(page).getByRole('button', { name: 'Close' }).click()],
    ['the backdrop', async (page: Page) => page.mouse.click(380, 400)],
  ] as const) {
    test(`${way} closes it, and focus returns to the button`, async ({ page }) => {
      await page.setViewportSize({ width: 400, height: 800 });
      await openMenu(page);
      await close(page);
      await expect(menu(page)).toBeHidden();
      await expect(menuButton(page)).toBeFocused();
      await expect(menuButton(page)).toHaveAttribute('aria-expanded', 'false');
    });
  }

  test('the footer sits at the bottom of the sheet', async ({ page }) => {
    const sheet = await openMenu(page);
    const footer = sheet.locator('.side-menu__footer');
    await expect(footer).toBeVisible();
    const sheetBox = await box(sheet);
    const footerBox = await box(footer);
    expect(sheetBox.y + sheetBox.height - (footerBox.y + footerBox.height)).toBeLessThan(48);
  });

  test('help puts a ? in the title bar', async ({ page }) => {
    const sheet = await openMenu(page);
    await expect(sheet.locator('.sheet__title-bar').getByRole('button', { name: 'Help: The demo menu' })).toBeVisible();
  });

  test('a collapsible group starts collapsed on each opening, unless the current page is in it', async ({ page }) => {
    let sheet = await openMenu(page);
    const group = () => menu(page).getByRole('button', { name: 'Navigation pages' });
    // On /navigation: the group holds the current page.
    await expect(group()).toHaveAttribute('aria-expanded', 'true');

    await sheet.getByRole('link', { name: 'Buttons' }).click();
    await expect(page).toHaveURL(/\/buttons$/);
    sheet = await openMenu(page);
    await expect(group()).toHaveAttribute('aria-expanded', 'false');

    await group().click();
    await expect(group()).toHaveAttribute('aria-expanded', 'true');
    await page.keyboard.press('Escape');
    await expect(sheet).toBeHidden();
    await openMenu(page);
    await expect(group()).toHaveAttribute('aria-expanded', 'false');
  });
});

test.describe('the side menu on a phone', () => {
  test.use({ viewport: { width: 390, height: 640 } });

  test('only the menu scrolls: its end is in view, the page behind never moves', async ({ page }) => {
    await openApp(page, '/');
    await page.getByRole('button', { name: 'Menu', exact: true }).click();
    const sheet = page.locator('dialog.sheet[open]');
    await expect(sheet).toBeVisible();
    // Done sliding in: wheel events during the slide don't scroll it yet.
    await expect.poll(() => sheet.evaluate((el) => el.getAnimations().length)).toBe(0);
    const body = sheet.locator('.sheet__body');
    expect(await body.evaluate((el) => getComputedStyle(el).overscrollBehaviorY)).toBe('contain');
    expect(await page.evaluate(() => getComputedStyle(document.documentElement).overflow)).toBe('hidden');
    // The sheet is the visible height, so its end can't sit under a toolbar.
    expect((await sheet.boundingBox())!.height).toBeCloseTo(await page.evaluate(() => innerHeight), 0);
    // More links than fit: the body scrolls, to its end, and no further.
    expect(await body.evaluate((el) => el.scrollHeight > el.clientHeight)).toBe(true);
    const box = (await body.boundingBox())!;
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    for (let i = 0; i < 6; i++) {
      await page.mouse.wheel(0, 300);
      await page.waitForTimeout(100);
    }
    await expect.poll(() => body.evaluate((el) => Math.ceil(el.scrollTop + el.clientHeight) >= el.scrollHeight)).toBe(true);
    expect(await page.evaluate(() => scrollY)).toBe(0);
    const last = body.getByRole('link').last();
    const lastBox = (await last.boundingBox())!;
    expect(lastBox.y + lastBox.height).toBeLessThanOrEqual(box.y + box.height + 0.5);
  });
});
