#!/bin/bash

function one_more()
{
    sleep 5;
}

function create_child()
{
    one_more &
    one_more &
    sleep 5;
}

echo $$
sleep 10

create_child &
pid1=$!
create_child &
pid2=$!

wait $pid1
wait $pid2
