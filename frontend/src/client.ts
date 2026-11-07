export type UpstreamAnalysis = {
  agentdb: { backend: string; indexed: number; analogue: string; similarity: number };
  dspy: { priority: string; demonstrations: number };
  sublinear: { converged: boolean; iterations: number; servicePressure: number[] };
  ruflo: { initialized: boolean; topology: string; swarmId: string };
};
export type RecoveryPlan = {
  id: string;
  objective: { text: string; sensitivity: string };
  tasks: { id: string; role: string; depends_on: string[] }[];
  agents: { id: string; name: string; score: number; roles: string[] }[];
  risk: number;
  contract_hash: string;
  status: string;
  correlation_id: string;
  upstream_analysis?: UpstreamAnalysis;
};
const API = import.meta.env.VITE_API_URL ?? 'http://127.0.0.1:18080';
const headers = (token: string) => ({
  Authorization: 'Bearer ' + token,
  'Content-Type': 'application/json',
  'X-Correlation-Id': crypto.randomUUID(),
});
async function decode<T>(response: Response): Promise<T> {
  const body = await response.json();
  if (!response.ok) throw new Error(body.message ?? `Request failed (${response.status})`);
  return body as T;
}
export async function createRecoveryPlan(
  token: string,
  objective: string,
  sensitivity: string,
): Promise<RecoveryPlan> {
  return decode(
    await fetch(`${API}/v1/recovery-plans`, {
      method: 'POST',
      headers: headers(token),
      body: JSON.stringify({ objective, sensitivity }),
    }),
  );
}
export async function listRecoveryPlans(token: string): Promise<RecoveryPlan[]> {
  return decode(await fetch(`${API}/v1/recovery-plans`, { headers: headers(token) }));
}
export async function getRecoveryPlan(token: string, id: string): Promise<RecoveryPlan> {
  return decode(await fetch(`${API}/v1/recovery-plans/${id}`, { headers: headers(token) }));
}
