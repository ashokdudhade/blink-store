import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'Blink-Store',
  tagline: 'In-memory key-value store. Single binary. Any language.',
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
      title: 'Blink-Store',
      logo: {
        alt: 'Blink-Store Logo',
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
          title: 'Documentation',
          items: [
            {label: 'Getting Started', to: '/docs/installation'},
            {label: 'Protocol Reference', to: '/docs/protocol'},
            {label: 'Language Guides', to: '/docs/guides/'},
            {label: 'Deployment', to: '/docs/deployment'},
          ],
        },
        {
          title: 'More',
          items: [
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
      copyright: `Copyright © ${new Date().getFullYear()} Blink-Store contributors.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ['bash', 'rust', 'go', 'python', 'powershell', 'yaml', 'toml'],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
