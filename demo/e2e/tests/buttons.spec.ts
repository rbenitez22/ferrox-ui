import type { Locator, Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// Button, IconButton and Toolbar on the demo's /buttons page.

/** The demo section titled `title` (demo/src/section.rs). */
function section(page: Page, title: string): Locator {
  return page.locator('section', { has: page.getByRole('heading', { name: title, exact: true }) });
}

/** A row of the section: 0 = filled, 1 = quiet (Families) or disabled/saving (States). */
function row(page: Page, title: string, index: number): Locator {
  return section(page, title).locator('.demo-row').nth(index);
}

async function box(locator: Locator) {
  const b = await locator.boundingBox();
  if (!b) throw new Error('element has no box');
  return b;
}

/** Long enough for a hover or press transition (--react-time is 200ms) to finish. */
const SETTLE_MS = 300;

test.beforeEach(async ({ page }) => {
  await openApp(page, '/buttons');
});

test.describe('Button', () => {
  test('each family and the quiet axis render their classes', async ({ page }) => {
    for (const family of ['Primary', 'Secondary', 'Cancel', 'Danger']) {
      const filled = row(page, 'Families', 0).getByRole('button', { name: family });
      const quiet = row(page, 'Families', 1).getByRole('button', { name: family });
      await expect(filled).toHaveClass(new RegExp(`\\bbtn--${family.toLowerCase()}\\b`));
      await expect(filled).not.toHaveClass(/btn--quiet/);
      await expect(quiet).toHaveClass(/\bbtn--quiet\b/);
    }
  });

  test('sizes render their classes', async ({ page }) => {
    const sizes = section(page, 'Sizes');
    await expect(sizes.getByRole('button', { name: 'Small' })).toHaveClass(/\bbtn--sm\b/);
    await expect(sizes.getByRole('button', { name: 'Medium' })).not.toHaveClass(/btn--(sm|lg)/);
    await expect(sizes.getByRole('button', { name: 'Large' })).toHaveClass(/\bbtn--lg\b/);
  });

  test('disabled buttons are disabled', async ({ page }) => {
    for (const button of await row(page, 'States', 0).getByRole('button').all()) {
      await expect(button).toBeDisabled();
    }
  });

  test('saving: sweeps, stays focusable, refuses clicks, then recovers', async ({ page }) => {
    const save = section(page, 'States').getByRole('button', { name: 'Save' });
    const clicks = section(page, 'States').getByText(/^clicks: \d+$/);

    // From the keyboard, so focus is on the button the same way in every browser.
    await save.focus();
    await page.keyboard.press('Enter');
    await expect(clicks).toHaveText('clicks: 1');
    await expect(save).toHaveClass(/\bis-saving\b/);
    await expect(save).toHaveAttribute('aria-disabled', 'true');
    // Not `disabled`: a disabled button would drop keyboard focus mid-request.
    // (Not `toBeEnabled()`: Playwright counts aria-disabled as disabled.)
    await expect(save).not.toHaveAttribute('disabled');
    await expect(save).toBeFocused();

    // Neither the keyboard, a pointer click, nor a synthetic click gets through.
    await page.keyboard.press('Enter');
    await save.click({ force: true });
    await save.dispatchEvent('click');
    await expect(clicks).toHaveText('clicks: 1');

    // The demo saves for 2s.
    await expect(save).not.toHaveClass(/is-saving/, { timeout: 5_000 });
    await expect(save).not.toHaveAttribute('aria-disabled');
    await save.click();
    await expect(clicks).toHaveText('clicks: 2');
  });
});

test.describe('no layout shift', () => {
  // A button reacts with transform and shadow only. Its neighbours must not
  // move. The old `.button:hover` bolded the label, which widened the button
  // and pushed the next one along.

  test('hovering or pressing a button does not move its neighbour', async ({ page }) => {
    const primary = row(page, 'Families', 0).getByRole('button', { name: 'Primary' });
    const neighbour = row(page, 'Families', 0).getByRole('button', { name: 'Secondary' });
    const before = await box(neighbour);

    await primary.hover();
    await page.waitForTimeout(SETTLE_MS);
    expect(await box(neighbour)).toEqual(before);

    await page.mouse.down();
    await page.waitForTimeout(SETTLE_MS);
    expect(await box(neighbour)).toEqual(before);
    await page.mouse.up();
  });

  test('hovering an icon button does not move its neighbour', async ({ page }) => {
    const icons = section(page, 'Icon buttons');
    const neighbour = icons.getByRole('button', { name: 'Camera (toggle)' });
    const before = await box(neighbour);

    await icons.getByRole('button', { name: 'Search' }).hover();
    await page.waitForTimeout(SETTLE_MS);
    expect(await box(neighbour)).toEqual(before);
  });

  test('a saving button does not move its neighbour', async ({ page }) => {
    // The saving border animates its colour only; its width stays 1px.
    const states = section(page, 'States');
    const neighbour = states.getByRole('button', { name: 'Neighbour' });
    const before = await box(neighbour);

    await states.getByRole('button', { name: 'Save' }).click();
    await expect(states.getByRole('button', { name: 'Save' })).toHaveClass(/is-saving/);
    expect(await box(neighbour)).toEqual(before);
  });
});

test.describe('IconButton', () => {
  test('every icon button has an accessible name', async ({ page }) => {
    for (const button of await section(page, 'Icon buttons').getByRole('button').all()) {
      await expect(button).toHaveAccessibleName(/\S/);
    }
  });

  test('a toggle flips aria-pressed', async ({ page }) => {
    const camera = section(page, 'Icon buttons').getByRole('button', { name: 'Camera (toggle)' });
    await expect(camera).toHaveAttribute('aria-pressed', 'true');
    await camera.click();
    await expect(camera).toHaveAttribute('aria-pressed', 'false');
    await camera.click();
    await expect(camera).toHaveAttribute('aria-pressed', 'true');
  });

  test('danger and disabled variants', async ({ page }) => {
    const icons = section(page, 'Icon buttons');
    await expect(icons.getByRole('button', { name: 'Remove' })).toHaveClass(/\bicon-btn--danger\b/);
    await expect(icons.getByRole('button', { name: 'Disabled' })).toBeDisabled();
  });
});

test.describe('Toolbar', () => {
  test('buttons sit right of the title, at the right edge', async ({ page }) => {
    const toolbar = section(page, 'Toolbars').locator('.toolbar').first();
    const title = await box(toolbar.locator('.toolbar__title'));
    const buttons = toolbar.getByRole('button');
    const first = await box(buttons.first());
    const last = await box(buttons.last());
    const bar = await box(toolbar);

    expect(first.x).toBeGreaterThanOrEqual(title.x + title.width);
    expect(bar.x + bar.width - (last.x + last.width)).toBeLessThan(2);
  });

  test('groups in a compact toolbar are split by a hairline', async ({ page }) => {
    const compact = section(page, 'Toolbars').locator('.toolbar--compact').first();
    const groups = compact.locator('.toolbar__group');
    await expect(groups).toHaveCount(2);
    const border = await groups.nth(1).evaluate((g) => getComputedStyle(g).borderLeftWidth);
    expect(border).toBe('1px');
  });
});
