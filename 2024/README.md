
## Install

```
npm install
npm run build  # to build .js from .ts
```

## Use
```
npm run benchmark  # to print benchmark output
npm run test  # to run unit tests
```


## Google cloud function

Deploy AoC bot to Google Cloud Run
```
gcloud functions deploy aoc-bot \
--gen2 \
--runtime=nodejs22 \
--region=europe-central2 \
--entry-point=aocBotEntrypoint \
--trigger-topic=aoc-triggers
```

## AOC day results

| day    |    p1 (ms) |    p2 (ms) | 
| ------ | ---------- | ---------- | 
| day 01 |       1.58 |       1.66 | 
| day 02 |       1.86 |       1.57 | 
| day 03 |       0.35 |       0.28 | 
| day 04 |       7.13 |       6.29 | 
| day 05 |      59.51 |      53.79 | 
| day 06 |       7.66 |       2.05 | 
| day 07 |     445.78 |     387.82 | 
