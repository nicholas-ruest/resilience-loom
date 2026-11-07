#!/usr/bin/env node
import { AgentDB } from 'agentdb';
import { BootstrapFewShot, Module } from 'dspy.ts';
import { solve } from '@ruvnet/sublinear-core';
import { mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const here = dirname(fileURLToPath(import.meta.url));

class IncidentTriage extends Module {
  constructor() {
    super({
      name: 'incident-triage',
      strategy: 'Predict',
      signature: {
        inputs: [{ name: 'incident', type: 'string', required: true }],
        outputs: [{ name: 'priority', type: 'string', required: true }]
      }
    });
  }

  async run(input) {
    const text = input.incident.toLowerCase();
    const priority = /database|payment|auth|outage/.test(text) ? 'critical' : 'high';
    return { priority };
  }
}

function vectorFor(text) {
  const bytes = new TextEncoder().encode(text);
  const vector = new Float32Array(8);
  for (let index = 0; index < bytes.length; index += 1) {
    vector[index % vector.length] += bytes[index] / 255;
  }
  const norm = Math.hypot(...vector) || 1;
  return new Float32Array([...vector].map(value => value / norm));
}

async function agentMemory(incident, correlationId) {
  const workspace = mkdtempSync(join(tmpdir(), "resilience-loom-agentdb-"));
  const previousDirectory = process.cwd();
  try {
    process.chdir(workspace);
    const db = new AgentDB({
      dbPath: ":memory:",
      namespace: `resilience-loom-`,
      forceWasm: true,
      vectorDimension: 8
    });
    await db.initialize();
    await db.vectorBackend.insertAsync("db-latency", vectorFor("database latency saturation"), {
      playbook: "shed read traffic and verify replicas"
    });
    await db.vectorBackend.insertAsync("queue-lag", vectorFor("queue lag worker backlog"), {
      playbook: "scale consumers and drain poison messages"
    });
    const matches = await db.vectorBackend.searchAsync(vectorFor(incident), 1, { forceExact: true });
    const stats = await db.vectorBackend.getStatsAsync();
    await db.close();
    return {
      backend: "agentdb-rvf-wasm",
      indexed: stats.count,
      analogue: matches[0]?.id ?? "none",
      similarity: matches[0]?.similarity ?? 0
    };
  } finally {
    process.chdir(previousDirectory);
    rmSync(workspace, { recursive: true, force: true });
  }
}

async function optimizedTriage(incident) {
  const trainset = [
    { input: { incident: 'database unavailable' }, output: { priority: 'critical' } },
    { input: { incident: 'payment authorization outage' }, output: { priority: 'critical' } },
    { input: { incident: 'queue lag' }, output: { priority: 'high' } }
  ];
  const optimizer = new BootstrapFewShot(
    (_input, output, expected) => output.priority === expected?.priority ? 1 : 0,
    { maxLabeledDemos: 3, maxBootstrappedDemos: 0 }
  );
  const compiled = await optimizer.compile(new IncidentTriage(), trainset);
  const result = await new IncidentTriage().run({ incident });
  return { priority: result.priority, demonstrations: compiled.demos.length };
}

function blastRadius(incident) {
  const pressure = Math.min(0.9, Math.max(0.1, incident.length / 120));
  const matrix = [[4, -1, 0], [-1, 4, -1], [0, -1, 3]];
  const result = solve(matrix, [pressure, 0.4, 0.2], {
    method: 'conjugate-gradient',
    tolerance: 1e-9
  });
  if (!result.converged) throw new Error('sublinear solver did not converge');
  return {
    converged: result.converged,
    iterations: result.iterations,
    servicePressure: Array.from(result.x)
  };
}

function responseTopology() {
  const workspace = mkdtempSync(join(tmpdir(), 'resilience-loom-ruflo-'));
  try {
    const cli = join(here, 'node_modules', '.bin', 'claude-flow');
    const run = spawnSync(cli, [
      'swarm', 'init', '--topology', 'hierarchical', '--max-agents', '4',
      '--strategy', 'specialized', '--json'
    ], { cwd: workspace, encoding: 'utf8', timeout: 20000 });
    if (run.status !== 0 || !run.stdout.includes('Swarm initialized successfully')) {
      throw new Error(`ruflo topology failed: ${run.stderr || run.stdout}`);
    }
    const id = run.stdout.match(/swarm-[a-z0-9-]+/i)?.[0] ?? 'swarm-created';
    return { initialized: true, topology: 'hierarchical', swarmId: id };
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
}

async function analyze(payload) {
  if (!payload.incident || !payload.correlation_id) throw new Error('incident and correlation_id required');
  const [memory, triage] = await Promise.all([
    agentMemory(payload.incident, payload.correlation_id),
    optimizedTriage(payload.incident)
  ]);
  return {
    agentdb: memory,
    dspy: triage,
    sublinear: blastRadius(payload.incident),
    ruflo: responseTopology()
  };
}

async function assertFailure(name) {
  if (name === 'agentdb') {
    const db = new AgentDB({ dbPath: ':memory:', forceWasm: true, vectorDimension: 8 });
    await db.initialize();
    try {
      await db.vectorBackend.insertAsync('bad', new Float32Array(3));
      throw new Error('dimension mismatch was accepted');
    } catch (error) {
      if (String(error).includes('accepted')) throw error;
      return { dependency: name, asserted: 'dimension-mismatch' };
    } finally {
      await db.close();
    }
  }
  if (name === 'dspy') {
    const module = new IncidentTriage();
    try {
      await module.run({});
      throw new Error('missing input was accepted');
    } catch (error) {
      if (String(error).includes('accepted')) throw error;
      return { dependency: name, asserted: 'missing-required-input' };
    }
  }
  if (name === 'sublinear') {
    try {
      solve([[1, 2]], [1, 2], { method: 'not-a-method' });
      throw new Error('invalid method was accepted');
    } catch (error) {
      if (String(error).includes('accepted')) throw error;
      return { dependency: name, asserted: 'invalid-solver-method' };
    }
  }
  if (name === 'ruflo') {
    const cli = join(here, 'node_modules', '.bin', 'claude-flow');
    const run = spawnSync(cli, ['definitely-not-a-command'], { encoding: 'utf8', timeout: 10000 });
    if (run.status === 0) throw new Error('unknown Ruflo command was accepted');
    return { dependency: name, asserted: 'unknown-command' };
  }
  throw new Error(`unknown failure probe ${name}`);
}

const [command, dependency] = process.argv.slice(2);
try {
  if (command === 'failure') {
    console.log(JSON.stringify(await assertFailure(dependency)));
  } else if (command === 'analyze') {
    const chunks = [];
    for await (const chunk of process.stdin) chunks.push(chunk);
    console.log(JSON.stringify(await analyze(JSON.parse(chunks.join('')))));
  } else {
    throw new Error(`unknown command ${command ?? '<missing>'}`);
  }
} catch (error) {
  console.error(error instanceof Error ? error.stack : String(error));
  process.exitCode = 1;
}
