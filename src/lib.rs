extern crate libc;
use libc::{c_int, c_long, c_ulong, size_t};

const MODULE_NAME: &'static str = "rusthscanhello";
const REDISMODULE_APIVER_1: c_int = 1;

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

    // void RedisModule_Log(RedisModuleCtx *ctx, const char *levelstr, const char *fmt, ...);
    static RedisModule_Log:
        extern "C" fn(ctx: *mut RedisModuleCtx, levelstr: *const u8, fmt: *const u8);

    // int RedisModule_ReplyWithArray(RedisModuleCtx *ctx, long len);
    static RedisModule_ReplyWithArray:
        extern "C" fn(ctx: *mut RedisModuleCtx, len: c_long) -> Status;

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
        extern "C" fn(reply: *mut RedisModuleCallReply, len: c_ulong) -> *const u8;

    // void RedisModule_FreeCallReply(RedisModuleCallReply *reply);
    static RedisModule_FreeCallReply: extern "C" fn(reply: *mut RedisModuleCallReply);

    // int RedisModule_ReplyWithLongLong(RedisModuleCtx *ctx, long long ll);
    static RedisModule_ReplyWithLongLong: extern "C" fn(ctx: *mut RedisModuleCtx, ll: c_ulong);

    // void RedisModule_ReplySetArrayLength(RedisModuleCtx *ctx, long len);
    static RedisModule_ReplySetArrayLength: extern "C" fn(ctx: *mut RedisModuleCtx, len: c_long);

}

extern "C" fn hscan_hello_redis_command(
    ctx: *mut RedisModuleCtx,
    _argv: *mut *mut RedisModuleString,
    _argc: c_int,
) -> Status {
    unsafe {
        const HELLO: &'static str = "hscanhello";
        RedisModule_ReplyWithStringBuffer(ctx, format!("{}", HELLO).as_ptr(), HELLO.len());
    }
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
