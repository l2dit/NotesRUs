<template>
    <textarea name="Editor" ref="easyMDE"></textarea>
</template>
<script>
import EasyMDE from "easymde";
import { unified } from 'unified';
import rehypeStringify from 'rehype-stringify';
import remarkMath from 'remark-math';
import remarkParse from 'remark-parse';
import remarkRehype from 'remark-rehype';
import rehypeSanitize from 'rehype-sanitize';
import rehypeMathjax from 'rehype-mathjax';
import rehypePrettyCode from "rehype-pretty-code";


export default {
    mounted() {
        let mdeEditor = this.$refs.easyMDE;
        const easyMDE = new EasyMDE({ element: mdeEditor, previewRender: (plainText, preview) => markdownParse(plainText, preview) });
    },
};

async function markdownParse(plainText, preview) {

    preview.innerHTML = "Loading..."

    let markdownHTML = await unified()
        .use(remarkParse)
        .use(remarkRehype)
        .use(rehypeSanitize)
        .use(rehypeStringify)
        .use(rehypeMathjax)
        .use(remarkMath)
        .use(rehypePrettyCode, { theme: 'slack-dark' })
        .process(plainText)
        .then((html_md) => { preview.innerHTML = html_md.value; return html_md.value });

};
</script>
<style>
:root {
    --pageWidth: 750px;
    --mobileBreakpoint: 600px;
    --tabletBreakpoint: 1000px;
    --sidePanelWidth: 380px;
    --topSpacing: 6rem;
    --fullPageWidth: var(--pageWidth) + 2 * var(--sidePanelWidth);
    --boldWeight: 700;
    --semiBoldWeight: 600;
    --normalWeight: 400;
}

figure[data-rehype-pretty-code-figure] {
    margin: 0;
    position: relative;
    line-height: 1.6rem;
}

figure[data-rehype-pretty-code-figure]>[data-rehype-pretty-code-title] {
    font-size: 0.9rem;
    padding: 0.1rem 0.5rem;
    border: 1px solid lightgray;
    width: max-content;
    border-radius: 5px;
    margin-bottom: -0.5rem;
    color: darkgray;
}

figure[data-rehype-pretty-code-figure]>pre {
    padding: 0;
}

pre {
    font-family: var(--codeFont);
    padding: 0 0.5rem;
    border-radius: 5px;
    overflow-x: auto;
    border: 1px solid var(--lightgray);
    position: relative;
}

pre:has(> code.mermaid) {
    border: none;
}

pre>code {
    background: none;
    padding: 0;
    font-size: 0.85rem;
    counter-reset: line;
    counter-increment: line 0;
    display: grid;
    padding: 0.5rem 0;
    overflow-x: scroll;
}

pre>code [data-highlighted-chars] {
    background-color: var(--highlight);
    border-radius: 5px;
}

pre>code>[data-line] {
    padding: 0 0.25rem;
    box-sizing: border-box;
    border-left: 3px solid transparent;
}

pre>code>[data-line][data-highlighted-line] {
    background-color: var(--highlight);
    border-left: 3px solid var(--secondary);
}

pre>code>[data-line]::before {
    content: counter(line);
    counter-increment: line;
    width: 1rem;
    margin-right: 1rem;
    display: inline-block;
    text-align: right;
    color: rgba(115, 138, 148, 0.6);
}

pre>code[data-line-numbers-max-digits="2"]>[data-line]::before {
    width: 2rem;
}

pre>code[data-line-numbers-max-digits="3"]>[data-line]::before {
    width: 3rem;
}

code {
    font-size: 0.9em;
    color: var(--dark);
    font-family: var(--codeFont);
    border-radius: 5px;
    padding: 0.1rem 0.2rem;
    background: var(--lightgray);
}
</style>