
## Install

```
npm install
npm run build  # to build .js from .ts
```

## Use
```
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