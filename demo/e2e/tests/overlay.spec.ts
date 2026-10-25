import type { Locator, Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// Dialog on the demo's /overlay page: a real modal (showModal), focus in /
// kept / returned, every way out through on_close once, Enter submits only
// from the right places, the fixed footer order, fresh content each opening,
// the page scroll lock (also nested), and the phone layout.

const trigger = (page: Page, name: string) => page.getByRole('button', { name, exact: true });
const dialog = (page: Page, title: string) => page.getByRole('dialog', { name: title });
const log = (page: Page) => page.locator('.demo-log li');

async function open(page: Page, button: string, title: string): Promise<Locator> {
  await trigger(page, button).click();
  const d = dialog(page, title);
  await expect(d).toBeVisible();
  return d;
}

/** What has focus: its accessible label, name attribute or text. */
const focused = (page: Page) =>
  page.evaluate(() => {
    const el = document.activeElement as HTMLElement | null;
    return el?.getAttribute('aria-label') ?? el?.getAttribute('name') ?? el?.textContent?.trim() ?? null;
  });

/** Both roots: a phone browser may scroll either one. */
const bodyLocked = (page: Page) =>
  page.evaluate(() => document.body.classList.contains('dialog-open') && document.documentElement.classList.contains('dialog-open'));

test.beforeEach(async ({ page }) => {
  await openApp(page, '/overlay');
});

test.describe('opening', () => {
  test('is a real modal, labelled by its title', async ({ page }) => {
    const d = await open(page, 'Edit account…', 'Edit account');
    expect(await d.evaluate((el) => el.matches(':modal'))).toBe(true);
  });

  test('the title bar: title on the left, the × at the right edge', async ({ page }) => {
    const d = await open(page, 'Edit account…', 'Edit account');
    const bar = (await d.locator('.dialog__title-bar').boundingBox())!;
    const close = (await d.getByRole('button', { name: 'Close' }).boundingBox())!;
    const title = (await d.locator('.dialog__title').boundingBox())!;
    // The bar's right padding is --space-lg (24px).
    expect(bar.x + bar.width - (close.x + close.width)).toBeCloseTo(24, 0);
    expect(title.x).toBeLessThan(close.x);
  });

  test('focus goes to the autofocus field, else to the close button', async ({ page }) => {
    await open(page, 'Edit account…', 'Edit account');
    await expect.poll(() => focused(page)).toBe('name');
    await page.keyboard.press('Escape');

    await open(page, 'Show splits…', 'Splits');
    await expect.poll(() => focused(page)).toBe('Close');
  });

  test('Tab never reaches the page behind', async ({ page }) => {
    const d = await open(page, 'Edit account…', 'Edit account');
    for (let i = 0; i < 8; i++) {
      await page.keyboard.press('Tab');
      const inside = await d.evaluate((el) => {
        const active = document.activeElement;
        return active === document.body || active === null || el.contains(active);
      });
      expect(inside, `after ${i + 1} Tab presses`).toBe(true);
    }
  });

  test('each opening starts fresh', async ({ page }) => {
    const d = await open(page, 'Edit account…', 'Edit account');
    await d.locator('input[name=name]').fill('Groceries');
    await page.keyboard.press('Escape');
    await open(page, 'Edit account…', 'Edit account');
    await expect(d.locator('input[name=name]')).toHaveValue('');
  });
});

test.describe('closing', () => {
  for (const [how, act] of [
    ['Esc', (page: Page) => page.keyboard.press('Escape')],
    ['the ×', (page: Page) => dialog(page, 'Edit account').getByRole('button', { name: 'Close' }).click()],
    ['Cancel', (page: Page) => dialog(page, 'Edit account').getByRole('button', { name: 'Cancel' }).click()],
  ] as const) {
    test(`${how} calls on_close once, closes, and gives focus back`, async ({ page }) => {
      await open(page, 'Edit account…', 'Edit account');
      await act(page);
      await expect(dialog(page, 'Edit account')).toBeHidden();
      await expect(log(page)).toHaveText(['form: closed']);
      await expect.poll(() => focused(page)).toBe('Edit account…');
    });
  }

  test('a click on the backdrop does not close a dialog', async ({ page }) => {
    await open(page, 'Edit account…', 'Edit account');
    await page.mouse.click(5, 5);
    await expect(dialog(page, 'Edit account')).toBeVisible();
    await expect(log(page)).toHaveCount(0);
  });
});

test.describe('Enter', () => {
  test('from a field submits once, through saving, then closes', async ({ page }) => {
    const d = await open(page, 'Edit account…', 'Edit account');
    await d.locator('input[name=name]').press('Enter');
    const save = d.getByRole('button', { name: 'Save', exact: true });
    await expect(save).toHaveAttribute('aria-disabled', 'true');
    await d.locator('input[name=name]').press('Enter'); // ignored while saving
    await expect(d).toBeHidden();
    await expect(log(page)).toHaveText(['form: saved']);
  });

  test('in a textarea is a newline', async ({ page }) => {
    const d = await open(page, 'Edit account…', 'Edit account');
    const notes = d.locator('textarea[name=notes]');
    await notes.press('Enter');
    await expect(notes).toHaveValue('\n');
    await expect(d.getByRole('button', { name: 'Save', exact: true })).not.toHaveAttribute('aria-disabled', 'true');
    await expect(d).toBeVisible();
  });

  test('on a focused button presses that button', async ({ page }) => {
    const d = await open(page, 'Edit account…', 'Edit account');
    await d.getByRole('button', { name: 'Save as draft' }).focus();
    await page.keyboard.press('Enter');
    await expect(d).toBeHidden();
    await expect(log(page)).toHaveText(['form: draft']);
  });
});

test.describe('footer', () => {
  test('order is fixed: cancel, alternate, affirm', async ({ page }) => {
    const d = await open(page, 'Edit account…', 'Edit account');
    const buttons = d.locator('.dialog__footer button');
    await expect(buttons).toHaveText(['Cancel', 'Save as draft', 'Save']);
    await expect(buttons.nth(0)).toHaveClass(/\bbtn--cancel\b/);
    await expect(buttons.nth(1)).toHaveClass(/\bbtn--secondary\b/);
    await expect(buttons.nth(2)).toHaveClass(/\bbtn--primary\b/);
  });

  test('no_cancel leaves only the affirm button; × and Esc still close', async ({ page }) => {
    const d = await open(page, 'Notice…', 'Saved');
    await expect(d.locator('.dialog__footer button')).toHaveText(['Got it']);
    await page.keyboard.press('Escape');
    await expect(d).toBeHidden();
    await expect(log(page)).toHaveText(['notice: closed']);
  });
});

test.describe('scroll lock', () => {
  test('the page is locked while a dialog is open', async ({ page }) => {
    expect(await bodyLocked(page)).toBe(false);
    await open(page, 'Edit account…', 'Edit account');
    expect(await bodyLocked(page)).toBe(true);
    expect(await page.evaluate(() => getComputedStyle(document.body).overflow)).toBe('hidden');
    expect(await page.evaluate(() => getComputedStyle(document.documentElement).overflow)).toBe('hidden');
    await page.keyboard.press('Escape');
    await expect.poll(() => bodyLocked(page)).toBe(false);
  });

  test('nested: stays locked until the last dialog closes', async ({ page }) => {
    const outer = await open(page, 'Nested…', 'Outer');
    await outer.getByRole('button', { name: 'Open the inner dialog' }).click();
    const inner = dialog(page, 'Inner');
    await expect(inner).toBeVisible();

    await inner.getByRole('button', { name: 'Done' }).click();
    await expect(inner).toBeHidden();
    await expect(outer).toBeVisible();
    expect(await bodyLocked(page)).toBe(true);

    await page.keyboard.press('Escape');
    await expect(outer).toBeHidden();
    await expect.poll(() => bodyLocked(page)).toBe(false);
    await expect(log(page)).toHaveText(['inner: done', 'outer: closed']);
  });
});

test.describe('size', () => {
  test('32rem by default, 52rem wide', async ({ page }) => {
    const md = await open(page, 'Edit account…', 'Edit account');
    expect((await md.boundingBox())!.width).toBeCloseTo(512, 0);
    await page.keyboard.press('Escape');
    const wide = await open(page, 'Show splits…', 'Splits');
    expect((await wide.boundingBox())!.width).toBeCloseTo(832, 0);
  });

  test('phone: full width inside an 8px margin, a 44px close button', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 700 });
    const d = await open(page, 'Show splits…', 'Splits');
    const box = (await d.boundingBox())!;
    expect(box.width).toBeCloseTo(375 - 16, 0);
    expect(box.x).toBeCloseTo(8, 0);
    const close = (await d.getByRole('button', { name: 'Close' }).boundingBox())!;
    expect(close.width).toBeCloseTo(44, 0);
  });
});

test.describe('ConfirmDialog', () => {
  const confirm = (page: Page) => page.getByRole('alertdialog', { name: 'Delete account?' });

  test('is an alertdialog whose last button is a danger verb', async ({ page }) => {
    await trigger(page, 'Delete account…').click();
    const d = confirm(page);
    await expect(d).toBeVisible();
    expect(await d.evaluate((el) => el.matches(':modal'))).toBe(true);
    await expect(d).toContainText('“Groceries” and its 214 transactions');
    const buttons = d.locator('.dialog__footer button');
    await expect(buttons).toHaveText(['Cancel', 'Delete']);
    await expect(buttons.last()).toHaveClass(/\bbtn--danger\b/);
  });

  test('opens with focus on the close button, not on Delete', async ({ page }) => {
    await trigger(page, 'Delete account…').click();
    await expect(confirm(page)).toBeVisible();
    await expect.poll(() => focused(page)).toBe('Close');
  });

  for (const [how, act] of [
    ['Cancel', (page: Page) => confirm(page).getByRole('button', { name: 'Cancel' }).click()],
    ['Esc', (page: Page) => page.keyboard.press('Escape')],
  ] as const) {
    test(`${how} calls on_cancel once and runs nothing`, async ({ page }) => {
      await trigger(page, 'Delete account…').click();
      await expect(confirm(page)).toBeVisible();
      await act(page);
      await expect(confirm(page)).toBeHidden();
      await expect(log(page)).toHaveText(['confirm: cancelled']);
    });
  }

  test('Delete runs once: saving refuses a second click, then it closes', async ({ page }) => {
    await trigger(page, 'Delete account…').click();
    const del = confirm(page).getByRole('button', { name: 'Delete' });
    await del.click();
    await expect(del).toHaveAttribute('aria-disabled', 'true');
    await del.click({ force: true });
    await expect(confirm(page)).toBeHidden();
    await expect(log(page)).toHaveText(['confirm: confirmed', 'confirm: deleted']);
  });

  test('reopening after a delete starts ready, not saving', async ({ page }) => {
    await trigger(page, 'Delete account…').click();
    await confirm(page).getByRole('button', { name: 'Delete' }).click();
    await expect(confirm(page)).toBeHidden();
    await trigger(page, 'Delete account…').click();
    await expect(confirm(page).getByRole('button', { name: 'Delete' })).not.toHaveAttribute('aria-disabled', 'true');
  });
});

test.describe('Sheet', () => {
  const sheet = (page: Page, title: string) => page.getByRole('dialog', { name: title });

  async function openSheet(page: Page, title: string): Promise<Locator> {
    await trigger(page, `${title}…`).click();
    const s = sheet(page, title);
    await expect(s).toBeVisible();
    // Let the slide-in finish before measuring.
    await expect.poll(() => s.evaluate((el) => getComputedStyle(el).transform)).toBe('none');
    return s;
  }

  test('each edge: pinned there, at its size', async ({ page }) => {
    const vp = page.viewportSize()!;

    const left = (await (await openSheet(page, 'Menu')).boundingBox())!;
    expect(left).toEqual({ x: 0, y: 0, width: 320, height: vp.height });
    await page.keyboard.press('Escape');

    const right = (await (await openSheet(page, 'Details')).boundingBox())!;
    expect(right.x + right.width).toBeCloseTo(vp.width, 0);
    expect(right.height).toBe(vp.height);
    await page.keyboard.press('Escape');

    const bottom = (await (await openSheet(page, 'Actions')).boundingBox())!;
    expect(bottom.y + bottom.height).toBeCloseTo(vp.height, 0);
    expect(bottom.width).toBe(vp.width);
    expect(bottom.height).toBeLessThanOrEqual(vp.height * 0.85);
  });

  test('is a real modal: focus inside, page locked, focus back on close', async ({ page }) => {
    const s = await openSheet(page, 'Menu');
    expect(await s.evaluate((el) => el.matches(':modal'))).toBe(true);
    await expect.poll(() => focused(page)).toBe('Close');
    expect(await bodyLocked(page)).toBe(true);
    await page.keyboard.press('Escape');
    await expect(s).toBeHidden();
    await expect.poll(() => bodyLocked(page)).toBe(false);
    await expect.poll(() => focused(page)).toBe('Menu…');
  });

  for (const [how, act] of [
    ['a backdrop click', (page: Page) => page.mouse.click(page.viewportSize()!.width - 10, 300)],
    ['Esc', (page: Page) => page.keyboard.press('Escape')],
    ['the ×', (page: Page) => sheet(page, 'Menu').getByRole('button', { name: 'Close' }).click()],
  ] as const) {
    test(`${how} calls on_close once`, async ({ page }) => {
      await openSheet(page, 'Menu');
      await act(page);
      await expect(sheet(page, 'Menu')).toBeHidden();
      await expect(log(page)).toHaveText(['menu: closed']);
    });
  }

  test('clicks inside do not close it; a press dragged out to the backdrop does not either', async ({ page }) => {
    const s = await openSheet(page, 'Menu');
    await s.locator('.sheet__body').click({ position: { x: 20, y: 200 } });
    const title = (await s.locator('.sheet__title').boundingBox())!;
    await page.mouse.move(title.x + 5, title.y + 5);
    await page.mouse.down();
    await page.mouse.move(page.viewportSize()!.width - 10, 300);
    await page.mouse.up();
    await expect(s).toBeVisible();
    await expect(log(page)).toHaveCount(0);
  });

  test('a choice inside closes it through the app', async ({ page }) => {
    const s = await openSheet(page, 'Menu');
    await s.getByRole('button', { name: 'Accounts' }).click();
    await expect(s).toBeHidden();
    await expect(log(page)).toHaveText(['menu: chose']);
  });

  test('slides in over --react-time', async ({ page }) => {
    const s = await openSheet(page, 'Menu');
    const t = await s.evaluate((el) => {
      const c = getComputedStyle(el);
      return { property: c.transitionProperty, duration: c.transitionDuration };
    });
    expect(t).toEqual({ property: 'transform', duration: '0.2s' });
  });
});

test.describe('Sheet, reduced motion', () => {
  test.use({ reducedMotion: 'reduce' });

  test('appears without sliding', async ({ page }) => {
    await trigger(page, 'Menu…').click();
    const s = page.getByRole('dialog', { name: 'Menu' });
    await expect(s).toBeVisible();
    expect(await s.evaluate((el) => getComputedStyle(el).transitionProperty)).toBe('none');
    expect(await s.evaluate((el) => getComputedStyle(el).transform)).toBe('none');
  });
});
