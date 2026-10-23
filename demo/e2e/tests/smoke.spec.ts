import { test, expect, openApp } from '../fixtures';

// Every demo route mounts without errors. The fixture fails the test on any
// page error or console error.
const ROUTES = [
  { path: '/', heading: 'Surfaces' },
  { path: '/buttons', heading: 'Families' },
];

for (const { path, heading } of ROUTES) {
  test(`${path} loads`, async ({ page }) => {
    await openApp(page, path);
    await expect(page.getByRole('heading', { name: heading, exact: true })).toBeVisible();
  });
}
