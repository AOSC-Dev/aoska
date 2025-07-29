import eslint from '@eslint/js';
import stylisticJs from '@stylistic/eslint-plugin-js';
import eslintConfigPrettier from 'eslint-config-prettier';
import eslintPluginVue from 'eslint-plugin-vue';
import globals from 'globals';
import typescriptEslint from 'typescript-eslint';

export default typescriptEslint.config(
  { ignores: ['**/*.d.ts', '**/coverage', '**/dist'] },
  {
    extends: [
      eslint.configs.recommended,
      ...typescriptEslint.configs.recommended,
      ...eslintPluginVue.configs[('flat/base', 'flat/essential', 'flat/strongly-recommended')],
    ],
    files: ['**/*.{ts,vue}'],
    languageOptions: {
      ecmaVersion: 'latest',
      sourceType: 'module',
      globals: globals.browser,
      parserOptions: {
        parser: typescriptEslint.parser,
      },
    },
    plugins: {
      // eslint将风格(stylistic)类的监控都转移到了另一个插件中
      // 理论上来说，只要运行prettier便无需stylisticJs提醒，但不是所有人都会在上传前运行prettier,所以我们保留stylisticJs
      myJs: stylisticJs,
      vue: eslintPluginVue
    },
    rules: {
      'myJs/semi': 2,
      'myJs/arrow-spacing': 2,
      'vue/require-default-prop': 0
    },
  },
  eslintConfigPrettier
);
