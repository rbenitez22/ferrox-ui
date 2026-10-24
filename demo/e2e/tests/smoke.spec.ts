import { test, expect, openApp } from '../fixtures';

// Every demo route mounts without errors. The fixture fails the test on any
// page error or console error.
const ROUTES = [
  { path: '/', heading: 'Surfaces' },
  { path: '/buttons', heading: 'Families' },
  { path: '/icons', heading: 'All icons' },
  { path: '/feedback', heading: 'Spinners' },
  { path: '/overlay', heading: 'Dialogs' },
  { path: '/forms', heading: 'Fields' },
  { path: '/help', heading: 'Help topics' },
  { path: '/navigation', heading: 'Link list' },
  { path: '/navigation/details', heading: 'Link list' },
  { path: '/layout', heading: 'Page panel' },
];

for (const { path, heading } of ROUTES) {
  test(`${path} loads`, async ({ page }) => {
    await openApp(page, path);
    await expect(page.getByRole('heading', { name: heading, exact: true })).toBeVisible();
  });
}
