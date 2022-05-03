# build a new release of lfs for distribution
# 
# WARNING: this is not intented for normal usage but
# for the official release. It involves a heavy tool
# chain running on linux.
#
# For your own usage, you should rather do
#
#     cargo install --path .
# 
# or
#
#     cargo build --release
#
version=$(./version.sh)
APP=clima

echo "Building release $version"

# clean previous build
rm -rf build
mkdir build

# compile all targets
./compile-all-targets.sh

# add the readme and changelog in the build directory
echo "This is $APP. More info and installation instructions on https://github.com/Canop/$APP" > build/README.md
# cp CHANGELOG.md build

# prepare the release archive
rm "${APP}_*.zip"
zip -r "${APP}_$version.zip" build/*

# copy it to releases folder
mkdir releases
cp "${APP}_$version.zip" releases
