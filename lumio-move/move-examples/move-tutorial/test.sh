#!/bin/sh

LUMIO=lumio

COMPILED="\
  step_1/basic_coin\
  step_2/basic_coin\
  step_2_sol/basic_coin\
  step_4/basic_coin\
  step_5/basic_coin\
  step_5_sol/basic_coin\
  step_6/basic_coin\
  step_7/basic_coin\
  step_8/basic_coin\
  step_8_sol/basic_coin\
"

TESTED="\
  step_2/basic_coin\
  step_2_sol/basic_coin\
  step_4/basic_coin\
  step_5/basic_coin\
  step_5_sol/basic_coin\
  step_6/basic_coin\
  step_7/basic_coin\
  step_8/basic_coin\
  step_8_sol/basic_coin\
"


for compiled in $COMPILED
do
  (
    cd $compiled
    $LUMIO move compile
  )
done

for tested in $TESTED
do
  (
    cd $tested
    $LUMIO move test
  )
done
