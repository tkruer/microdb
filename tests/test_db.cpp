#define CATCH_CONFIG_MAIN
#include <catch2/catch.hpp>

#include <array>
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

// Helper: runs "./db test.db" with input from a temporary file
std::vector<std::string> run_script(const std::vector<std::string> &commands) {
  // Write commands to a temporary file.
  const std::string temp_file = "temp_test_input.txt";
  {
    std::ofstream ofs(temp_file);
    if (!ofs) {
      throw std::runtime_error("Unable to open temporary file for writing.");
    }
    for (const auto &cmd : commands) {
      ofs << cmd << "\n";
    }
  }
  // Build command: run our binary "db" with the test database filename
  std::string cmd = "./db test.db < " + temp_file;

  std::array<char, 128> buffer;
  std::string result;
  FILE *pipe = popen(cmd.c_str(), "r");
  if (!pipe) {
    throw std::runtime_error("popen() failed!");
  }
  while (fgets(buffer.data(), buffer.size(), pipe) != nullptr) {
    result += buffer.data();
  }
  pclose(pipe);
  // Clean up temporary file.
  std::remove(temp_file.c_str());

  // Split the output into lines.
  std::istringstream iss(result);
  std::vector<std::string> lines;
  std::string line;
  while (std::getline(iss, line)) {
    lines.push_back(line);
  }
  return lines;
}

// Remove any existing test database file before each test.
void remove_test_db() { std::remove("test.db"); }

TEST_CASE("inserts and retrieves a row", "[database]") {
  remove_test_db();
  std::vector<std::string> result = run_script({
      "insert 1 user1 person1@example.com",
      "select",
      ".exit",
  });
  // Expected output may vary slightly depending on prompt behavior.
  // Here we assume the following output (adjust as needed).
  std::vector<std::string> expected = {"db > Executed.",
                                       "db > (1, user1, person1@example.com)",
                                       "Executed.", "db > "};
  REQUIRE(result == expected);
}

TEST_CASE("prints error message if id is negative", "[database]") {
  remove_test_db();
  std::vector<std::string> result = run_script({
      "insert -1 cstack foo@bar.com",
      "select",
      ".exit",
  });
  std::vector<std::string> expected = {"db > ID must be positive.",
                                       "db > Executed.", "db > "};
  REQUIRE(result == expected);
}

TEST_CASE("prints error message if there is a duplicate id", "[database]") {
  remove_test_db();
  std::vector<std::string> result = run_script({
      "insert 1 user1 person1@example.com",
      "insert 1 user1 person1@example.com",
      "select",
      ".exit",
  });
  std::vector<std::string> expected = {
      "db > Executed.", "db > Error: Duplicate key.",
      "db > (1, user1, person1@example.com)", "Executed.", "db > "};
  REQUIRE(result == expected);
}
