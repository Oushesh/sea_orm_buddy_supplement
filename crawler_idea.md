The Content Discovery algorithm is what separates a basic "Save as Markdown" script from a high-quality LLM data pipeline. In Firecrawl, this logic is primarily about noise reduction and semantic weight.

If you look at the code in apps/api/src/scraper/, the algorithm follows a "greedy" approach: it assumes everything is junk until proven otherwise.

The "Discovery" Pipeline Logic
1. The Heavy Pruning (Hard-coded Negatives)
Before any "smart" logic happens, Firecrawl aggressively kills known noise. In Rust, you can do this using the ammonia crate or by iterating through a DOM tree with scraper.

Removed immediately: <script>, <style>, <link>, <meta>, <noscript>, <svg>.

The "Boilerplate" Kill-list: It targets common IDs and classes associated with non-content, like #footer, .nav-menu, .sidebar, .ad-container, and .cookie-banner.

2. Visual Filtering (The Headless Advantage)
This is where Firecrawl beats simple Python scrapers. If you use the playwright or puppeteer mode, the algorithm checks the computed style of elements:

Visibility Check: If an element has display: none, visibility: hidden, or an opacity of 0, it is deleted.

Size Heuristics: Elements with very small height/width (like 1px tracking pixels) are removed.

Positioning: Elements that are "off-screen" (negative CSS offsets) are often discarded.

3. Text Density Analysis (The "Magic")
This is the core heuristic used to find the "Main Content." The logic looks like this:

Link Density: It calculates the ratio of text-length to link-length. If a <div> has 500 characters but 450 of them are inside <a> tags, it’s flagged as a "Navigation Menu" or "Link Farm" and deprioritized.

Sibling Scoring: It looks for clusters of similar tags (like many <p> tags inside one <div>). The container with the highest concentration of text-heavy sibling tags is scored as the "Candidate Main."

Keyword Boosting: Containers holding keywords like "article," "post," "content," or "main" receive a score multiplier.

Implementing it in Rust
To replicate the "Text Density" logic in Rust, you would write a recursive function that walks the scraper::Html tree and assigns a score to each node.