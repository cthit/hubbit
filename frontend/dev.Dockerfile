FROM node:16

WORKDIR /app

ENV NODE_ENV=development

CMD yarn && yarn dev
