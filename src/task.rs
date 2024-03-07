/*! Tasks */
use crate::*;

/** Run a closure `Unit -> A` as a `Task A` */
#[inline(always)]
pub unsafe fn lean_task_spawn(c: lean_obj_arg, prio: lean_obj_arg) -> lean_obj_res {
    lean_task_spawn_core(c, lean_unbox(prio) as c_uint, false)
}

/** `Task.bind (x : Task A) (f : A -> Task B) (prio : Nat) : Task B` */
#[inline(always)]
pub unsafe fn lean_task_bind(
    x: lean_obj_arg,
    f: lean_obj_arg,
    sync: u8,
    prio: lean_obj_arg,
) -> lean_obj_res {
    lean_task_bind_core(x, f, lean_unbox(prio) as c_uint, sync != 0, false)
}

/** `Task.map (f : A -> B) (t : Task A) (prio : Nat) : Task B` */
#[inline(always)]
pub unsafe fn lean_task_map(
    f: lean_obj_arg,
    t: lean_obj_arg,
    sync: u8,
    prio: lean_obj_arg,
) -> lean_obj_res {
    lean_task_map_core(f, t, lean_unbox(prio) as c_uint, sync != 0, false)
}

/** Primitive for implementing `Task.get : Task A -> A` */
#[inline]
pub unsafe fn lean_task_get_own(t: b_lean_obj_arg) -> lean_obj_res {
    let r = lean_task_get(t);
    lean_inc(r);
    lean_dec(t);
    r
}

extern "C" {
    pub fn lean_init_task_manager();
    pub fn lean_init_task_manager_using(num_workers: c_uint);
    pub fn lean_finalize_task_manager();
    pub fn lean_task_spawn_core(c: lean_obj_arg, prio: c_uint, keep_alive: bool) -> lean_obj_res;
    /** Convert a value `a : A` into `Task A` */
    pub fn lean_task_pure(a: lean_obj_arg) -> lean_obj_res;
    pub fn lean_task_bind_core(
        x: lean_obj_arg,
        f: lean_obj_arg,
        prio: c_uint,
        sync: bool,
        keep_alive: bool,
    ) -> lean_obj_res;
    pub fn lean_task_map_core(
        f: lean_obj_arg,
        t: lean_obj_arg,
        prio: c_uint,
        sync: bool,
        keep_alive: bool,
    ) -> lean_obj_res;
    pub fn lean_task_get(t: b_lean_obj_arg) -> lean_obj_res;
    /** primitive for implementing `IO.checkCanceled : IO Bool` */
    pub fn lean_io_check_canceled_core() -> bool;
    /** primitive for implementing `IO.cancel : Task a -> IO Unit` */
    pub fn lean_io_cancel_core(t: b_lean_obj_arg);
    /** primitive for implementing `IO.hasFinished : Task a -> IO Unit` */
    pub fn lean_io_has_finished_core(t: b_lean_obj_arg) -> bool;
    /** primitive for implementing `IO.waitAny : List (Task a) -> IO (Task a)` */
    pub fn lean_io_wait_any_core(task_list: b_lean_obj_arg) -> b_lean_obj_res;
}
