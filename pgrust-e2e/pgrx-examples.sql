-- pgrx examples under pgrust: strings, arrays, srf, json
\set ON_ERROR_STOP 0
CREATE EXTENSION strings;
SELECT strings.return_static();
SELECT strings.to_lowercase('HeLLo WoRLD');
SELECT strings.substring('hello world', 0, 5);
SELECT strings.append('hello', ' world');
SELECT strings.split('a,b,c', ',');
SELECT * FROM strings.split_set('a,b,c', ',');
SELECT * FROM strings.split_table('a,b,c', ',');

CREATE EXTENSION arrays;
SELECT arrays.sq_euclid_pgrx(ARRAY[1.0,2.0,3.0]::real[], ARRAY[4.0,6.0,8.0]::real[]);
SELECT arrays.sum_array(ARRAY[1,2,3,4]);
SELECT arrays.sum_array();
SELECT arrays.sum_vec(ARRAY[1,NULL,3]);
SELECT arrays.static_names();
SELECT * FROM arrays.static_names_set();
SELECT arrays.i32_array_no_nulls();
SELECT arrays.i32_array_with_nulls();
SELECT arrays.strip_nulls(ARRAY[1,NULL,3,NULL]);
SELECT arrays.return_vec_of_customtype();
SELECT vectors.sum_vector_array(ARRAY[1.5,2.5]::real[]);
SELECT vectors.sum_vector_vec(ARRAY[1.5,2.5]::real[]);
SELECT vectors.sum_vector_slice(ARRAY[1.5,2.5]::real[]);
SELECT array_length(vectors.random_vector(5), 1);

CREATE EXTENSION srf;
SELECT * FROM srf.generate_series(1, 5);
SELECT * FROM srf.generate_series(1, 10, 3);
SELECT * FROM srf.generate_series_table(1, 3);
SELECT count(*), min(index), max(index) FROM srf.random_values(4);
SELECT * FROM srf.result_table();
SELECT * FROM srf.one_col();
SELECT srf.generate_series(1, 3);

CREATE EXTENSION json;
SELECT text_array_to_json_doc(ARRAY['a','b']);
SELECT bytea_array_to_json_doc(ARRAY['\x0102'::bytea, '\x03'::bytea]);

-- spi example (extension spi_example, schema spi)
CREATE EXTENSION spi_example;
SELECT * FROM spi.spi_return_query() LIMIT 2;
SELECT spi.spi_query_by_id(1);
SELECT spi.spi_query_random_id() IS NOT NULL;
SELECT spi.spi_query_title('Hello There!');
SELECT spi.spi_insert_title('New Title');
SELECT spi.spi_insert_title2('New Title2');
SELECT * FROM spi.spi_example ORDER BY id;

-- aggregate example
CREATE EXTENSION aggregate;
CREATE TABLE demo_table (value INTEGER);
INSERT INTO demo_table (value) VALUES (1), (2), (3);
SELECT DEMOAVG(value) FROM demo_table;
INSERT INTO demo_table (value) VALUES (NULL);
SELECT DEMOAVG(value) FROM demo_table;

-- composite_type example
CREATE EXTENSION composite_type;
SELECT create_dog('Fido', 42);
SELECT scritch_dog(create_dog('Fido', 42));
SELECT make_friendship(create_dog('Fido', 42), ROW('Tom', 3)::Cat);
SELECT add_scritches_to_dog(create_dog('Fido', 1), 5);

-- pg_jsonschema
CREATE EXTENSION pg_jsonschema;
SELECT json_matches_schema('{"type": "object"}', '{}');
SELECT jsonb_matches_schema('{"type": "object", "properties": {"a": {"type": "integer"}}}', '{"a": 1}');
SELECT jsonb_matches_schema('{"type": "object", "properties": {"a": {"type": "integer"}}}', '{"a": "x"}');
SELECT jsonschema_is_valid('{"type": "object"}');
SELECT jsonschema_is_valid('{"type": "bogus"}');
SELECT jsonschema_validation_errors('{"type": "object", "required": ["a"]}', '{}');
SELECT json_matches_compiled_schema('{"type": "object"}'::json::jsonschema, '{}');
SELECT jsonb_matches_compiled_schema('{"type": "object", "required": ["a"]}'::json::jsonschema, '{"b": 1}');
SELECT jsonb_matches_compiled_schema('{"type": "object", "required": ["a"]}'::json::jsonschema, '{"a": 1}');
SELECT jsonb_validation_errors_compiled('{"type": "object", "required": ["a"]}'::jsonb::jsonschema, '{}');

-- pg_graphql
CREATE EXTENSION pg_graphql;
CREATE TABLE account (id serial primary key, email varchar(255) not null);
INSERT INTO account (email) VALUES ('a@x.com'), ('b@x.com');
COMMENT ON SCHEMA public IS '@graphql({"inflect_names": true})';
SELECT graphql.resolve($$ { __typename } $$);
SELECT graphql.resolve($$ { accountCollection { edges { node { id email } } } } $$);
SELECT graphql.resolve($$ mutation { insertIntoAccountCollection(objects: [{email: "c@x.com"}]) { affectedCount } } $$);
SELECT count(*) FROM account;

-- guc_example (custom GUCs from _PG_init)
CREATE EXTENSION guc_example;
SELECT get_paranoia(), get_verbose(), get_ratio(), get_label();
SET guc_example.paranoia = 7;
SET guc_example.verbose = on;
SET guc_example.ratio = 0.25;
SET guc_example.label = 'hello';
SELECT get_paranoia(), get_verbose(), get_ratio(), get_label();
SHOW guc_example.paranoia;
RESET guc_example.paranoia;
SELECT get_paranoia();
SET guc_example.paranoia = 11;
