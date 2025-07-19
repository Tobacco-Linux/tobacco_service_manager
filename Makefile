run_dev:
	clang -Wall -Wpedantic -Wextra -O0 -g main.c include/*.c  -o tobacco_service_manager_temp `pkg-config --cflags --libs gtk4 dbus-1` && ./tobacco_service_manager_temp && rm tobacco_service_manager_temp

run_optimized:
	clang -Wall -Wpedantic -Wextra -Ofast -flto  main.c include/*.c  -o tobacco_service_manager_temp `pkg-config --cflags --libs gtk4 dbus-1` && ./tobacco_service_manager_temp && rm tobacco_service_manager

run_optimized_native:
	clang -Wall -Wpedantic -Wextra -Ofast -flto -march=native -mtune=native  main.c include/*.c  -o tobacco_service_manager_temp `pkg-config --cflags --libs gtk4 dbus-1` && ./tobacco_service_manager_temp && rm tobacco_service_manager

build_dev:
	clang -Wall -Wpedantic -Wextra -O0 -g main.c  include/*.c -o build/tobacco_service_manager `pkg-config --cflags --libs gtk4 dbus-1`

build_optimized:
	clang -Wall -Wpedantic -Wextra -Ofast -flto  main.c  include/*.c -o build/tobacco_service_manager `pkg-config --cflags --libs gtk4 dbus-1`

build_optimized_native:
	clang -Wall -Wpedantic -Wextra -Ofast -flto -march=native -mtune=native  main.c  include/*.c -o build/tobacco_service_manager `pkg-config --cflags --libs gtk4 dbus-1`
