/// An ABI stable tuple of 0 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple0<>();
impl<> From<()> for Tuple0<> {
    fn from(value: ()) -> Self {
        let () = value;
        Self()
    }
}
#[allow(clippy::unused_unit)]
impl<> From<Tuple0<>> for () {
    fn from(value: Tuple0<>) -> Self {
        let Tuple0() = value;
        ()
    }
}

/// An ABI stable tuple of 1 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple1<T0, >(pub T0, );
impl<T0, > From<(T0, )> for Tuple1<T0, > {
    fn from(value: (T0, )) -> Self {
        let (field0, ) = value;
        Self(field0, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, > From<Tuple1<T0, >> for (T0, ) {
    fn from(value: Tuple1<T0, >) -> Self {
        let Tuple1(field0, ) = value;
        (field0, )
    }
}

/// An ABI stable tuple of 2 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple2<T0, T1, >(pub T0, pub T1, );
impl<T0, T1, > From<(T0, T1, )> for Tuple2<T0, T1, > {
    fn from(value: (T0, T1, )) -> Self {
        let (field0, field1, ) = value;
        Self(field0, field1, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, > From<Tuple2<T0, T1, >> for (T0, T1, ) {
    fn from(value: Tuple2<T0, T1, >) -> Self {
        let Tuple2(field0, field1, ) = value;
        (field0, field1, )
    }
}

/// An ABI stable tuple of 3 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple3<T0, T1, T2, >(pub T0, pub T1, pub T2, );
impl<T0, T1, T2, > From<(T0, T1, T2, )> for Tuple3<T0, T1, T2, > {
    fn from(value: (T0, T1, T2, )) -> Self {
        let (field0, field1, field2, ) = value;
        Self(field0, field1, field2, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, > From<Tuple3<T0, T1, T2, >> for (T0, T1, T2, ) {
    fn from(value: Tuple3<T0, T1, T2, >) -> Self {
        let Tuple3(field0, field1, field2, ) = value;
        (field0, field1, field2, )
    }
}

/// An ABI stable tuple of 4 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple4<T0, T1, T2, T3, >(pub T0, pub T1, pub T2, pub T3, );
impl<T0, T1, T2, T3, > From<(T0, T1, T2, T3, )> for Tuple4<T0, T1, T2, T3, > {
    fn from(value: (T0, T1, T2, T3, )) -> Self {
        let (field0, field1, field2, field3, ) = value;
        Self(field0, field1, field2, field3, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, > From<Tuple4<T0, T1, T2, T3, >> for (T0, T1, T2, T3, ) {
    fn from(value: Tuple4<T0, T1, T2, T3, >) -> Self {
        let Tuple4(field0, field1, field2, field3, ) = value;
        (field0, field1, field2, field3, )
    }
}

/// An ABI stable tuple of 5 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple5<T0, T1, T2, T3, T4, >(pub T0, pub T1, pub T2, pub T3, pub T4, );
impl<T0, T1, T2, T3, T4, > From<(T0, T1, T2, T3, T4, )> for Tuple5<T0, T1, T2, T3, T4, > {
    fn from(value: (T0, T1, T2, T3, T4, )) -> Self {
        let (field0, field1, field2, field3, field4, ) = value;
        Self(field0, field1, field2, field3, field4, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, > From<Tuple5<T0, T1, T2, T3, T4, >> for (T0, T1, T2, T3, T4, ) {
    fn from(value: Tuple5<T0, T1, T2, T3, T4, >) -> Self {
        let Tuple5(field0, field1, field2, field3, field4, ) = value;
        (field0, field1, field2, field3, field4, )
    }
}

/// An ABI stable tuple of 6 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple6<T0, T1, T2, T3, T4, T5, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, );
impl<T0, T1, T2, T3, T4, T5, > From<(T0, T1, T2, T3, T4, T5, )> for Tuple6<T0, T1, T2, T3, T4, T5, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, ) = value;
        Self(field0, field1, field2, field3, field4, field5, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, > From<Tuple6<T0, T1, T2, T3, T4, T5, >> for (T0, T1, T2, T3, T4, T5, ) {
    fn from(value: Tuple6<T0, T1, T2, T3, T4, T5, >) -> Self {
        let Tuple6(field0, field1, field2, field3, field4, field5, ) = value;
        (field0, field1, field2, field3, field4, field5, )
    }
}

/// An ABI stable tuple of 7 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple7<T0, T1, T2, T3, T4, T5, T6, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, );
impl<T0, T1, T2, T3, T4, T5, T6, > From<(T0, T1, T2, T3, T4, T5, T6, )> for Tuple7<T0, T1, T2, T3, T4, T5, T6, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, > From<Tuple7<T0, T1, T2, T3, T4, T5, T6, >> for (T0, T1, T2, T3, T4, T5, T6, ) {
    fn from(value: Tuple7<T0, T1, T2, T3, T4, T5, T6, >) -> Self {
        let Tuple7(field0, field1, field2, field3, field4, field5, field6, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, )
    }
}

/// An ABI stable tuple of 8 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple8<T0, T1, T2, T3, T4, T5, T6, T7, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, > From<(T0, T1, T2, T3, T4, T5, T6, T7, )> for Tuple8<T0, T1, T2, T3, T4, T5, T6, T7, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, > From<Tuple8<T0, T1, T2, T3, T4, T5, T6, T7, >> for (T0, T1, T2, T3, T4, T5, T6, T7, ) {
    fn from(value: Tuple8<T0, T1, T2, T3, T4, T5, T6, T7, >) -> Self {
        let Tuple8(field0, field1, field2, field3, field4, field5, field6, field7, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, )
    }
}

/// An ABI stable tuple of 9 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple9<T0, T1, T2, T3, T4, T5, T6, T7, T8, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, )> for Tuple9<T0, T1, T2, T3, T4, T5, T6, T7, T8, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, > From<Tuple9<T0, T1, T2, T3, T4, T5, T6, T7, T8, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, ) {
    fn from(value: Tuple9<T0, T1, T2, T3, T4, T5, T6, T7, T8, >) -> Self {
        let Tuple9(field0, field1, field2, field3, field4, field5, field6, field7, field8, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, )
    }
}

/// An ABI stable tuple of 10 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, )> for Tuple10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, > From<Tuple10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, ) {
    fn from(value: Tuple10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, >) -> Self {
        let Tuple10(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, )
    }
}

/// An ABI stable tuple of 11 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, )> for Tuple11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, > From<Tuple11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, ) {
    fn from(value: Tuple11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, >) -> Self {
        let Tuple11(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, )
    }
}

/// An ABI stable tuple of 12 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, )> for Tuple12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, > From<Tuple12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, ) {
    fn from(value: Tuple12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, >) -> Self {
        let Tuple12(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, )
    }
}

/// An ABI stable tuple of 13 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, )> for Tuple13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, > From<Tuple13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, ) {
    fn from(value: Tuple13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, >) -> Self {
        let Tuple13(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, )
    }
}

/// An ABI stable tuple of 14 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, )> for Tuple14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, > From<Tuple14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, ) {
    fn from(value: Tuple14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, >) -> Self {
        let Tuple14(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, )
    }
}

/// An ABI stable tuple of 15 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, )> for Tuple15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, > From<Tuple15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, ) {
    fn from(value: Tuple15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, >) -> Self {
        let Tuple15(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, )
    }
}

/// An ABI stable tuple of 16 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, )> for Tuple16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, > From<Tuple16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, ) {
    fn from(value: Tuple16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, >) -> Self {
        let Tuple16(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, )
    }
}

/// An ABI stable tuple of 17 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, pub T16, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, )> for Tuple17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, > From<Tuple17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, ) {
    fn from(value: Tuple17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, >) -> Self {
        let Tuple17(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, )
    }
}

/// An ABI stable tuple of 18 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, pub T16, pub T17, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, )> for Tuple18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, > From<Tuple18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, ) {
    fn from(value: Tuple18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, >) -> Self {
        let Tuple18(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, )
    }
}

/// An ABI stable tuple of 19 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, pub T16, pub T17, pub T18, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, )> for Tuple19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, > From<Tuple19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, ) {
    fn from(value: Tuple19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, >) -> Self {
        let Tuple19(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, )
    }
}

/// An ABI stable tuple of 20 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, pub T16, pub T17, pub T18, pub T19, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, )> for Tuple20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, > From<Tuple20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, ) {
    fn from(value: Tuple20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, >) -> Self {
        let Tuple20(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, )
    }
}

/// An ABI stable tuple of 21 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, pub T16, pub T17, pub T18, pub T19, pub T20, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, )> for Tuple21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, > From<Tuple21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, ) {
    fn from(value: Tuple21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, >) -> Self {
        let Tuple21(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, )
    }
}

/// An ABI stable tuple of 22 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, pub T16, pub T17, pub T18, pub T19, pub T20, pub T21, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, )> for Tuple22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, > From<Tuple22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, ) {
    fn from(value: Tuple22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, >) -> Self {
        let Tuple22(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, )
    }
}

/// An ABI stable tuple of 23 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, pub T16, pub T17, pub T18, pub T19, pub T20, pub T21, pub T22, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, )> for Tuple23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, > From<Tuple23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, ) {
    fn from(value: Tuple23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, >) -> Self {
        let Tuple23(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, )
    }
}

/// An ABI stable tuple of 24 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, pub T16, pub T17, pub T18, pub T19, pub T20, pub T21, pub T22, pub T23, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, )> for Tuple24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, > From<Tuple24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, ) {
    fn from(value: Tuple24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, >) -> Self {
        let Tuple24(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, )
    }
}

/// An ABI stable tuple of 25 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, pub T16, pub T17, pub T18, pub T19, pub T20, pub T21, pub T22, pub T23, pub T24, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, )> for Tuple25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, > From<Tuple25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, ) {
    fn from(value: Tuple25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, >) -> Self {
        let Tuple25(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, )
    }
}

/// An ABI stable tuple of 26 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, pub T16, pub T17, pub T18, pub T19, pub T20, pub T21, pub T22, pub T23, pub T24, pub T25, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, )> for Tuple26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, > From<Tuple26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, ) {
    fn from(value: Tuple26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, >) -> Self {
        let Tuple26(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, )
    }
}

/// An ABI stable tuple of 27 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, pub T16, pub T17, pub T18, pub T19, pub T20, pub T21, pub T22, pub T23, pub T24, pub T25, pub T26, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, )> for Tuple27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, > From<Tuple27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, ) {
    fn from(value: Tuple27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, >) -> Self {
        let Tuple27(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, )
    }
}

/// An ABI stable tuple of 28 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, pub T16, pub T17, pub T18, pub T19, pub T20, pub T21, pub T22, pub T23, pub T24, pub T25, pub T26, pub T27, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, )> for Tuple28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, > From<Tuple28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, ) {
    fn from(value: Tuple28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, >) -> Self {
        let Tuple28(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, )
    }
}

/// An ABI stable tuple of 29 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, pub T16, pub T17, pub T18, pub T19, pub T20, pub T21, pub T22, pub T23, pub T24, pub T25, pub T26, pub T27, pub T28, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, )> for Tuple29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, field28, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, field28, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, > From<Tuple29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, ) {
    fn from(value: Tuple29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, >) -> Self {
        let Tuple29(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, field28, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, field28, )
    }
}

/// An ABI stable tuple of 30 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, pub T16, pub T17, pub T18, pub T19, pub T20, pub T21, pub T22, pub T23, pub T24, pub T25, pub T26, pub T27, pub T28, pub T29, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, )> for Tuple30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, field28, field29, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, field28, field29, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, > From<Tuple30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, ) {
    fn from(value: Tuple30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, >) -> Self {
        let Tuple30(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, field28, field29, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, field28, field29, )
    }
}

/// An ABI stable tuple of 31 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, pub T16, pub T17, pub T18, pub T19, pub T20, pub T21, pub T22, pub T23, pub T24, pub T25, pub T26, pub T27, pub T28, pub T29, pub T30, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, )> for Tuple31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, field28, field29, field30, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, field28, field29, field30, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, > From<Tuple31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, ) {
    fn from(value: Tuple31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, >) -> Self {
        let Tuple31(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, field28, field29, field30, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, field28, field29, field30, )
    }
}

/// An ABI stable tuple of 32 elements.
#[crate::stabby]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tuple32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, >(pub T0, pub T1, pub T2, pub T3, pub T4, pub T5, pub T6, pub T7, pub T8, pub T9, pub T10, pub T11, pub T12, pub T13, pub T14, pub T15, pub T16, pub T17, pub T18, pub T19, pub T20, pub T21, pub T22, pub T23, pub T24, pub T25, pub T26, pub T27, pub T28, pub T29, pub T30, pub T31, );
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, > From<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, )> for Tuple32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, > {
    fn from(value: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, )) -> Self {
        let (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, field28, field29, field30, field31, ) = value;
        Self(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, field28, field29, field30, field31, )
    }
}
#[allow(clippy::unused_unit)]
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, > From<Tuple32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, >> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, ) {
    fn from(value: Tuple32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, >) -> Self {
        let Tuple32(field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, field28, field29, field30, field31, ) = value;
        (field0, field1, field2, field3, field4, field5, field6, field7, field8, field9, field10, field11, field12, field13, field14, field15, field16, field17, field18, field19, field20, field21, field22, field23, field24, field25, field26, field27, field28, field29, field30, field31, )
    }
}

