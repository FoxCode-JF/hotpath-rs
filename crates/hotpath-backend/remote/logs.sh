#!/bin/bash
ssh $TARGET_NODE 'tail -f /root/hotpath-backend/dbg.log'
