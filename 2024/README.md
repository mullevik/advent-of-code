
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
| day 01 |       1.46 |       1.51 | 
| day 02 |       1.96 |       1.46 | 
| day 03 |       0.40 |       0.28 | 
| day 04 |       7.16 |       8.74 | 
| day 05 |      55.21 |      48.27 | 
| day 06 |       7.91 |       2.32 | 
| day 07 |     253.63 |     225.99 | 
