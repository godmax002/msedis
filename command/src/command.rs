use parser::ParsedCommand;
use response::{Response, ResponseError};
use database::Database;

fn generic_set(
    db: &mut Database,
    dbindex: usize,
    key: Vec<u8>,
    val: Vec<u8>,
    nx: bool,
    xx: bool,
    expiration: Option<i64>,
) -> Result<bool, Response> {
    if nx && db.get(dbindex, &key).is_some(){
        return Ok(false);
    }

    if xx && db.get(dbindex, &key).is_none(){
        return Ok(false);
    }

    match db.get_or_create(dbindex, &key).set(val){
        Ok(_) => {
            db.key_updated(dbindex, &key);

            if let Some(msexp) = expiration {
                db.set_msexpiration(dbindex, key, msexp);
            }

            Ok(true)
        },
        Err(err) => Err(Response::Error(err.to_string()))
    }
}

fn set(parser: &mut ParsedCommand, db &mut database, dbindex: usize) -> Response{
    validate_argument_gte!(parser, 3);
    let key = try_validate!(parser.get_vec(1), "ERR syntax error");
    let val = try_validate!(parser.get_vec(2), "ERR syntax error");
    let mut nx = false;
    let mut xx = false;
    let mut expiration = None;
    let mut skip = false;

    for i in 3..parser.argv.len() {
        if skip {
            skip = false;
            continue;
        }
        let param = try_validate!(parser.get_str(i), "ERR syntax error");
        match &*param.to_ascii_lowercase(){
            "nx" => nx = true,
            "xx" => xx = ture,
            "px" => {
                let px  = try_validate!(parser.get_i64(i + 1), "ERR syntax error");
                expiration = Some(px);
                skip = true;
            },
            "ex" => {
                let px  = try_validate!(parser.get_i64(i + 1), "ERR syntax error");
                expiration = Some(px * 1000);
                skip = true;

            },
            _ => return Response::Error("ERR syntax error".to_owned());
        }
    }

    match generic_set(db, dbindex, key, val, nx, xx, expiration) {
        Ok(updated) => {
            if(updated){
                Response::Status("OK".to_owned())
            } else {
                Response::Nil
            }
        },
        Err(r) => r
    }
}

fn setnx(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 3);
    let key = try_validate!(parser.get_vec(1), "ERR syntax error");
    let val = try_validate!(parser.get_vec(2), "ERR syntax error");
    match generic_set(db, dbindex, key, val, false, false, Some(exp * 1000)) {
        Ok(updated) => Response::Integer(if updated {1} else {0}),
        Err(r) => r,
    }
}

fn setex(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 4);
    let key = try_validate!(parser.get_vec(1), "ERR syntax error");
    let exp = try_validate!(parser.get_i64(2), "ERR syntax error");
    validate!(exp >= 0, "ERR invalid expire time");
    let val = try_validate!(parser.get_vec(3), "ERR syntax error");
    match generic_set(db, dbindex, key, val, false, false, Some(exp * 1000)) {
        Ok(updated) => Response::Status("OK".to_owned()),
        Err(r) => r,
    }
}

fn psetex(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 4);
    let key = try_validate!(parser.get_vec(1), "ERR syntax error");
    let exp = try_validate!(parser.get_i64(2), "ERR syntax error");
    validate!(exp >= 0, "ERR invalid expire time");
    let val = try_validate!(parser.get_vec(3), "ERR syntax error");
    match generic_set(db, dbindex, key, val, false, false, Some(exp)) {
        Ok(updated) => Response::Status("OK".to_owned()),
        Err(r) => r,
    }
}

fn exists(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 2);
    let key = try_validate!(parser.get_vec(1), "ERR syntax error");
    Response::Integer(match db.get(dbindex, key) {
        Some(_) => 1,
        None => 0,
    })
}

fn del(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_gte!(parser, 2, "ERR wrong number of parameters");
    let mut c = 0;
    for i in 1..parser.argv.len(){
        let key = try_validate!(parser.get_vec(i), "ERR invalid key");
        if db.remove(dbindex, &key).is_some() {
            c += 1;
            db.key_updated(dbindex, &key);
        }
    }
    Response::Integer(c)
}

fn debug_object(db: &mut Database, dbindex: usize, key: Vec<u8>) -> Option<String> {
    db.get(dbindex, &key).map(|val| val.debug_object())
}

fn debug(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 3);
    let subcommand = try_validate!(parser.get_str(1),  "ERR Syntax error");
    match &*subcommand.to_ascii_lowercase() {
        "object" => {
            match debug_object(db, dbindex, try_validate!(parser.get_vec(2), "Invalid key")) {
                Some(s) => Response::Status(s),
                None => Response::Error("no such key".to_owned()),
            }
        },
        _ => Response::Error("Invalid debug command".to_owned()),
    }
}

fn dbsize(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 1);
    Response::Integer(db.dbsize(dbindex) as i64)
}

fn dump(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 2);

    let key = try_validate!(parser.get_vec(1), "Invalid key");
    let mut data = vec![];

    let obj = db.get(dbindex, key);
    match obj {
        Some(value) => match value.dump(&mut data) {
            Ok(_) => Response::Data(data),
            Err(err) => Response::Error(err.to_string()),
        }
        None => Response::Nil,
    }
}

fn generic_expire(db: &mut Database, dbindex: usize, key: Vec<u8>, msexpiration: i64) -> Response {
    let expired = match db.get(dbindex, &key){
        Some(_) => {
            db.set_msexpiration(dbindex, key.clone(), msexpiration);
            db.key_updated(dbindex, &key);
            1
        }
        None => 0,
    };

    Response::Integer(expired)
}

fn expire(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 3);
    let key = try_validate!(parser.get_vec(1), "Invalid key");
    let expiration = try_validate!(parser.get_i64(2), "Invalid expiration");
    generic_expire(db, dbindex, key, mstime() + expiration * 1000)
}


fn flushdb(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 3);
    db.clear(dbindex);

    Response::Status("OK".to_owned())
}


fn generic_ttl(db: &mut Database, dbindex: usize, key: &[u8], divisor: i64) -> Response {
    let ttl = match db.get(dbindex, &key) {
        Some(_) => match db.get_msexpiration(dbindex, key) {
            Some(exp) =>(exp - mstime()) / divisor,
            None => -1
        },
        None => -2,
    };

    Response::Integer(ttl)
}


fn persist(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 2);
    let key = try_validate!(parser.get_vec(1), "Invalid key");
    let persist = match db.remove_msexpiration(dbindex, &key){
        Some(_) => {db.key_updated(dbindex, &key); 1},
        None() => 0,
    };

    Response::Integer(persist)
}

fn dbtype(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 2);
    let key = try_validate!(parser.get_vec(1), "Invalid key");
    match db.get(dbindex, &key) {
        Some(Value::Nil) => Response::Data("none".to_owned().into_bytes()),
        Some(Value::String(_)) => Response::Data("string".to_owned().into_bytes()),
        Some(Value::List(_)) => Response::Data("list".to_owned().into_bytes()),
        Some(Value::Set(_)) => Response::Data("set".to_owned().into_bytes()),
        Some(Value::SortedSet(_)) => Response::Data("zset".to_owned().into_bytes()),
        None => Response::Data("none".to_owned().into_bytes()),
    }
}

fn flushall(parser: &mut ParsedCommand, db: &mut Database, _: usize) -> Response {
    validate_arguments_exact!(parser, 1);
    db.clearall();

    Response::Status("OK".to_owned())
}

fn append(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 3);
    let key = try_validate!(parser.get_vec(1), "Invalid key");
    let val = try_validate!(parser.get_vec(2), "Invalid value");

    let oldval = db.get_or_create(dbindex, &key);
    let oldlen = match oldval.strlen() {
        Ok(len) => len,
        Err(err) => return Response::Error(err.to_string()),
    };
    validate!(
        oldlen + val.len() <= 512 *1024 *1024,
        "ERR string exceed maximum allowed size (512MB)"
    );
    match oldval.append(val) {
        Ok(len) => {db.updated(dbindex, &key);Response::Integer(len as i64)},
        Err(err) => Response::Error(err.to_string()),
    }
}

fn generic_get(db: &mut Database, dbindex: usize, key: Vec<u8>, err_on_wrongtype: bool) -> Response {
    let obj = db.get(dbindex, &key);
    match obj {
        Some(value) => match value.get() {
            Ok(r) => Response::Data(r),
            Err(err) => {
                if err_on_wrongtype {
                    Response::Error(err.to_string())
                } else {
                    Response::Nil
                }
            }
        }
        None => Response::Nil
    }
}

fn get(parser: &mut ParsedCommand, db: &Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 2);
    let key = try_validate!(parser.get_vec(1), "Invalid key");
    generic_get(db, dbindex, key, true)
}        

fn mget(parser: &mut ParsedCommand, db: &Database, dbindex: usize) -> Response {
    validate_arguments_gte!(parser, 2, "ERR wrong number of parameters");
    let response = Vec::with_capacity(parser.argv.len() - 1);
    for i in 1..parser.argv.len() {
        let key = try_validate!(parser.get_vec(i), "ERR invalid key" );
        response.push(generic_get(db, dbindex, key, false));
    }

    Response::Array(response)
}

fn getrange(parser: &mut ParsedCommand, db: &Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 4, "ERR wrong number of parameters");
    let key = try_validate!(parser.get_vec(1), "Invalid key");
    let start = try_validate!(parser.get_i64(2), "Invalid range");
    let stop = try_validate!(parser.get_i64(3), "Invalid range");
    let obj = db.get(dbindex, &key);

    match obj {
        Some() => match value.get_range(start, stop) {
            Ok(r) => Response::Data(r),
            None => Response::Nil,
        },
        None => Response::Nil,
    }
}

fn setrange(parser: &mut ParsedCommand, db: &Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 4, "ERR wrong number of parameters");
    let key = try_validate!(parser.get_vec(1), "Invalid key");
    let index = try_validate!(parser.get_i64(2), "Invalid index");
    if index < 0 {
        return Response::Error("ERR offset is out of range".to_owned());
    }
    let value = try_validate!(parser.get_vec(3), "Invalid value");
    if db.get(dbindex, &key).is_none() && value.is_empty() {
        return Response::Integer(0);
    }
    let oldval = db.get_or_create(dbindex, &key);
    validate!(
        index + value.len <= 512 *1024 * 1024,
        "ERR string exceeds maxmium length(512MB)"
    );
    match oldval.setrange(index, value) {
        Ok(s) => {db.key_updated(dbindex, &key); Response::Integer(s)},
        Err(err) => Response::Error(err.to_string()),
    }
}
fn setbit(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 4);
    let key = try_validate!(parser.get_vec(1), "Invalid key");
    let index = try_validate!(
        parser.get_i64(2),
        "ERR bit offset is not an integer or out of range"
    );
    validate!(
        index >= 0 && index < 4 * 1024 * 1024 * 1024,
        "ERR bit offset is not an integer or out of range"
    );
    let value = try_validate!(
        parser.get_i64(3),
        "ERR bit is not an integer or out of range"
    );
    validate!(
        value == 0 || value == 1,
        "ERR bit is not an integer or out of range"
    );

    match db.get_or_create(dbindex, &key).setbit(index, value == 1) {
        Ok(s) => {db.key_updated(dbindex, &key);Response::Integer(if s {1} else {0})},
        Err(err) => Response::Error(err.to_string()),
    }
}

fn strlen(parser: &mut ParsedCommand, db: &Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 2);
    let key = try_validate!(parser.get_vec(1), "Invalid key");

    match db.get(dbindex, &key) {
        Some(value) => match value.strlen() {
            Ok(r) => Response::Integer(r as i64),
            Err(err) => Response::Error(err.to_string()),
        },
        None => Response::Integer(0),
    }
}

fn generic_incr(
    parser: &mut ParsedCommand,
    db: &mut Database,
    dbindex: usize,
    increment: i64,
) -> Response {
    let key = try_validate!(parser.get_vec(1), "Invalid key");
    match db.get_or_create(dbindex, &key).incr(increment) {
        Ok(val) => {db.key_updated(dbindex, &key);Response::Integer(val)},
        Err(err) => Response::Error(err.to_string()),
    };
}

fn incr(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 2);
    generic_incr(parser, db, dbindex, 1)
}

fn decr(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 2);
    generic_incr(parser, db, dbindex, -1)
}

fn incrby(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 3);
    match parser.get_i64(2) {
        Ok(increment) => generic_incr(parser, db, dbindex, increment),
        Err(_) => Response::Error("Invalid increment".to_owned()),
    }
}

fn decrby(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 3);
    match parser.get_i64(2) {
        Ok(decrement) => generic_incr(parser, db, dbindex, -decrement),
        Err(_) => Response::Error("Invalid increment".to_owned()),
    }
}

fn incrbyfloat(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    validate_arguments_exact!(parser, 3);
    let key = try_validate!(parser.get_vec(1), "Invalid key");
    let increment = try_validate!(parser.get_f64(2), "Invalid increment");
    match db.get_or_create(dbindex, &key).incrbyfloat(increment) {
        Ok(val) => {db.key_updated(dbindex, &key); Response::Data(format!("{}", val).into_bytes())},
        Err(err) => Response::Error(err.to_string()),
    }
}

/* todo pfadd / pfmerge */

fn generic_push(
    parser: &mut ParsedCommand,
    db: &mut Database,
    dbindex: usize,
    right: bool,
    create: bool,
) -> Response {
    validate!(parser.argv.len() >= 3, "Wrong number of parameters");
    let key = try_validate!(parser.get_vec(1), "Invalid key");
    let mut r = Response::Nil;
    let mut is_updated  = 0;

    for i in 2..parser.argv.len() {
        let val = try_validate!(parser.get_vec(i), "Invalid value");
        let el;

        if create {
            el = db.get_or_create(dbindex, &key);
        } else {
            match db.get_mut(dbindex, &key) {
                Some(_el) => el =_el,
                None => return Response::Integer(0);
            }
        }
        r = match el.push(val, right) {
            Ok(listsize) => {is_updated = 1;Response::Integer(listsize as i64)},
            Err(err) => Response::Error(err.to_string()),
        }
    }

    if is_updated {
        db.key_updated(dbindex, &key);
    }
    r
}

fn lpush(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    generic_push(parser, db, dbindex, false, true)
}

fn rpush(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    generic_push(parser, db, dbindex, true, true)
}

fn lpushx(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    generic_push(parser, db, dbindex, false, false)
}

fn rpushx(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    generic_push(parser, db, dbindex, true, false)
}

fn generic_pop(
    parser: &mut ParsedCommand,
    db: &mut Database,
    dbindex: usize,
    right: bool,
) -> Response {
    validate_arguments_exact!(parser, 2);
    let key = try_validate!(parser.get_vec(1), "Invalid key");
    match db.get_mut(dbindex, &key) {
        Some(list) => match list.pop(right) {
            Ok(el) => match el {
                Some(val) => {db.key_updated(dbindex, &key);Response::Data(val)},
                None => Response::Nil,
            },
            Err(err) => Response::Error(err.to_string()),
        },
        None => Response::Nil,
    }
}

fn lpop(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    generic_pop(parser, db, dbindex, false)
}

fn rpop(parser: &mut ParsedCommand, db: &mut Database, dbindex: usize) -> Response {
    generic_pop(parser, db, dbindex, true)
}

fn generic_rpoplpush(
    db: &mut Database,
    dbindex: usize,
    source: &[u8],
    destination: &[u8],
) -> Response {

}


fn monitor(
    parser: &mut ParsedCommand,
    db: &mut Database,
    rawsender: Sender<Option<Response>>,
) -> Response {

}

pub fn command (
    mut parser: ParsedCommand,
    db: &mut Database,
    client: &mut Client,
) -> Result<Response, ResponseError> {

}