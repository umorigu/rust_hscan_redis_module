extern crate libc;
use libc::{c_int, size_t};

const MODULE_NAME: &'static str = "rusthscanhello";
const REDISMODULE_APIVER_1: c_int = 1;

pub enum RedisModuleCtx {}
pub enum RedisModuleString {}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum Status {
    Ok = 0,  // const REDISMODULE_OK: c_int = 0;
    Err = 1, // const REDISMODULE_ERR: c_int = 1;
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
