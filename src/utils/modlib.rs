use std::fmt;

const MODLIB_NAMESPACE: &'static str = "Modlib";

pub enum ModlibFuncs {
    RegisterModule,
    ExecutePhrase,
    ClearList,
    RegisterEventHandler,
    RegisterTimerHandler,
    DeleteEventHandler,
    GetStringById,
    ClearGlobPhrase,
}

impl fmt::Display for ModlibFuncs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let func = match self {
            ModlibFuncs::RegisterModule => "RegisterModule",
            ModlibFuncs::ExecutePhrase => "ExecutePhrase",
            ModlibFuncs::ClearList => "ClearList",
            ModlibFuncs::RegisterEventHandler => "CreateEventHandler",
            ModlibFuncs::RegisterTimerHandler => "CreateTimerHandler",
            ModlibFuncs::DeleteEventHandler => "DeleteEventHandler",
            ModlibFuncs::GetStringById => "GetStringById",
            ModlibFuncs::ClearGlobPhrase => "ClearGlobPhrase",
        };
        write!(f, "{}.{}", MODLIB_NAMESPACE, func)
    }
}
