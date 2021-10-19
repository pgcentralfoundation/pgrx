// Copyright 2020-2021 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

//! Access to Postgres' logging system

/// Postgres' various logging levels
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum PgLogLevel {
    /// Debugging messages, in categories of decreasing detail
    DEBUG5 = crate::pg_sys::DEBUG5 as isize,

    /// Debugging messages, in categories of decreasing detail
    DEBUG4 = crate::pg_sys::DEBUG4 as isize,

    /// Debugging messages, in categories of decreasing detail
    DEBUG3 = crate::pg_sys::DEBUG3 as isize,

    /// Debugging messages, in categories of decreasing detail
    DEBUG2 = crate::pg_sys::DEBUG2 as isize,

    /// Debugging messages, in categories of decreasing detail
    /// NOTE:  used by GUC debug_* variables
    DEBUG1 = crate::pg_sys::DEBUG1 as isize,

    /// Server operational messages; sent only to server log by default.
    LOG = crate::pg_sys::LOG as isize,

    /// Same as LOG for server reporting, but never sent to client.
    #[allow(non_camel_case_types)]
    LOG_SERVER_ONLY = crate::pg_sys::LOG_SERVER_ONLY as isize,

    /// Messages specifically requested by user (eg VACUUM VERBOSE output); always sent to client
    /// regardless of client_min_messages, but by default not sent to server log.
    INFO = crate::pg_sys::INFO as isize,

    /// Helpful messages to users about query operation; sent to client and not to server log by default.
    NOTICE = crate::pg_sys::NOTICE as isize,

    /// Warnings.  \[NOTICE\] is for expected messages like implicit sequence creation by SERIAL.
    /// \[WARNING\] is for unexpected messages.
    WARNING = crate::pg_sys::WARNING as isize,

    /// user error - abort transaction; return to known state
    ERROR = crate::pg_sys::ERROR as isize,

    /// fatal error - abort process
    FATAL = crate::pg_sys::FATAL as isize,

    /// take down the other backends with me
    PANIC = crate::pg_sys::PANIC as isize,
}

/// This list of SQL Error Codes is taken directly from Postgres 12's generated "utils/errcodes.h"
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum PgSqlErrorCode {
    /// Class 00 - Successful Completion
    ERRCODE_SUCCESSFUL_COMPLETION = MAKE_SQLSTATE('0', '0', '0', '0', '0') as isize,

    /// Class 01 - Warning
    ERRCODE_WARNING = MAKE_SQLSTATE('0', '1', '0', '0', '0') as isize,
    ERRCODE_WARNING_DYNAMIC_RESULT_SETS_RETURNED = MAKE_SQLSTATE('0', '1', '0', '0', 'C') as isize,
    ERRCODE_WARNING_IMPLICIT_ZERO_BIT_PADDING = MAKE_SQLSTATE('0', '1', '0', '0', '8') as isize,
    ERRCODE_WARNING_NULL_VALUE_ELIMINATED_IN_SET_FUNCTION =
        MAKE_SQLSTATE('0', '1', '0', '0', '3') as isize,
    ERRCODE_WARNING_PRIVILEGE_NOT_GRANTED = MAKE_SQLSTATE('0', '1', '0', '0', '7') as isize,
    ERRCODE_WARNING_PRIVILEGE_NOT_REVOKED = MAKE_SQLSTATE('0', '1', '0', '0', '6') as isize,
    ERRCODE_WARNING_STRING_DATA_RIGHT_TRUNCATION = MAKE_SQLSTATE('0', '1', '0', '0', '4') as isize,
    ERRCODE_WARNING_DEPRECATED_FEATURE = MAKE_SQLSTATE('0', '1', 'P', '0', '1') as isize,

    /// Class 02 - No Data (this is also a warning class per the SQL standard) as isize,
    ERRCODE_NO_DATA = MAKE_SQLSTATE('0', '2', '0', '0', '0') as isize,
    ERRCODE_NO_ADDITIONAL_DYNAMIC_RESULT_SETS_RETURNED =
        MAKE_SQLSTATE('0', '2', '0', '0', '1') as isize,

    /// Class 03 - SQL Statement Not Yet Complete
    ERRCODE_SQL_STATEMENT_NOT_YET_COMPLETE = MAKE_SQLSTATE('0', '3', '0', '0', '0') as isize,

    /// Class 08 - Connection Exception
    ERRCODE_CONNECTION_EXCEPTION = MAKE_SQLSTATE('0', '8', '0', '0', '0') as isize,
    ERRCODE_CONNECTION_DOES_NOT_EXIST = MAKE_SQLSTATE('0', '8', '0', '0', '3') as isize,
    ERRCODE_CONNECTION_FAILURE = MAKE_SQLSTATE('0', '8', '0', '0', '6') as isize,
    ERRCODE_SQLCLIENT_UNABLE_TO_ESTABLISH_SQLCONNECTION =
        MAKE_SQLSTATE('0', '8', '0', '0', '1') as isize,
    ERRCODE_SQLSERVER_REJECTED_ESTABLISHMENT_OF_SQLCONNECTION =
        MAKE_SQLSTATE('0', '8', '0', '0', '4') as isize,
    ERRCODE_TRANSACTION_RESOLUTION_UNKNOWN = MAKE_SQLSTATE('0', '8', '0', '0', '7') as isize,
    ERRCODE_PROTOCOL_VIOLATION = MAKE_SQLSTATE('0', '8', 'P', '0', '1') as isize,

    /// Class 09 - Triggered Action Exception
    ERRCODE_TRIGGERED_ACTION_EXCEPTION = MAKE_SQLSTATE('0', '9', '0', '0', '0') as isize,

    /// Class 0A - Feature Not Supported
    ERRCODE_FEATURE_NOT_SUPPORTED = MAKE_SQLSTATE('0', 'A', '0', '0', '0') as isize,

    /// Class 0B - Invalid Transaction Initiation
    ERRCODE_INVALID_TRANSACTION_INITIATION = MAKE_SQLSTATE('0', 'B', '0', '0', '0') as isize,

    /// Class 0F - Locator Exception
    ERRCODE_LOCATOR_EXCEPTION = MAKE_SQLSTATE('0', 'F', '0', '0', '0') as isize,
    ERRCODE_L_E_INVALID_SPECIFICATION = MAKE_SQLSTATE('0', 'F', '0', '0', '1') as isize,

    /// Class 0L - Invalid Grantor
    ERRCODE_INVALID_GRANTOR = MAKE_SQLSTATE('0', 'L', '0', '0', '0') as isize,
    ERRCODE_INVALID_GRANT_OPERATION = MAKE_SQLSTATE('0', 'L', 'P', '0', '1') as isize,

    /// Class 0P - Invalid Role Specification
    ERRCODE_INVALID_ROLE_SPECIFICATION = MAKE_SQLSTATE('0', 'P', '0', '0', '0') as isize,

    /// Class 0Z - Diagnostics Exception
    ERRCODE_DIAGNOSTICS_EXCEPTION = MAKE_SQLSTATE('0', 'Z', '0', '0', '0') as isize,
    ERRCODE_STACKED_DIAGNOSTICS_ACCESSED_WITHOUT_ACTIVE_HANDLER =
        MAKE_SQLSTATE('0', 'Z', '0', '0', '2') as isize,

    /// Class 20 - Case Not Found
    ERRCODE_CASE_NOT_FOUND = MAKE_SQLSTATE('2', '0', '0', '0', '0') as isize,

    /// Class 21 - Cardinality Violation
    ERRCODE_CARDINALITY_VIOLATION = MAKE_SQLSTATE('2', '1', '0', '0', '0') as isize,

    /// Class 22 - Data Exception
    ERRCODE_DATA_EXCEPTION = MAKE_SQLSTATE('2', '2', '0', '0', '0') as isize,
    ERRCODE_ARRAY_ELEMENT_ERROR = MAKE_SQLSTATE('2', '2', '0', '2', 'E') as isize,
    //    ERRCODE_ARRAY_SUBSCRIPT_ERROR = MAKE_SQLSTATE('2', '2', '0', '2', 'E') as isize,
    ERRCODE_CHARACTER_NOT_IN_REPERTOIRE = MAKE_SQLSTATE('2', '2', '0', '2', '1') as isize,
    ERRCODE_DATETIME_FIELD_OVERFLOW = MAKE_SQLSTATE('2', '2', '0', '0', '8') as isize,
    //    ERRCODE_DATETIME_VALUE_OUT_OF_RANGE = MAKE_SQLSTATE('2', '2', '0', '0', '8') as isize,
    ERRCODE_DIVISION_BY_ZERO = MAKE_SQLSTATE('2', '2', '0', '1', '2') as isize,
    ERRCODE_ERROR_IN_ASSIGNMENT = MAKE_SQLSTATE('2', '2', '0', '0', '5') as isize,
    ERRCODE_ESCAPE_CHARACTER_CONFLICT = MAKE_SQLSTATE('2', '2', '0', '0', 'B') as isize,
    ERRCODE_INDICATOR_OVERFLOW = MAKE_SQLSTATE('2', '2', '0', '2', '2') as isize,
    ERRCODE_INTERVAL_FIELD_OVERFLOW = MAKE_SQLSTATE('2', '2', '0', '1', '5') as isize,
    ERRCODE_INVALID_ARGUMENT_FOR_LOG = MAKE_SQLSTATE('2', '2', '0', '1', 'E') as isize,
    ERRCODE_INVALID_ARGUMENT_FOR_NTILE = MAKE_SQLSTATE('2', '2', '0', '1', '4') as isize,
    ERRCODE_INVALID_ARGUMENT_FOR_NTH_VALUE = MAKE_SQLSTATE('2', '2', '0', '1', '6') as isize,
    ERRCODE_INVALID_ARGUMENT_FOR_POWER_FUNCTION = MAKE_SQLSTATE('2', '2', '0', '1', 'F') as isize,
    ERRCODE_INVALID_ARGUMENT_FOR_WIDTH_BUCKET_FUNCTION =
        MAKE_SQLSTATE('2', '2', '0', '1', 'G') as isize,
    ERRCODE_INVALID_CHARACTER_VALUE_FOR_CAST = MAKE_SQLSTATE('2', '2', '0', '1', '8') as isize,
    ERRCODE_INVALID_DATETIME_FORMAT = MAKE_SQLSTATE('2', '2', '0', '0', '7') as isize,
    ERRCODE_INVALID_ESCAPE_CHARACTER = MAKE_SQLSTATE('2', '2', '0', '1', '9') as isize,
    ERRCODE_INVALID_ESCAPE_OCTET = MAKE_SQLSTATE('2', '2', '0', '0', 'D') as isize,
    ERRCODE_INVALID_ESCAPE_SEQUENCE = MAKE_SQLSTATE('2', '2', '0', '2', '5') as isize,
    ERRCODE_NONSTANDARD_USE_OF_ESCAPE_CHARACTER = MAKE_SQLSTATE('2', '2', 'P', '0', '6') as isize,
    ERRCODE_INVALID_INDICATOR_PARAMETER_VALUE = MAKE_SQLSTATE('2', '2', '0', '1', '0') as isize,
    ERRCODE_INVALID_PARAMETER_VALUE = MAKE_SQLSTATE('2', '2', '0', '2', '3') as isize,
    ERRCODE_INVALID_PRECEDING_OR_FOLLOWING_SIZE = MAKE_SQLSTATE('2', '2', '0', '1', '3') as isize,
    ERRCODE_INVALID_REGULAR_EXPRESSION = MAKE_SQLSTATE('2', '2', '0', '1', 'B') as isize,
    ERRCODE_INVALID_ROW_COUNT_IN_LIMIT_CLAUSE = MAKE_SQLSTATE('2', '2', '0', '1', 'W') as isize,
    ERRCODE_INVALID_ROW_COUNT_IN_RESULT_OFFSET_CLAUSE =
        MAKE_SQLSTATE('2', '2', '0', '1', 'X') as isize,
    ERRCODE_INVALID_TABLESAMPLE_ARGUMENT = MAKE_SQLSTATE('2', '2', '0', '2', 'H') as isize,
    ERRCODE_INVALID_TABLESAMPLE_REPEAT = MAKE_SQLSTATE('2', '2', '0', '2', 'G') as isize,
    ERRCODE_INVALID_TIME_ZONE_DISPLACEMENT_VALUE = MAKE_SQLSTATE('2', '2', '0', '0', '9') as isize,
    ERRCODE_INVALID_USE_OF_ESCAPE_CHARACTER = MAKE_SQLSTATE('2', '2', '0', '0', 'C') as isize,
    ERRCODE_MOST_SPECIFIC_TYPE_MISMATCH = MAKE_SQLSTATE('2', '2', '0', '0', 'G') as isize,
    ERRCODE_NULL_VALUE_NOT_ALLOWED = MAKE_SQLSTATE('2', '2', '0', '0', '4') as isize,
    ERRCODE_NULL_VALUE_NO_INDICATOR_PARAMETER = MAKE_SQLSTATE('2', '2', '0', '0', '2') as isize,
    ERRCODE_NUMERIC_VALUE_OUT_OF_RANGE = MAKE_SQLSTATE('2', '2', '0', '0', '3') as isize,
    ERRCODE_SEQUENCE_GENERATOR_LIMIT_EXCEEDED = MAKE_SQLSTATE('2', '2', '0', '0', 'H') as isize,
    ERRCODE_STRING_DATA_LENGTH_MISMATCH = MAKE_SQLSTATE('2', '2', '0', '2', '6') as isize,
    ERRCODE_STRING_DATA_RIGHT_TRUNCATION = MAKE_SQLSTATE('2', '2', '0', '0', '1') as isize,
    ERRCODE_SUBSTRING_ERROR = MAKE_SQLSTATE('2', '2', '0', '1', '1') as isize,
    ERRCODE_TRIM_ERROR = MAKE_SQLSTATE('2', '2', '0', '2', '7') as isize,
    ERRCODE_UNTERMINATED_C_STRING = MAKE_SQLSTATE('2', '2', '0', '2', '4') as isize,
    ERRCODE_ZERO_LENGTH_CHARACTER_STRING = MAKE_SQLSTATE('2', '2', '0', '0', 'F') as isize,
    ERRCODE_FLOATING_POINT_EXCEPTION = MAKE_SQLSTATE('2', '2', 'P', '0', '1') as isize,
    ERRCODE_INVALID_TEXT_REPRESENTATION = MAKE_SQLSTATE('2', '2', 'P', '0', '2') as isize,
    ERRCODE_INVALID_BINARY_REPRESENTATION = MAKE_SQLSTATE('2', '2', 'P', '0', '3') as isize,
    ERRCODE_BAD_COPY_FILE_FORMAT = MAKE_SQLSTATE('2', '2', 'P', '0', '4') as isize,
    ERRCODE_UNTRANSLATABLE_CHARACTER = MAKE_SQLSTATE('2', '2', 'P', '0', '5') as isize,
    ERRCODE_NOT_AN_XML_DOCUMENT = MAKE_SQLSTATE('2', '2', '0', '0', 'L') as isize,
    ERRCODE_INVALID_XML_DOCUMENT = MAKE_SQLSTATE('2', '2', '0', '0', 'M') as isize,
    ERRCODE_INVALID_XML_CONTENT = MAKE_SQLSTATE('2', '2', '0', '0', 'N') as isize,
    ERRCODE_INVALID_XML_COMMENT = MAKE_SQLSTATE('2', '2', '0', '0', 'S') as isize,
    ERRCODE_INVALID_XML_PROCESSING_INSTRUCTION = MAKE_SQLSTATE('2', '2', '0', '0', 'T') as isize,
    ERRCODE_DUPLICATE_JSON_OBJECT_KEY_VALUE = MAKE_SQLSTATE('2', '2', '0', '3', '0') as isize,
    ERRCODE_INVALID_JSON_TEXT = MAKE_SQLSTATE('2', '2', '0', '3', '2') as isize,
    ERRCODE_INVALID_SQL_JSON_SUBSCRIPT = MAKE_SQLSTATE('2', '2', '0', '3', '3') as isize,
    ERRCODE_MORE_THAN_ONE_SQL_JSON_ITEM = MAKE_SQLSTATE('2', '2', '0', '3', '4') as isize,
    ERRCODE_NO_SQL_JSON_ITEM = MAKE_SQLSTATE('2', '2', '0', '3', '5') as isize,
    ERRCODE_NON_NUMERIC_SQL_JSON_ITEM = MAKE_SQLSTATE('2', '2', '0', '3', '6') as isize,
    ERRCODE_NON_UNIQUE_KEYS_IN_A_JSON_OBJECT = MAKE_SQLSTATE('2', '2', '0', '3', '7') as isize,
    ERRCODE_SINGLETON_SQL_JSON_ITEM_REQUIRED = MAKE_SQLSTATE('2', '2', '0', '3', '8') as isize,
    ERRCODE_SQL_JSON_ARRAY_NOT_FOUND = MAKE_SQLSTATE('2', '2', '0', '3', '9') as isize,
    ERRCODE_SQL_JSON_MEMBER_NOT_FOUND = MAKE_SQLSTATE('2', '2', '0', '3', 'A') as isize,
    ERRCODE_SQL_JSON_NUMBER_NOT_FOUND = MAKE_SQLSTATE('2', '2', '0', '3', 'B') as isize,
    ERRCODE_SQL_JSON_OBJECT_NOT_FOUND = MAKE_SQLSTATE('2', '2', '0', '3', 'C') as isize,
    ERRCODE_TOO_MANY_JSON_ARRAY_ELEMENTS = MAKE_SQLSTATE('2', '2', '0', '3', 'D') as isize,
    ERRCODE_TOO_MANY_JSON_OBJECT_MEMBERS = MAKE_SQLSTATE('2', '2', '0', '3', 'E') as isize,
    ERRCODE_SQL_JSON_SCALAR_REQUIRED = MAKE_SQLSTATE('2', '2', '0', '3', 'F') as isize,

    /// Class 23 - Integrity Constraint Violation
    ERRCODE_INTEGRITY_CONSTRAINT_VIOLATION = MAKE_SQLSTATE('2', '3', '0', '0', '0') as isize,
    ERRCODE_RESTRICT_VIOLATION = MAKE_SQLSTATE('2', '3', '0', '0', '1') as isize,
    ERRCODE_NOT_NULL_VIOLATION = MAKE_SQLSTATE('2', '3', '5', '0', '2') as isize,
    ERRCODE_FOREIGN_KEY_VIOLATION = MAKE_SQLSTATE('2', '3', '5', '0', '3') as isize,
    ERRCODE_UNIQUE_VIOLATION = MAKE_SQLSTATE('2', '3', '5', '0', '5') as isize,
    ERRCODE_CHECK_VIOLATION = MAKE_SQLSTATE('2', '3', '5', '1', '4') as isize,
    ERRCODE_EXCLUSION_VIOLATION = MAKE_SQLSTATE('2', '3', 'P', '0', '1') as isize,

    /// Class 24 - Invalid Cursor State
    ERRCODE_INVALID_CURSOR_STATE = MAKE_SQLSTATE('2', '4', '0', '0', '0') as isize,

    /// Class 25 - Invalid Transaction State
    ERRCODE_INVALID_TRANSACTION_STATE = MAKE_SQLSTATE('2', '5', '0', '0', '0') as isize,
    ERRCODE_ACTIVE_SQL_TRANSACTION = MAKE_SQLSTATE('2', '5', '0', '0', '1') as isize,
    ERRCODE_BRANCH_TRANSACTION_ALREADY_ACTIVE = MAKE_SQLSTATE('2', '5', '0', '0', '2') as isize,
    ERRCODE_HELD_CURSOR_REQUIRES_SAME_ISOLATION_LEVEL =
        MAKE_SQLSTATE('2', '5', '0', '0', '8') as isize,
    ERRCODE_INAPPROPRIATE_ACCESS_MODE_FOR_BRANCH_TRANSACTION =
        MAKE_SQLSTATE('2', '5', '0', '0', '3') as isize,
    ERRCODE_INAPPROPRIATE_ISOLATION_LEVEL_FOR_BRANCH_TRANSACTION =
        MAKE_SQLSTATE('2', '5', '0', '0', '4') as isize,
    ERRCODE_NO_ACTIVE_SQL_TRANSACTION_FOR_BRANCH_TRANSACTION =
        MAKE_SQLSTATE('2', '5', '0', '0', '5') as isize,
    ERRCODE_READ_ONLY_SQL_TRANSACTION = MAKE_SQLSTATE('2', '5', '0', '0', '6') as isize,
    ERRCODE_SCHEMA_AND_DATA_STATEMENT_MIXING_NOT_SUPPORTED =
        MAKE_SQLSTATE('2', '5', '0', '0', '7') as isize,
    ERRCODE_NO_ACTIVE_SQL_TRANSACTION = MAKE_SQLSTATE('2', '5', 'P', '0', '1') as isize,
    ERRCODE_IN_FAILED_SQL_TRANSACTION = MAKE_SQLSTATE('2', '5', 'P', '0', '2') as isize,
    ERRCODE_IDLE_IN_TRANSACTION_SESSION_TIMEOUT = MAKE_SQLSTATE('2', '5', 'P', '0', '3') as isize,

    /// Class 26 - Invalid SQL Statement Name
    ERRCODE_INVALID_SQL_STATEMENT_NAME = MAKE_SQLSTATE('2', '6', '0', '0', '0') as isize,

    /// Class 27 - Triggered Data Change Violation
    ERRCODE_TRIGGERED_DATA_CHANGE_VIOLATION = MAKE_SQLSTATE('2', '7', '0', '0', '0') as isize,

    /// Class 28 - Invalid Authorization Specification
    ERRCODE_INVALID_AUTHORIZATION_SPECIFICATION = MAKE_SQLSTATE('2', '8', '0', '0', '0') as isize,
    ERRCODE_INVALID_PASSWORD = MAKE_SQLSTATE('2', '8', 'P', '0', '1') as isize,

    /// Class 2B - Dependent Privilege Descriptors Still Exist
    ERRCODE_DEPENDENT_PRIVILEGE_DESCRIPTORS_STILL_EXIST =
        MAKE_SQLSTATE('2', 'B', '0', '0', '0') as isize,
    ERRCODE_DEPENDENT_OBJECTS_STILL_EXIST = MAKE_SQLSTATE('2', 'B', 'P', '0', '1') as isize,

    /// Class 2D - Invalid Transaction Termination
    ERRCODE_INVALID_TRANSACTION_TERMINATION = MAKE_SQLSTATE('2', 'D', '0', '0', '0') as isize,

    /// Class 2F - SQL Routine Exception
    ERRCODE_SQL_ROUTINE_EXCEPTION = MAKE_SQLSTATE('2', 'F', '0', '0', '0') as isize,
    ERRCODE_S_R_E_FUNCTION_EXECUTED_NO_RETURN_STATEMENT =
        MAKE_SQLSTATE('2', 'F', '0', '0', '5') as isize,
    ERRCODE_S_R_E_MODIFYING_SQL_DATA_NOT_PERMITTED =
        MAKE_SQLSTATE('2', 'F', '0', '0', '2') as isize,
    ERRCODE_S_R_E_PROHIBITED_SQL_STATEMENT_ATTEMPTED =
        MAKE_SQLSTATE('2', 'F', '0', '0', '3') as isize,
    ERRCODE_S_R_E_READING_SQL_DATA_NOT_PERMITTED = MAKE_SQLSTATE('2', 'F', '0', '0', '4') as isize,

    /// Class 34 - Invalid Cursor Name
    ERRCODE_INVALID_CURSOR_NAME = MAKE_SQLSTATE('3', '4', '0', '0', '0') as isize,

    /// Class 38 - External Routine Exception
    ERRCODE_EXTERNAL_ROUTINE_EXCEPTION = MAKE_SQLSTATE('3', '8', '0', '0', '0') as isize,
    ERRCODE_E_R_E_CONTAINING_SQL_NOT_PERMITTED = MAKE_SQLSTATE('3', '8', '0', '0', '1') as isize,
    ERRCODE_E_R_E_MODIFYING_SQL_DATA_NOT_PERMITTED =
        MAKE_SQLSTATE('3', '8', '0', '0', '2') as isize,
    ERRCODE_E_R_E_PROHIBITED_SQL_STATEMENT_ATTEMPTED =
        MAKE_SQLSTATE('3', '8', '0', '0', '3') as isize,
    ERRCODE_E_R_E_READING_SQL_DATA_NOT_PERMITTED = MAKE_SQLSTATE('3', '8', '0', '0', '4') as isize,

    /// Class 39 - External Routine Invocation Exception
    ERRCODE_EXTERNAL_ROUTINE_INVOCATION_EXCEPTION = MAKE_SQLSTATE('3', '9', '0', '0', '0') as isize,
    ERRCODE_E_R_I_E_INVALID_SQLSTATE_RETURNED = MAKE_SQLSTATE('3', '9', '0', '0', '1') as isize,
    ERRCODE_E_R_I_E_NULL_VALUE_NOT_ALLOWED = MAKE_SQLSTATE('3', '9', '0', '0', '4') as isize,
    ERRCODE_E_R_I_E_TRIGGER_PROTOCOL_VIOLATED = MAKE_SQLSTATE('3', '9', 'P', '0', '1') as isize,
    ERRCODE_E_R_I_E_SRF_PROTOCOL_VIOLATED = MAKE_SQLSTATE('3', '9', 'P', '0', '2') as isize,
    ERRCODE_E_R_I_E_EVENT_TRIGGER_PROTOCOL_VIOLATED =
        MAKE_SQLSTATE('3', '9', 'P', '0', '3') as isize,

    /// Class 3B - Savepoint Exception
    ERRCODE_SAVEPOINT_EXCEPTION = MAKE_SQLSTATE('3', 'B', '0', '0', '0') as isize,
    ERRCODE_S_E_INVALID_SPECIFICATION = MAKE_SQLSTATE('3', 'B', '0', '0', '1') as isize,

    /// Class 3D - Invalid Catalog Name
    ERRCODE_INVALID_CATALOG_NAME = MAKE_SQLSTATE('3', 'D', '0', '0', '0') as isize,

    /// Class 3F - Invalid Schema Name
    ERRCODE_INVALID_SCHEMA_NAME = MAKE_SQLSTATE('3', 'F', '0', '0', '0') as isize,

    /// Class 40 - Transaction Rollback
    ERRCODE_TRANSACTION_ROLLBACK = MAKE_SQLSTATE('4', '0', '0', '0', '0') as isize,
    ERRCODE_T_R_INTEGRITY_CONSTRAINT_VIOLATION = MAKE_SQLSTATE('4', '0', '0', '0', '2') as isize,
    ERRCODE_T_R_SERIALIZATION_FAILURE = MAKE_SQLSTATE('4', '0', '0', '0', '1') as isize,
    ERRCODE_T_R_STATEMENT_COMPLETION_UNKNOWN = MAKE_SQLSTATE('4', '0', '0', '0', '3') as isize,
    ERRCODE_T_R_DEADLOCK_DETECTED = MAKE_SQLSTATE('4', '0', 'P', '0', '1') as isize,

    /// Class 42 - Syntax Error or Access Rule Violation
    ERRCODE_SYNTAX_ERROR_OR_ACCESS_RULE_VIOLATION = MAKE_SQLSTATE('4', '2', '0', '0', '0') as isize,
    ERRCODE_SYNTAX_ERROR = MAKE_SQLSTATE('4', '2', '6', '0', '1') as isize,
    ERRCODE_INSUFFICIENT_PRIVILEGE = MAKE_SQLSTATE('4', '2', '5', '0', '1') as isize,
    ERRCODE_CANNOT_COERCE = MAKE_SQLSTATE('4', '2', '8', '4', '6') as isize,
    ERRCODE_GROUPING_ERROR = MAKE_SQLSTATE('4', '2', '8', '0', '3') as isize,
    ERRCODE_WINDOWING_ERROR = MAKE_SQLSTATE('4', '2', 'P', '2', '0') as isize,
    ERRCODE_INVALID_RECURSION = MAKE_SQLSTATE('4', '2', 'P', '1', '9') as isize,
    ERRCODE_INVALID_FOREIGN_KEY = MAKE_SQLSTATE('4', '2', '8', '3', '0') as isize,
    ERRCODE_INVALID_NAME = MAKE_SQLSTATE('4', '2', '6', '0', '2') as isize,
    ERRCODE_NAME_TOO_LONG = MAKE_SQLSTATE('4', '2', '6', '2', '2') as isize,
    ERRCODE_RESERVED_NAME = MAKE_SQLSTATE('4', '2', '9', '3', '9') as isize,
    ERRCODE_DATATYPE_MISMATCH = MAKE_SQLSTATE('4', '2', '8', '0', '4') as isize,
    ERRCODE_INDETERMINATE_DATATYPE = MAKE_SQLSTATE('4', '2', 'P', '1', '8') as isize,
    ERRCODE_COLLATION_MISMATCH = MAKE_SQLSTATE('4', '2', 'P', '2', '1') as isize,
    ERRCODE_INDETERMINATE_COLLATION = MAKE_SQLSTATE('4', '2', 'P', '2', '2') as isize,
    ERRCODE_WRONG_OBJECT_TYPE = MAKE_SQLSTATE('4', '2', '8', '0', '9') as isize,
    ERRCODE_GENERATED_ALWAYS = MAKE_SQLSTATE('4', '2', '8', 'C', '9') as isize,
    ERRCODE_UNDEFINED_COLUMN = MAKE_SQLSTATE('4', '2', '7', '0', '3') as isize,
    //    ERRCODE_UNDEFINED_CURSOR = MAKE_SQLSTATE('3', '4', '0', '0', '0') as isize,
    //    ERRCODE_UNDEFINED_DATABASE = MAKE_SQLSTATE('3', 'D', '0', '0', '0') as isize,
    ERRCODE_UNDEFINED_FUNCTION = MAKE_SQLSTATE('4', '2', '8', '8', '3') as isize,
    //    ERRCODE_UNDEFINED_PSTATEMENT = MAKE_SQLSTATE('2', '6', '0', '0', '0') as isize,
    //    ERRCODE_UNDEFINED_SCHEMA = MAKE_SQLSTATE('3', 'F', '0', '0', '0') as isize,
    ERRCODE_UNDEFINED_TABLE = MAKE_SQLSTATE('4', '2', 'P', '0', '1') as isize,
    ERRCODE_UNDEFINED_PARAMETER = MAKE_SQLSTATE('4', '2', 'P', '0', '2') as isize,
    ERRCODE_UNDEFINED_OBJECT = MAKE_SQLSTATE('4', '2', '7', '0', '4') as isize,
    ERRCODE_DUPLICATE_COLUMN = MAKE_SQLSTATE('4', '2', '7', '0', '1') as isize,
    ERRCODE_DUPLICATE_CURSOR = MAKE_SQLSTATE('4', '2', 'P', '0', '3') as isize,
    ERRCODE_DUPLICATE_DATABASE = MAKE_SQLSTATE('4', '2', 'P', '0', '4') as isize,
    ERRCODE_DUPLICATE_FUNCTION = MAKE_SQLSTATE('4', '2', '7', '2', '3') as isize,
    ERRCODE_DUPLICATE_PSTATEMENT = MAKE_SQLSTATE('4', '2', 'P', '0', '5') as isize,
    ERRCODE_DUPLICATE_SCHEMA = MAKE_SQLSTATE('4', '2', 'P', '0', '6') as isize,
    ERRCODE_DUPLICATE_TABLE = MAKE_SQLSTATE('4', '2', 'P', '0', '7') as isize,
    ERRCODE_DUPLICATE_ALIAS = MAKE_SQLSTATE('4', '2', '7', '1', '2') as isize,
    ERRCODE_DUPLICATE_OBJECT = MAKE_SQLSTATE('4', '2', '7', '1', '0') as isize,
    ERRCODE_AMBIGUOUS_COLUMN = MAKE_SQLSTATE('4', '2', '7', '0', '2') as isize,
    ERRCODE_AMBIGUOUS_FUNCTION = MAKE_SQLSTATE('4', '2', '7', '2', '5') as isize,
    ERRCODE_AMBIGUOUS_PARAMETER = MAKE_SQLSTATE('4', '2', 'P', '0', '8') as isize,
    ERRCODE_AMBIGUOUS_ALIAS = MAKE_SQLSTATE('4', '2', 'P', '0', '9') as isize,
    ERRCODE_INVALID_COLUMN_REFERENCE = MAKE_SQLSTATE('4', '2', 'P', '1', '0') as isize,
    ERRCODE_INVALID_COLUMN_DEFINITION = MAKE_SQLSTATE('4', '2', '6', '1', '1') as isize,
    ERRCODE_INVALID_CURSOR_DEFINITION = MAKE_SQLSTATE('4', '2', 'P', '1', '1') as isize,
    ERRCODE_INVALID_DATABASE_DEFINITION = MAKE_SQLSTATE('4', '2', 'P', '1', '2') as isize,
    ERRCODE_INVALID_FUNCTION_DEFINITION = MAKE_SQLSTATE('4', '2', 'P', '1', '3') as isize,
    ERRCODE_INVALID_PSTATEMENT_DEFINITION = MAKE_SQLSTATE('4', '2', 'P', '1', '4') as isize,
    ERRCODE_INVALID_SCHEMA_DEFINITION = MAKE_SQLSTATE('4', '2', 'P', '1', '5') as isize,
    ERRCODE_INVALID_TABLE_DEFINITION = MAKE_SQLSTATE('4', '2', 'P', '1', '6') as isize,
    ERRCODE_INVALID_OBJECT_DEFINITION = MAKE_SQLSTATE('4', '2', 'P', '1', '7') as isize,

    /// Class 44 - WITH CHECK OPTION Violation
    ERRCODE_WITH_CHECK_OPTION_VIOLATION = MAKE_SQLSTATE('4', '4', '0', '0', '0') as isize,

    /// Class 53 - Insufficient Resources
    ERRCODE_INSUFFICIENT_RESOURCES = MAKE_SQLSTATE('5', '3', '0', '0', '0') as isize,
    ERRCODE_DISK_FULL = MAKE_SQLSTATE('5', '3', '1', '0', '0') as isize,
    ERRCODE_OUT_OF_MEMORY = MAKE_SQLSTATE('5', '3', '2', '0', '0') as isize,
    ERRCODE_TOO_MANY_CONNECTIONS = MAKE_SQLSTATE('5', '3', '3', '0', '0') as isize,
    ERRCODE_CONFIGURATION_LIMIT_EXCEEDED = MAKE_SQLSTATE('5', '3', '4', '0', '0') as isize,

    /// Class 54 - Program Limit Exceeded
    ERRCODE_PROGRAM_LIMIT_EXCEEDED = MAKE_SQLSTATE('5', '4', '0', '0', '0') as isize,
    ERRCODE_STATEMENT_TOO_COMPLEX = MAKE_SQLSTATE('5', '4', '0', '0', '1') as isize,
    ERRCODE_TOO_MANY_COLUMNS = MAKE_SQLSTATE('5', '4', '0', '1', '1') as isize,
    ERRCODE_TOO_MANY_ARGUMENTS = MAKE_SQLSTATE('5', '4', '0', '2', '3') as isize,

    /// Class 55 - Object Not In Prerequisite State
    ERRCODE_OBJECT_NOT_IN_PREREQUISITE_STATE = MAKE_SQLSTATE('5', '5', '0', '0', '0') as isize,
    ERRCODE_OBJECT_IN_USE = MAKE_SQLSTATE('5', '5', '0', '0', '6') as isize,
    ERRCODE_CANT_CHANGE_RUNTIME_PARAM = MAKE_SQLSTATE('5', '5', 'P', '0', '2') as isize,
    ERRCODE_LOCK_NOT_AVAILABLE = MAKE_SQLSTATE('5', '5', 'P', '0', '3') as isize,
    ERRCODE_UNSAFE_NEW_ENUM_VALUE_USAGE = MAKE_SQLSTATE('5', '5', 'P', '0', '4') as isize,

    /// Class 57 - Operator Intervention
    ERRCODE_OPERATOR_INTERVENTION = MAKE_SQLSTATE('5', '7', '0', '0', '0') as isize,
    ERRCODE_QUERY_CANCELED = MAKE_SQLSTATE('5', '7', '0', '1', '4') as isize,
    ERRCODE_ADMIN_SHUTDOWN = MAKE_SQLSTATE('5', '7', 'P', '0', '1') as isize,
    ERRCODE_CRASH_SHUTDOWN = MAKE_SQLSTATE('5', '7', 'P', '0', '2') as isize,
    ERRCODE_CANNOT_CONNECT_NOW = MAKE_SQLSTATE('5', '7', 'P', '0', '3') as isize,
    ERRCODE_DATABASE_DROPPED = MAKE_SQLSTATE('5', '7', 'P', '0', '4') as isize,

    /// Class 58 - System Error (errors external to PostgreSQL itself) as isize,
    ERRCODE_SYSTEM_ERROR = MAKE_SQLSTATE('5', '8', '0', '0', '0') as isize,
    ERRCODE_IO_ERROR = MAKE_SQLSTATE('5', '8', '0', '3', '0') as isize,
    ERRCODE_UNDEFINED_FILE = MAKE_SQLSTATE('5', '8', 'P', '0', '1') as isize,
    ERRCODE_DUPLICATE_FILE = MAKE_SQLSTATE('5', '8', 'P', '0', '2') as isize,

    /// Class 72 - Snapshot Failure
    ERRCODE_SNAPSHOT_TOO_OLD = MAKE_SQLSTATE('7', '2', '0', '0', '0') as isize,

    /// Class F0 - Configuration File Error
    ERRCODE_CONFIG_FILE_ERROR = MAKE_SQLSTATE('F', '0', '0', '0', '0') as isize,
    ERRCODE_LOCK_FILE_EXISTS = MAKE_SQLSTATE('F', '0', '0', '0', '1') as isize,

    /// Class HV - Foreign Data Wrapper Error (SQL/MED) as isize,
    ERRCODE_FDW_ERROR = MAKE_SQLSTATE('H', 'V', '0', '0', '0') as isize,
    ERRCODE_FDW_COLUMN_NAME_NOT_FOUND = MAKE_SQLSTATE('H', 'V', '0', '0', '5') as isize,
    ERRCODE_FDW_DYNAMIC_PARAMETER_VALUE_NEEDED = MAKE_SQLSTATE('H', 'V', '0', '0', '2') as isize,
    ERRCODE_FDW_FUNCTION_SEQUENCE_ERROR = MAKE_SQLSTATE('H', 'V', '0', '1', '0') as isize,
    ERRCODE_FDW_INCONSISTENT_DESCRIPTOR_INFORMATION =
        MAKE_SQLSTATE('H', 'V', '0', '2', '1') as isize,
    ERRCODE_FDW_INVALID_ATTRIBUTE_VALUE = MAKE_SQLSTATE('H', 'V', '0', '2', '4') as isize,
    ERRCODE_FDW_INVALID_COLUMN_NAME = MAKE_SQLSTATE('H', 'V', '0', '0', '7') as isize,
    ERRCODE_FDW_INVALID_COLUMN_NUMBER = MAKE_SQLSTATE('H', 'V', '0', '0', '8') as isize,
    ERRCODE_FDW_INVALID_DATA_TYPE = MAKE_SQLSTATE('H', 'V', '0', '0', '4') as isize,
    ERRCODE_FDW_INVALID_DATA_TYPE_DESCRIPTORS = MAKE_SQLSTATE('H', 'V', '0', '0', '6') as isize,
    ERRCODE_FDW_INVALID_DESCRIPTOR_FIELD_IDENTIFIER =
        MAKE_SQLSTATE('H', 'V', '0', '9', '1') as isize,
    ERRCODE_FDW_INVALID_HANDLE = MAKE_SQLSTATE('H', 'V', '0', '0', 'B') as isize,
    ERRCODE_FDW_INVALID_OPTION_INDEX = MAKE_SQLSTATE('H', 'V', '0', '0', 'C') as isize,
    ERRCODE_FDW_INVALID_OPTION_NAME = MAKE_SQLSTATE('H', 'V', '0', '0', 'D') as isize,
    ERRCODE_FDW_INVALID_STRING_LENGTH_OR_BUFFER_LENGTH =
        MAKE_SQLSTATE('H', 'V', '0', '9', '0') as isize,
    ERRCODE_FDW_INVALID_STRING_FORMAT = MAKE_SQLSTATE('H', 'V', '0', '0', 'A') as isize,
    ERRCODE_FDW_INVALID_USE_OF_NULL_POINTER = MAKE_SQLSTATE('H', 'V', '0', '0', '9') as isize,
    ERRCODE_FDW_TOO_MANY_HANDLES = MAKE_SQLSTATE('H', 'V', '0', '1', '4') as isize,
    ERRCODE_FDW_OUT_OF_MEMORY = MAKE_SQLSTATE('H', 'V', '0', '0', '1') as isize,
    ERRCODE_FDW_NO_SCHEMAS = MAKE_SQLSTATE('H', 'V', '0', '0', 'P') as isize,
    ERRCODE_FDW_OPTION_NAME_NOT_FOUND = MAKE_SQLSTATE('H', 'V', '0', '0', 'J') as isize,
    ERRCODE_FDW_REPLY_HANDLE = MAKE_SQLSTATE('H', 'V', '0', '0', 'K') as isize,
    ERRCODE_FDW_SCHEMA_NOT_FOUND = MAKE_SQLSTATE('H', 'V', '0', '0', 'Q') as isize,
    ERRCODE_FDW_TABLE_NOT_FOUND = MAKE_SQLSTATE('H', 'V', '0', '0', 'R') as isize,
    ERRCODE_FDW_UNABLE_TO_CREATE_EXECUTION = MAKE_SQLSTATE('H', 'V', '0', '0', 'L') as isize,
    ERRCODE_FDW_UNABLE_TO_CREATE_REPLY = MAKE_SQLSTATE('H', 'V', '0', '0', 'M') as isize,
    ERRCODE_FDW_UNABLE_TO_ESTABLISH_CONNECTION = MAKE_SQLSTATE('H', 'V', '0', '0', 'N') as isize,

    /// Class P0 - PL/pgSQL Error
    ERRCODE_PLPGSQL_ERROR = MAKE_SQLSTATE('P', '0', '0', '0', '0') as isize,
    ERRCODE_RAISE_EXCEPTION = MAKE_SQLSTATE('P', '0', '0', '0', '1') as isize,
    ERRCODE_NO_DATA_FOUND = MAKE_SQLSTATE('P', '0', '0', '0', '2') as isize,
    ERRCODE_TOO_MANY_ROWS = MAKE_SQLSTATE('P', '0', '0', '0', '3') as isize,
    ERRCODE_ASSERT_FAILURE = MAKE_SQLSTATE('P', '0', '0', '0', '4') as isize,

    /// Class XX - Internal Error
    ERRCODE_INTERNAL_ERROR = MAKE_SQLSTATE('X', 'X', '0', '0', '0') as isize,
    ERRCODE_DATA_CORRUPTED = MAKE_SQLSTATE('X', 'X', '0', '0', '1') as isize,
    ERRCODE_INDEX_CORRUPTED = MAKE_SQLSTATE('X', 'X', '0', '0', '2') as isize,
}

#[allow(non_snake_case)]
#[inline]
const fn PGSIXBIT(ch: i32) -> i32 {
    (((ch) - '0' as i32) & 0x3F) as i32
}

#[allow(non_snake_case)]
#[inline]
const fn MAKE_SQLSTATE(ch1: char, ch2: char, ch3: char, ch4: char, ch5: char) -> i32 {
    (PGSIXBIT(ch1 as i32)
        + (PGSIXBIT(ch2 as i32) << 6)
        + (PGSIXBIT(ch3 as i32) << 12)
        + (PGSIXBIT(ch4 as i32) << 18)
        + (PGSIXBIT(ch5 as i32) << 24)) as i32
}

/// Emit a Postgres log message.
///
/// Log messages of level `pg_sys::ERROR` will cause the current transaction to abort
pub fn elog(level: PgLogLevel, message: &str) {
    use std::ffi::CString;
    use std::os::raw::c_char;

    unsafe {
        extern "C" {
            fn pgx_elog(level: i32, message: *const c_char);
        }

        match CString::new(message) {
            Ok(s) => crate::guard(|| pgx_elog(level as i32, s.as_ptr())),
            Err(_) => crate::guard(|| {
                pgx_elog(
                    level as i32,
                    std::ffi::CStr::from_bytes_with_nul(b"log message was null\0")
                        .unwrap()
                        .as_ptr(),
                )
            }),
        }
    }
}

/// Emit a Postgres `ereport` message.
///
/// Messages of level `pg_sys::ERROR` will cause the current transaction to abort
pub fn ereport(
    level: PgLogLevel,
    code: PgSqlErrorCode,
    message: &str,
    file: &str,
    lineno: u32,
    colno: u32,
) {
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::os::raw::c_char;

    extern "C" {
        fn pgx_ereport(
            level: i32,
            code: i32,
            message: *const c_char,
            file: *const c_char,
            lineno: i32,
            colno: i32,
        );
    }

    let message = match CString::new(message) {
        Ok(s) => s,
        Err(_) => CString::from(
            CStr::from_bytes_with_nul(b"error message was null\0")
                .expect("hardcoded error message failed"),
        ),
    };

    let file = match CString::new(file) {
        Ok(f) => f,
        Err(_) => CString::from(
            CStr::from_bytes_with_nul(b"filename was null\0")
                .expect("hardcoded error message failed"),
        ),
    };

    unsafe {
        crate::guard(|| {
            pgx_ereport(
                level as i32,
                code as i32,
                message.as_ptr(),
                file.as_ptr(),
                lineno as i32,
                colno as i32,
            );
        });
    }
}

/// Log to Postgres' `debug5` log level.
///
/// This macro accepts arguments like the [`println`](std::println) and [`format`](std::format) macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// The output these logs goes to the PostgreSQL log file at `DEBUG5` level, depending on how the 
/// [PostgreSQL settings](https://www.postgresql.org/docs/current/runtime-config-logging.html) are configured.
#[macro_export]
macro_rules! debug5 {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::PgLogLevel::DEBUG5, format!($($arg)*).as_str());
    )
}

/// Log to Postgres' `debug4` log level.
///
/// This macro accepts arguments like the [`println`](std::println) and [`format`](std::format) macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// The output these logs goes to the PostgreSQL log file at `DEBUG4` level, depending on how the 
/// [PostgreSQL settings](https://www.postgresql.org/docs/current/runtime-config-logging.html) are configured.
#[macro_export]
macro_rules! debug4 {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::PgLogLevel::DEBUG4, format!($($arg)*).as_str());
    )
}

/// Log to Postgres' `debug3` log level.
///
/// This macro accepts arguments like the [`println`](std::println) and [`format`](std::format) macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// The output these logs goes to the PostgreSQL log file at `DEBUG3` level, depending on how the 
/// [PostgreSQL settings](https://www.postgresql.org/docs/current/runtime-config-logging.html) are configured.
#[macro_export]
macro_rules! debug3 {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::PgLogLevel::DEBUG3, format!($($arg)*).as_str());
    )
}

/// Log to Postgres' `debug2` log level.
///
/// This macro accepts arguments like the [`println`](std::println) and [`format`](std::format) macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// The output these logs goes to the PostgreSQL log file at `DEBUG2` level, depending on how the 
/// [PostgreSQL settings](https://www.postgresql.org/docs/current/runtime-config-logging.html) are configured.
#[macro_export]
macro_rules! debug2 {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::PgLogLevel::DEBUG2, format!($($arg)*).as_str());
    )
}

/// Log to Postgres' `debug1` log level.
///
/// This macro accepts arguments like the [`println`](std::println) and [`format`](std::format) macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// The output these logs goes to the PostgreSQL log file at `DEBUG1` level, depending on how the 
/// [PostgreSQL settings](https://www.postgresql.org/docs/current/runtime-config-logging.html) are configured.
#[macro_export]
macro_rules! debug1 {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::PgLogLevel::DEBUG1, format!($($arg)*).as_str());
    )
}

/// Log to Postgres' `log` log level.
///
/// This macro accepts arguments like the [`println`](std::println) and [`format`](std::format) macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// The output these logs goes to the PostgreSQL log file at `LOG` level, depending on how the 
/// [PostgreSQL settings](https://www.postgresql.org/docs/current/runtime-config-logging.html) are configured.
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::PgLogLevel::LOG, format!($($arg)*).as_str());
    )
}

/// Log to Postgres' `info` log level.
///
/// This macro accepts arguments like the [`println`](std::println) and [`format`](std::format) macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// Given some function:
///
/// ```rust,no_run
/// use pgx::*;
///
/// #[pg_extern]
/// fn sum_array(input: Array<i32>) -> i64 {
///     let mut sum = 0 as i64;
/// 
///     for i in input {
///         pgx::info!("i={index:?}, sum={}", sum, index = i);
///         sum += i.unwrap_or(-1) as i64;
///     }
/// 
///     sum
/// }
/// ```
///
/// When run inside PostgreSQL would output:
///
/// ```sql
/// arrays=# SELECT arrays.sum_array('{1,2,3}');
/// INFO:  i=Some(1), sum=0
/// INFO:  i=Some(2), sum=1
/// INFO:  i=Some(3), sum=3
///  sum_array 
/// -----------
///          6
/// (1 row)
/// ```
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::PgLogLevel::INFO, format!($($arg)*).as_str());
    )
}

/// Log to Postgres' `notice` log level.
///
/// This macro accepts arguments like the [`println`](std::println) and [`format`](std::format) macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// Given some function:
///
/// ```rust,no_run
/// use pgx::*;
///
/// #[pg_extern]
/// fn sum_array(input: Array<i32>) -> i64 {
///     let mut sum = 0 as i64;
/// 
///     for i in input {
///         pgx::notice!("i={index:?}, sum={}", sum, index = i);
///         sum += i.unwrap_or(-1) as i64;
///     }
/// 
///     sum
/// }
/// ```
///
/// When run inside PostgreSQL would output:
///
/// ```sql
/// arrays=# SELECT arrays.sum_array('{1,2,3}');
/// NOTICE:  i=Some(1), sum=0
/// NOTICE:  i=Some(2), sum=1
/// NOTICE:  i=Some(3), sum=3
///  sum_array 
/// -----------
///          6
/// (1 row)
/// ```
#[macro_export]
macro_rules! notice {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::PgLogLevel::NOTICE, format!($($arg)*).as_str());
    )
}

/// Log to Postgres' `warning` log level.
///
/// This macro accepts arguments like the [`println`](std::println) and [`format`](std::format) macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// Given some function:
///
/// ```rust,no_run
/// use pgx::*;
///
/// #[pg_extern]
/// fn sum_array(input: Array<i32>) -> i64 {
///     let mut sum = 0 as i64;
/// 
///     for i in input {
///         pgx::warning!("i={index:?}, sum={}", sum, index = i);
///         sum += i.unwrap_or(-1) as i64;
///     }
/// 
///     sum
/// }
/// ```
///
/// When run inside PostgreSQL would output:
///
/// ```sql
/// arrays=# SELECT arrays.sum_array('{1,2,3}');
/// WARNING:  i=Some(1), sum=0
/// WARNING:  i=Some(2), sum=1
/// WARNING:  i=Some(3), sum=3
///  sum_array 
/// -----------
///          6
/// (1 row)
/// ```
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => (
        $crate::log::elog($crate::log::PgLogLevel::WARNING, format!($($arg)*).as_str());
    )
}

/// Log to Postgres' `error` log level.  This will abort the current Postgres transaction.
///
/// This macro accepts arguments like the [`println`](std::println) and [`format`](std::format) macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// Given some function:
///
/// ```rust,no_run
/// use pgx::*;
///
/// #[pg_extern]
/// fn sum_array(input: Array<i32>) -> i64 {
///     let mut sum = 0 as i64;
/// 
///     for i in input {
///         pgx::error!("i={index:?}, sum={}", sum, index = i);
///         sum += i.unwrap_or(-1) as i64;
///     }
/// 
///     sum
/// }
/// ```
///
/// When run inside PostgreSQL would output:
///
/// ```sql
/// arrays=# SELECT arrays.sum_array('{1,2,3}');
/// ERROR:  i=Some(1), sum=0
/// CONTEXT:  src/lib.rs:37:9
/// ```
#[macro_export]
macro_rules! error {
    () => ({ panic!("explicit ERROR") });
    ($msg:expr) => ({ panic!($msg) });
    ($msg:expr,) => ({ panic!($msg) });
    ($fmt:expr, $($arg:tt)+) => ({
        panic!($fmt, $($arg)+)
    });
}

/// Log to Postgres' `fatal` log level.  This will abort the current Postgres backend connection processs.
///
/// This macro accepts arguments like the [`println`](std::println) and [`format`](std::format) macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// Given some function:
///
/// ```rust,no_run
/// use pgx::*;
///
/// #[pg_extern]
/// fn sum_array(input: Array<i32>) -> i64 {
///     let mut sum = 0 as i64;
/// 
///     for i in input {
///         pgx::FATAL!("i={index:?}, sum={}", sum, index = i);
///         sum += i.unwrap_or(-1) as i64;
///     }
/// 
///     sum
/// }
/// ```
///
/// When run inside PostgreSQL would output:
///
/// ```sql
/// arrays=# SELECT arrays.sum_array('{1,2,3}');
/// FATAL:  i=Some(1), sum=0
/// server closed the connection unexpectedly
///         This probably means the server terminated abnormally
///         before or while processing the request.
/// The connection to the server was lost. Attempting reset: Succeeded.
/// ```
#[allow(non_snake_case)]
#[macro_export]
macro_rules! FATAL {
    ($($arg:tt)*) => (
        { $crate::log::elog($crate::log::PgLogLevel::FATAL, format!($($arg)*).as_str()); unreachable!("elog failed"); }
    )
}

/// Log to Postgres' `panic` log level.  This will cause the entire Postgres cluster to crash.
///
/// This macro accepts arguments like the [`println`](std::println) and [`format`](std::format) macros.
/// See [`fmt`](std::fmt) for information about options.
///
/// Given some function:
///
/// ```rust,no_run
/// use pgx::*;
///
/// #[pg_extern]
/// fn sum_array(input: Array<i32>) -> i64 {
///     let mut sum = 0 as i64;
/// 
///     for i in input {
///         pgx::PANIC!("i={index:?}, sum={}", sum, index = i);
///         sum += i.unwrap_or(-1) as i64;
///     }
/// 
///     sum
/// }
/// ```
///
/// When run inside PostgreSQL would output:
///
/// ```sql
/// arrays=# SELECT arrays.sum_array('{1,2,3}');
/// PANIC:  i=Some(1), sum=0
/// server closed the connection unexpectedly
///         This probably means the server terminated abnormally
///         before or while processing the request.
/// The connection to the server was lost. Attempting reset: Failed.
/// ```
#[allow(non_snake_case)]
#[macro_export]
macro_rules! PANIC {
    ($($arg:tt)*) => (
        { $crate::log::elog($crate::log::PgLogLevel::PANIC, format!($($arg)*).as_str()); unreachable!("elog failed"); }
    )
}

/// Emit a test output message for highlighting during `cargo pgx test`
#[cfg(test)]
#[macro_export]
macro_rules! testmsg {
    ($($arg:tt)*) => (
        eprintln!("{}", format!("TMSG: {}:{}:{}:  {}", file!(), line!(), column!(), format!($($arg)*)));
    )
}

/// Is an interrupt pending?
#[cfg(any(feature = "pg10", feature = "pg11"))]
#[inline]
pub fn interrupt_pending() -> bool {
    unsafe { crate::pg_sys::InterruptPending }
}

/// Is an interrupt pending?
#[cfg(any(feature = "pg12", feature = "pg13"))]
#[inline]
pub fn interrupt_pending() -> bool {
    unsafe { crate::pg_sys::InterruptPending != 0 }
}

/// If an interrupt is pending (perhaps a user-initiated "cancel query" message to this backend),
/// this will safely abort the current transaction
#[macro_export]
macro_rules! check_for_interrupts {
    () => {
        #[cfg(any(feature = "pg10", feature = "pg11"))]
        #[allow(unused_unsafe)]
        unsafe {
            if $crate::pg_sys::InterruptPending {
                $crate::pg_sys::ProcessInterrupts();
            }
        }

        #[cfg(feature = "pg12")]
        #[allow(unused_unsafe)]
        unsafe {
            if $crate::pg_sys::InterruptPending != 0 {
                $crate::pg_sys::ProcessInterrupts();
            }
        }
    };
}
