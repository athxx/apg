#!/bin/bash

cargo prisma generate # outputs client to src/prisma.rs
#cargo prisma db push  # outputs sqlite db to prisma/dev.db (specified in schema.prisma)
#cargo seed
