#!/bin/bash

if mdbook --version ; then
    mdbook build tutorial
else
    echo "please 'cargo install mdbook' and test again"
fi