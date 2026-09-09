import * as path from "node:path";
import { defineConfig } from "@rspress/core";

import katex from "rspress-plugin-katex";
import viz from "rspress-plugin-viz";
import { pluginLlms } from "@rspress/plugin-llms";

export default defineConfig({
    root: path.join(__dirname, "src"),
    title: "Waveless",
    plugins: [katex(), viz(), pluginLlms()],
    globalStyles: path.join(__dirname, "theme/main.css"),
    themeConfig: {
        lastUpdated: {
            author: true,
        },
        editLink: {
            docRepoBaseUrl:
                "https://github.com/nv0skar/Waveless/tree/main/docs/src",
        },
        socialLinks: [
            {
                icon: "github",
                mode: "link",
                content: "https://github.com/nv0skar/Waveless",
            },
        ],
        llmsUI: true,
    },
});
