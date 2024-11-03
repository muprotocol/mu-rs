import { defineConfig } from 'vite';
import { muJsRollupPlugin } from 'mu';

export default defineConfig({
    plugins: [muJsRollupPlugin()],
    define: {
        global: 'window'
    }
});