#!/bin/bash
function release() {
  MODULE_DIR=$1

  pushd $MODULE_DIR
  export RELEASE_VERSION=`echo $CI_COMMIT_TAG | cut -c2-`                        # pull version from tag name
  mvn versions:set -DnewVersion=$RELEASE_VERSION                                 # set version to tagged version
  mvn clean deploy -DskipTests -Dmaven.javadoc.skip=true --settings settings.xml
  popd
}

set -eux
release wrappers/java