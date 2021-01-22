use super::term::*;
use crate::unifiable;
use swipl_sys::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Functor {
    functor: functor_t,
}

impl Functor {
    pub unsafe fn new(functor: functor_t) -> Self {
        Self { functor }
    }
}

unifiable! {
    (self: &Functor, term) => {
        let result = unsafe {PL_unify_compound(term.term_ptr(), self.functor)};

        result != 0
    }
}

unifiable! {
    (self: Functor, term) => {
        (&self).unify(term)
    }
}

#[cfg(test)]
mod tests {
    use crate::context::*;
    use crate::engine::*;

    #[test]
    fn unify_same_functor_twice_succeeds() {
        initialize_swipl_noengine();
        let engine = Engine::new();
        let activation = engine.activate();
        let context: Context<_> = activation.into();

        let f = context.new_functor("moocows", 3);
        let term = context.new_term_ref();
        assert!(term.unify(&f));
        assert!(term.unify(&f));
    }

    #[test]
    fn unify_different_functor_arity_fails() {
        initialize_swipl_noengine();
        let engine = Engine::new();
        let activation = engine.activate();
        let context: Context<_> = activation.into();

        let f1 = context.new_functor("moocows", 3);
        let f2 = context.new_functor("moocows", 4);
        let term = context.new_term_ref();
        assert!(term.unify(&f1));
        assert!(!term.unify(&f2));
    }

    #[test]
    fn unify_different_functor_name_fails() {
        initialize_swipl_noengine();
        let engine = Engine::new();
        let activation = engine.activate();
        let context: Context<_> = activation.into();

        let f1 = context.new_functor("moocows", 3);
        let f2 = context.new_functor("oinkpigs", 3);
        let term = context.new_term_ref();
        assert!(term.unify(&f1));
        assert!(!term.unify(&f2));
    }

    #[test]
    #[should_panic]
    fn functor_creation_with_too_high_arity_panics() {
        initialize_swipl_noengine();
        let engine = Engine::new();
        let activation = engine.activate();
        let context: Context<_> = activation.into();

        let _f = context.new_functor("moocows", 1025);
    }

    #[test]
    fn functor_arg_unify_and_get_succeeds() {
        initialize_swipl_noengine();
        let engine = Engine::new();
        let activation = engine.activate();
        let context: Context<_> = activation.into();

        let f = context.new_functor("moocows", 2);
        let term = context.new_term_ref();
        assert_eq!(None, term.get_arg::<u64>(1));
        assert!(term.unify(f));
        assert_eq!(None, term.get_arg::<u64>(1));
        assert!(term.unify_arg(1, 42_u64));
        assert_eq!(Some(42_u64), term.get_arg(1));
        assert!(term.unify_arg(1, 42_u64));
        assert!(!term.unify_arg(1, 43_u64));

        assert!(term.unify_arg(2, 24_u64));
        assert_eq!(Some(24_u64), term.get_arg(2));

        assert!(!term.unify_arg(3, 24_u64));
        assert_eq!(None, term.get_arg::<u64>(3));
    }
}
