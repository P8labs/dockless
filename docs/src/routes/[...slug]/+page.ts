import { error } from "@sveltejs/kit";

export async function load({ params }) {
  try {
    const path = `../../content/${params.slug}.md`;
    const pages = import.meta.glob(`../../content/**/*.md`);

    const post: any = await pages[path]();

    return {
      content: post.default,
      meta: post.metadata,
    };
  } catch (e: any) {
    error(404, `Could not find ${params.slug}`);
  }
}
