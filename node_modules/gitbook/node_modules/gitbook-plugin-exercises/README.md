Interactive exercices in a gitbook
==============

With this plugin, a book can contain interactive exercises (currently only in Javascript). An exercise is a code challenge provided to the reader, who is given a code editor to write a solution which is checked against the book author's validation code.


## How to use it?

You can use install it via **NPM**:

```
$ npm install gitbook-plugin-exercies
```

And use it for your book with:

```
$ gitbook build ./ --plugins=exercises
```

:warning: Don't forget to add it to your package.json and book.json.


## Exercises format

An exercise is defined by 4 simple parts:

* Exercise **Message**/Goals (in markdown/text)
* **Initial** code to show to the user, providing a starting point
* **Solution** code, being a correct solution to the exercise
* **Validation** code that tests the correctness of the user's input

Exercises need to start and finish with a separation bar (```---``` or ```***```). It should contain 3 code elements (**base**, **solution** and **validation**). It can contain a 4th element that provides **context** code (functions, imports of libraries, etc which shouldn't be displayed to the user).

    ---

    Define a variable `x` equal to 10.

    ```js
    var x =
    ```

    ```js
    var x = 10;
    ```

    ```js
    assert(x == 10);
    ```

    ```js
    // This is context code available everywhere
    // The user will be able to call magicFunc in his code
    function magicFunc() {
        return 3;
    }
    ```

    ---

