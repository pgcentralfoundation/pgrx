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
#include "funcapi.h"
#include "miscadmin.h"
#include "pgstat.h"

#include "access/amapi.h"
#include "access/genam.h"
#include "access/generic_xlog.h"
#include "access/gin.h"
#include "access/gist.h"
#include "access/heapam.h"
#include "access/htup.h"
#include "access/htup_details.h"
#include "access/multixact.h"
#include "access/relation.h"
#include "access/reloptions.h"
#include "access/relscan.h"
#include "access/rmgr.h"
#include "access/skey.h"
#include "access/sysattr.h"
#include "access/table.h"
#include "access/visibilitymap.h"
#include "access/xact.h"
#include "access/xlog_internal.h"
#include "access/xlogreader.h"
#include "access/xlogutils.h"
#include "catalog/dependency.h"
#include "catalog/index.h"
#include "catalog/indexing.h"
#include "catalog/namespace.h"
#include "catalog/objectaccess.h"
#include "catalog/objectaddress.h"
#include "catalog/pg_am.h"
#include "catalog/pg_amop.h"
#include "catalog/pg_amproc.h"
#include "catalog/pg_authid.h"
#include "catalog/pg_class.h"
#include "catalog/pg_collation.h"
#include "catalog/pg_database.h"
#include "catalog/pg_enum.h"
#include "catalog/pg_extension.h"
#include "catalog/pg_foreign_data_wrapper.h"
#include "catalog/pg_foreign_server.h"
#include "catalog/pg_foreign_table.h"
#include "catalog/pg_operator.h"
#include "catalog/pg_opclass.h"
#include "catalog/pg_opfamily.h"
#include "catalog/pg_proc.h"
#include "catalog/pg_namespace.h"
#include "catalog/pg_seclabel.h"
#include "catalog/pg_tablespace.h"
#include "catalog/pg_trigger.h"
#include "catalog/pg_type.h"
#include "catalog/pg_user_mapping.h"
#include "catalog/storage.h"
#include "commands/comment.h"
#include "commands/copy.h"
#include "commands/dbcommands.h"
#include "commands/defrem.h"
#include "commands/event_trigger.h"
#include "commands/explain.h"
#include "commands/extension.h"
#include "commands/prepare.h"
#include "commands/proclang.h"
#include "commands/seclabel.h"
#include "commands/tablespace.h"
#include "commands/tablecmds.h"
#include "commands/trigger.h"
#include "commands/user.h"
#include "commands/vacuum.h"
#include "common/config_info.h"
#include "common/controldata_utils.h"
#include "executor/execExpr.h"
#include "executor/executor.h"
#include "executor/spi.h"
#include "executor/tuptable.h"
#include "foreign/fdwapi.h"
#include "foreign/foreign.h"
#include "jit/jit.h"
#include "lib/stringinfo.h"
#include "libpq/pqformat.h"
#include "mb/pg_wchar.h"
#include "nodes/execnodes.h"
#include "nodes/extensible.h"
#include "nodes/makefuncs.h"
#include "nodes/nodeFuncs.h"
#include "nodes/nodes.h"
#include "nodes/parsenodes.h"
#include "nodes/primnodes.h"
#include "nodes/print.h"
#include "nodes/replnodes.h"
#include "nodes/supportnodes.h"
#include "nodes/tidbitmap.h"
#include "nodes/value.h"
#include "optimizer/appendinfo.h"
#include "optimizer/clauses.h"
#include "optimizer/cost.h"
#include "optimizer/optimizer.h"
#include "optimizer/pathnode.h"
#include "optimizer/paths.h"
#include "optimizer/plancat.h"
#include "optimizer/planmain.h"
#include "optimizer/planner.h"
#include "optimizer/restrictinfo.h"
#include "optimizer/tlist.h"
#include "parser/analyze.h"
#include "parser/collate.h"
#include "parser/parse_expr.h"
#include "parser/parse_func.h"
#include "parser/parse_oper.h"
#include "parser/parse_relation.h"
#include "parser/parse_type.h"
#include "parser/parse_coerce.h"
#include "parser/parser.h"
#include "parser/parsetree.h"
#include "parser/scansup.h"
#include "plpgsql.h"
#include "postmaster/bgworker.h"
#include "postmaster/postmaster.h"
#include "postmaster/syslogger.h"
#include "replication/logical.h"
#include "replication/output_plugin.h"
#include "rewrite/rewriteHandler.h"
#include "rewrite/rowsecurity.h"
#include "storage/block.h"
#include "storage/bufmgr.h"
#include "storage/buffile.h"
#include "storage/bufpage.h"
#include "storage/ipc.h"
#include "storage/itemptr.h"
#include "storage/lmgr.h"
#include "storage/lwlock.h"
#include "storage/procarray.h"
#include "storage/relfilenode.h"
#include "storage/smgr.h"
#include "storage/spin.h"
#include "storage/sync.h"
#include "tcop/pquery.h"
#include "tcop/tcopprot.h"
#include "tcop/utility.h"
#include "tsearch/ts_public.h"
#include "tsearch/ts_utils.h"
#include "utils/builtins.h"
#include "utils/date.h"
#include "utils/datetime.h"
#include "utils/elog.h"
#include "utils/float.h"
#include "utils/fmgroids.h"
#include "utils/fmgrprotos.h"
#include "utils/geo_decls.h"
#include "utils/guc.h"
#include "utils/json.h"
#include "utils/jsonb.h"
#include "utils/lsyscache.h"
#include "utils/memutils.h"
#include "utils/numeric.h"
#include "utils/palloc.h"
#include "utils/rel.h"
#include "utils/regproc.h"
#include "utils/relcache.h"
#include "utils/resowner.h"
#include "utils/resowner_private.h"
#include "utils/ruleutils.h"
#include "utils/sampling.h"
#include "utils/selfuncs.h"
#include "utils/snapmgr.h"
#include "utils/sortsupport.h"
#include "utils/catcache.h"
#include "utils/syscache.h"
#include "utils/tuplestore.h"
#include "utils/typcache.h"
#include "utils/rangetypes.h"
#include "utils/rel.h"
#include "utils/varlena.h"
