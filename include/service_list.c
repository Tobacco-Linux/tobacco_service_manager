#include "service_list.h"
#include <gtk/gtk.h>

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

GtkWidget *service_list_new(GListStore *model) {
  GtkListItemFactory *factory = gtk_signal_list_item_factory_new();
  g_signal_connect(factory, "setup", G_CALLBACK(setup_list_item), NULL);
  g_signal_connect(factory, "bind", G_CALLBACK(bind_list_item), NULL);

  GtkWidget *list = gtk_list_view_new(
      GTK_SELECTION_MODEL(gtk_no_selection_new(G_LIST_MODEL(model))), factory);
  GtkWidget *scroll = gtk_scrolled_window_new();
  gtk_scrolled_window_set_child(GTK_SCROLLED_WINDOW(scroll), list);
  return scroll;
}
