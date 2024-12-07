
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
| day 01 |       2.85 |       2.63 | 
| day 02 |       2.94 |       2.98 | 
| day 03 |       0.61 |       0.36 | 
| day 04 |      10.37 |      12.64 | 
| day 05 |     116.32 |      79.07 | 
| day 06 |       7.03 |       2.19 | 
| day 07 |     445.78 |     387.82 | 
