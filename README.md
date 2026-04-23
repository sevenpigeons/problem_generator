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
- [ ] Write question parser, with number generator for questions ( variables encoded in question text with `{{ }}` potentially?)
- [ ] Port axum to run on [shuttle.dev](https://www.shuttle.dev/)
- [ ] Write a front-end page with a basic form that would open a new tab with the pdf requested on [my website](https://tangled.org/sevenpigeons.ca/sevenpigeons.ca)

## Fonts

I am not sure about distributing Roboto font for now, please add 

- `Roboto-Bold.ttf`
- `Roboto-BoldItalic.ttf`
- `Roboto-Italic.ttf`
- `Roboto-Regular.ttf`

in the `fonts` folder, you get get it from [google fonts](fonts.google.com/specimen/Roboto)
