# Mongo DB Instructions

## Installations 

Ubuntu 14.04

~~~~
sudo apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv 7F0CEB10

echo "deb http://repo.mongodb.org/apt/ubuntu "$(lsb_release -sc)"/mongodb-org/3.0 multiverse" | sudo tee /etc/apt/sources.list.d/mongodb-org-3.0.list

sudo apt-get update

sudo apt-get install -y mongodb-org

service mongod status
~~~~

## Check Query - After POST reqest

rust-users -> DB
users -> Collections

~~~~
mongo
use rust-users
db.users.find( { "firstname": "VIKI" } ).pretty()
~~~~

## Ref:

* [Installation instructions](https://www.digitalocean.com/community/tutorials/how-to-install-mongodb-on-ubuntu-14-04)
* [Query Documentation](https://docs.mongodb.com/getting-started/shell/query/)