run_dev:
	clang `pkg-config --cflags gtk4` -Wall -Wpedantic -Wextra -O0 -g main.c -o tobacco_service_manager_temp `pkg-config --libs gtk4` && ./tobacco_service_manager_temp && rm tobacco_service_manager_temp

run_optimized:
	clang `pkg-config --cflags gtk4` -Wall -Wpedantic -Wextra -Ofast -flto  main.c -o tobacco_service_manager_temp `pkg-config --libs gtk4` && ./tobacco_service_manager_temp && rm tobacco_service_manager

run_optimized_native:
	clang `pkg-config --cflags gtk4` -Wall -Wpedantic -Wextra -Ofast -flto -march=native -mtune=native  main.c -o tobacco_service_manager_temp `pkg-config --libs gtk4` && ./tobacco_service_manager_temp && rm tobacco_service_manager

build_dev:
	clang `pkg-config --cflags gtk4` -Wall -Wpedantic -Wextra -O0 -g main.c -o build/tobacco_service_manager `pkg-config --libs gtk4`

build_optimized:
	clang `pkg-config --cflags gtk4` -Wall -Wpedantic -Wextra -Ofast -flto  main.c -o build/tobacco_service_manager `pkg-config --libs gtk4`

build_optimized_native:
	clang `pkg-config --cflags gtk4` -Wall -Wpedantic -Wextra -Ofast -flto -march=native -mtune=native  main.c -o build/tobacco_service_manager `pkg-config --libs gtk4`
