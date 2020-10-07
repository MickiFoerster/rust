#!/bin/bash

awk -i inplace '
  /authors/ { print "authors = [\"micki <github@com-science.de>\"]"; }
! /authors/          { print; }
' $(find -name Cargo.toml)

