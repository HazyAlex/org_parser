
# Org Parser

Org Mode parser that outputs JSON.

## Usage

    cargo run examples/test.org examples/output.json

## Example

### Input file

```org
#+TODO: TODO(t) LOW(l) CRITICAL(c) | DONE(d)
* Heading 1
** Heading 2
*** Heading 3-1

Content:
 * Some line
 * Another

*** Heading 3-2

* New H1 header

Some text
```

### Output file

```json
{
  "options": [
    {
      "SEQ_TODO": [
        {
          "TODO": [
            "TODO(t)",
            "LOW(l)",
            "CRITICAL(c)"
          ]
        },
        {
          "DONE": [
            "DONE(d)"
          ]
        }
      ]
    }
  ],
  "headers": [
    {
      "level": 1,
      "title": "Heading 1",
      "text": []
    },
    {
      "level": 2,
      "title": "Heading 2",
      "text": []
    },
    {
      "level": 3,
      "title": "Heading 3-1",
      "text": [
        "",
        "Content:",
        " * Some line",
        " * Another",
        ""
      ]
    },
    {
      "level": 3,
      "title": "Heading 3-2",
      "text": [
        ""
      ]
    },
    {
      "level": 1,
      "title": "New H1 header",
      "text": [
        "",
        "Some text"
      ]
    }
  ]
}
```