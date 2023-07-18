FILE=PngSuite-2017jul19.tgz
if [ -f "$FILE" ]; then
    echo "$FILE exists."
else 
    echo "$FILE does not exist.";
    echo "Downloading";
    wget http://www.schaik.com/pngsuite/PngSuite-2017jul19.tgz
fi
echo "Extracting";

if [ -d "pngsuite" ]; then
  tar zxf $FILE -C pngsuite/
else
  mkdir pngsuite && tar zxf $FILE -C pngsuite/
fi

