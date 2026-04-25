# Problem set generator

API to generate random problem sets by subject.
WIP for now.


## Format

GET request that will return a `application/pdf`, with generated questions, and page of answers.

Path would be `https://$url/<Subject>/<Theme>/<Option<Pages>>`

- `Subject` being Math or Physics,
- `Theme` breakdown by subject, representing a single chapter in Mechanics by taylor or
Multivariable Calculus by Stewart.
- `Pages` optional how many pages you would like, default being 1.
- On error, a string with api documentation would be returned? (openapi generated docs would be nice)

## Goals
- [ ] Create handler function to extract problem request parameters
- [ ] Write a question bank by chapter (separate text files)
- [x] Write question text parser, 
- [ ] Write a number generator for questions ( variables encoded in question text with `{{ }}` potentially?)
- [ ] Port axum to run on [shuttle.dev](https://www.shuttle.dev/)
- [ ] Write a front-end page with a basic form that would open a new tab with the pdf requested on [my website](https://tangled.org/sevenpigeons.ca/sevenpigeons.ca)

## Questions 

Questions are represented by a `struct` with it's subject, theme, and the text of the question.

To parse from string (and text file), first line is the Subject, second Theme, and everything afterwards the text of the question.

## Fonts

~I am not sure about distributing Roboto font for now, please add~
I read the license yes I can, the Roboto font is under the Open Font License.

In the `fonts` folder, please find the following ttf files:

- `Roboto-Bold.ttf`
- `Roboto-BoldItalic.ttf`
- `Roboto-Italic.ttf`
- `Roboto-Regular.ttf`

If you wish to change the font, you will have to provide `Bold, BoldItalic, Italic`,and `Regular` ttf files of the font you'd like to use.
