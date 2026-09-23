#include <cstdlib>
#include <iio.h>
#include <iostream>

int main() {
  iio_context *ctx = iio_create_context_from_uri("ip:192.168.2.1");
  if (!ctx) {
    std::cout << "Could not connect to Pluto\n";
    return EXIT_FAILURE;
  }

  std::cout << "Context: " << iio_context_get_name(ctx) << "\n";

  unsigned int ndev = iio_context_get_devices_count(ctx);
  std::cout << "Found " << ndev << " iio devices.\n";

  for (unsigned int i = 0; i < ndev; ++i) {
    iio_device *dev = iio_context_get_device(ctx, i);
    std::cout << "  [" << i << "] " << iio_device_get_name(dev) << " ("
              << iio_device_get_channels_count(dev) << " channels)\n";

    for (unsigned int c = 0; c < iio_device_get_channels_count(dev); ++c) {
      iio_channel *ch = iio_device_get_channel(dev, c);
      const char *is_output = iio_channel_is_output(ch) ? "output" : "input";
      std::cout << "    - " << iio_channel_get_id(ch) << " (" << is_output
                << ")\n";
    }
  }

  iio_context_destroy(ctx);
  return 0;
}
