
# Org Parser

Org Mode parser that outputs JSON.

## Usage

    cargo run examples/test.org examples/output.json

## Example

### Input file

```org
#+TODO: TODO(t) LOW(l) CRITICAL(c) | DONE(d)
* TODO Heading 1
** TODO Heading 2
*** DONE Heading 3-1

Content:
 * Some line
 * Another

*** Heading 3-2
*** CRITICAL Heading 3-3

* DONE New H1 header

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
            "TODO",
            "LOW",
            "CRITICAL"
          ]
    },
        {
          "DONE": [
            "DONE"
          ]
        }
      ]
    }
  ],
  "headers": [
    {
      "index": 0,
      "data": {
        "level": 1,
        "title": "Heading 1",
        "text": [],
        "state": "TODO"
      },
      "parent": null,
      "previous": null,
      "next": null,
      "children": [
        {
          "index": 1
        }
      ]
    },
    {
      "index": 1,
      "data": {
        "level": 2,
        "title": "Heading 2",
        "text": [],
        "state": "TODO"
      },
      "parent": 0,
      "previous": null,
      "next": null,
      "children": [
        {
          "index": 2
        },
        {
          "index": 3
        },
        {
          "index": 4
        }
      ]
    },
    {
      "index": 2,
      "data": {
        "level": 3,
        "title": "Heading 3-1",
        "text": [
          "",
          "Content:",
          " * Some line",
          " * Another",
          ""
        ],
        "state": "DONE"
      },
      "parent": 1,
      "previous": null,
      "next": null,
      "children": []
    },
    {
      "index": 3,
      "data": {
        "level": 3,
        "title": "Heading 3-2",
        "text": [],
        "state": null
      },
      "parent": 1,
      "previous": 2,
      "next": 2,
      "children": []
    },
    {
      "index": 4,
      "data": {
        "level": 3,
        "title": "Heading 3-3",
        "text": [
          ""
        ],
        "state": "CRITICAL"
      },
      "parent": 1,
      "previous": 3,
      "next": 3,
      "children": []
    },
    {
      "index": 5,
      "data": {
        "level": 1,
        "title": "New H1 header",
        "text": [
          "",
          "Some text"
        ],
        "state": "DONE"
      },
      "parent": null,
      "previous": 0,
      "next": 0,
      "children": []
    }
  ]
}
```