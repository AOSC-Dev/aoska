import eslint from '@eslint/js';
import eslintPluginStylistic from '@stylistic/eslint-plugin';
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
      stylistic: eslintPluginStylistic,
      vue: eslintPluginVue
    },
    rules: {
      'stylistic/semi': 2,
      'stylistic/arrow-spacing': 2,
      'stylistic/eol-last': ["error", "always"],
      'stylistic/indent': ["error", 2],
      'vue/require-default-prop': 0
    },
  },
  eslintConfigPrettier
);
