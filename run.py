import subprocess

subprocess.run(["cargo", "run", "--release"])
# subprocess.run(["./target/release/yapl"])

print("\n------------------")

subprocess.run(["python", "out/out.py"])
