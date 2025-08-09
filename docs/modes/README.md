# Modes

Neira exposes distinct modes that share a common interface. Every mode can be
started and stopped and may hand control to another mode when its task is
complete.

## Relationships

- **Tutorial mode** walks new users through initial setup before handing
  control to other modes.
- **Resource manager** handles local resources. Data prepared here is
  available to any subsequent mode.

All modes can access shared services such as the event bus and configuration
system, enabling smooth transitions between them.
