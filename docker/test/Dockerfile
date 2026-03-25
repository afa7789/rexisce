FROM prosody/prosody:latest

USER root

# Copy test config directly to the main config path (not conf.avail)
# and the startup script that creates test accounts.
COPY prosody.cfg.lua /etc/prosody/prosody.cfg.lua
COPY entrypoint.sh   /usr/local/bin/test-entrypoint.sh
RUN chmod +x /usr/local/bin/test-entrypoint.sh \
 && mkdir -p /var/run/prosody \
 && chown prosody:prosody /var/run/prosody \
 && mkdir -p /etc/prosody/certs \
 && openssl req -x509 -newkey rsa:2048 -keyout /etc/prosody/certs/localhost.key \
      -out /etc/prosody/certs/localhost.crt -days 365 -nodes \
      -subj "/CN=localhost" 2>/dev/null \
 && chown -R prosody:prosody /etc/prosody/certs

USER prosody

EXPOSE 5222

ENTRYPOINT ["/usr/local/bin/test-entrypoint.sh"]
