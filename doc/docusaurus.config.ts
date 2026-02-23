import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'Blink Store',
  tagline: 'Blazing-fast in-memory key-value store. Single binary. Any language.',
  favicon: 'img/favicon.ico',

  future: {
    v4: true,
  },

  url: 'https://ashokdudhade.github.io',
  baseUrl: '/blink-store/',

  organizationName: 'ashokdudhade',
  projectName: 'blink-store',

  onBrokenLinks: 'throw',

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  headTags: [
    {
      tagName: 'meta',
      attributes: {
        property: 'og:image',
        content: 'https://ashokdudhade.github.io/blink-store/img/logo.png',
      },
    },
  ],

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          editUrl:
            'https://github.com/ashokdudhade/blink-store/edit/main/doc/',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    colorMode: {
      defaultMode: 'light',
      respectPrefersColorScheme: true,
    },
    navbar: {
      title: 'Blink Store',
      logo: {
        alt: 'Blink Store Logo',
        src: 'img/logo.png',
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'docsSidebar',
          position: 'left',
          label: 'Docs',
        },
        {
          to: '/docs/benchmarks',
          position: 'left',
          label: 'Benchmarks',
        },
        {
          href: 'https://github.com/ashokdudhade/blink-store',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Getting Started',
          items: [
            {label: 'Introduction', to: '/docs/introduction'},
            {label: 'Installation', to: '/docs/installation'},
            {label: 'Protocol Reference', to: '/docs/protocol'},
          ],
        },
        {
          title: 'Guides',
          items: [
            {label: 'Python', to: '/docs/guides/python'},
            {label: 'Node.js', to: '/docs/guides/nodejs'},
            {label: 'Go', to: '/docs/guides/go'},
            {label: 'Rust', to: '/docs/guides/rust'},
          ],
        },
        {
          title: 'Resources',
          items: [
            {label: 'Deployment', to: '/docs/deployment'},
            {label: 'Benchmarks', to: '/docs/benchmarks'},
            {
              label: 'GitHub',
              href: 'https://github.com/ashokdudhade/blink-store',
            },
            {
              label: 'Releases',
              href: 'https://github.com/ashokdudhade/blink-store/releases',
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Blink Store contributors.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ['bash', 'rust', 'go', 'python', 'powershell', 'yaml', 'toml'],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
