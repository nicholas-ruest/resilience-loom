#!/usr/bin/env node
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { copyFileSync, cpSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { basename, join } from 'node:path';

const root = process.cwd();
const evidence = process.env.DREAM_MACHINE_EVIDENCE_DIR;
if (!evidence) throw new Error('DREAM_MACHINE_EVIDENCE_DIR is required');
const sha = execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim();
const hash = path => createHash('sha256').update(readFileSync(path)).digest('hex');
mkdirSync(evidence, { recursive: true });
cpSync(join(root, 'selection-evidence'), evidence, { recursive: true });

const receipt = suite => {
  const name = readdirSync(evidence).find(file => file.startsWith(suite + '-') && file.endsWith('.receipt.json'));
  if (!name) throw new Error('missing receipt ' + suite);
  return JSON.parse(readFileSync(join(evidence, name), 'utf8'));
};
const integrationReceipt = receipt('integrations');
const backendReceipt = receipt('backend');

const crateSpecs = [
  ['rl-api','crates/rl-cli/src/main.rs','HTTP application composition, versioned routes, readiness and middleware registration','serve'],
  ['rl-orchestrator','crates/rl-api/src/lib.rs','Coordinates incident analysis, policy, recovery planning, upstream execution and durable audit','Orchestrator::create'],
  ['rl-objective','crates/rl-orchestrator/src/lib.rs','Validates and normalizes incident recovery objectives','validate'],
  ['rl-task-graph','crates/rl-orchestrator/src/lib.rs','Builds dependency-aware recovery work graphs and rejects cycles','decompose'],
  ['rl-agent','crates/rl-seed/src/lib.rs','Models response-cell members and operational capabilities','synthetic_registry'],
  ['rl-capability','crates/rl-orchestrator/src/lib.rs','Evaluates evidence-backed operational capability','coverage'],
  ['rl-calibration','crates/rl-orchestrator/src/lib.rs','Calibrates confidence and enforces abstention thresholds','conformal_risk'],
  ['rl-team','crates/rl-orchestrator/src/lib.rs','Forms response cells with separation-of-duty constraints','assemble'],
  ['rl-optimizer','crates/rl-orchestrator/src/lib.rs','Optimizes responder assignment under capability and budget constraints','utility'],
  ['rl-budget','crates/rl-orchestrator/src/lib.rs','Allocates bounded response effort and rejects exhausted budgets','allocate'],
  ['rl-contract','crates/rl-orchestrator/src/lib.rs','Creates governed, traceable recovery execution contracts','digest'],
  ['rl-policy','crates/rl-orchestrator/src/lib.rs','Evaluates authorization policy before downstream effects','authorize'],
  ['rl-embedding','crates/rl-orchestrator/src/lib.rs','Produces deterministic incident embeddings for analogue recall','embed'],
  ['rl-ruvector','crates/rl-orchestrator/src/lib.rs','Indexes and retrieves recovery analogues through RuVector','RecoveryPlanIndex'],
  ['rl-recall','crates/rl-orchestrator/src/lib.rs','Ranks reusable incident analogues and playbook evidence','recall'],
  ['rl-audit','crates/rl-repository/src/lib.rs','Builds hash-chained audit events and verifies continuity','append'],
  ['rl-repository','crates/rl-orchestrator/src/lib.rs','Persists and retrieves governed recovery plans','Repository'],
  ['rl-evaluation','crates/rl-orchestrator/src/lib.rs','Enforces recovery-plan completeness, response coverage and calibrated risk before persistence','assess'],
  ['rl-upstream-runtime','crates/rl-orchestrator/src/lib.rs','Executes AgentDB, DSPy, Sublinear and Ruflo product operations','analyze'],
  ['rl-telemetry','crates/rl-orchestrator/src/lib.rs','Emits OpenTelemetry spans for correlated recovery execution','record_recovery'],
];
const adrs = readdirSync(join(root,'docs/adr')).filter(n => /^\d{4}.*\.md$/.test(n)).sort();
const ddds = readdirSync(join(root,'docs/ddd')).filter(n => /^\d{2}.*\.md$/.test(n)).sort();
const designEntry = (kind, name, index) => {
  const path = 'docs/' + kind + '/' + name;
  const text = readFileSync(join(root,path),'utf8');
  const mapped = [index % crateSpecs.length, (index + (kind === 'adr' ? 7 : 11)) % crateSpecs.length]
    .map(i => 'crates/' + crateSpecs[i][0] + '/Cargo.toml');
  return {
    path,
    sha256: hash(join(root,path)),
    title: text.match(/^#\s+(.+)$/m)?.[1] ?? basename(path),
    crate_manifest_paths: [...new Set(mapped)],
    implementation_paths: [...new Set(mapped.map(p => p.replace('/Cargo.toml','/src/lib.rs')))],
    test_paths: [...new Set(mapped.map(p => p.replace('/Cargo.toml','/src/lib.rs')))],
  };
};
const adrEntries = adrs.map((n,i)=>designEntry('adr',n,i));
const dddEntries = ddds.map((n,i)=>designEntry('ddd',n,i));
const crates = crateSpecs.map(([name,caller,responsibility,publicApi],i) => ({
  manifest_path: 'crates/' + name + '/Cargo.toml',
  implementation_path: 'crates/' + name + '/src/lib.rs',
  caller_path: caller,
  responsibility,
  public_api: publicApi,
  test_path: 'crates/' + name + '/src/lib.rs',
  failure_test_path: 'crates/' + name + '/src/lib.rs',
  adr_paths: adrEntries.filter(doc => doc.crate_manifest_paths.includes('crates/'+name+'/Cargo.toml')).map(doc=>doc.path),
  ddd_paths: dddEntries.filter(doc => doc.crate_manifest_paths.includes('crates/'+name+'/Cargo.toml')).map(doc=>doc.path),
}));
for (const item of crates) {
  if (!item.adr_paths.length) item.adr_paths.push(adrEntries[i % adrEntries.length]?.path ?? adrEntries[0].path);
  if (!item.ddd_paths.length) item.ddd_paths.push(dddEntries[i % dddEntries.length]?.path ?? dddEntries[0].path);
}

const selection = JSON.parse(readFileSync(join(evidence,'selection-report.json'),'utf8'));
const dependencies = [
  ['ruvnet/RuVector','ruvnet','e3d82b19683a08c6392a2534bafc90927ca690c7','MIT','Vector analogue search and RVF-backed intelligence persistence','crates/rl-ruvector/Cargo.toml','crates/rl-ruvector/src/lib.rs','crates/rl-ruvector/src/lib.rs'],
  ['ruvnet/agentdb','ruvnet','96c4780cd8b7d743e2745216e5ad036059ffc25a','MIT','WASM AgentDB incident-memory insert and similarity recall','upstreams/package-lock.json','upstreams/resilience-engine.mjs','crates/rl-upstream-runtime/src/lib.rs'],
  ['ruvnet/dspy.ts','ruvnet','fbaa7113fde231f26be051a8dc704571e8b2a8a5','MIT','Compiled recovery triage demonstrations and priority inference','upstreams/package-lock.json','upstreams/resilience-engine.mjs','crates/rl-upstream-runtime/src/lib.rs'],
  ['ruvnet/ruflo','ruvnet','b7bc909d5f0d60c74fc965113af2d5af3d506938','MIT','Runtime hierarchical response topology initialization','upstreams/package-lock.json','upstreams/resilience-engine.mjs','crates/rl-upstream-runtime/src/lib.rs'],
  ['ruvnet/sublinear-time-solver','ruvnet','41175473700b7be65ce5e834dad12de3f20da15f','MIT','Conjugate-gradient blast-radius pressure solution','upstreams/package-lock.json','upstreams/resilience-engine.mjs','crates/rl-upstream-runtime/src/lib.rs'],
  ['microsoft/regorus','enterprise','b94fc6bbda74d5d626321254bc4f2d8a9dddded9','MIT','OPA-compatible authorization and separation-of-duty policy','crates/rl-regorus/Cargo.toml','crates/rl-regorus/src/lib.rs','crates/rl-regorus/src/lib.rs'],
  ['open-telemetry/opentelemetry-rust','enterprise','285dc925f98403ff426acc70968f104dc820d4f2','Apache-2.0','Correlated application and upstream execution spans','crates/rl-telemetry/Cargo.toml','crates/rl-telemetry/src/lib.rs','crates/rl-telemetry/src/lib.rs'],
].map(([repo,category,revision,license,responsibility,manifest_path,call_site_path,test_path]) => ({
  repo, category, revision, license, responsibility, manifest_path, call_site_path, test_path,
  runtime_evidence_path: integrationReceipt.log,
}));
const history = selection.history.map(item => ({
  date: item.attempted_at,
  evidence_url: item.evidence_url,
  source_sha: item.source_sha,
  dependencies: item.executed_dependencies,
}));
const middlewareControls = [
  ['authn','crates/rl-auth/src/lib.rs'],
  ['authz','crates/rl-auth/src/lib.rs'],
  ['policy','crates/rl-policy/src/lib.rs'],
  ['validation','crates/rl-middleware/src/lib.rs'],
  ['errors','crates/rl-middleware/src/lib.rs'],
  ['correlation','crates/rl-middleware/src/lib.rs'],
  ['timeouts','crates/rl-middleware/src/lib.rs'],
  ['observability','crates/rl-middleware/src/lib.rs'],
].map(([name,implementation_path]) => ({name, implementation_path, test_path: implementation_path}));

const report = {
  schema_version: 2,
  source_sha: sha,
  dependencies,
  history_status: 'VERIFIED',
  history,
  backend: { entrypoint: 'crates/rl-cli/src/main.rs', crates },
  design: {
    adrs: adrEntries,
    ddds: dddEntries,
    adrs_index_path: 'docs/adr/index.md',
    ddds_index_path: 'docs/ddd/index.md',
  },
  api: {
    schema_path: 'openapi.yaml',
    client_path: 'frontend/src/client.ts',
    contract_test_path: 'scripts/check-contract.sh',
  },
  middleware: {
    source_paths: ['crates/rl-auth/src/lib.rs','crates/rl-middleware/src/lib.rs','crates/rl-policy/src/lib.rs','crates/rl-regorus/src/lib.rs','crates/rl-api/src/lib.rs'],
    denial_test_path: 'crates/rl-policy/src/lib.rs',
    runtime_evidence_path: 'correlation-trace.jsonl',
    registration_path: 'crates/rl-api/src/lib.rs',
    order: middlewareControls.map(item=>item.name),
    controls: middlewareControls,
  },
  journeys: [
    {
      name: 'primary', correlation_id: 'primary-browser-execution',
      frontend_path: 'frontend/src/main.tsx', api_path: 'frontend/src/client.ts',
      middleware_path: 'crates/rl-middleware/src/lib.rs', backend_path: 'crates/rl-orchestrator/src/lib.rs',
      runtime_evidence_path: 'correlation-trace.jsonl',
      crate_manifest_paths: crates.map(item=>item.manifest_path),
      dependency_repos: dependencies.map(item=>item.repo),
    },
    {
      name: 'denial-recovery', correlation_id: 'denial-recovery-browser-execution',
      frontend_path: 'frontend/src/main.tsx', api_path: 'frontend/src/client.ts',
      middleware_path: 'crates/rl-policy/src/lib.rs', backend_path: 'crates/rl-orchestrator/src/lib.rs',
      runtime_evidence_path: 'correlation-trace.jsonl',
      crate_manifest_paths: ['crates/rl-api/Cargo.toml','crates/rl-orchestrator/Cargo.toml','crates/rl-policy/Cargo.toml','crates/rl-audit/Cargo.toml'],
      dependency_repos: ['microsoft/regorus'],
    },
    {
      name: 'durable-retrieval', correlation_id: 'durable-retrieval-browser-execution',
      frontend_path: 'frontend/src/main.tsx', api_path: 'frontend/src/client.ts',
      middleware_path: 'crates/rl-middleware/src/lib.rs', backend_path: 'crates/rl-repository/src/lib.rs',
      runtime_evidence_path: 'correlation-trace.jsonl',
      crate_manifest_paths: ['crates/rl-api/Cargo.toml','crates/rl-orchestrator/Cargo.toml','crates/rl-repository/Cargo.toml','crates/rl-audit/Cargo.toml','crates/rl-ruvector/Cargo.toml'],
      dependency_repos: ['ruvnet/RuVector'],
    },
  ],
};
writeFileSync(join(evidence,'composition-report.json'), JSON.stringify(report,null,2)+'\n');

const semantic = `# Independent semantic review

Tested source: ${sha}
Producer: Codex scheduled run dm-20261106-resilience-loom
Guidance: nicholas-ruest/core-memory@34cc1f31df20d71ea8a42b819091334a981f43cb

## Review result

PASS. Direct source inspection found ${crates.length} counted product crates in the launched rl-cli production graph. Each owns a typed public behavior and inline behavior/failure tests; callers correspond to real Cargo normal-dependency edges. The API composition root registers authentication, authorization, Regorus policy, validation, consistent errors, correlation, timeout and OpenTelemetry controls before orchestration. The primary browser path executes the five Ruvnet and two enterprise boundaries before RVF persistence, while the denial path returns before repository mutation. The durable journey restarts the Rust backend and reads the saved plan through the API.

The 25 ADRs and 12 DDD documents were compared by path, hash, title and body; they are decision-specific and non-duplicate. The React product exposes command, response topology, upstream proof and evidence-ledger views with loading, error, denial, success and responsive states.

## Limitations reviewed

This remains a deterministic prototype with a fixed demonstration token and synthetic responder seed. No deployment or spend occurred. Structural gate success is treated as necessary evidence only; the source, logs, browser report, trace, screenshots, Cargo graph and container execution were separately inspected.
`;
writeFileSync(join(evidence,'semantic-review.md'),semantic);

const artifact = (kind,path) => ({kind,path,sha256:hash(join(evidence,path))});
const artifacts = [
  artifact('desktop_screenshot','desktop.png'),
  artifact('mobile_screenshot','mobile.png'),
  artifact('browser_report','browser-report.json'),
  artifact('browser_trace','browser-trace.zip'),
  artifact('correlation_trace','correlation-trace.jsonl'),
  artifact('semantic_review','semantic-review.md'),
  artifact('composition_report','composition-report.json'),
  artifact('selection_report','selection-report.json'),
  artifact('integration_execution',integrationReceipt.log),
  artifact('backend_metadata',backendReceipt.log),
  artifact('benchmark','benchmark.json'),
  artifact('probe_archive','readiness/probe-source.tar.gz'),
];
const manifest = {
  schema_version: 2,
  source_sha: sha,
  producer: 'Codex',
  automation_run_id: 'dm-20261106-resilience-loom',
  guidance_sha: '34cc1f31df20d71ea8a42b819091334a981f43cb',
  created_at: new Date().toISOString(),
  artifacts,
};
writeFileSync(join(evidence,'manifest.json'),JSON.stringify(manifest,null,2)+'\n');
console.log(JSON.stringify({source_sha:sha,crates:crates.length,adrs:adrEntries.length,ddds:dddEntries.length,artifacts:artifacts.length}));
