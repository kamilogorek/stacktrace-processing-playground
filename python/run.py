from symbolic import SmCache

LINES_OF_CONTEXT = 5

def discover_sourcemap(content):
    parsed_body = content.split("\n")

    if len(parsed_body) > 10:
        possibilities = parsed_body[:5] + parsed_body[-5:]
    else:
        possibilities = parsed_body

    for line in possibilities:
        if line[:21] in ("//# sourceMappingURL=", "//@ sourceMappingURL="):
            return line[21:].rstrip()

    raise ValueError('missing sourcemap url')

# def trim_line(line, column=0):
#     """
#     Trims a line down to a goal of 140 characters, with a little
#     wiggle room to be sensible and tries to trim around the given
#     `column`. So it tries to extract 60 characters before and after
#     the provided `column` and yield a better context.
#     """
#     line = line.strip("\n")
#     ll = len(line)
#     if ll <= 150:
#         return line
#     if column > ll:
#         column = ll
#     start = max(column - 60, 0)
#     # Round down if it brings us close to the edge
#     if start < 5:
#         start = 0
#     end = min(start + 140, ll)
#     # Round up to the end if it's close
#     if end > ll - 5:
#         end = ll
#     # If we are bumped all the way to the end,
#     # make sure we still get a full 140 characters in the line
#     if end == ll:
#         start = max(end - 140, 0)
#     line = line[start:end]
#     if end < ll:
#         # we've snipped from the end
#         line += " {snip}"
#     if start > 0:
#         # we've snipped from the beginning
#         line = "{snip} " + line
#     return line


# def get_source_context(source, lineno, colno, context=LINES_OF_CONTEXT):
#     if not source:
#         return None, None, None

#     # lineno's in JS are 1-indexed
#     # just in case. sometimes math is hard
#     if lineno > 0:
#         lineno -= 1

#     lower_bound = max(0, lineno - context)
#     upper_bound = min(lineno + 1 + context, len(source))

#     try:
#         pre_context = [trim_line(x) for x in source[lower_bound:lineno]]
#     except IndexError:
#         pre_context = []

#     try:
#         context_line = trim_line(source[lineno], colno)
#     except IndexError:
#         context_line = ""

#     try:
#         post_context = [trim_line(x) for x in source[(lineno + 1) : upper_bound]]
#     except IndexError:
#         post_context = []

#     return pre_context or None, context_line, post_context or None


def process_frame(frame):
    source_file = open("fixtures/{path}".format(path = frame['abs_path']), "r")
    source_content = source_file.read()
    source_file.close()

    sourcemap_ref_url = discover_sourcemap(source_content)

    sourcemap_file = open("fixtures/{path}".format(path = sourcemap_ref_url), "r")
    sourcemap_content = sourcemap_file.read()
    sourcemap_file.close()

    cache = SmCache.from_bytes(source_content, sourcemap_content)
    token = cache.lookup(frame['lineno'], frame['colno'])

    frame['lineno'] = token.line
    frame['colno'] = token.col
    frame['abs_path'] = token.src
    frame['function'] = token.function_name


frames = [
    {
        'abs_path': "sentry.js",
        'function': "HTMLButtonElement.i",
        'lineno': 1,
        'colno': 51239,
    },
    {
        'abs_path': "sentry.js",
        'function': "HTMLButtonElement.ln",
        'lineno': 1,
        'colno': 60099,
    },
    {
        'abs_path': "sentry.js",
        'function': "dn",
        'lineno': 1,
        'colno': 58944,
    },
    {
        'abs_path': "sentry.js",
        'lineno': 1,
        'colno': 58931,
    },
]

for frame in frames:
    process_frame(frame)

for frame in frames:
    print(frame)
