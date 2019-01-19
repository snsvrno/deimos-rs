# deimos

A pure rust lua interpreter with the intent to evaluate and analyze lua code.

## Project Layout

### deimos-core

The main library, contains everything needed to integrate lua or deimos into your project.

### deimos

The stand-alone CLI app that can be run like a lua interpreter. Allows you to lint and run Lua code without lua installed.

### lua-test

Should only be used for _deimos_ development. Tests and helper functions to make sure that _deimos_ adheres to the Lua 5.1 spec.

## Current Features

- none

## Resources
- https://ruslanspivak.com/lsbasi-part1/
- https://michael-f-bryan.github.io/static-analyser-in-rust/book/parse/ast.html
- https://en.wikipedia.org/wiki/Abstract_syntax_tree
