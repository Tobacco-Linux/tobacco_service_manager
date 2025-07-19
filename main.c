#include "include/service.h"
#include <gtk/gtk.h>

typedef struct {
  GListStore *store;
} AppData;

static void setup_list_item(GtkListItemFactory *factory, GtkListItem *item,
                            gpointer user_data) {
  (void)factory;
  (void)user_data;
  gtk_list_item_set_child(item, gtk_label_new(""));
}

static void bind_list_item(GtkListItemFactory *factory, GtkListItem *item,
                           gpointer user_data) {
  (void)factory;
  (void)user_data;
  GtkStringObject *obj = gtk_list_item_get_item(item);
  gtk_label_set_text(GTK_LABEL(gtk_list_item_get_child(item)),
                     gtk_string_object_get_string(obj));
}

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
  gtk_window_set_title(GTK_WINDOW(window), "Systemd Services");
  gtk_window_set_default_size(GTK_WINDOW(window), 400, 600);

  AppData *data = g_new0(AppData, 1);
  data->store = g_list_store_new(GTK_TYPE_STRING_OBJECT);

  GtkListItemFactory *factory = gtk_signal_list_item_factory_new();
  g_signal_connect(factory, "setup", G_CALLBACK(setup_list_item), NULL);
  g_signal_connect(factory, "bind", G_CALLBACK(bind_list_item), NULL);

  GtkWidget *list = gtk_list_view_new(
      GTK_SELECTION_MODEL(gtk_no_selection_new(G_LIST_MODEL(data->store))),
      factory);

  GtkWidget *scroll = gtk_scrolled_window_new();
  gtk_scrolled_window_set_child(GTK_SCROLLED_WINDOW(scroll), list);
  gtk_window_set_child(GTK_WINDOW(window), scroll);

  g_idle_add(populate_store, data);
  gtk_window_present(GTK_WINDOW(window));
}

int main(int argc, char **argv) {
  GtkApplication *app;
  int status;

  app = gtk_application_new("org.gtk.example", G_APPLICATION_DEFAULT_FLAGS);
  g_signal_connect(app, "activate", G_CALLBACK(activate), NULL);
  status = g_application_run(G_APPLICATION(app), argc, argv);
  g_object_unref(app);

  return status;
}
