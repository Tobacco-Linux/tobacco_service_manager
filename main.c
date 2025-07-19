#include "include/get_services.h"
#include "include/service_list.h"
#include <gtk/gtk.h>

typedef struct {
  GListStore *store;
} AppData;

static gboolean populate_store(gpointer data) {
  AppData *app = data;
  size_t count;
  char **services = get_systemd_services(&count);

  if (!services)
    return G_SOURCE_REMOVE;

  for (size_t i = 0; i < count; i++) {
    g_list_store_append(app->store, gtk_string_object_new(services[i]));
    free(services[i]);
  }
  free(services);
  return G_SOURCE_REMOVE;
}

static void activate(GtkApplication *app, gpointer user_data) {
  (void)user_data;
  GtkWidget *window = gtk_application_window_new(app);
  gtk_window_set_title(GTK_WINDOW(window), "Service Managers");
  gtk_window_set_default_size(GTK_WINDOW(window), 400, 600);

  AppData *data = g_new0(AppData, 1);
  data->store = g_list_store_new(GTK_TYPE_STRING_OBJECT);

  GtkWidget *list_view = service_list_new(data->store);
  gtk_window_set_child(GTK_WINDOW(window), list_view);

  g_idle_add(populate_store, data);
  gtk_window_present(GTK_WINDOW(window));
}

int main(int argc, char **argv) {
  GtkApplication *app;
  int status;

  app = gtk_application_new("org.TobaccoLinux.service_manager",
                            G_APPLICATION_DEFAULT_FLAGS);
  g_signal_connect(app, "activate", G_CALLBACK(activate), NULL);
  status = g_application_run(G_APPLICATION(app), argc, argv);
  g_object_unref(app);

  return status;
}
