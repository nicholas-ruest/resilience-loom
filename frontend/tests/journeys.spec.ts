import { execFileSync } from 'node:child_process';
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';

const API = 'http://127.0.0.1:18080/v1/recovery-plans';
const auth = { Authorization: 'Bearer demo-token' };
const evidence = process.env.DREAM_MACHINE_EVIDENCE_DIR;

async function openCommand(page: Page) {
  await page.goto('/');
  await page.getByLabel('Session token').fill('demo-token');
}

async function savedCount(page: Page) {
  const response = await page.request.get(API, { headers: auth });
  expect(response.ok()).toBeTruthy();
  return (await response.json()).length as number;
}

test('[journey:primary] incident becomes a rendered governed recovery', async ({ page }, info) => {
  await openCommand(page);
  await page.getByRole('button', { name: 'Compose recovery plan' }).click();
  await expect(page.getByRole('heading', { name: 'Response topology' })).toBeVisible();
  await expect(page.getByText('hierarchical', { exact: true })).toBeVisible();
  await expect(page.getByText('Converged')).toBeVisible();
  await expect(page.getByText('Atlas')).toBeVisible();
  if (evidence && info.project.name === 'desktop') await page.screenshot({ path: path.join(evidence, 'desktop.png'), fullPage: true, scale: 'css' });
  if (evidence && info.project.name === 'mobile') await page.screenshot({ path: path.join(evidence, 'mobile.png'), fullPage: false, scale: 'css' });
});

test('[journey:denial-recovery] policy denial writes nothing and recovery succeeds', async ({ page }) => {
  await openCommand(page);
  const before = await savedCount(page);
  await page.getByLabel('Data sensitivity').selectOption('restricted');
  await page.getByRole('button', { name: 'Compose recovery plan' }).click();
  await expect(page.getByRole('alert')).toContainText('policy denied');
  expect(await savedCount(page)).toBe(before);
  await page.getByLabel('Data sensitivity').selectOption('internal');
  await page.getByRole('button', { name: 'Compose recovery plan' }).click();
  await expect(page.getByRole('heading', { name: 'Response topology' })).toBeVisible();
  expect(await savedCount(page)).toBeGreaterThan(before);
});

test('[journey:durable-retrieval] saved result survives refresh and backend restart', async ({ page }) => {
  await openCommand(page);
  await page.getByLabel('Incident narrative').fill('Queue saturation threatens checkout; preserve this restart-durable recovery evidence');
  await page.getByRole('button', { name: 'Compose recovery plan' }).click();
  await expect(page.getByText('hierarchical', { exact: true })).toBeVisible();
  execFileSync('../scripts/restart-journey.sh', { cwd: process.cwd(), env: process.env, stdio: 'inherit' });
  await page.reload();
  await page.getByLabel('Session token').fill('demo-token');
  await page.getByRole('button', { name: 'Evidence ledger' }).click();
  await page.getByRole('button', { name: 'Refresh from RVF' }).click();
  await expect(page.getByText('Queue saturation threatens checkout', { exact: false }).first()).toBeVisible();
});
