#!/usr/bin/env bash

read s

mkdir -p $s
cd $s
touch "$s.py" "$s.cpp" "$s.sh"
echo "#!/usr/bin/env python3" >> "$s.py"
echo "#include <bits/stdc++.h>" >> "$s.cpp"
echo "#!/usr/bin/env bash" >> "$s.sh"
