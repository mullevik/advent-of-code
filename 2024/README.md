
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
| day 01 |       1.61 |       1.51 | 
| day 02 |       1.79 |       1.53 | 
| day 04 |       6.99 |       5.88 | 
