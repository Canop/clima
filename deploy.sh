# build the release zip
./release.sh

version=$(./version.sh)
APP=clima

### # # deploy on dystroy.org
rm -rf ~/dev/www/dystroy/$APP/download/*
cp -r build/* ~/dev/www/dystroy/$APP/download/
cp ${APP}_${version}.zip  ~/dev/www/dystroy/$APP/download/
~/dev/www/dystroy/deploy.sh
