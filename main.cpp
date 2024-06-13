#include <fmt/core.h>
#include <fmt/printf.h>
#include <optional>

auto GetEnv(const char *var) -> std::optional<std::string> {
  if (auto val{std::getenv(var)})
    return val;

  return std::nullopt;
}

auto FindDataHome() -> std::string {
  std::string path;

#ifdef __linux__
  if (auto val{GetEnv("XDG_DATA_HOME")})
    path = *val;
  else if (auto val{GetEnv("HOME")})
    path = *val + "/.local/share";
#elif _WIN32
  if (auto val{GetEnv("LOCALAPPDATA")})
    path = *val;
#elif __APPLE__
  if (auto val{GetEnv("HOME")})
    path = *val + "/Library/Application Support";
#endif

  if (path.empty())
    path = "./.woah";
  else
    path += "/woah";

  return path;
}

auto main(void) -> int {
  fmt::println("{}", FindDataHome());

  return 0;
}
