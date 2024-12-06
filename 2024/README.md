
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
| day 01 |       3.04 |       1.91 | 
| day 02 |       1.71 |       1.25 | 
| day 03 |       0.30 |       0.30 | 
| day 04 |       6.30 |       5.64 | 
| day 05 |      53.02 |      46.48 | 
| day 06 |       3.87 |       0.98 | 
