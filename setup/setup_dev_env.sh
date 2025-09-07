rm -rf ../.build
rm -rf ../dist
rm -rf ../toraniko.egg-info
rm -rf ../.venv
python3 -m venv ../.venv
source ../.venv/bin/activate
pip install setuptools
pip install -r ../dev-requirements.txt
cd ..
python3 ./setup.py clean
python3 ./setup.py install