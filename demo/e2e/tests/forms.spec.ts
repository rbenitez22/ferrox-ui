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

test.describe('Checkbox', () => {
  test('the words are its label: clicking them ticks it', async ({ page }) => {
    const box = page.getByRole('checkbox', { name: 'Repeat every month' });
    await expect(box).not.toBeChecked();
    await page.getByText('Repeat every month').click();
    await expect(box).toBeChecked();
    await expect(echo(page, 'recurring')).toHaveText('true');
  });

  test('Space ticks it from the keyboard', async ({ page }) => {
    const box = page.getByRole('checkbox', { name: 'Repeat every month' });
    await box.focus();
    await page.keyboard.press('Space');
    await expect(box).toBeChecked();
  });

  test('hint, then error, as in a Field', async ({ page }) => {
    const recurring = page.getByRole('checkbox', { name: 'Repeat every month' });
    expect(await description(recurring)).toBe('Until you stop it');
    await expect(recurring).not.toHaveAttribute('aria-invalid', 'true');

    const terms = page.getByRole('checkbox', { name: 'I accept the terms' });
    expect(await description(terms)).toBe('Tick this to continue.');
    await expect(terms).toHaveAttribute('aria-invalid', 'true');
    await terms.check();
    await expect(terms).not.toHaveAttribute('aria-invalid', 'true');
    expect(await description(terms)).toBeNull();
  });
});

test.describe('Switch', () => {
  test('is a switch named by its words; click and Space flip it', async ({ page }) => {
    const sw = page.getByRole('switch', { name: 'Show archived accounts' });
    await expect(sw).toHaveAttribute('aria-checked', 'false');
    await sw.click();
    await expect(sw).toHaveAttribute('aria-checked', 'true');
    await expect(echo(page, 'archived')).toHaveText('true');
    await page.keyboard.press('Space');
    await expect(sw).toHaveAttribute('aria-checked', 'false');
  });

  test('the knob slides across when on', async ({ page }) => {
    const sw = page.getByRole('switch', { name: 'Show archived accounts' });
    const knob = sw.locator('.switch__knob');
    const before = (await knob.boundingBox())!.x;
    await sw.click();
    await expect.poll(async () => (await knob.boundingBox())!.x - before).toBeGreaterThan(10);
  });
});

test.describe('Select', () => {
  const currency = (page: Page) => page.getByLabel('Currency');
  const account = (page: Page) => page.locator('select[name=account]');

  test('starts on its placeholder, which cannot be chosen', async ({ page }) => {
    await expect(currency(page)).toHaveValue('');
    const placeholder = currency(page).locator('option').first();
    await expect(placeholder).toHaveText('Choose a currency');
    await expect(placeholder).toHaveAttribute('disabled', '');
  });

  test('choosing sets the value; setting the value chooses', async ({ page }) => {
    await currency(page).selectOption('usd');
    await expect(echo(page, 'currency')).toHaveText('usd');
    await page.getByRole('button', { name: 'Choose pounds from code' }).click();
    await expect(currency(page)).toHaveValue('gbp');
  });

  test('loading: disabled, busy, and says so', async ({ page }) => {
    await page.getByRole('switch', { name: 'Accounts loading' }).click();
    await expect(account(page)).toBeDisabled();
    await expect(account(page)).toHaveAttribute('aria-busy', 'true');
    await expect(account(page).locator('option').first()).toHaveText('Loading…');
    await page.getByRole('switch', { name: 'Accounts loading' }).click();
    await expect(account(page)).toBeEnabled();
  });

  test('refresh is a button; a spinner holds its place while refreshing', async ({ page }) => {
    const refresh = page.locator('.select').getByRole('button', { name: 'Refresh' });
    await refresh.click();
    await expect(refresh).toBeDisabled();
    await expect(refresh.getByRole('status')).toHaveText('Refreshing…');
    await expect(account(page).locator('option', { hasText: 'Brokerage' })).toHaveCount(1);
    await expect(refresh).toBeEnabled();
    await expect(refresh.locator('svg.icon')).toHaveAttribute('data-icon', 'refresh');
  });

  test('the same element keeps focus and its choice when options change', async ({ page }) => {
    await account(page).selectOption('savings');
    await page.locator('.select').getByRole('button', { name: 'Refresh' }).click();
    await account(page).evaluate((el) => {
      el.setAttribute('data-mark', 'kept');
      (el as HTMLSelectElement).focus();
    });
    await expect(account(page).locator('option', { hasText: 'Brokerage' })).toHaveCount(1);
    expect(await page.evaluate(() => document.activeElement?.getAttribute('data-mark'))).toBe('kept');
    await expect(account(page)).toHaveValue('savings');
  });

  test('draws its own caret', async ({ page }) => {
    const image = await currency(page).evaluate((el) => getComputedStyle(el).backgroundImage);
    expect(image).toContain('linear-gradient');
  });
});

test.describe('Combobox', () => {
  const input = (page: Page) => page.locator('input[name=account-combo]');
  const listbox = async (page: Page) => page.locator(`[id="${await input(page).getAttribute('aria-controls')}"]`);
  const options = async (page: Page) => (await listbox(page)).getByRole('option');

  test('is a combobox whose list opens as you type, filtered', async ({ page }) => {
    await expect(input(page)).toHaveAttribute('role', 'combobox');
    await expect(input(page)).toHaveAttribute('aria-expanded', 'false');
    await input(page).fill('util');
    await expect(input(page)).toHaveAttribute('aria-expanded', 'true');
    await expect(await options(page)).toHaveText(['Expenses:Utilities:Power', 'Expenses:Utilities:Water', 'Expenses:Utilities:Internet']);
    expect(await (await listbox(page)).evaluate((el) => el.matches(':popover-open'))).toBe(true);
  });

  test('arrows move the active option; Enter chooses it', async ({ page }) => {
    await input(page).fill('util');
    await input(page).press('ArrowDown');
    await input(page).press('ArrowDown');
    await input(page).press('ArrowUp');
    const first = (await options(page)).first();
    await expect(first).toHaveAttribute('aria-selected', 'true');
    await expect(input(page)).toHaveAttribute('aria-activedescendant', (await first.getAttribute('id'))!);
    await input(page).press('Enter');
    await expect(input(page)).toHaveValue('Expenses:Utilities:Power');
    await expect(echo(page, 'combo-chosen')).toHaveText('expenses-utilities-power');
    await expect(input(page)).toHaveAttribute('aria-expanded', 'false');
  });

  test('a click chooses; Tab chooses the active one and moves on', async ({ page }) => {
    await input(page).fill('groc');
    await (await options(page)).first().click();
    await expect(echo(page, 'combo-chosen')).toHaveText('expenses-groceries');
    await expect(input(page)).toBeFocused();

    await input(page).fill('rent');
    await input(page).press('ArrowDown');
    await input(page).press('Tab');
    await expect(echo(page, 'combo-chosen')).toHaveText('expenses-rent');
    await expect(input(page)).not.toBeFocused();
  });

  test('Esc closes the list, then clears the text', async ({ page }) => {
    await input(page).fill('sav');
    await input(page).press('Escape');
    await expect(input(page)).toHaveAttribute('aria-expanded', 'false');
    await expect(input(page)).toHaveValue('sav');
    await input(page).press('Escape');
    await expect(input(page)).toHaveValue('');
    await expect(echo(page, 'combo-chosen')).toHaveText('(none)');
  });

  test('leaving with text that names nothing reports no choice', async ({ page }) => {
    await input(page).fill('nothing like it');
    await expect((await options(page)).first()).toHaveText('No matches');
    await page.getByLabel('Name').focus();
    await expect(echo(page, 'combo-chosen')).toHaveText('(none)');
    await expect(echo(page, 'combo-reports')).toHaveText('1');
  });

  test('the chevron shows everything; clear empties it; both stay out of the Tab order', async ({ page }) => {
    const field = page.locator('.combobox').filter({ has: input(page) });
    const chevron = field.getByRole('button', { name: 'Show all' });
    await expect(chevron).toHaveAttribute('tabindex', '-1');
    await chevron.click();
    await expect(await options(page)).toHaveCount(24);
    await expect(input(page)).toBeFocused();

    await (await options(page)).nth(1).click();
    await field.getByRole('button', { name: 'Clear' }).click();
    await expect(input(page)).toHaveValue('');
    await expect(echo(page, 'combo-chosen')).toHaveText('(none)');
  });

  test('refresh shows a spinner, then the new items', async ({ page }) => {
    const refresh = page.locator('.combobox').filter({ has: input(page) }).getByRole('button', { name: 'Refresh' });
    await refresh.click();
    await expect(refresh).toBeDisabled();
    await expect(refresh).toBeEnabled();
    await input(page).fill('pets');
    await expect(await options(page)).toHaveText(['Expenses:Pets']);
  });

  test('inside a dialog: not clipped, Enter chooses without saving, Esc keeps the dialog', async ({ page }) => {
    await page.getByRole('button', { name: 'Combobox in a dialog…' }).click();
    const dialog = page.getByRole('dialog', { name: 'Move a split' });
    const field = dialog.locator('input[name=dialog-combo]');
    await expect(field).toBeFocused();
    await field.fill('e');
    const box = dialog.locator(`[id="${await field.getAttribute('aria-controls')}"]`);
    const last = box.getByRole('option').last();
    await last.scrollIntoViewIfNeeded();
    const b = (await last.boundingBox())!;
    const dialogBox = (await dialog.boundingBox())!;
    expect(b.y + b.height).toBeGreaterThan(dialogBox.y + dialogBox.height - 80);
    const hit = await page.evaluate(([x, y]) => document.elementFromPoint(x, y)?.textContent, [b.x + b.width / 2, b.y + b.height / 2]);
    expect(hit).toBe(await last.textContent());

    await field.press('ArrowDown');
    await field.press('Enter');
    await expect(echo(page, 'dialog-saves')).toHaveText('0');
    await expect(dialog).toBeVisible();
    await expect(echo(page, 'dialog-chosen')).not.toHaveText('(none)');

    await field.fill('as');
    await field.press('Escape');
    await expect(dialog).toBeVisible();
    await expect(field).toHaveAttribute('aria-expanded', 'false');
  });
});
