Interactive quizzes in a gitbook
==============

With this plugin, a book can contain interactive quizzes.


## How to use it?

You can use install it via **NPM**:

```
$ npm install gitbook-plugin-quizzes
```

And use it for your book with:

```
$ gitbook build ./ --plugins=quizzes
```

:warning: Don't forget to add it to your package.json and book.json.

## Quizzes format

Quizzes need to start and finish with a separation bar (```---``` or ```***```).

    ---

    Here's a quiz about Gitbook

    |                  | Good | Bad |
    | ---------------- | ---- | --- |
    | What is Gitbook? | (x)  | ( ) |

    > Gitbook is good

    What does Gitbook support?
    - [x] Table-based questions with radio buttons
    - [x] Table-based questions with checkboxes
    - [ ] Telepathy
    - [x] List-based questions with checkboxes
    - [x] List-based questions with radio buttons
    - [ ] Moon-on-a-stick

    > Gitbook supports table and list based quiz questions using either radio buttons or checkboxes.
    >
    > Gitbook is not telepathic and does not give you the moon on a stick.

    ---

