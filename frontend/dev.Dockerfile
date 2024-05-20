FROM node:20

WORKDIR /app

ENV NODE_ENV=development

CMD yarn && yarn dev
