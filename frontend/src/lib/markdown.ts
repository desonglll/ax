import DOMPurify from "dompurify";
import MarkdownIt from "markdown-it";

const md = new MarkdownIt({ html: false, linkify: true, breaks: true, typographer: false });

// External links open in a new tab; in-app links (attachments) stay put.
const defaultLink = md.renderer.rules.link_open || ((tokens, idx, options, _env, self) => self.renderToken(tokens, idx, options));
md.renderer.rules.link_open = (tokens, idx, options, env, self) => {
  const href = String(tokens[idx].attrGet("href") ?? "");
  if (/^https?:\/\//i.test(href)) {
    tokens[idx].attrSet("target", "_blank");
    tokens[idx].attrSet("rel", "noopener noreferrer");
  }
  return defaultLink(tokens, idx, options, env, self);
};

md.renderer.rules.image = (tokens, idx, options, env, self) => {
  tokens[idx].attrSet("loading", "lazy");
  return self.renderToken(tokens, idx, options);
};

/** Markdown → sanitized HTML. */
export const renderMarkdown = (source: string) =>
  DOMPurify.sanitize(md.render(source), { ADD_ATTR: ["target", "loading"] });

/** Plain-text excerpt for previews and titles. */
export const excerpt = (source: string, max = 140) => {
  const text = source
    .replace(/!\[[^\]]*\]\([^)]*\)/g, "")
    .replace(/\[([^\]]*)\]\([^)]*\)/g, "$1")
    .replace(/[#>*_`~-]+/g, " ")
    .replace(/\s+/g, " ")
    .trim();
  return text.length > max ? `${text.slice(0, max)}…` : text;
};

/** True when the content embeds the file as an image. */
export const embedsImage = (source: string, fileId: string) =>
  new RegExp(`!\\[[^\\]]*\\]\\([^)]*/files/${fileId}/`).test(source);
