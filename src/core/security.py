"""Simple token/role based access control utilities."""

from __future__ import annotations

from typing import Dict, Set

# Mapping of API tokens to roles. In a real system these would be stored securely
# and not hard coded. For the purposes of unit tests and examples we keep a very
# small in memory mapping.
API_TOKENS: Dict[str, str] = {
    "public": "user",
    "admin-token": "admin",
}

# Roles to permissions mapping. Permissions are represented by arbitrary string
# identifiers. Components can check for the presence of a permission before
# performing sensitive operations.
ROLE_PERMISSIONS: Dict[str, Set[str]] = {
    "user": {"event.publish", "event.subscribe", "external.search"},
    "admin": {"event.publish", "event.subscribe", "external.search"},
}


def get_role(token: str | None) -> str | None:
    """Return the role associated with ``token`` or ``None`` if unknown."""

    if token is None:
        return None
    return API_TOKENS.get(token)


def has_permission(token: str, permission: str) -> bool:
    """Check whether ``token`` grants ``permission``."""

    role = get_role(token)
    if role is None:
        return False
    return permission in ROLE_PERMISSIONS.get(role, set())


def require_permission(token: str, permission: str) -> None:
    """Raise ``PermissionError`` if ``token`` lacks ``permission``."""

    if not has_permission(token, permission):
        raise PermissionError(f"Token does not have permission: {permission}")


__all__ = [
    "API_TOKENS",
    "ROLE_PERMISSIONS",
    "get_role",
    "has_permission",
    "require_permission",
]
