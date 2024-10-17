Configuring MongoDB to recognize /database (linux)

1. Stop MongoDB "sudo systemctl stop mongod"

2. Configure MongoDB
	If you have any Mongo DBs worth backing up, do that (look up how to do that)
	Open the the mongod config file ("/etc/mongod.conf") as root in a text editor
	Under "storage:" add the line: "  directoryPerDB: true"
	Clear the contents of the Mongo storage path (See the conf file, probably /var/lib/mongodb) so it's an empty directory

3. Create Symlink
	In the mongo storage directory, there should be multiple folders, such as admin and config, these are your databases
	Create a symlink here to the database directory in git "sudo ln -s /path/to/repo/backend/database/ HP", where "HP" is whatever you want the database to be named on your system

4. Fix Permissions/Ownership
	Set the owner of the symlink to mongodb "sudo chown -h mongodb:mongodb HP"
	Check the ownership and permssions of /database with "ls -ld database"
	You should get something like this "drwxr-xr-x 2 mongodb mongodb 4096 Oct 16 16:15 database/"
	If not, it may require chown and chmod to fix.
	Mongo also needs to be able to access the parent directory of /database
	In the parent directory of /database, run "sudo setfacl -R -m u:mongodb:rx ."

5. Check that it worked
	Start Mongo with "sudo systemctl start mongod"
	Run "mongosh", then in mongo shell, run "show dbs"
	If the database shows up in the list and it gives no errors, you're done
	If not, let Caleb know