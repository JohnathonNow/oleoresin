# oleoresin
A system for serverless debugging.

# CloudLab Set Up  
I run the existing [openwhisk profile](https://www.cloudlab.us/show-profile.php?uuid=225e608c-67f8-11eb-b1eb-e4434b2381fc)
from [CU Boulder](https://github.com/CU-BISON-LAB/cloudlab-openwhisk).  
Importantly, on node1, run `/local/respository/user_setup.sh`

It will tell you to run  `wsk property get --auth`.
Take the auth from running that, and anywhere of interest, run
`wsk property set --auth $AUTH`, where $AUTH is the value you just got.

Also run  `wsk property set --apihost $NODE1:$PORT` where $NODE1 is the URL to
node1, and $PORT is the port mentioned by the `user_setup.sh` script.
