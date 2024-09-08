#!/bin/bash

command="rsync -avhzP rsync://ftp.ensembl.org/ensembl/pub/current_gtf/"

max_jobs=16  # Adjust this number based on your system's capabilities

for letter in {a..z}; do
    while [ $(jobs -r | wc -l) -ge $max_jobs ]; do
        sleep 1
    done
    $command${letter}*/*2.gtf.gz . &
done

wait

