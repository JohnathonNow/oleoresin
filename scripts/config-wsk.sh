#!/bin/bash

#usage: ./config-whisk.sh IP_OF_NODE_1 [username]

NODE1=$1
NAME="${2:-johnbot}"
PORT="31001"

ssh $NAME@$NODE1 /local/repository/user_setup.sh
auth=$(ssh $NAME@$NODE1 wsk property get --auth | cut -f 3)

wsk property set --apihost $NODE1:$PORT
wsk property set --auth $auth
