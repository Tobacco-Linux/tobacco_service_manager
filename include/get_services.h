#ifndef SYSTEMD_SERVICES_H
#define SYSTEMD_SERVICES_H

#include <stddef.h>

/**
 * Fetches all systemd service names via D-Bus.
 *
 * @param count Output parameter for number of services returned
 * @return Array of strings (must be freed by caller), NULL on error
 */
char **get_systemd_services(size_t *count);

#endif /* SYSTEMD_SERVICES_H */
