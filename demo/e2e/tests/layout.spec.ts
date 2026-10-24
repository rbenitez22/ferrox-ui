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

  test('the title sits left, the actions right, in the order given', async ({ page }) => {
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

  test('each action runs once per click', async ({ page }) => {
    const lists = panel(page, 'Shopping Lists');
    await lists.getByRole('button', { name: 'Add' }).click();
    await lists.getByRole('button', { name: 'Add' }).click();
    await expect(lists.getByText('Added 2 times')).toBeVisible();

    const items = panel(page, 'Items');
    await items.getByRole('button', { name: 'Search items' }).click();
    await expect(items.getByText('Searched 1 times')).toBeVisible();
  });

  test('an action the app shows behind a permission comes and goes', async ({ page }) => {
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

  test('on a phone the title ends in an ellipsis beside the buttons, which are 44px', async ({ page }) => {
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

test.describe('Panel, collapsible', () => {
  /** The toggle of the collapsible panel named `title`. */
  function toggle(page: Page, title: string): Locator {
    return panel(page, title).locator('.panel__toggle');
  }

  /** The body a toggle controls (by id, so also while it's hidden). */
  async function body(page: Page, title: string): Promise<Locator> {
    const id = await toggle(page, title).getAttribute('aria-controls');
    if (!id) throw new Error('toggle has no aria-controls');
    return page.locator(`[id="${id}"]`);
  }

  test('the toggle is a button inside the heading, and the region is named by the title alone', async ({ page }) => {
    const notes = panel(page, 'Notes');
    await expect(notes).toBeVisible();
    const heading = notes.getByRole('heading', { level: 2 });
    const button = heading.getByRole('button');
    await expect(button).toHaveAttribute('aria-expanded', 'false');
    await expect(button).toHaveClass(/\bpanel__toggle\b/);
    await expect(notes).toHaveClass(/\bpanel--collapsible\b/);
  });

  test('starts shut, with no body built until it is first opened', async ({ page }) => {
    const notesBody = await body(page, 'Notes');
    await expect(notesBody).toBeHidden();
    await expect(notesBody.locator('input')).toHaveCount(0);
    await expect(page.getByText('Notes body built 0 times')).toBeVisible();

    await toggle(page, 'Notes').click();
    await expect(toggle(page, 'Notes')).toHaveAttribute('aria-expanded', 'true');
    await expect(notesBody).toBeVisible();
    await expect(page.getByText('Notes body built 1 times')).toBeVisible();
  });

  test('shutting hides the body; typed text survives reopening, and the body is built once', async ({ page }) => {
    await toggle(page, 'Notes').click();
    const notesBody = await body(page, 'Notes');
    await notesBody.getByRole('textbox', { name: 'Note' }).fill('milk');

    await toggle(page, 'Notes').click();
    await expect(notesBody).toBeHidden();
    await toggle(page, 'Notes').click();
    await expect(notesBody.getByRole('textbox', { name: 'Note' })).toHaveValue('milk');
    await expect(page.getByText('Notes body built 1 times')).toBeVisible();
  });

  test('the summary shows only while shut', async ({ page }) => {
    const summary = panel(page, 'Notes').locator('.panel__summary');
    await expect(summary).toHaveText('2 notes');
    await toggle(page, 'Notes').click();
    await expect(summary).toHaveCount(0);
    await toggle(page, 'Notes').click();
    await expect(summary).toHaveText('2 notes');
  });

  test('Enter and Space toggle it, and the chevron turns', async ({ page }) => {
    const button = toggle(page, 'Notes');
    const chevron = button.locator('svg.panel__chevron');
    await expect(chevron).toHaveCSS('transform', 'none');
    await button.focus();
    await page.keyboard.press('Enter');
    await expect(button).toHaveAttribute('aria-expanded', 'true');
    await expect(chevron).not.toHaveCSS('transform', 'none');
    await page.keyboard.press('Space');
    await expect(button).toHaveAttribute('aria-expanded', 'false');
  });

  test('the actions show only while open, outside the toggle', async ({ page }) => {
    const notes = panel(page, 'Notes');
    const add = notes.getByRole('button', { name: 'Add note' });
    await expect(add).toBeHidden();
    // The ? stays: help is there to explain a shut panel too.
    await expect(notes.getByRole('button', { name: 'Help: Items' })).toBeVisible();

    await toggle(page, 'Notes').click();
    await expect(add).toBeVisible();
    await expect(toggle(page, 'Notes').getByRole('button')).toHaveCount(0);
    await add.click();
    await expect(page.getByText('added 1 times')).toBeVisible();
    await expect(toggle(page, 'Notes')).toHaveAttribute('aria-expanded', 'true');
  });

  test('start_expanded starts it open', async ({ page }) => {
    await expect(toggle(page, 'Opens expanded')).toHaveAttribute('aria-expanded', 'true');
    await expect(await body(page, 'Opens expanded')).toBeVisible();
  });

  test('an owned state is two-way', async ({ page }) => {
    const button = toggle(page, 'Owned by the page');
    await expect(button).toHaveAttribute('aria-expanded', 'false');
    await page.getByRole('button', { name: 'Toggle the owned panel' }).click();
    await expect(button).toHaveAttribute('aria-expanded', 'true');
    await button.click();
    await expect(page.getByText('Owned panel: shut')).toBeVisible();
  });

  test('a shut panel draws one line at its bottom, not two', async ({ page }) => {
    const header = panel(page, 'Danger zone').locator('.panel__header');
    expect(await css(header, 'border-bottom-width')).toBe('0px');
  });

  test('the toggle reaches the band\'s left edge, and is 44px tall on a phone', async ({ page }) => {
    for (const title of ['Notes', 'Compact collapsible']) {
      const bandBox = await box(panel(page, title).locator('.panel__header'));
      const toggleBox = await box(toggle(page, title));
      expect(toggleBox.x - bandBox.x).toBeLessThan(2);
    }
    await page.setViewportSize({ width: 390, height: 844 });
    expect((await box(toggle(page, 'Notes'))).height).toBeGreaterThanOrEqual(44);
  });
});

test.describe('Card', () => {
  /** The card whose heading is `title`. */
  function card(page: Page, title: string): Locator {
    return page.getByRole('article').filter({ has: page.getByRole('heading', { name: title, exact: true }) });
  }

  test('is an article with its heading at the level asked for', async ({ page }) => {
    const plain = card(page, 'Plain card');
    await expect(plain).toHaveClass(/\bcard\b/);
    await expect(plain.getByRole('heading', { level: 3, name: 'Plain card' })).toBeVisible();
  });

  test('href: the title is the link, and a click anywhere on the card follows it in the router', async ({ page }) => {
    const ada = card(page, 'Ada Lovelace');
    const link = ada.getByRole('link');
    await expect(link).toHaveCount(1);
    await expect(link).toHaveAccessibleName('Ada Lovelace');
    await expect(ada).toHaveClass(/\bcard--clickable\b/);

    // A marker that a full page load would wipe: the router must handle it.
    await page.evaluate(() => ((window as unknown as { marker: number }).marker = 1));
    await ada.scrollIntoViewIfNeeded();
    const b = await box(ada);
    await page.mouse.click(b.x + b.width / 2, b.y + b.height - 10);
    await expect(page).toHaveURL(/\/navigation\/details$/);
    expect(await page.evaluate(() => (window as unknown as { marker?: number }).marker)).toBe(1);
  });

  test('on_click: the title is a button; a click anywhere, Enter and Space each run it once', async ({ page }) => {
    const grace = card(page, 'Grace Hopper');
    const button = grace.getByRole('button', { name: 'Grace Hopper' });
    await expect(button).toBeVisible();

    await grace.scrollIntoViewIfNeeded();
    const b = await box(grace);
    await page.mouse.click(b.x + 20, b.y + b.height - 10);
    await expect(page.getByText('opened Grace 1 times')).toBeVisible();

    await button.focus();
    await page.keyboard.press('Enter');
    await expect(page.getByText('opened Grace 2 times')).toBeVisible();
    await page.keyboard.press('Space');
    await expect(page.getByText('opened Grace 3 times')).toBeVisible();
  });

  test('an action is its own target: it never follows the card', async ({ page }) => {
    await card(page, 'Ada Lovelace').getByRole('button', { name: 'Remove' }).click();
    await card(page, 'Grace Hopper').getByRole('button', { name: 'Remove' }).click();
    await expect(page.getByText('Removed 2 times; opened Grace 0 times')).toBeVisible();
    await expect(page).toHaveURL(/\/layout$/);
  });

  test('hover and press change colour only, not size', async ({ page }) => {
    const ada = card(page, 'Ada Lovelace');
    await ada.scrollIntoViewIfNeeded();
    const before = await box(ada);
    const restingBorder = await css(ada, 'border-top-color');
    // The mouse, not hover(): the stretched link covers the body on purpose.
    await page.mouse.move(before.x + before.width / 2, before.y + before.height - 10);
    await expect.poll(() => css(ada, 'border-top-color')).not.toBe(restingBorder);
    expect(await box(ada)).toEqual(before);

    const hovered = await css(ada, 'background-color');
    await page.mouse.down();
    await expect.poll(() => css(ada, 'background-color')).not.toBe(hovered);
    expect(await box(ada)).toEqual(before);
    // Released off the card, so no click follows the link.
    await page.mouse.move(0, 0);
    await page.mouse.up();
  });

  test('the focus ring goes round the whole card', async ({ page }) => {
    const grace = card(page, 'Grace Hopper');
    await card(page, 'Ada Lovelace').getByRole('link').focus();
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await expect(grace.getByRole('button', { name: 'Grace Hopper' })).toBeFocused();
    expect(await css(grace, 'outline-style')).toBe('solid');
  });

  test('a plain card has no link, no button and no pointer', async ({ page }) => {
    const plain = card(page, 'Plain card');
    await expect(plain.getByRole('link')).toHaveCount(0);
    await expect(plain.getByRole('button')).toHaveCount(0);
    await expect(plain).not.toHaveClass(/card--clickable/);
    expect(await css(plain.locator('.card__body'), 'cursor')).not.toBe('pointer');
  });

  test('compact tightens the padding', async ({ page }) => {
    const compact = card(page, 'Compact card');
    await expect(compact).toHaveClass(/\bcard--compact\b/);
    expect(parseFloat(await css(compact, 'padding-top'))).toBeLessThan(parseFloat(await css(card(page, 'Plain card'), 'padding-top')));
  });
});

test.describe('Page layout', () => {
  const boxesOf = (page: Page, selector: string) =>
    page.locator(selector).evaluate((el) => Array.from(el.children).map((c) => c.getBoundingClientRect().toJSON() as DOMRect));

  test('Stack: one under another, a token gap between, full width', async ({ page }) => {
    const stack = page.locator('[data-demo=stack]');
    const width = (await box(stack)).width;
    const kids = await boxesOf(page, '[data-demo=stack]');
    expect(kids).toHaveLength(3);
    for (let i = 1; i < kids.length; i++) expect(Math.round(kids[i].y - (kids[i - 1].y + kids[i - 1].height))).toBe(24);
    for (const k of kids) expect(Math.abs(k.width - width)).toBeLessThan(1);
  });

  test('Cluster: side by side; Between puts the first at the start and the last at the end', async ({ page }) => {
    const cluster = page.locator('[data-demo=cluster]');
    const c = await box(cluster);
    const cancel = await box(cluster.getByRole('button', { name: 'Cancel' }));
    const publish = await box(cluster.getByRole('button', { name: 'Publish' }));
    const draft = await box(cluster.getByRole('button', { name: 'Save draft' }));
    expect(Math.abs(cancel.x - c.x)).toBeLessThan(1);
    expect(Math.abs(publish.x + publish.width - (c.x + c.width))).toBeLessThan(1);
    expect(Math.round(publish.x - (draft.x + draft.width))).toBe(8);
  });

  test('PageGrid: a side column beside the main one, capped width, padded', async ({ page }) => {
    const grid = page.locator('[data-demo=grid-end] .page-grid');
    const g = await box(grid);
    expect(g.width).toBeLessThanOrEqual(64 * 16 + 1);
    const main = await box(grid.locator('.page-grid__main'));
    const side = await box(grid.locator('.page-grid__side'));
    expect(side.x).toBeGreaterThan(main.x + main.width);
    expect(Math.round(side.width)).toBe(288);
    expect(Math.round(main.x - g.x)).toBe(24);
    const cards = await boxesOf(page, '[data-demo=grid-end] .page-grid__main');
    expect(Math.round(cards[1].y - (cards[0].y + cards[0].height))).toBe(24);
  });

  test('PageGrid: the side at the start; Narrow caps at 40rem', async ({ page }) => {
    const grid = page.locator('[data-demo=grid-start] .page-grid');
    expect((await box(grid)).width).toBeLessThanOrEqual(40 * 16 + 1);
    const main = await box(grid.locator('.page-grid__main'));
    const side = await box(grid.locator('.page-grid__side'));
    expect(side.x + side.width).toBeLessThan(main.x);
  });

  test('PageGrid on a phone: one column, the side under the main one', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    for (const which of ['grid-end', 'grid-start']) {
      const grid = page.locator(`[data-demo=${which}] .page-grid`);
      const main = await box(grid.locator('.page-grid__main'));
      const side = await box(grid.locator('.page-grid__side'));
      expect(side.y).toBeGreaterThan(main.y + main.height - 1);
      expect(Math.round(main.x - (await box(grid)).x)).toBe(16);
    }
  });

  test('landmark=false: the frames add no main or aside', async ({ page }) => {
    await expect(page.locator('[data-demo^=grid] main, [data-demo^=grid] aside')).toHaveCount(0);
  });
});

test.describe('App bar and facts', () => {
  test('AppBar: a header band; start, the linked name taking the space, actions at the end', async ({ page }) => {
    const bar = page.locator('[data-demo=app-bar] header.app-bar');
    const b = await box(bar);
    const start = await box(bar.getByRole('button', { name: 'Open the menu' }));
    const name = bar.getByRole('link', { name: 'Cookbook' });
    await expect(name).toHaveAttribute('href', '/layout');
    const end = await box(bar.getByRole('button', { name: 'Sign in' }));
    expect(Math.round(start.x - b.x)).toBe(24);
    expect(Math.round(b.x + b.width - (end.x + end.width))).toBe(24);
    expect((await box(name)).x).toBeGreaterThan(start.x + start.width);
    expect(await css(bar, 'position')).toBe('static');
    await bar.getByRole('button', { name: 'Open the menu' }).click();
    await expect(page.locator('[data-echo="opened"]')).toHaveText('1');
  });

  test('Facts: a <dl>, labels beside values; on a phone, above them', async ({ page }) => {
    const facts = page.locator('[data-demo=facts] dl.facts');
    const label = facts.locator('dt', { hasText: 'Time' });
    const value = facts.locator('dd', { hasText: '30 minutes' });
    expect(Math.abs((await box(label)).y - (await box(value)).y)).toBeLessThan(2);
    expect((await box(value)).x).toBeGreaterThan((await box(label)).x + (await box(label)).width);
    await page.setViewportSize({ width: 390, height: 844 });
    expect((await box(value)).y).toBeGreaterThan((await box(label)).y + (await box(label)).height - 1);
  });

  test('a plain link takes the theme\'s link colour; a component\'s own link keeps its own', async ({ page }) => {
    const plain = page.getByRole('link', { name: 'back to the top' });
    const primaryText = await page.evaluate(() => {
      const probe = document.createElement('span');
      probe.style.color = getComputedStyle(document.documentElement).getPropertyValue('--primary-text');
      document.body.append(probe);
      const c = getComputedStyle(probe).color;
      probe.remove();
      return c;
    });
    expect(await css(plain, 'color')).toBe(primaryText);
    expect(await css(plain, 'text-decoration-line')).toBe('none');
    const cardLink = page.locator('.card__link').first();
    expect(await css(cardLink, 'color')).not.toBe(primaryText);
  });
});
