import os
import subprocess
import unittest


class DatabaseTest(unittest.TestCase):
    def setUp(self):
        if os.path.exists("test.db"):
            os.remove("test.db")

    def run_script(self, commands):
        result = []
        process = subprocess.Popen(
            ["cargo", "run", "--release", "test.db"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        try:
            for command in commands:
                process.stdin.write(command + "\n")
            process.stdin.close()
            output = process.stdout.read()
            result = output.split("\n")
        except BrokenPipeError:
            pass
        finally:
            process.wait()
        return result

    def test_inserts_and_retrieves_a_row(self):
        result = self.run_script(
            [
                "insert 1 user1 person1@example.com",
                "select",
                ".exit",
            ]
        )
        self.assertEqual(
            result,
            [
                "[INFO] db > Executed.",
                "[INFO] db > (1, user1, person1@example.com)",
                "Executed.",
                "[INFO] db > ",
            ],
        )

    def test_keeps_data_after_closing_connection(self):
        result1 = self.run_script(
            [
                "insert 1 user1 person1@example.com",
                ".exit",
            ]
        )
        self.assertEqual(result1, ["db > Executed.", "db > "])

        result2 = self.run_script(
            [
                "select",
                ".exit",
            ]
        )
        self.assertEqual(
            result2, ["db > (1, user1, person1@example.com)", "Executed.", "db > "]
        )

    def test_prints_error_message_when_table_is_full(self):
        script = [f"insert {i} user{i} person{i}@example.com" for i in range(1, 1402)]
        script.append(".exit")
        result = self.run_script(script)
        self.assertEqual(result[-2:], ["db > Executed.", "db > "])

    def test_allows_inserting_maximum_length_strings(self):
        long_username = "a" * 32
        long_email = "a" * 255
        result = self.run_script(
            [
                f"insert 1 {long_username} {long_email}",
                "select",
                ".exit",
            ]
        )
        self.assertEqual(
            result,
            [
                "db > Executed.",
                f"db > (1, {long_username}, {long_email})",
                "Executed.",
                "db > ",
            ],
        )

    def test_prints_error_if_strings_are_too_long(self):
        long_username = "a" * 33
        long_email = "a" * 256
        result = self.run_script(
            [
                f"insert 1 {long_username} {long_email}",
                "select",
                ".exit",
            ]
        )
        self.assertEqual(
            result, ["db > String is too long.", "db > Executed.", "db > "]
        )

    def test_successful_exit(self):
        result = self.run_script([".exit"])
        print(10 * "-")
        print(result)
        self.assertEqual(result, ["Exiting...", "Closing database..."])

    def test_prints_error_if_id_is_negative(self):
        result = self.run_script(
            [
                "insert -1 cstack foo@bar.com",
                "select",
                ".exit",
            ]
        )
        self.assertEqual(
            result, ["db > ID must be positive.", "db > Executed.", "db > "]
        )

    def test_prints_error_if_duplicate_id(self):
        result = self.run_script(
            [
                "insert 1 user1 person1@example.com",
                "insert 1 user1 person1@example.com",
                "select",
                ".exit",
            ]
        )
        self.assertEqual(
            result,
            [
                "db > Executed.",
                "db > Error: Duplicate key.",
                "db > (1, user1, person1@example.com)",
                "Executed.",
                "db > ",
            ],
        )


if __name__ == "__main__":
    unittest.main()
