import type { Locator, Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// Spinner and LoadingOverlay on the demo's /feedback page: status roles and
// labels, the reveal armed on --loading-delay, sizes, reduced motion, and the
// overlay covering the page for its two seconds.

/** The demo section titled `title` (demo/src/section.rs). */
function section(page: Page, title: string): Locator {
  return page.locator('section', { has: page.getByRole('heading', { name: title, exact: true }) });
}

/** A spinner's status wrapper, found by its screen-reader label. */
const status = (page: Page, label: string) => page.getByRole('status').filter({ hasText: label });

type Running = { name: string; duration: number; delay: number; currentTime: number };

/** The CSS animations running on an element, with their timing. */
function animations(ring: Locator): Promise<Running[]> {
  return ring.evaluate((el) =>
    el.getAnimations().map((a) => {
      const timing = a.effect!.getTiming();
      return {
        name: (a as CSSAnimation).animationName,
        duration: Number(timing.duration),
        delay: Number(timing.delay),
        currentTime: Number(a.currentTime),
      };
    }),
  );
}

const opacity = (ring: Locator) => ring.evaluate((el) => getComputedStyle(el).opacity);

async function showSpinners(page: Page) {
  await section(page, 'Spinners').getByRole('button', { name: 'Show spinners' }).click();
}

test.beforeEach(async ({ page }) => {
  await openApp(page, '/feedback');
});

test.describe('Spinner', () => {
  test('each kind is a status carrying its label, read out but not shown', async ({ page }) => {
    await showSpinners(page);
    for (const label of ['Loading accounts', 'Deleting the row', 'Looking up the rate']) {
      const spinner = status(page, label);
      await expect(spinner).toHaveCount(1);
      const text = spinner.locator('.sr-only');
      await expect(text).toHaveText(label);
      const box = await text.boundingBox();
      expect(box!.width * box!.height).toBeLessThanOrEqual(1);
    }
  });

  test('is invisible until --loading-delay, then appears', async ({ page }) => {
    await showSpinners(page);
    const ring = status(page, 'Loading accounts').locator('.spinner');

    // Read in one go, so the check holds however slow the machine is: while
    // the reveal has not started, the ring is transparent.
    const [early, appear] = await ring.evaluate((el) => {
      const a = el.getAnimations().find((x) => (x as CSSAnimation).animationName === 'ferrox-spinner-appear')!;
      return [
        { opacity: getComputedStyle(el).opacity, currentTime: Number(a.currentTime) },
        { delay: Number(a.effect!.getTiming().delay) },
      ];
    });
    expect(appear.delay).toBe(400);
    if (early.currentTime < 400) expect(early.opacity).toBe('0');

    await expect.poll(() => opacity(ring)).toBe('1');
  });

  test('sizes: region 28px, inline and field 1rem', async ({ page }) => {
    await showSpinners(page);
    for (const [label, size] of [['Loading accounts', 28], ['Deleting the row', 16], ['Looking up the rate', 16]] as const) {
      // Layout size, not the bounding box: the ring is rotating, and a
      // rotated square's bounding box is larger than the square.
      const layout = await status(page, label).locator('.spinner').evaluate((el) => {
        const e = el as HTMLElement;
        return { width: e.offsetWidth, height: e.offsetHeight };
      });
      expect(layout, label).toEqual({ width: size, height: size });
    }
  });

  test('spins at 0.8s', async ({ page }) => {
    await showSpinners(page);
    const spin = (await animations(status(page, 'Loading accounts').locator('.spinner'))).find((a) => a.name === 'ferrox-spin');
    expect(spin?.duration).toBe(800);
  });
});

test.describe('reduced motion', () => {
  test.use({ reducedMotion: 'reduce' });

  test('the spin slows to 2.4s but the spinner still appears', async ({ page }) => {
    await showSpinners(page);
    const ring = status(page, 'Loading accounts').locator('.spinner');
    const running = await animations(ring);
    expect(running.find((a) => a.name === 'ferrox-spin')?.duration).toBe(2400);
    expect(running.find((a) => a.name === 'ferrox-spinner-appear')?.delay).toBe(400);
    await expect.poll(() => opacity(ring)).toBe('1');
  });
});

test.describe('LoadingOverlay', () => {
  test('covers the page with its text, then goes', async ({ page }) => {
    await section(page, 'Loading overlay').getByRole('button', { name: 'Show the overlay for 2s' }).click();
    const overlay = page.locator('.spinner-overlay');
    await expect(overlay).toHaveAttribute('role', 'status');
    await expect(overlay).toHaveAttribute('aria-live', 'polite');
    await expect(overlay.locator('.spinner-overlay__text')).toHaveText('Signing in…');

    const box = await overlay.boundingBox();
    const viewport = page.viewportSize()!;
    expect(box).toEqual({ x: 0, y: 0, width: viewport.width, height: viewport.height });

    await expect.poll(() => opacity(overlay.locator('.spinner'))).toBe('1');
    await expect(overlay).toHaveCount(0, { timeout: 4000 });
  });
});

test.describe('Alert', () => {
  test.use({ colorScheme: 'light' });

  const SEVERITIES = [
    { severity: 'info', role: 'status', icon: 'info', text: 'Changes apply', token: 'info' },
    { severity: 'success', role: 'status', icon: 'success', text: 'reconciled', token: 'success' },
    { severity: 'warning', role: 'status', icon: 'warning', text: 'no transactions', token: 'warning' },
    { severity: 'error', role: 'alert', icon: 'error', text: 'refused', token: 'danger' },
  ];

  /** The alert of a severity, found by part of its text. */
  const alert = (page: Page, severity: string, text: string) =>
    section(page, 'Alerts').locator(`.alert--${severity}`).filter({ hasText: text });

  /** `var(--name)` as the page computes it for `property`, e.g. a colour. */
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

  test('only an error interrupts: role alert, the rest status', async ({ page }) => {
    for (const { severity, role, text } of SEVERITIES) {
      await expect(alert(page, severity, text)).toHaveAttribute('role', role);
    }
  });

  test("each severity draws its own icon, and icon=false draws none", async ({ page }) => {
    for (const { severity, icon, text } of SEVERITIES) {
      const icons = alert(page, severity, text).locator('svg.alert__icon');
      await expect(icons).toHaveCount(1);
      await expect(icons).toHaveAttribute('data-icon', icon);
    }
    await expect(alert(page, 'info', 'No icon').locator('svg')).toHaveCount(0);
  });

  test("text and border take the severity's tokens, in every theme", async ({ page }) => {
    const check = async () => {
      for (const { severity, text, token } of SEVERITIES) {
        const style = await alert(page, severity, text).evaluate((el) => {
          const s = getComputedStyle(el);
          return { color: s.color, border: s.borderLeftColor };
        });
        expect(style.color, severity).toBe(await resolved(page, 'color', `var(--${token}-text)`));
        expect(style.border, severity).toBe(await resolved(page, 'color', `var(--${token}-base)`));
      }
    };
    await check();
    await page.getByLabel('Theme').selectOption('jamaica');
    await page.getByLabel('Mode').selectOption('dark');
    await expect(page.locator('html')).toHaveAttribute('data-theme', 'jamaica-dark');
    await check();
  });

  test('only a dismissible alert has a close button; it calls on_close', async ({ page }) => {
    for (const { severity, text } of SEVERITIES) {
      await expect(alert(page, severity, text).getByRole('button')).toHaveCount(0);
    }
    const dismissible = alert(page, 'error', 'Dismissible');
    await dismissible.getByRole('button', { name: 'Close' }).click();
    await expect(dismissible).toHaveCount(0);

    await section(page, 'Alerts').getByRole('button', { name: 'Show the dismissible alert again' }).click();
    await expect(dismissible).toHaveCount(1);
  });

  test('a one-line alert is as tall with a close button as without', async ({ page }) => {
    // Wide enough that both texts fit on one line.
    await page.setViewportSize({ width: 1600, height: 900 });
    const plain = await alert(page, 'error', 'refused').boundingBox();
    const withClose = await alert(page, 'error', 'Dismissible').boundingBox();
    expect(withClose!.height).toBeCloseTo(plain!.height, 0);
  });
});
