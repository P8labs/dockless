import adapter from "@sveltejs/adapter-static";
import { mdsvex } from "mdsvex";
import remarkGfm from "remark-gfm";
import remarkBreaks from "remark-breaks";
import remarkSmartypants from "remark-smartypants";
import rehypeSlug from "rehype-slug";
import rehypePrettyCode from "rehype-pretty-code";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapter({
      precompress: true,
    }),
    prerender: {
      entries: ["*"],
    },

    output: {
      bundleStrategy: "inline",
    },
  },

  extensions: [".md", ".svelte", ".svx"],
  preprocess: [
    mdsvex({
      extensions: [".md", ".svx"],

      remarkPlugins: [remarkGfm, remarkBreaks, remarkSmartypants],
      highlight: true,
      rehypePlugins: [
        rehypeSlug,
        [
          rehypePrettyCode,
          {
            theme: "rose-pine",
            keepBackground: false,
          },
        ],
      ],
    }),
  ],
};

export default config;
