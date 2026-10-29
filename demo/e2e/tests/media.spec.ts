import type { Page } from '@playwright/test';
import { test, expect, openApp } from '../fixtures';

// Carousel on the demo's /media page: 20 tiles, 6 to a page (4 pages), its
// page echoed and set from code; and 4 tiles that fit on one page.

const carousel = (page: Page) => page.getByRole('region', { name: 'Tiles', exact: true });
const track = (page: Page) => carousel(page).locator('.carousel__track');
const dot = (page: Page, n: number) => carousel(page).getByRole('button', { name: `Page ${n} of 4`, exact: true });
const previous = (page: Page) => carousel(page).getByRole('button', { name: 'Previous page' });
const next = (page: Page) => carousel(page).getByRole('button', { name: 'Next page' });
const tile = (page: Page, n: number) => carousel(page).getByRole('button', { name: `Open ${n}`, exact: true });
const echo = (page: Page) => page.locator('[data-echo="carousel-page"]');

test.beforeEach(async ({ page }) => {
  await page.setViewportSize({ width: 1200, height: 900 });
  await openApp(page, '/media');
});

test('items in pages: a named group each, the last one short', async ({ page }) => {
  const pages = carousel(page).getByRole('group');
  await expect(pages).toHaveCount(4);
  await expect(pages.first()).toHaveAttribute('aria-label', '1 of 4');
  await expect(pages.first().getByRole('listitem')).toHaveCount(6);
  await expect(pages.last().getByRole('listitem')).toHaveCount(2);
  await expect(carousel(page)).toHaveAttribute('aria-roledescription', 'carousel');
});

test('one page shows at a time', async ({ page }) => {
  await expect(tile(page, 1)).toBeInViewport();
  await expect(tile(page, 7)).not.toBeInViewport();
});

test('Previous and Next step, and are disabled at the ends', async ({ page }) => {
  await expect(previous(page)).toBeDisabled();
  await next(page).click();
  await expect(echo(page)).toHaveText('2');
  await expect(tile(page, 7)).toBeInViewport();
  await expect(tile(page, 1)).not.toBeInViewport();
  await previous(page).click();
  await expect(echo(page)).toHaveText('1');
  await expect(tile(page, 1)).toBeInViewport();
  await dot(page, 4).click();
  await expect(next(page)).toBeDisabled();
});

test('a dot goes to its page and is the current one', async ({ page }) => {
  await expect(dot(page, 1)).toHaveAttribute('aria-current', 'page');
  await dot(page, 3).click();
  await expect(echo(page)).toHaveText('3');
  await expect(dot(page, 3)).toHaveAttribute('aria-current', 'page');
  await expect(carousel(page).locator('[aria-current]')).toHaveCount(1);
  await expect(tile(page, 13)).toBeInViewport();
});

test('a page set from code scrolls the track to it', async ({ page }) => {
  await page.getByRole('button', { name: 'Go to page 3 from code' }).click();
  await expect(tile(page, 13)).toBeInViewport();
  await expect(dot(page, 3)).toHaveAttribute('aria-current', 'page');
});

test('a swipe (the track scrolled) moves the dots', async ({ page }) => {
  await track(page).evaluate((el) => {
    el.scrollLeft = el.clientWidth * 3;
  });
  await expect(echo(page)).toHaveText('4');
  await expect(dot(page, 4)).toHaveAttribute('aria-current', 'page');
});

test('tabbing into the next page scrolls it into view', async ({ page }) => {
  await tile(page, 6).focus();
  await page.keyboard.press('Tab');
  await expect(tile(page, 7)).toBeFocused();
  await expect(tile(page, 7)).toBeInViewport();
  await expect(echo(page)).toHaveText('2');
});

test('one page: no controls', async ({ page }) => {
  const few = page.getByRole('region', { name: 'Few tiles' });
  await expect(few.getByRole('group')).toHaveCount(1);
  await expect(few.getByRole('button', { name: 'Next page' })).toHaveCount(0);
  await expect(few.locator('.carousel__dot')).toHaveCount(0);
});
