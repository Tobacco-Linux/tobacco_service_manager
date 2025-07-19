#include <dbus/dbus.h>
#include <stdlib.h>
#include <string.h>
#include <stddef.h>

char **get_systemd_services(size_t *count) {
    DBusError err;
    DBusConnection *conn = NULL;
    DBusMessage *msg = NULL, *reply = NULL;
    char **services = NULL;
    *count = 0;

    dbus_error_init(&err);
    if (!(conn = dbus_bus_get(DBUS_BUS_SYSTEM, &err))) goto cleanup;
    if (!(msg = dbus_message_new_method_call("org.freedesktop.systemd1",
        "/org/freedesktop/systemd1", "org.freedesktop.systemd1.Manager",
        "ListUnits"))) goto cleanup;
    if (!(reply = dbus_connection_send_with_reply_and_block(conn, msg, -1, &err))) goto cleanup;

    DBusMessageIter iter, array_iter;
    if (!dbus_message_iter_init(reply, &iter)) goto success;
    if (dbus_message_iter_get_arg_type(&iter) != DBUS_TYPE_ARRAY) goto success;

    dbus_message_iter_recurse(&iter, &array_iter);
    while (dbus_message_iter_get_arg_type(&array_iter) == DBUS_TYPE_STRUCT) {
        DBusMessageIter struct_iter;
        const char *unit_name;
        dbus_message_iter_recurse(&array_iter, &struct_iter);
        dbus_message_iter_get_basic(&struct_iter, &unit_name);
        size_t len = strlen(unit_name);
        if (len > 8 && !strcmp(unit_name + len - 8, ".service")) (*count)++;
        dbus_message_iter_next(&array_iter);
    }

    if (!*count) goto success;
    if (!(services = malloc(*count * sizeof(char*)))) goto cleanup;

    dbus_message_iter_init(reply, &iter);
    dbus_message_iter_recurse(&iter, &array_iter);
    size_t idx = 0;
    while (dbus_message_iter_get_arg_type(&array_iter) == DBUS_TYPE_STRUCT) {
        DBusMessageIter struct_iter;
        const char *unit_name;
        dbus_message_iter_recurse(&array_iter, &struct_iter);
        dbus_message_iter_get_basic(&struct_iter, &unit_name);
        size_t len = strlen(unit_name);
        if (len > 8 && !strcmp(unit_name + len - 8, ".service")) {
            if (!(services[idx] = strdup(unit_name))) goto strdup_fail;
            idx++;
        }
        dbus_message_iter_next(&array_iter);
    }
    goto success;

strdup_fail:
    while (idx) free(services[--idx]);
    free(services);
    services = NULL;
    *count = 0;

cleanup:
    if (reply) dbus_message_unref(reply);
    if (msg) dbus_message_unref(msg);
    if (conn) dbus_connection_unref(conn);
    dbus_error_free(&err);
    return services;

success:
    dbus_message_unref(reply);
    dbus_message_unref(msg);
    dbus_connection_unref(conn);
    dbus_error_free(&err);
    return services;
}
