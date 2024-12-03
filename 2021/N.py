import sys
from pathlib import Path
# Add the ../ directory to the Python path
aoc_root_path = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(aoc_root_path))
import utils.python.aoc

ANS1=0
ANS2=0

print("ans1={}".format(ANS1))
print("ans2={}".format(ANS2))
