
/// call a STM function from inside of a STM block
/// 
/// if we had control over stack unwinding we could use that
#[macro_export]
macro_rules! stm_call {
    ( $l:expr, $e:expr )     => ({
        use $crate::StmResult::*;

        let ret = $e.intern_run($l);
        match ret {
            Success(s)  => s,
            Retry       => return Retry,
            Failure     => return Failure
        }
    })
}


/// declare a block that uses STM
#[macro_export]
macro_rules! stm {
    ( $l:ident, $e:expr )    => {{
        let func = |$l: &mut _| {
            $crate::StmResult::Success($e)
        };
        $crate::STM::new(func)
    }}
}
