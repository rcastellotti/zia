from datetime import datetime
import subprocess
from pathlib import Path
import json
import tomllib


class Zia:
    def __init__(self):
        pass

    def run_command(self, bin, command: str) -> dict:
        full_cmd = str(bin) + " " + command
        start_time = datetime.now()
        result = subprocess.run(full_cmd, shell=True, text=True, capture_output=True)
        end_time = datetime.now()

        return {
            "cmd": full_cmd,
            "stdout": result.stdout,
            "stderr": result.stderr,
            "retcode": result.returncode,
            "duration": (end_time - start_time).total_seconds(),
        }

    def results(self) -> str:
        results = [
            {
                "header": cmd,
                "l": self.run_command(self.bin1, cmd),
                "r": self.run_command(self.bin2, cmd),
            }
            for cmd in self.cmds
        ]
        return results

    def save_to_file(self, file: Path):
        with Path(file).open("w") as f:
            json.dump(self.results(), f, ensure_ascii=False, indent=4)

    def load_config(self, config_file: Path):
        with Path(config_file).open("rb") as f:
            config = tomllib.load(f)
            self.bin1 = config["bin1"]
            self.bin2 = config["bin1"]
            self.cmds = config["cmds"]
