# PNGme

- Implementation of [PNGme](https://jrdngr.github.io/pngme_book/introduction.html)
- [PNG file structure specification](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)

- There are four commands to run this project with:

  - ## encode:

    ```sh
    > cargo run encode PATH_TO_PNG_FILE FOUR_LETTER_WORD [OPTIONAL_MESSAGE_TO_ENCODE]
    ```

    - example

    ```sh
    > cargo run encode ./my_image.png mine "stage blood is not enough"
    ```

  - ## decode:

    ```sh
    > cargo run decode PATH_TO_PNG_FILE FOUR_LETTER_WORD
    ```

    - example

    ```sh
    > cargo run decode ./my_image.png mine
    ```

  - ## remove:

    ```sh
    > cargo run remove PATH_TO_PNG_FILE FOUR_LETTER_WORD
    ```

    - example

    ```sh
    > cargo run remove ./my_image.png mine
    ```

  - ## print:

    ```sh
    > cargo run print PATH_TO_PNG_FILE
    ```

    - example

    ```sh
    > cargo run print ./my_image.png
    ```
