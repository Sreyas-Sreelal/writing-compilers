// auto-generated: "lalrpop 0.19.8"
// sha3: f9d85f60f2fee2d82b556693b90457a7389b3bc40a171d6adc2d1410d5e83473
use crate::executor::ast::*;
use crate::lexer::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate alloc;
extern crate core;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__SourceUnit {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]

    use crate::lexer::*;
    use crate::executor::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(TokenType),
        Variant1(Expression),
        Variant2(SourceUnit),
        Variant3(SourceUnitPart),
        Variant4(alloc::vec::Vec<SourceUnitPart>),
        Variant5(Statement),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 19, 20, 21, 22, 23, 5,
        // State 1
        0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 19, 20, 21, 22, 23, 5,
        // State 2
        0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 19, 20, 21, 22, 23, 0,
        // State 3
        0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 19, 20, 21, 22, 23, 0,
        // State 4
        0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 19, 20, 21, 22, 23, 0,
        // State 5
        0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 19, 20, 21, 22, 23, 0,
        // State 6
        0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 19, 20, 21, 22, 23, 0,
        // State 7
        0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 19, 20, 21, 22, 23, 0,
        // State 8
        0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 19, 20, 21, 22, 23, 0,
        // State 9
        0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 19, 20, 21, 22, 23, 0,
        // State 10
        0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 19, 20, 21, 22, 23, 0,
        // State 11
        0, 0, -4, 0, 6, 7, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        9, 0, -3, 10, -3, -3, 11, -3, -3, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, -11, 0, 0, 0, 0, 0, 0, 0, -11, -11, -11, -11, -11, -11, -11,
        // State 16
        0, -10, 0, 0, 0, 0, 0, 0, 0, -10, -10, -10, -10, -10, -10, -10,
        // State 17
        -8, 0, -8, -8, -8, -8, -8, -8, -8, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        -17, 0, -17, -17, -17, -17, -17, -17, -17, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        -19, 0, -19, -19, -19, -19, -19, -19, -19, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        -20, 0, -20, -20, -20, -20, -20, -20, -20, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        -16, 0, -16, -16, -16, -16, -16, -16, -16, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        -18, 0, -18, -18, -18, -18, -18, -18, -18, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, -12, 0, 0, 0, 0, 0, 0, 0, -12, -12, -12, -12, -12, -12, -12,
        // State 24
        0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        9, 0, -1, 10, -1, -1, 11, -1, -1, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        9, 0, -2, 10, -2, -2, 11, -2, -2, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        -7, 0, -7, -7, -7, -7, -7, -7, -7, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        -5, 0, -5, -5, -5, -5, -5, -5, -5, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        -6, 0, -6, -6, -6, -6, -6, -6, -6, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        -21, 0, -21, -21, -21, -21, -21, -21, -21, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, -13, 0, 0, 0, 0, 0, 0, 0, -13, -13, -13, -13, -13, -13, -13,
        // State 35
        0, -15, 0, 0, 0, 0, 0, 0, 0, -15, -15, -15, -15, -15, -15, -15,
        // State 36
        0, -14, 0, 0, 0, 0, 0, 0, 0, -14, -14, -14, -14, -14, -14, -14,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 16 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -9,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        -22,
        // State 15
        -11,
        // State 16
        -10,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        -12,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        -13,
        // State 35
        -15,
        // State 36
        -14,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => 11,
            1 => match state {
                2 => 24,
                3 => 25,
                4 => 26,
                7 => 29,
                _ => 12,
            },
            2 => match state {
                5 => 27,
                6 => 28,
                _ => 13,
            },
            3 => 14,
            4 => match state {
                1 => 23,
                _ => 15,
            },
            5 => 1,
            6 => 16,
            7 => match state {
                8 => 30,
                9 => 31,
                10 => 32,
                _ => 17,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""%""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###""-""###,
            r###""/""###,
            r###"";""###,
            r###""=""###,
            r###"Declaration"###,
            r###"Identifier"###,
            r###"InputNumber"###,
            r###"InputString"###,
            r###"Integer"###,
            r###"StringLiteral"###,
            r###"Write"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'__0>
    where 
    {
        input: &'__0 str,
        __phantom: core::marker::PhantomData<()>,
    }
    impl<'__0> __state_machine::ParserDefinition for __StateMachine<'__0>
    where 
    {
        type Location = ();
        type Error = &'static str;
        type Token = TokenType;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = SourceUnit;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 16 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &TokenType,
        _: core::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            TokenType::Modulo if true => Some(0),
            TokenType::OpenParantheses if true => Some(1),
            TokenType::CloseParantheses if true => Some(2),
            TokenType::Product if true => Some(3),
            TokenType::Plus if true => Some(4),
            TokenType::Minus if true => Some(5),
            TokenType::Divide if true => Some(6),
            TokenType::SemiColon if true => Some(7),
            TokenType::Assignment if true => Some(8),
            TokenType::Declaration if true => Some(9),
            TokenType::Symbol(usize) if true => Some(10),
            TokenType::InputNumber if true => Some(11),
            TokenType::InputString if true => Some(12),
            TokenType::Integer(i64) if true => Some(13),
            TokenType::Literal(usize) if true => Some(14),
            TokenType::Write if true => Some(15),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: TokenType,
        _: core::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => __Symbol::Variant0(__token),
            _ => unreachable!(),
        }
    }
    pub struct SourceUnitParser {
        _priv: (),
    }

    impl SourceUnitParser {
        pub fn new() -> SourceUnitParser {
            SourceUnitParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            input: &str,
            __tokens0: __TOKENS,
        ) -> Result<SourceUnit, __lalrpop_util::ParseError<(), TokenType, &'static str>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        input: &str,
        __action: i8,
        __lookahead_start: Option<&()>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<SourceUnit,__lalrpop_util::ParseError<(), TokenType, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(input, __lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                // __SourceUnit = SourceUnit => ActionFn(0);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Expression, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), SourceUnit, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), SourceUnitPart, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Statement, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), TokenType, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), alloc::vec::Vec<SourceUnitPart>, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArithExpression = ArithExpression, "+", Factor => ActionFn(7);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce1<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArithExpression = ArithExpression, "-", Factor => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce2<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArithExpression = Factor => ActionFn(9);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce3<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = ArithExpression => ActionFn(6);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce4<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "*", Term => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "/", Term => ActionFn(11);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce6<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "%", Term => ActionFn(12);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce7<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Factor = Term => ActionFn(13);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce8<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnit = SourceUnitPart+ => ActionFn(1);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce9<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnitPart = Statement => ActionFn(2);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce10<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnitPart+ = SourceUnitPart => ActionFn(20);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce11<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // SourceUnitPart+ = SourceUnitPart+, SourceUnitPart => ActionFn(21);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action21::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce12<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Declaration, Expression, ";" => ActionFn(3);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action3::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce13<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Expression, "=", Expression, ";" => ActionFn(4);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action4::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (4, 6)
    }
    pub(crate) fn __reduce14<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = Write, Expression, ";" => ActionFn(5);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce15<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Integer => ActionFn(14);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce16<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Identifier => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce17<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = StringLiteral => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce18<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = InputNumber => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce19<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = InputString => ActionFn(18);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce20<
    >(
        input: &str,
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(19);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action19::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 7)
    }
}
pub use self::__parse__SourceUnit::SourceUnitParser;

#[allow(unused_variables)]
fn __action0(input: &str, (_, __0, _): ((), SourceUnit, ())) -> SourceUnit {
    __0
}

#[allow(unused_variables)]
fn __action1(input: &str, (_, __0, _): ((), alloc::vec::Vec<SourceUnitPart>, ())) -> SourceUnit {
    SourceUnit(__0)
}

#[allow(unused_variables)]
fn __action2(input: &str, (_, __0, _): ((), Statement, ())) -> SourceUnitPart {
    SourceUnitPart::Statement(__0)
}

#[allow(unused_variables)]
fn __action3(
    input: &str,
    (_, _, _): ((), TokenType, ()),
    (_, id, _): ((), Expression, ()),
    (_, _, _): ((), TokenType, ()),
) -> Statement {
    Statement::Declaration(id)
}

#[allow(unused_variables)]
fn __action4(
    input: &str,
    (_, l, _): ((), Expression, ()),
    (_, _, _): ((), TokenType, ()),
    (_, r, _): ((), Expression, ()),
    (_, _, _): ((), TokenType, ()),
) -> Statement {
    Statement::Assignment(l, r)
}

#[allow(unused_variables)]
fn __action5(
    input: &str,
    (_, _, _): ((), TokenType, ()),
    (_, e, _): ((), Expression, ()),
    (_, _, _): ((), TokenType, ()),
) -> Statement {
    Statement::Write(e)
}

#[allow(unused_variables)]
fn __action6(input: &str, (_, __0, _): ((), Expression, ())) -> Expression {
    __0
}

#[allow(unused_variables)]
fn __action7(
    input: &str,
    (_, l, _): ((), Expression, ()),
    (_, _, _): ((), TokenType, ()),
    (_, r, _): ((), Expression, ()),
) -> Expression {
    Expression::Add(Box::new(l), Box::new(r))
}

#[allow(unused_variables)]
fn __action8(
    input: &str,
    (_, l, _): ((), Expression, ()),
    (_, _, _): ((), TokenType, ()),
    (_, r, _): ((), Expression, ()),
) -> Expression {
    Expression::Subtract(Box::new(l), Box::new(r))
}

#[allow(unused_variables)]
fn __action9(input: &str, (_, __0, _): ((), Expression, ())) -> Expression {
    __0
}

#[allow(unused_variables)]
fn __action10(
    input: &str,
    (_, l, _): ((), Expression, ()),
    (_, _, _): ((), TokenType, ()),
    (_, r, _): ((), Expression, ()),
) -> Expression {
    Expression::Multiply(Box::new(l), Box::new(r))
}

#[allow(unused_variables)]
fn __action11(
    input: &str,
    (_, l, _): ((), Expression, ()),
    (_, _, _): ((), TokenType, ()),
    (_, r, _): ((), Expression, ()),
) -> Expression {
    Expression::Divide(Box::new(l), Box::new(r))
}

#[allow(unused_variables)]
fn __action12(
    input: &str,
    (_, l, _): ((), Expression, ()),
    (_, _, _): ((), TokenType, ()),
    (_, r, _): ((), Expression, ()),
) -> Expression {
    Expression::Modulo(Box::new(l), Box::new(r))
}

#[allow(unused_variables)]
fn __action13(input: &str, (_, __0, _): ((), Expression, ())) -> Expression {
    __0
}

#[allow(unused_variables)]
fn __action14(input: &str, (_, v, _): ((), TokenType, ())) -> Expression {
    Expression::Integer(v)
}

#[allow(unused_variables)]
fn __action15(input: &str, (_, id, _): ((), TokenType, ())) -> Expression {
    Expression::Symbol(id)
}

#[allow(unused_variables)]
fn __action16(input: &str, (_, string, _): ((), TokenType, ())) -> Expression {
    Expression::StringLiteral(string)
}

#[allow(unused_variables)]
fn __action17(input: &str, (_, d, _): ((), TokenType, ())) -> Expression {
    Expression::InputNumber
}

#[allow(unused_variables)]
fn __action18(input: &str, (_, d, _): ((), TokenType, ())) -> Expression {
    Expression::InputString
}

#[allow(unused_variables)]
fn __action19(
    input: &str,
    (_, _, _): ((), TokenType, ()),
    (_, e, _): ((), Expression, ()),
    (_, _, _): ((), TokenType, ()),
) -> Expression {
    e
}

#[allow(unused_variables)]
fn __action20(
    input: &str,
    (_, __0, _): ((), SourceUnitPart, ()),
) -> alloc::vec::Vec<SourceUnitPart> {
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action21(
    input: &str,
    (_, v, _): ((), alloc::vec::Vec<SourceUnitPart>, ()),
    (_, e, _): ((), SourceUnitPart, ()),
) -> alloc::vec::Vec<SourceUnitPart> {
    {
        let mut v = v;
        v.push(e);
        v
    }
}

pub trait __ToTriple {
    fn to_triple(
        value: Self,
    ) -> Result<((), TokenType, ()), __lalrpop_util::ParseError<(), TokenType, &'static str>>;
}

impl __ToTriple for TokenType {
    fn to_triple(
        value: Self,
    ) -> Result<((), TokenType, ()), __lalrpop_util::ParseError<(), TokenType, &'static str>> {
        Ok(((), value, ()))
    }
}
impl __ToTriple for Result<TokenType, &'static str> {
    fn to_triple(
        value: Self,
    ) -> Result<((), TokenType, ()), __lalrpop_util::ParseError<(), TokenType, &'static str>> {
        match value {
            Ok(v) => Ok(((), v, ())),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
