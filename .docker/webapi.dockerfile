FROM debian:buster-slim as runtime
ARG APP=/usr/src/app

ENV APP_PORT=5000
ENV SECRET_KEY=my-secret-key

HEALTHCHECK --interval=5m --timeout=3s \
  CMD curl -f http://localhost/health || exit 1

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY ./release/trenako-web-api ${APP}/trenako-web-api
RUN touch ${APP}/application.yml

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./trenako-web-api"]