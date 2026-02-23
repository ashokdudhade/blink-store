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
    <header className="hero hero--primary">
      <div className="container" style={{textAlign: 'center'}}>
        <img
          src={logoUrl}
          alt="Blink-Store Logo"
          style={{width: 140, height: 140, marginBottom: '1rem'}}
        />
        <h1 className="hero__title">{siteConfig.title}</h1>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <div style={{display: 'flex', gap: '1rem', marginTop: '1.5rem', justifyContent: 'center'}}>
          <Link className="button button--secondary button--lg" to="/docs/introduction">
            Get Started
          </Link>
          <Link
            className="button button--outline button--lg"
            style={{color: 'white', borderColor: 'white'}}
            to="/docs/installation">
            Install
          </Link>
        </div>
      </div>
    </header>
  );
}

function FeaturesSection() {
  const features = [
    {
      title: 'Single Binary, Zero Config',
      description:
        'One curl command to install. One flag to start. No config files, no daemon setup, no package manager.',
    },
    {
      title: 'Any Language',
      description:
        'Plain-text TCP protocol. If your language can open a socket and send a line of text, it can use Blink-Store.',
    },
    {
      title: 'Built-in Memory Management',
      description:
        'Set a byte limit with --memory-limit. Automatic LRU eviction when the limit is reached.',
    },
    {
      title: 'Written in Rust',
      description:
        'No unsafe code. Result-based error handling throughout. Structured logging via tracing.',
    },
    {
      title: 'Concurrent by Design',
      description:
        'DashMap for lock-free concurrent reads. Tokio async runtime. Each connection is a lightweight task.',
    },
    {
      title: 'Cross-Platform',
      description:
        'Pre-built binaries for Linux, macOS, and Windows (x86_64 and ARM64). Also runs in Docker.',
    },
  ];

  return (
    <section style={{padding: '3rem 0'}}>
      <div className="container">
        <div className="features-grid">
          {features.map((f, i) => (
            <div key={i} className="feature-card">
              <h3>{f.title}</h3>
              <p>{f.description}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

function QuickStartSection() {
  return (
    <section style={{padding: '2rem 0 4rem'}}>
      <div className="container">
        <h2>Quick Start</h2>
        <p>Install and run in two commands — no Git clone, no Rust toolchain:</p>
        <CodeBlock language="bash">
{`curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/scripts/install-from-github.sh \\
  | bash -s -- latest ./bin

./bin/blink-store serve --tcp 127.0.0.1:8765`}
        </CodeBlock>
        <p style={{marginTop: '1.5rem'}}>Then from another terminal:</p>
        <CodeBlock language="bash">
{`echo "SET user alice" | nc 127.0.0.1 8765    # → OK
echo "GET user"       | nc 127.0.0.1 8765    # → VALUE YWxpY2U=
echo "USAGE"          | nc 127.0.0.1 8765    # → USAGE 9
echo "DELETE user"    | nc 127.0.0.1 8765    # → OK`}
        </CodeBlock>
        <div style={{marginTop: '2rem'}}>
          <Link className="button button--primary button--lg" to="/docs/introduction">
            Read the Docs →
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
        <QuickStartSection />
      </main>
    </Layout>
  );
}
