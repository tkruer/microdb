import os
import subprocess
import unittest


def run_script(commands):
    """
    Runs the database executable via Cargo in release mode with the test database filename.

    Launches the process with the command:
      cargo run --release -- test.db

    Sends each command (with a newline) to its stdin and returns a list of output lines.
    """
    # Launch the process using Cargo.
    process = subprocess.Popen(
        ["cargo", "run", "--release", "--", "test.db"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )

    for command in commands:
        process.stdin.write(command + "\n")
    process.stdin.close()

    raw_output = process.stdout.read()
    process.wait()

    # Split output into lines, trimming whitespace.
    lines = [line.strip() for line in raw_output.splitlines()]
    return lines


class DatabaseSpec(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        # Remove the test database before running tests.
        if os.path.exists("test.db"):
            os.system("rm -rf test.db")

    def test_keeps_data_after_closing_connection(self):
        result1 = run_script(
            [
                "insert 1 user1 person1@example.com",
                ".exit",
            ]
        )
        self.assertEqual(result1, ["db > Executed.", "db >"])

        result2 = run_script(
            [
                "select",
                ".exit",
            ]
        )
        self.assertEqual(
            result2, ["db > (1, user1, person1@example.com)", "Executed.", "db >"]
        )

    def test_table_full_error(self):
        script = [f"insert {i} user{i} person{i}@example.com" for i in range(1, 1402)]
        script.append(".exit")
        result = run_script(script)
        self.assertIn("Error: Table full.", result)


if __name__ == "__main__":
    unittest.main()
