#!/usr/bin/env sh

# The script may output `ls: cannot access '*.h': No such file or directory`
# because some headers are missing in some PostgreSQL versions. This message
# can be safely ignored.

cat << EOF
//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#include "postgres.h"
#include "pg_config.h"

EOF

(cd "$1/server" && ls -1 \
    access/*.h \
    catalog/*.h \
    commands/*.h \
    common/config_info.h \
    common/controldata_utils.h \
    common/pg_lzcompress.h \
    executor/*.h \
    foreign/*.h \
    funcapi.h \
    jit/jit.h \
    lib/stringinfo.h \
    libpq/pqformat.h \
    mb/pg_wchar.h \
    miscadmin.h \
    nodes/*.h \
    optimizer/*.h \
    parser/*.h \
    partitioning/*.h \
    pgstat.h \
    plpgsql.h \
    postmaster/*.h \
    replication/*.h \
    rewrite/*.h \
    statistics/*.h \
    storage/*.h \
    tcop/*.h \
    tsearch/*.h \
    utils/acl.h \
    utils/builtins.h \
    utils/catcache.h \
    utils/date.h \
    utils/datetime.h \
    utils/datum.h \
    utils/elog.h \
    utils/float.h \
    utils/fmgroids.h \
    utils/fmgrprotos.h \
    utils/geo_decls.h \
    utils/guc.h \
    utils/guc_tables.h \
    utils/json.h \
    utils/jsonb.h \
    utils/lsyscache.h \
    utils/memutils.h \
    utils/numeric.h \
    utils/palloc.h \
    utils/ps_status.h \
    utils/rangetypes.h \
    utils/regproc.h \
    utils/rel.h \
    utils/relcache.h \
    utils/resowner.h \
    utils/resowner_private.h \
    utils/rls.h \
    utils/ruleutils.h \
    utils/sampling.h \
    utils/selfuncs.h \
    utils/snapmgr.h \
    utils/sortsupport.h \
    utils/spccache.h \
    utils/syscache.h \
    utils/tuplesort.h \
    utils/tuplestore.h \
    utils/typcache.h \
    utils/varlena.h \
    utils/wait_event.h \
    varatt.h \
    | grep -v '^access/rmgrlist\.h$' \
    | grep -v '^catalog/pg_.*_d\.h$' \
    | grep -v '^catalog/syscache_ids\.h$' \
    | grep -v '^catalog/syscache_info\.h$' \
    | grep -v '^nodes/nodetags\.h$' \
    | grep -v '^parser/gram\.h$' \
    | grep -v '^parser/kwlist\.h$' \
    | grep -v '^postmaster/proctypelist\.h$' \
    | grep -v '^replication/backup_manifest\.h$' \
    | grep -v '^storage/checksum_block_internal\.h$' \
    | grep -v '^storage/subsystemlist\.h$' \
    | grep -v '^storage/lwlocklist\.h$' \
    | grep -v '^storage/lwlocknames\.h$' \
    | grep -v '^tcop/cmdtaglist\.h$' \
    | sed 's/^/#include "/' \
    | sed 's/$/"/')

cat << EOF

#if PG_VERSION_NUM < 140000
#ifndef WIN32
#define PGERROR ERROR
#endif
#endif
EOF
