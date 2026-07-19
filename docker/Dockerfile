FROM izumiko/discuz:latest

LABEL maintainer="TSDM.net"
LABEL description="TSDM Pokemon Plugin for Discuz X5"

COPY plugin/ /plugin/
COPY template/ /template/
COPY migrations/ /migrations/
COPY docker/init.d/ /docker-entrypoint-initdb.d/
COPY docker/entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
CMD ["frankenphp", "run", "--config", "/etc/caddy/Caddyfile"]
