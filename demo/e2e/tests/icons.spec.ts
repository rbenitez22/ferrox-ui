import type { Locator, Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// ferrox-icons on the demo's /icons page: every icon draws, in the SVG
// namespace, decorative, sized by the font, coloured by the text, and an icon
// chosen at runtime swaps its drawing.

const SVG_NS = 'http://www.w3.org/2000/svg';

const FIRST_SET = [
  'ban', 'camera', 'check', 'chevron-down', 'chevron-right', 'close', 'delete',
  'help', 'lock', 'menu', 'plus', 'refresh', 'search', 'unlock',
];

/** The demo section titled `title` (demo/src/section.rs). */
function section(page: Page, title: string): Locator {
  return page.locator('section', { has: page.getByRole('heading', { name: title, exact: true }) });
}

const galleryIcons = (page: Page) => section(page, 'All icons').locator('svg.icon');

test.beforeEach(async ({ page }) => {
  await openApp(page, '/icons');
});

test.describe('gallery', () => {
  test('shows every icon once, sorted by name, labelled with its name', async ({ page }) => {
    const names = await galleryIcons(page).evaluateAll((svgs) => svgs.map((s) => s.getAttribute('data-icon')));
    expect(names).toEqual([...names].sort());
    expect(new Set(names).size).toBe(names.length);
    expect(names).toEqual(expect.arrayContaining(FIRST_SET));
    const labels = await section(page, 'All icons').locator('code').allTextContents();
    expect(labels).toEqual(names);
  });

  test('every icon draws: SVG-namespace children and a non-empty box', async ({ page }) => {
    for (const icon of await galleryIcons(page).all()) {
      const name = await icon.getAttribute('data-icon');
      const children = await icon.evaluate((svg) => Array.from(svg.children).map((c) => c.namespaceURI));
      expect(children.length, name!).toBeGreaterThan(0);
      expect(children.every((ns) => ns === SVG_NS), name!).toBe(true);
      const drawn = await icon.evaluate((svg) => {
        const b = (svg as SVGSVGElement).getBBox();
        return { width: b.width, height: b.height };
      });
      expect(drawn.width * drawn.height, name!).toBeGreaterThan(0);
    }
  });

  test('icons are decorative and not focusable', async ({ page }) => {
    for (const icon of await galleryIcons(page).all()) {
      await expect(icon).toHaveAttribute('aria-hidden', 'true');
      await expect(icon).toHaveAttribute('focusable', 'false');
    }
  });

  test("ferrox-ui's .icon sheet applies: block, no pointer events", async ({ page }) => {
    const style = await galleryIcons(page).first().evaluate((svg) => {
      const s = getComputedStyle(svg);
      return { display: s.display, pointerEvents: s.pointerEvents };
    });
    expect(style).toEqual({ display: 'block', pointerEvents: 'none' });
  });
});

test('size follows the font: 1em at 16, 24 and 32px', async ({ page }) => {
  const icons = section(page, 'Size follows the font').locator('svg.icon');
  await expect(icons).toHaveCount(3);
  for (const [i, size] of [16, 24, 32].entries()) {
    const box = await icons.nth(i).boundingBox();
    expect(box?.width).toBeCloseTo(size, 0);
    expect(box?.height).toBeCloseTo(size, 0);
  }
});

test.describe('colour', () => {
  test.use({ colorScheme: 'light' });

  test('the stroke is the text colour, in every theme', async ({ page }) => {
    const icon = galleryIcons(page).first();
    const colours = () => icon.evaluate((svg) => {
      const s = getComputedStyle(svg);
      return { stroke: s.stroke, color: s.color };
    });

    const light = await colours();
    expect(light.stroke).toBe(light.color);

    await page.getByLabel('Theme').selectOption('jamaica');
    await page.getByLabel('Mode').selectOption('dark');
    await expect(page.locator('html')).toHaveAttribute('data-theme', 'jamaica-dark');
    const dark = await colours();
    expect(dark.stroke).toBe(dark.color);
    expect(dark.color).not.toBe(light.color);
  });
});

test('an icon chosen at runtime swaps its drawing', async ({ page }) => {
  const button = section(page, 'Chosen at runtime').getByRole('button');
  const icon = button.locator('svg.icon');
  await expect(button).toHaveText('Expand');
  await expect(icon).toHaveAttribute('data-icon', 'chevron-right');
  const before = await icon.innerHTML();

  await button.click();
  await expect(button).toHaveText('Collapse');
  await expect(icon).toHaveAttribute('data-icon', 'chevron-down');
  expect(await icon.innerHTML()).not.toBe(before);
  const children = await icon.evaluate((svg) => Array.from(svg.children).map((c) => c.namespaceURI));
  expect(children).toEqual([SVG_NS]);
});
