import re
LM=r'^\s*(?:[-*+]|\d+[.)])?\s*'
def label_line(s):
    t=s.strip()
    return bool(re.match(LM+r'(\*\*[^*]+\*\*|\*[^*\s][^*]*\*|__[^_]+__)\s*:?\s*$',t)) or t.endswith(':**') or t.endswith('**:') or (t.endswith(':') and len(t)<60)
def starts_item(s):
    t=s.lstrip()
    return bool(re.match(r'(\*\*[^*]{1,60}?\*\*\s*:|\*\*[^*]{1,60}?:\*\*|\*[A-Z][^*]{0,40}:\*|\*\[|\(([ivx]+|[a-z]|\d+)\)\s|\$\$)',t))
def indented_block(prev,nxt):
    ip=len(prev)-len(prev.lstrip()); inx=len(nxt)-len(nxt.lstrip())
    return inx>=ip+4
def mid_sentence(prev):
    return bool(re.search(r'[A-Za-z0-9,(—–-]\s*$',prev))
def numbered(nxt):
    # CommonMark: an ordered list not starting at 1 cannot interrupt a paragraph,
    # so "4. ..." here renders as paragraph text; the author meant a new line.
    return bool(re.match(r'\s*\d+[.)]\s',nxt))
def bq(s):
    return re.sub(r'^\s*(>\s?)+','',s)
def boundary(prev,nxt):
    prev,nxt=bq(prev),bq(nxt)
    return figure_or_ial(prev,nxt) or label_line(prev) or numbered(nxt) or (starts_item(nxt) and not mid_sentence(prev))
def figure_or_ial(prev,nxt):
    # House figure convention: an embed line, then its Kramdown IAL line.
    return prev.lstrip().startswith('![[') or nxt.lstrip().startswith('{#') or nxt.lstrip().startswith('{:')
def labelled_item(prev,nxt):
    # "1. **Label** — gloss." then an indented explanation under it.
    return bool(re.match(r'\s*(\d+[.)]|[-*+])\s+\*\*', prev)) and len(nxt)-len(nxt.lstrip()) >= 2
def want_keep(prev,nxt):
    prev,nxt=bq(prev),bq(nxt)
    return figure_or_ial(prev,nxt) or labelled_item(prev,nxt) or label_line(prev) or numbered(nxt) or (starts_item(nxt) and not mid_sentence(prev)) or indented_block(prev,nxt)
