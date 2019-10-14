extern crate libc;
use libc::{c_int, c_long, c_ulong, size_t};
use std::string;

const MODULE_NAME: &'static str = "rusthscanhello";
const REDISMODULE_APIVER_1: c_int = 1;

const REDISMODULE_POSTPONED_ARRAY_LEN: c_long = -1;

pub enum RedisModuleCtx {}
pub enum RedisModuleString {}
pub enum RedisModuleCallReply {}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum Status {
    Ok = 0,  // const REDISMODULE_OK: c_int = 0;
    Err = 1, // const REDISMODULE_ERR: c_int = 1;
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum ReplyType {
    Unknown = -1,
    String = 0,
    Error = 1,
    Integer = 2,
    Array = 3, // #define REDISMODULE_REPLY_ARRAY 3
    Nil = 4,
}

pub type RedisModuleCmdFunc = extern "C" fn(
    ctx: *mut RedisModuleCtx,
    argv: *mut *mut RedisModuleString,
    argc: c_int,
) -> Status;

extern "C" {
    pub fn Export_RedisModule_Init(
        ctx: *mut RedisModuleCtx,
        modulename: *const u8,
        module_version: c_int,
        api_version: c_int,
    ) -> Status;

    // int RedisModule_CreateCommand(RedisModuleCtx *ctx, const char *name,
    //   RedisModuleCmdFunc cmdfunc, const char *strflags, int firstkey,
    //   int lastkey, int keystep);
    static RedisModule_CreateCommand: extern "C" fn(
        ctx: *mut RedisModuleCtx,
        name: *const u8,
        fmdfunc: RedisModuleCmdFunc,
        strflags: *const u8,
        firstkey: c_int,
        lastkey: c_int,
        keystep: c_int,
    ) -> Status;

    // int RedisModule_ReplyWithStringBuffer(RedisModuleCtx *ctx,
    //   const char *buf, size_t len);
    static RedisModule_ReplyWithStringBuffer:
        extern "C" fn(ctx: *mut RedisModuleCtx, str: *const u8, len: size_t) -> Status;

    // int RedisModule_WrongArity(RedisModuleCtx *ctx);
    static RedisModule_WrongArity: extern "C" fn(ctx: *mut RedisModuleCtx) -> Status;

    // void RedisModule_Log(RedisModuleCtx *ctx, const char *levelstr, const char *fmt, ...);
    static RedisModule_Log:
        extern "C" fn(ctx: *mut RedisModuleCtx, levelstr: *const u8, fmt: *const u8);

    // int RedisModule_ReplyWithArray(RedisModuleCtx *ctx, long len);
    static RedisModule_ReplyWithArray:
        extern "C" fn(ctx: *mut RedisModuleCtx, len: c_long) -> Status;

    // RedisModuleCallReply *RedisModule_Call(RedisModuleCtx *ctx, const char *cmdname, const char *fmt, ...);
    static RedisModule_Call: extern "C" fn(
        ctx: *mut RedisModuleCtx,
        cmdname: *const u8,
        fmt: *const u8,
        arg0: *const u8,
        arg1: *const u8,
    ) -> *mut RedisModuleCallReply;

    // int RedisModule_CallReplyType(RedisModuleCallReply *reply);
    static RedisModule_CallReplyType: extern "C" fn(reply: *mut RedisModuleCallReply) -> ReplyType;

    // size_t RedisModule_CallReplyLength(RedisModuleCallReply *reply);
    static RedisModule_CallReplyLength: extern "C" fn(reply: *mut RedisModuleCallReply) -> c_ulong;

    // RedisModuleCallReply *RedisModule_CallReplyArrayElement(RedisModuleCallReply *reply,
    //   size_t idx);
    static RedisModule_CallReplyArrayElement:
        extern "C" fn(reply: *mut RedisModuleCallReply, idx: c_ulong) -> *mut RedisModuleCallReply;

    // const char *RedisModule_CallReplyStringPtr(RedisModuleCallReply *reply, size_t *len);
    static RedisModule_CallReplyStringPtr:
        extern "C" fn(reply: *mut RedisModuleCallReply, len: *mut size_t) -> *const u8;

    // void RedisModule_FreeCallReply(RedisModuleCallReply *reply);
    static RedisModule_FreeCallReply: extern "C" fn(reply: *mut RedisModuleCallReply);

    // int RedisModule_ReplyWithLongLong(RedisModuleCtx *ctx, long long ll);
    static RedisModule_ReplyWithLongLong: extern "C" fn(ctx: *mut RedisModuleCtx, ll: c_ulong);

    // void RedisModule_ReplySetArrayLength(RedisModuleCtx *ctx, long len);
    static RedisModule_ReplySetArrayLength: extern "C" fn(ctx: *mut RedisModuleCtx, len: c_long);

    // const char *RedisModule_StringPtrLen(const RedisModuleString *str, size_t *len);
    static RedisModule_StringPtrLen:
        extern "C" fn(str: *mut RedisModuleString, len: *mut size_t) -> *const u8;
}

// https://github.com/brandur/redis-cell/blob/master/src/redis/raw.rs
pub fn string_ptr_len(str: *mut RedisModuleString, len: *mut size_t) -> *const u8 {
    unsafe { RedisModule_StringPtrLen(str, len) }
}

// parse_args() from https://github.com/brandur/redis-cell/blob/master/src/redis/mod.rs
fn from_byte_string(byte_str: *const u8, length: size_t) -> Result<String, string::FromUtf8Error> {
    let mut vec_str: Vec<u8> = Vec::with_capacity(length as usize);
    for offset in 0..length {
        let byte: u8 = unsafe { *byte_str.add(offset) };
        vec_str.insert(offset, byte);
    }

    String::from_utf8(vec_str)
}

// parse_args() from https://github.com/brandur/redis-cell/blob/master/src/redis/mod.rs
fn manifest_redis_string(
    redis_str: *mut RedisModuleString,
) -> Result<String, string::FromUtf8Error> {
    let mut length: size_t = 0;
    let bytes = string_ptr_len(redis_str, &mut length);
    from_byte_string(bytes, length)
}

// parse_args() from https://github.com/brandur/redis-cell/blob/master/src/redis/mod.rs
fn parse_args(
    argv: *mut *mut RedisModuleString,
    argc: c_int,
) -> Result<Vec<String>, string::FromUtf8Error> {
    let mut args: Vec<String> = Vec::with_capacity(argc as usize);
    for i in 0..argc {
        let redis_str = unsafe { *argv.offset(i as isize) };
        args.push(manifest_redis_string(redis_str)?);
    }
    Ok(args)
}

fn rm_log(ctx: *mut RedisModuleCtx, level_str: &str, fmt: &str) {
    unsafe {
        RedisModule_Log(
            ctx,
            format!("{}", level_str).as_ptr(),
            format!("{}", fmt).as_ptr(),
        );
    }
}
fn rm_wrong_arity(ctx: *mut RedisModuleCtx) -> Status {
    unsafe {
        return RedisModule_WrongArity(ctx);
    }
}
fn rm_reply_with_array(ctx: *mut RedisModuleCtx, len: c_long) -> Status {
    unsafe {
        return RedisModule_ReplyWithArray(ctx, len);
    }
}
fn rm_reply_with_string_buffer(ctx: *mut RedisModuleCtx, s: &str) -> Status {
    unsafe {
        return RedisModule_ReplyWithStringBuffer(ctx, format!("{}", s).as_ptr(), s.len());
    }
}
fn rm_call(
    ctx: *mut RedisModuleCtx,
    cmdname: &str,
    arg0: &str,
    arg1: &str,
) -> *mut RedisModuleCallReply {
    unsafe {
        return RedisModule_Call(
            ctx,
            format!("{}\0", cmdname).as_ptr(),
            format!("{}\0", "cc").as_ptr(),
            format!("{}\0", arg0).as_ptr(),
            format!("{}\0", arg1).as_ptr(),
        );
    }
}
fn rm_call_reply_type(reply: *mut RedisModuleCallReply) -> ReplyType {
    unsafe { RedisModule_CallReplyType(reply) }
}
fn rm_call_reply_length(reply: *mut RedisModuleCallReply) -> c_ulong {
    unsafe { RedisModule_CallReplyLength(reply) }
}
fn rm_call_reply_array_element(
    reply: *mut RedisModuleCallReply,
    idx: c_ulong,
) -> *mut RedisModuleCallReply {
    unsafe {
        return RedisModule_CallReplyArrayElement(reply, idx);
    }
}
fn rm_call_reply_string_ptr(str: *mut RedisModuleCallReply, len: *mut size_t) -> *const u8 {
    unsafe { RedisModule_CallReplyStringPtr(str, len) }
}

fn rm_reply_with_long_long(ctx: *mut RedisModuleCtx, ll: c_ulong) {
    unsafe { RedisModule_ReplyWithLongLong(ctx, ll) }
}
fn rm_reply_set_array_length(ctx: *mut RedisModuleCtx, len: c_long) {
    unsafe { RedisModule_ReplySetArrayLength(ctx, len) }
}
fn rm_free_call_reply(reply: *mut RedisModuleCallReply) {
    unsafe { RedisModule_FreeCallReply(reply) }
}

extern "C" fn hscan_hello_redis_command(
    ctx: *mut RedisModuleCtx,
    argv: *mut *mut RedisModuleString,
    argc: c_int,
) -> Status {
    let args = parse_args(argv, argc).unwrap();
    if args.len() != 2 {
        return rm_wrong_arity(ctx);
    }
    let key_str = &args[1];

    rm_log(ctx, "notice", "Before call()");
    rm_reply_with_array(ctx, REDISMODULE_POSTPONED_ARRAY_LEN);
    rm_reply_with_string_buffer(ctx, key_str);
    let reply = rm_call(ctx, "HSCAN", key_str, "0");
    if rm_call_reply_type(reply) != ReplyType::Array {
        rm_log(ctx, "warning", "not array");
        return Status::Err;
    }
    rm_log(ctx, "notice", "After call()");
    let length0 = rm_call_reply_length(reply);
    if length0 != 2 {
        rm_log(ctx, "warning", "length0 is NOT 2");
        return Status::Err;
    }
    let r0 = rm_call_reply_array_element(reply, 0);
    if rm_call_reply_type(r0) == ReplyType::String {
        let mut len = 0;
        let s = rm_call_reply_string_ptr(r0, &mut len);
        match from_byte_string(s, len) {
            Ok(result) => {
                let _ = rm_reply_with_string_buffer(ctx, &result);
            }
            Err(_msg) => rm_log(ctx, "error", "from_utf8_error"),
        }
    } else {
        rm_reply_with_string_buffer(ctx, "ERR");
    }
    let r1 = rm_call_reply_array_element(reply, 1);
    let length = rm_call_reply_length(r1);

    for i in 0..length {
        let r = rm_call_reply_array_element(r1, i);
        if rm_call_reply_type(r) == ReplyType::String {
            let mut len = 0;
            let s = rm_call_reply_string_ptr(r, &mut len);
            match from_byte_string(s, len) {
                Ok(result) => {
                    let _ = rm_reply_with_string_buffer(ctx, &result);
                }
                Err(_msg) => rm_log(ctx, "error", "from_utf8_error"),
            }
        } else {
            rm_reply_with_string_buffer(ctx, "[non str]");
        }
    }
    rm_free_call_reply(reply);
    rm_log(ctx, "notice", "NOTICE!");
    rm_reply_with_long_long(ctx, length);
    rm_reply_with_string_buffer(ctx, "end");
    rm_reply_set_array_length(ctx, (length + 4) as i64);
    return Status::Ok;
}
#[no_mangle]
pub extern "C" fn RedisModule_OnLoad(
    ctx: *mut RedisModuleCtx,
    _argv: *mut *mut RedisModuleString,
    _argc: c_int,
) -> Status {
    unsafe {
        Export_RedisModule_Init(
            ctx,
            format!("{}\0", MODULE_NAME).as_ptr(),
            1,
            REDISMODULE_APIVER_1,
        );
        if RedisModule_CreateCommand(
            ctx,
            format!("{}\0", "rusthello").as_ptr(),
            hscan_hello_redis_command,
            format!("{}\0", "readonly").as_ptr(),
            0,
            0,
            0,
        ) == Status::Err
        {
            return Status::Err;
        }
    }
    return Status::Ok;
}
