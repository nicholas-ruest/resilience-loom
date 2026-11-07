import React, { useMemo, useState } from 'react';
import { createRoot } from 'react-dom/client';
import {
  createRecoveryPlan,
  listRecoveryPlans,
  type RecoveryPlan,
} from './client';
import './style.css';
import './product.css';

type View = 'command' | 'topology' | 'evidence';
const starter = 'Database latency threatens payment authorization; isolate blast radius, preserve evidence, and verify recovery';

function App() {
  const [token, setToken] = useState('');
  const [objective, setObjective] = useState(starter);
  const [sensitivity, setSensitivity] = useState('internal');
  const [view, setView] = useState<View>('command');
  const [items, setItems] = useState<RecoveryPlan[]>([]);
  const [busy, setBusy] = useState(false);
  const [loaded, setLoaded] = useState(false);
  const [error, setError] = useState('');
  const [selected, setSelected] = useState<RecoveryPlan | null>(null);

  const priority = selected?.upstream_analysis?.dspy.priority ?? 'awaiting signal';
  const pressure = useMemo(
    () => selected?.upstream_analysis?.sublinear.servicePressure ?? [],
    [selected],
  );

  async function load() {
    setError('');
    try {
      const recovered = await listRecoveryPlans(token);
      setItems(recovered);
      setLoaded(true);
      if (!selected && recovered.length) setSelected(recovered[0]);
    } catch (caught) {
      setError(String(caught));
    }
  }

  async function submit(event: React.FormEvent) {
    event.preventDefault();
    setBusy(true);
    setError('');
    try {
      const plan = await createRecoveryPlan(token, objective, sensitivity);
      setSelected(plan);
      setView('topology');
      await load();
    } catch (caught) {
      setError(String(caught));
    } finally {
      setBusy(false);
    }
  }

  return (
    <main>
      <header className="hero">
        <div>
          <p className="eyebrow">INCIDENT RECOVERY CONTROL PLANE</p>
          <h1>Resilience Loom</h1>
          <p className="lede">
            Weave incident memory, sparse blast-radius analysis, calibrated response teams,
            policy proof, and restart-durable audit into one recovery plan.
          </p>
        </div>
        <div className="pulse" aria-label="system readiness">
          <span />
          Seven upstream boundaries ready
        </div>
      </header>

      <nav aria-label="Product views">
        {(['command', 'topology', 'evidence'] as View[]).map((name) => (
          <button
            className={view === name ? 'nav-active' : 'nav-button'}
            key={name}
            onClick={() => setView(name)}
          >
            {name === 'command' ? 'Command board' : name === 'topology' ? 'Response topology' : 'Evidence ledger'}
          </button>
        ))}
      </nav>

      {error && <div className="error-banner" role="alert">{error}</div>}

      {view === 'command' && (
        <section className="command-layout">
          <form onSubmit={submit} aria-label="Create recovery plan">
            <div className="section-kicker">01 / Frame the incident</div>
            <h2>Open a governed recovery</h2>
            <label>
              Operator session token
              <input
                aria-label="Session token"
                type="password"
                value={token}
                onChange={(event) => setToken(event.target.value)}
                required
              />
            </label>
            <label>
              Incident narrative
              <textarea
                value={objective}
                onChange={(event) => setObjective(event.target.value)}
                required
                minLength={24}
              />
            </label>
            <div className="control-row">
              <label>
                Data sensitivity
                <select value={sensitivity} onChange={(event) => setSensitivity(event.target.value)}>
                  <option value="public">public</option>
                  <option value="internal">internal</option>
                  <option value="restricted">restricted</option>
                </select>
              </label>
              <div className="guardrail">
                <b>Policy guardrail</b>
                Restricted incidents require separation of duties and may be denied before any write.
              </div>
            </div>
            <button className="primary-action" disabled={busy}>
              {busy ? 'Composing real upstreams&' : 'Compose recovery plan'}
            </button>
          </form>
          <aside className="signal-panel">
            <div className="section-kicker">LIVE DECISION SIGNALS</div>
            <div className="metric"><span>Priority</span><strong>{priority}</strong></div>
            <div className="metric"><span>Calibrated risk</span><strong>{selected ? `${(selected.risk * 100).toFixed(1)}%` : ''}</strong></div>
            <div className="metric"><span>Analogue</span><strong>{selected?.upstream_analysis?.agentdb.analogue ?? ''}</strong></div>
            <div className="metric"><span>Policy</span><strong>{selected ? 'authorized' : 'pending'}</strong></div>
            <p className="muted">Every displayed signal is returned by the running Rust API after middleware and upstream execution.</p>
          </aside>
        </section>
      )}

      {view === 'topology' && (
        <section>
          <div className="section-title">
            <div><div className="section-kicker">02 / Coordinate response</div><h2>Response topology</h2></div>
            <button className="secondary" onClick={() => setView('command')}>Revise incident</button>
          </div>
          {!selected ? <p className="empty">Create or retrieve a recovery plan to inspect its topology.</p> : (
            <div className="topology-grid">
              <article className="topology-card">
                <span>Ruflo plan</span>
                <h3>{selected.upstream_analysis?.ruflo.topology ?? 'unavailable'}</h3>
                <p>Swarm {selected.upstream_analysis?.ruflo.swarmId ?? ''}</p>
              </article>
              <article className="topology-card">
                <span>Sparse blast radius</span>
                <h3>{selected.upstream_analysis?.sublinear.converged ? 'Converged' : 'Review required'}</h3>
                <div className="pressure-bars">{pressure.map((value, index) => <i key={index} style={{height: `${Math.max(12, value * 300)}px`}} title={value.toFixed(3)} />)}</div>
              </article>
              <article className="topology-card wide">
                <span>Least-authority responder chain</span>
                <ol className="task-chain">{selected.tasks.map((task) => <li key={task.id}><b>{task.role}</b><small>{task.id}</small></li>)}</ol>
              </article>
              <article className="topology-card wide">
                <span>Selected responders</span>
                <div className="agents">{selected.agents.map((agent) => <div key={agent.id}><b>{agent.name}</b><small>{agent.roles.join(' · ')}</small><em>{(agent.score * 100).toFixed(0)} confidence</em></div>)}</div>
              </article>
            </div>
          )}
        </section>
      )}

      {view === 'evidence' && (
        <section>
          <div className="section-title">
            <div><div className="section-kicker">03 / Prove and recover</div><h2>Restart-durable evidence ledger</h2></div>
            <button className="secondary" onClick={() => void load()}>Refresh from RVF</button>
          </div>
          {!loaded && <p className="empty">Refresh to read the durable RVF audit through the typed API.</p>}
          {loaded && items.length === 0 && <p className="empty">No saved recovery plans yet.</p>}
          {items.length > 0 && (
            <div className="ledger">
              {items.map((plan) => (
                <button key={plan.id} onClick={() => setSelected(plan)} className={selected?.id === plan.id ? 'ledger-active' : ''}>
                  <span><b>{plan.objective.text}</b><small>{plan.correlation_id}</small></span>
                  <span><b>{plan.upstream_analysis?.dspy.priority ?? 'legacy'}</b><small>{plan.status}</small></span>
                  <span><b>{(plan.risk * 100).toFixed(1)}%</b><small>{plan.contract_hash.slice(0, 12)}</small></span>
                </button>
              ))}
            </div>
          )}
        </section>
      )}
    </main>
  );
}
createRoot(document.getElementById('root')!).render(<React.StrictMode><App /></React.StrictMode>);
