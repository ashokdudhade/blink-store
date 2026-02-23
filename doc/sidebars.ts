import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  docsSidebar: [
    'introduction',
    'installation',
    'protocol',
    {
      type: 'category',
      label: 'Language Guides',
      link: {type: 'doc', id: 'guides/index'},
      items: [
        'guides/python',
        'guides/nodejs',
        'guides/go',
        'guides/shell',
        'guides/rust',
        'guides/http-backend',
      ],
    },
    'deployment',
    'benchmarks',
  ],
};

export default sidebars;
