# Remote Access Configuration — Удалённый доступ к Neira

## Quick Start — Быстрый старт

### 1. Start Server with Remote Access — Запуск сервера с удалённым доступом

Windows:
```powershell
# Remote access already configured in run.bat (binds to 0.0.0.0:9090)
.\run.bat
```

Linux/macOS:
```bash
NEIRA_BIND_ADDR="0.0.0.0:9090" ./run.sh
```

### 2. Access Dashboard — Доступ к дашборду

Local machine — Локально:
```
http://localhost:9090/static/consciousness/
```

Remote access — Удалённый доступ:
```
http://YOUR_IP:9090/static/consciousness/
```

Replace `YOUR_IP` with your machine's IP address. Find it with:
```powershell
# Windows
ipconfig | Select-String "IPv4"

# Linux/macOS
ip addr show | grep inet
```

## Environment Variables — Переменные окружения

```bash
# Bind address (default in run.bat: 0.0.0.0:9090)
NEIRA_BIND_ADDR="0.0.0.0:9090"

# Enable OrganBuilder (required for organ creation)
ORGANS_BUILDER_ENABLED="true"

# Enable StemCellFactory
FACTORY_ADAPTER_ENABLED="true"

# Port override (optional, use NEIRA_BIND_ADDR instead)
PORT="9090"
```

## Security Considerations — Безопасность

⚠️ **WARNING — ПРЕДУПРЕЖДЕНИЕ**: By default, Neira allows all origins (CORS permissive mode) for development convenience. This is **NOT SECURE** for production.

### For Production — Для продакшена:

1. **Enable authentication** — Configure API keys or OAuth:
   ```toml
   # config/auth.toml
   [auth]
   enabled = true
   api_keys = ["your-secret-key-here"]
   ```

2. **Restrict CORS** — Modify `main.rs` to allow specific origins:
   ```rust
   .layer(
       CorsLayer::new()
           .allow_origin("https://your-domain.com".parse::<HeaderValue>().unwrap())
           .allow_methods([Method::GET, Method::POST])
           .allow_headers([CONTENT_TYPE, AUTHORIZATION])
   )
   ```

3. **Use HTTPS** — Set up reverse proxy with TLS:
   ```nginx
   server {
       listen 443 ssl;
       server_name your-domain.com;
       
       ssl_certificate /path/to/cert.pem;
       ssl_certificate_key /path/to/key.pem;
       
       location / {
           proxy_pass http://localhost:9090;
           proxy_set_header Host $host;
           proxy_set_header X-Real-IP $remote_addr;
       }
   }
   ```

4. **Firewall rules** — Restrict access by IP:
   ```bash
   # Linux iptables example
   iptables -A INPUT -p tcp --dport 3000 -s TRUSTED_IP -j ACCEPT
   iptables -A INPUT -p tcp --dport 3000 -j DROP
   ```

## Port Forwarding — Проброс портов

If your machine is behind NAT/router, configure port forwarding:

1. **Find local IP** (usually 192.168.x.x or 10.x.x.x)
2. **Access router admin panel** (usually http://192.168.1.1)
3. **Add port forwarding rule**:
   - External port: 3000
   - Internal IP: Your machine's local IP
   - Internal port: 3000
   - Protocol: TCP

4. **Find public IP**: https://whatismyipaddress.com/
5. **Access remotely**: `http://YOUR_PUBLIC_IP:9090/static/consciousness/`

⚠️ Note: Your public IP may change (dynamic IP). Consider using:
- DDNS service (No-IP, DuckDNS)
- VPN/tunnel service (Tailscale, Cloudflare Tunnel)

## Cloudflare Tunnel (Recommended for Remote Access)

Secure, free alternative to port forwarding:

```bash
# Install cloudflared
# Windows: Download from https://github.com/cloudflare/cloudflared/releases

# Create tunnel
cloudflared tunnel create neira-tunnel

# Route tunnel to local server
cloudflared tunnel route dns neira-tunnel neira.yourdomain.com

# Run tunnel
cloudflared tunnel run --url http://localhost:9090 neira-tunnel
```

Now access via: `https://neira.yourdomain.com/static/consciousness/`

## Testing Remote Access — Тестирование удалённого доступа

### From another device — С другого устройства:

1. **Test connectivity**:
   ```bash
   curl http://YOUR_IP:9090/health
   # Should return: {"status":"ok"}
   ```

2. **Check consciousness API**:
   ```bash
   curl http://YOUR_IP:9090/api/neira/consciousness/stats
   # Should return JSON with thoughts/biases/tasks counts
   ```

3. **Open dashboard** in browser:
   ```
   http://YOUR_IP:9090/static/consciousness/
   ```

### Troubleshooting — Решение проблем:

**Cannot connect remotely?**
- ✅ Check firewall: `netsh advfirewall show currentprofile` (Windows)
- ✅ Verify server listens on 0.0.0.0: `netstat -an | Select-String "3000"`
- ✅ Confirm no errors in Neira logs
- ✅ Try accessing from another device on same network first

**Dashboard loads but no data?**
- ✅ Check browser console (F12) for API errors
- ✅ Verify consciousness endpoints respond: `/api/neira/consciousness/stats`
- ✅ Ensure ORGANS_BUILDER_ENABLED=true in environment

**Slow performance?**
- ✅ Reduce auto-refresh interval in `dashboard.js` (default: 5 seconds)
- ✅ Check network latency: `ping YOUR_IP`
- ✅ Monitor server resources: CPU/memory usage

## API Endpoints — Эндпоинты API

See full API documentation: [docs/api/consciousness.md](./api/consciousness.md)

### Consciousness Endpoints:
- `GET /api/neira/consciousness/stats` — Get statistics (thoughts, biases, tasks)
- `POST /api/neira/consciousness/thought` — Record new thought trace
- `GET /api/neira/consciousness/personality` — Get current personality traits
- `POST /api/neira/consciousness/personality/snapshot` — Create personality snapshot
- `GET /api/neira/consciousness/growth-report?days=7` — Generate growth report

### Organ Builder Endpoints:
- `GET /organs` — List all organs
- `POST /organs/build` — Create new organ
- `POST /organs/grow/{organ_id}` — Advance organ to next stage

### System Endpoints:
- `GET /health` — Health check
- `GET /metrics` — Prometheus metrics
- `GET /api/neira/queues/status` — Queue status

## WebSocket Support (Future)

Planned for v2.0:
- Real-time thought stream: `ws://YOUR_IP:9090/ws/consciousness/thoughts`
- Organ state changes: `ws://YOUR_IP:9090/ws/organs/events`
- Live metrics: `ws://YOUR_IP:9090/ws/metrics`

Track progress: [ROADMAP.md](../roadmap.md)

---

**Next steps**:
- [API Documentation](./api/consciousness.md) — Detailed endpoint reference
- [Security Guide](./security.md) — Production security checklist
- [Deployment Guide](./deployment.md) — Docker, systemd, cloud deployment
