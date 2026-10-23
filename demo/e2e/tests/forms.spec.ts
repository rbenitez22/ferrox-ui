import type { Locator, Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// Field, Input and Textarea on the demo's /forms page: the label names the
// control, one message describes it (hint or error, never both), an error
// marks the control invalid, ids are unique, values bind both ways.

const echo = (page: Page, name: string) => page.locator(`[data-echo="${name}"]`);

/** The text of whatever `aria-describedby` points at, or null. */
const description = (control: Locator) =>
  control.evaluate((el) => {
    const id = el.getAttribute('aria-describedby');
    return id ? document.getElementById(id)?.textContent ?? '(missing)' : null;
  });

/** `var(--name)` as the page computes it for `property`. */
function resolved(page: Page, property: string, value: string): Promise<string> {
  return page.evaluate(([p, v]) => {
    const probe = document.createElement('div');
    probe.style.setProperty(p, v);
    document.body.append(probe);
    const out = getComputedStyle(probe).getPropertyValue(p);
    probe.remove();
    return out;
  }, [property, value]);
}

test.beforeEach(async ({ page }) => {
  await openApp(page, '/forms');
});

test.describe('Field', () => {
  test('the label names its control', async ({ page }) => {
    await expect(page.getByLabel('Name')).toHaveAttribute('name', 'name');
    await expect(page.getByLabel('Email')).toHaveAttribute('name', 'email');
    await expect(page.getByLabel('Notes')).toHaveAttribute('name', 'notes');
  });

  test('the hint describes the control while it is fine', async ({ page }) => {
    const email = page.getByLabel('Email');
    expect(await description(email)).toBe('Where receipts go');
    await expect(email).not.toHaveAttribute('aria-invalid', 'true');
    await expect(email).not.toHaveClass(/\bis-invalid\b/);
  });

  test('an error replaces the hint and marks the control invalid', async ({ page }) => {
    const email = page.getByLabel('Email');
    await email.fill('nobody');
    expect(await description(email)).toBe('An email address has an @ in it.');
    await expect(page.getByText('Where receipts go')).toHaveCount(0);
    await expect(email).toHaveAttribute('aria-invalid', 'true');
    await expect(email).toHaveClass(/\bis-invalid\b/);
    const danger = await resolved(page, 'color', 'var(--danger-base)');
    // After the border's colour transition.
    await expect.poll(() => email.evaluate((el) => getComputedStyle(el).borderTopColor)).toBe(danger);

    await email.fill('nobody@example.com');
    expect(await description(email)).toBe('Where receipts go');
    await expect(email).not.toHaveAttribute('aria-invalid', 'true');
  });

  test('a field with no hint or error has no description', async ({ page }) => {
    expect(await description(page.getByLabel('Notes'))).toBeNull();
  });

  test('every control has its own id, inside a field or not', async ({ page }) => {
    const ids = await page.locator('.form-input, .form-textarea').evaluateAll((els) => els.map((e) => e.id));
    expect(ids).toHaveLength(4);
    expect(ids.every((id) => id.length > 0)).toBe(true);
    expect(new Set(ids).size).toBe(ids.length);
  });
});

test.describe('Input and Textarea', () => {
  test('typing updates the value', async ({ page }) => {
    await page.getByLabel('Name').fill('Rent');
    await page.getByPlaceholder('Search (no field)').fill('rent');
    await page.getByLabel('Notes').fill('first\nsecond');
    await expect(echo(page, 'name')).toHaveText('Rent');
    await expect(echo(page, 'query')).toHaveText('rent');
    await expect(echo(page, 'notes')).toHaveText('first\nsecond');
  });

  test('setting the value updates the control', async ({ page }) => {
    await page.getByRole('button', { name: 'Fill from code' }).click();
    await expect(page.getByLabel('Name')).toHaveValue('Groceries');
    await expect(page.getByLabel('Notes')).toHaveValue('Line one\nLine two');
  });

  test('kinds and attributes pass through', async ({ page }) => {
    await expect(page.getByLabel('Email')).toHaveAttribute('type', 'email');
    await expect(page.getByLabel('Email')).toHaveAttribute('autocomplete', 'email');
    await expect(page.getByPlaceholder('Search (no field)')).toHaveAttribute('type', 'search');
    await expect(page.getByLabel('Notes')).toHaveAttribute('rows', '4');
  });

  test('on a phone, text is never below 16px (no zoom on focus)', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 700 });
    for (const control of [page.getByLabel('Name'), page.getByLabel('Notes')]) {
      const size = await control.evaluate((el) => parseFloat(getComputedStyle(el).fontSize));
      expect(size).toBeGreaterThanOrEqual(16);
    }
  });
});
