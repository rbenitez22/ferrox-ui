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
    expect(ids.length).toBeGreaterThanOrEqual(4);
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

test.describe('PasswordInput', () => {
  const password = (page: Page) => page.getByLabel('Password', { exact: true });
  const newPassword = (page: Page) => page.getByLabel('New password');
  const meter = (page: Page) => page.locator('.password-strength');

  test('hidden by default; the toggle is a button that shows and hides it', async ({ page }) => {
    const field = password(page);
    await field.fill('hunter2');
    await expect(field).toHaveAttribute('type', 'password');

    const toggle = page.getByRole('button', { name: 'Show password' }).first();
    await expect(toggle).toHaveAttribute('aria-pressed', 'false');
    await expect(toggle.locator('svg')).toHaveAttribute('data-icon', 'eye');

    await toggle.click();
    await expect(field).toHaveAttribute('type', 'text');
    await expect(toggle).toHaveAttribute('aria-pressed', 'true');
    await expect(toggle.locator('svg')).toHaveAttribute('data-icon', 'eye-off');
    await expect(field).toHaveValue('hunter2');

    await toggle.focus();
    await page.keyboard.press('Space');
    await expect(field).toHaveAttribute('type', 'password');
  });

  test('the toggle sits inside the field, and text never runs under it', async ({ page }) => {
    const field = (await password(page).boundingBox())!;
    const toggle = (await page.getByRole('button', { name: 'Show password' }).first().boundingBox())!;
    expect(toggle.x + toggle.width).toBeLessThanOrEqual(field.x + field.width);
    expect(toggle.y).toBeGreaterThanOrEqual(field.y);
    const padding = await password(page).evaluate((el) => parseFloat(getComputedStyle(el).paddingRight));
    expect(padding).toBeGreaterThanOrEqual(toggle.width);
  });

  test('autocomplete tells password managers which password it is', async ({ page }) => {
    await expect(password(page)).toHaveAttribute('autocomplete', 'current-password');
    await expect(newPassword(page)).toHaveAttribute('autocomplete', 'new-password');
  });

  test('no meter unless asked for, and none while empty', async ({ page }) => {
    await password(page).fill('anything at all');
    await expect(meter(page)).toHaveCount(0);
    await expect(newPassword(page)).toHaveValue('');
    await expect(meter(page)).toHaveCount(0);
  });

  for (const [typed, level, label] of [
    ['abc', 'very-weak', 'Very weak'],
    ['abcdefgh', 'weak', 'Weak'],
    ['Abcdefgh12', 'fair', 'Fair'],
    ['Abcdefgh12!x-Zq9', 'strong', 'Strong'],
  ] as const) {
    test(`"${typed}" reads ${label}`, async ({ page }) => {
      await newPassword(page).fill(typed);
      await expect(meter(page)).toHaveClass(new RegExp(`\\bpassword-strength--${level}\\b`));
      await expect(meter(page).locator('.password-strength__label')).toHaveText(label);
    });
  }

  test('the field is described by its hint and its meter', async ({ page }) => {
    const field = newPassword(page);
    await field.fill('abc');
    const described = await field.evaluate((el) =>
      (el.getAttribute('aria-describedby') ?? '').split(' ').map((id) => document.getElementById(id)?.textContent),
    );
    expect(described).toEqual(['At least 80 bits: long, and mixed', 'Very weak']);
  });

  test('the value binds both ways through the toggle', async ({ page }) => {
    await newPassword(page).fill('s3cret!');
    await expect(echo(page, 'new-password')).toHaveText('s3cret!');
  });
});
