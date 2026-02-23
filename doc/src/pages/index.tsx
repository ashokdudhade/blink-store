import React from 'react';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import CodeBlock from '@theme/CodeBlock';
import useBaseUrl from '@docusaurus/useBaseUrl';

function HeroSection() {
  const {siteConfig} = useDocusaurusContext();
  const logoUrl = useBaseUrl('/img/logo.png');
  return (
    <header className="hero--blink">
      <div className="container">
        <img src={logoUrl} alt="Blink Store" className="hero__logo" />
        <h1 className="hero__title">{siteConfig.title}</h1>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <div className="hero__actions">
          <Link className="button button--secondary button--lg" to="/docs/introduction">
            Get Started
          </Link>
          <Link className="button button--outline button--lg" to="/docs/installation">
            Install
          </Link>
        </div>
      </div>
    </header>
  );
}

const features = [
  {
    icon: '📦',
    title: 'Single Binary, Zero Config',
    description:
      'One curl command to install. One flag to start. No config files, no daemon setup, no package manager required.',
  },
  {
    icon: '🌐',
    title: 'Any Language',
    description:
      'Plain-text TCP protocol. Python, Node.js, Go, Rust, Shell — if it can open a socket, it works.',
  },
  {
    icon: '🧠',
    title: 'Smart Memory Management',
    description:
      'Set a byte limit with --memory-limit. Sampled eviction (like Redis) automatically reclaims space when full.',
  },
  {
    icon: '⚡',
    title: 'Blazing Fast',
    description:
      'Sub-50 µs p50 latency. 16K+ ops/sec on a single connection. Lock-free DashMap and async Tokio runtime.',
  },
  {
    icon: '🦀',
    title: 'Written in Rust',
    description:
      'No unsafe code. Result-based error handling — no panics in production paths. Structured logging via tracing.',
  },
  {
    icon: '🖥️',
    title: 'Cross-Platform',
    description:
      'Pre-built binaries for Linux, macOS, and Windows (x86_64 + ARM64). Docker images for linux/amd64 and linux/arm64.',
  },
];

function FeaturesSection() {
  return (
    <section className="features-section">
      <div className="container">
        <h2 className="features-section__title">Why Blink Store?</h2>
        <p className="features-section__desc">
          A lightweight cache that gets out of your way. No drivers, no SDKs, no infrastructure overhead.
        </p>
        <div className="features-grid">
          {features.map((f, i) => (
            <div key={i} className="feature-card">
              <span className="feature-card__icon">{f.icon}</span>
              <h3>{f.title}</h3>
              <p>{f.description}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

function ComparisonSection() {
  return (
    <section className="comparison-section">
      <div className="container">
        <h2>Blink Store vs. the alternatives</h2>
        <p>Blink Store fills the gap between in-process caches and full-featured distributed stores.</p>
        <table>
          <thead>
            <tr>
              <th></th>
              <th>Blink Store</th>
              <th>Redis / Valkey</th>
              <th>In-process cache</th>
            </tr>
          </thead>
          <tbody>
            <tr><td><strong>Setup</strong></td><td>Single binary, one flag</td><td>Package manager or Docker</td><td>Library import</td></tr>
            <tr><td><strong>Cross-language</strong></td><td>Any language via TCP</td><td>Any language via RESP</td><td>Same language only</td></tr>
            <tr><td><strong>Protocol</strong></td><td>5 text commands</td><td>400+ commands</td><td>Function calls</td></tr>
            <tr><td><strong>Persistence</strong></td><td>None (ephemeral)</td><td>RDB / AOF</td><td>None</td></tr>
            <tr><td><strong>Clustering</strong></td><td>Single node</td><td>Built-in</td><td>Single process</td></tr>
            <tr><td><strong>Memory control</strong></td><td>--memory-limit + eviction</td><td>maxmemory + policies</td><td>Manual</td></tr>
            <tr><td><strong>Dependencies</strong></td><td>None</td><td>libc, jemalloc</td><td>Language runtime</td></tr>
            <tr><td><strong>Best for</strong></td><td>Local/sidecar cache, CI, prototyping</td><td>Production distributed cache</td><td>Single-app hot path</td></tr>
          </tbody>
        </table>
      </div>
    </section>
  );
}

function QuickStartSection() {
  return (
    <section className="quickstart-section">
      <div className="container">
        <h2>Quick Start</h2>
        <p>Install and run in two commands — no Git clone, no Rust toolchain:</p>
        <CodeBlock language="bash">
{`# Install the latest release
curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/scripts/install-from-github.sh \\
  | bash -s -- latest ./bin

# Start the server
./bin/blink-store serve --tcp 127.0.0.1:8765`}
        </CodeBlock>
        <p style={{marginTop: '1.5rem'}}>Then from another terminal:</p>
        <CodeBlock language="bash">
{`echo "SET user alice" | nc 127.0.0.1 8765    # → OK
echo "GET user"       | nc 127.0.0.1 8765    # → VALUE YWxpY2U=
echo "USAGE"          | nc 127.0.0.1 8765    # → USAGE 9
echo "DELETE user"    | nc 127.0.0.1 8765    # → OK`}
        </CodeBlock>
        <div style={{marginTop: '2rem', display: 'flex', gap: '1rem', flexWrap: 'wrap'}}>
          <Link className="button button--primary button--lg" to="/docs/introduction">
            Read the Docs
          </Link>
          <Link className="button button--outline button--primary button--lg" to="/docs/benchmarks">
            View Benchmarks
          </Link>
        </div>
      </div>
    </section>
  );
}

export default function Home(): React.JSX.Element {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout title={siteConfig.title} description={siteConfig.tagline}>
      <HeroSection />
      <main>
        <FeaturesSection />
        <ComparisonSection />
        <QuickStartSection />
      </main>
    </Layout>
  );
}
