mod core {
    use deku::prelude::*;
    use deku::{DekuRead, DekuWrite};
    use std::hash::Hash;
    enum Bytes {
        Dynamic(Vec<u8>),
        Static(&'static [u8]),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Bytes {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Bytes::Dynamic(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Dynamic",
                        &__self_0,
                    )
                }
                Bytes::Static(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Static",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Bytes {
        #[inline]
        fn clone(&self) -> Bytes {
            match self {
                Bytes::Dynamic(__self_0) => {
                    Bytes::Dynamic(::core::clone::Clone::clone(__self_0))
                }
                Bytes::Static(__self_0) => {
                    Bytes::Static(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Bytes {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Vec<u8>>;
            let _: ::core::cmp::AssertParamIsEq<&'static [u8]>;
        }
    }
    impl PartialEq for Bytes {
        fn eq(&self, other: &Self) -> bool {
            {
                ::std::io::_print(format_args!("Match Bytes: {0:?}\n", self));
            };
            {
                ::std::io::_print(format_args!("Match Bytes: {0:?}\n", other));
            };
            match (self, other) {
                (Bytes::Static(a), Bytes::Static(b)) => a == b,
                (Bytes::Dynamic(a), Bytes::Dynamic(b)) => a == b,
                (Bytes::Static(a), Bytes::Dynamic(b)) => *a == b.as_slice(),
                (Bytes::Dynamic(a), Bytes::Static(b)) => a.as_slice() == *b,
                _ => false,
            }
        }
    }
    impl Hash for Bytes {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            {
                ::std::io::_print(format_args!("Hashing Bytes: {0:?}\n", self));
            };
            match self {
                Bytes::Static(bytes) => bytes.hash(state),
                Bytes::Dynamic(bytes) => bytes.hash(state),
            }
        }
    }
    impl<Ctx: Copy> DekuReader<'static, Ctx> for Bytes {
        fn from_reader_with_ctx<R>(
            reader: &mut Reader<R>,
            inner_ctx: Ctx,
        ) -> Result<Self, DekuError>
        where
            R: std::io::Read + std::io::Seek,
            Self: Sized,
        {
            let mut buffer = [0u8; 3];
            reader.read_bytes_const(&mut buffer)?;
            Ok(Bytes::Dynamic(buffer.to_vec()))
        }
    }
    impl<Ctx: Copy> DekuWriter<Ctx> for Bytes {
        fn to_writer<W: std::io::Write + std::io::Seek>(
            &self,
            writer: &mut Writer<W>,
            _ctx: Ctx,
        ) -> Result<(), DekuError> {
            match self {
                Bytes::Static(bytes) => writer.write_bytes(bytes),
                Bytes::Dynamic(bytes) => writer.write_bytes(bytes),
            }
        }
    }
    #[deku(id_type = "Bytes")]
    pub enum HciEventMsg {
        #[deku(id = "Bytes::Static(&[0x3e, 0x13, 0x01])")]
        LeConnectionComplete {
            status: HciStatus,
            connection_handle: u16,
            role: Role,
            peer_address_type: AddressType,
            peer_address: [u8; 6],
            connection_interval: u16,
            peripheral_latency: u16,
            supervision_timeout: u16,
            central_clock_accuracy: ClockAccuracy,
        },
        #[deku(id = "Bytes::Static(&[0x0E, 0x04])")]
        CommandComplete {
            num_hci_command_packets: u8,
            command_opcode: u16,
            status: HciStatus,
        },
        #[deku(id = "Bytes::Static(&[0x0F, 0x04])")]
        CommandStatus {
            status: HciStatus,
            num_hci_command_packets: u8,
            command_opcode: u16,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for HciEventMsg {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                HciEventMsg::LeConnectionComplete {
                    status: __self_0,
                    connection_handle: __self_1,
                    role: __self_2,
                    peer_address_type: __self_3,
                    peer_address: __self_4,
                    connection_interval: __self_5,
                    peripheral_latency: __self_6,
                    supervision_timeout: __self_7,
                    central_clock_accuracy: __self_8,
                } => {
                    let names: &'static _ = &[
                        "status",
                        "connection_handle",
                        "role",
                        "peer_address_type",
                        "peer_address",
                        "connection_interval",
                        "peripheral_latency",
                        "supervision_timeout",
                        "central_clock_accuracy",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        __self_0,
                        __self_1,
                        __self_2,
                        __self_3,
                        __self_4,
                        __self_5,
                        __self_6,
                        __self_7,
                        &__self_8,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "LeConnectionComplete",
                        names,
                        values,
                    )
                }
                HciEventMsg::CommandComplete {
                    num_hci_command_packets: __self_0,
                    command_opcode: __self_1,
                    status: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "CommandComplete",
                        "num_hci_command_packets",
                        __self_0,
                        "command_opcode",
                        __self_1,
                        "status",
                        &__self_2,
                    )
                }
                HciEventMsg::CommandStatus {
                    status: __self_0,
                    num_hci_command_packets: __self_1,
                    command_opcode: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "CommandStatus",
                        "status",
                        __self_0,
                        "num_hci_command_packets",
                        __self_1,
                        "command_opcode",
                        &__self_2,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for HciEventMsg {
        #[inline]
        fn clone(&self) -> HciEventMsg {
            match self {
                HciEventMsg::LeConnectionComplete {
                    status: __self_0,
                    connection_handle: __self_1,
                    role: __self_2,
                    peer_address_type: __self_3,
                    peer_address: __self_4,
                    connection_interval: __self_5,
                    peripheral_latency: __self_6,
                    supervision_timeout: __self_7,
                    central_clock_accuracy: __self_8,
                } => {
                    HciEventMsg::LeConnectionComplete {
                        status: ::core::clone::Clone::clone(__self_0),
                        connection_handle: ::core::clone::Clone::clone(__self_1),
                        role: ::core::clone::Clone::clone(__self_2),
                        peer_address_type: ::core::clone::Clone::clone(__self_3),
                        peer_address: ::core::clone::Clone::clone(__self_4),
                        connection_interval: ::core::clone::Clone::clone(__self_5),
                        peripheral_latency: ::core::clone::Clone::clone(__self_6),
                        supervision_timeout: ::core::clone::Clone::clone(__self_7),
                        central_clock_accuracy: ::core::clone::Clone::clone(__self_8),
                    }
                }
                HciEventMsg::CommandComplete {
                    num_hci_command_packets: __self_0,
                    command_opcode: __self_1,
                    status: __self_2,
                } => {
                    HciEventMsg::CommandComplete {
                        num_hci_command_packets: ::core::clone::Clone::clone(__self_0),
                        command_opcode: ::core::clone::Clone::clone(__self_1),
                        status: ::core::clone::Clone::clone(__self_2),
                    }
                }
                HciEventMsg::CommandStatus {
                    status: __self_0,
                    num_hci_command_packets: __self_1,
                    command_opcode: __self_2,
                } => {
                    HciEventMsg::CommandStatus {
                        status: ::core::clone::Clone::clone(__self_0),
                        num_hci_command_packets: ::core::clone::Clone::clone(__self_1),
                        command_opcode: ::core::clone::Clone::clone(__self_2),
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for HciEventMsg {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for HciEventMsg {
        #[inline]
        fn eq(&self, other: &HciEventMsg) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        HciEventMsg::LeConnectionComplete {
                            status: __self_0,
                            connection_handle: __self_1,
                            role: __self_2,
                            peer_address_type: __self_3,
                            peer_address: __self_4,
                            connection_interval: __self_5,
                            peripheral_latency: __self_6,
                            supervision_timeout: __self_7,
                            central_clock_accuracy: __self_8,
                        },
                        HciEventMsg::LeConnectionComplete {
                            status: __arg1_0,
                            connection_handle: __arg1_1,
                            role: __arg1_2,
                            peer_address_type: __arg1_3,
                            peer_address: __arg1_4,
                            connection_interval: __arg1_5,
                            peripheral_latency: __arg1_6,
                            supervision_timeout: __arg1_7,
                            central_clock_accuracy: __arg1_8,
                        },
                    ) => {
                        __self_0 == __arg1_0 && __self_1 == __arg1_1
                            && __self_2 == __arg1_2 && __self_3 == __arg1_3
                            && __self_4 == __arg1_4 && __self_5 == __arg1_5
                            && __self_6 == __arg1_6 && __self_7 == __arg1_7
                            && __self_8 == __arg1_8
                    }
                    (
                        HciEventMsg::CommandComplete {
                            num_hci_command_packets: __self_0,
                            command_opcode: __self_1,
                            status: __self_2,
                        },
                        HciEventMsg::CommandComplete {
                            num_hci_command_packets: __arg1_0,
                            command_opcode: __arg1_1,
                            status: __arg1_2,
                        },
                    ) => {
                        __self_0 == __arg1_0 && __self_1 == __arg1_1
                            && __self_2 == __arg1_2
                    }
                    (
                        HciEventMsg::CommandStatus {
                            status: __self_0,
                            num_hci_command_packets: __self_1,
                            command_opcode: __self_2,
                        },
                        HciEventMsg::CommandStatus {
                            status: __arg1_0,
                            num_hci_command_packets: __arg1_1,
                            command_opcode: __arg1_2,
                        },
                    ) => {
                        __self_0 == __arg1_0 && __self_1 == __arg1_1
                            && __self_2 == __arg1_2
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for HciEventMsg {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<HciStatus>;
            let _: ::core::cmp::AssertParamIsEq<u16>;
            let _: ::core::cmp::AssertParamIsEq<Role>;
            let _: ::core::cmp::AssertParamIsEq<AddressType>;
            let _: ::core::cmp::AssertParamIsEq<[u8; 6]>;
            let _: ::core::cmp::AssertParamIsEq<ClockAccuracy>;
            let _: ::core::cmp::AssertParamIsEq<u8>;
        }
    }
    impl core::convert::TryFrom<&'_ [u8]> for HciEventMsg {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: &'_ [u8]) -> core::result::Result<Self, Self::Error> {
            let total_len = input.len();
            let mut cursor = ::deku::no_std_io::Cursor::new(input);
            let (amt_read, res) = <Self as ::deku::DekuContainerRead>::from_reader((
                &mut cursor,
                0,
            ))?;
            if (amt_read / 8) != total_len {
                extern crate alloc;
                use alloc::borrow::Cow;
                return Err(::deku::DekuError::Parse(Cow::from("Too much data")));
            }
            Ok(res)
        }
    }
    impl ::deku::DekuContainerRead<'_> for HciEventMsg {
        #[allow(non_snake_case)]
        #[inline]
        fn from_reader<'a, R: ::deku::no_std_io::Read + ::deku::no_std_io::Seek>(
            __deku_input: (&'a mut R, usize),
        ) -> core::result::Result<(usize, Self), ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            let __deku_reader = &mut deku::reader::Reader::new(__deku_input.0);
            if __deku_input.1 != 0 {
                __deku_reader.skip_bits(__deku_input.1)?;
            }
            let __deku_value = Self::from_reader_with_ctx(__deku_reader, ())?;
            Ok((__deku_reader.bits_read, __deku_value))
        }
        #[allow(non_snake_case)]
        #[inline]
        fn from_bytes(
            __deku_input: (&'_ [u8], usize),
        ) -> core::result::Result<((&'_ [u8], usize), Self), ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            let mut __deku_cursor = deku::no_std_io::Cursor::new(__deku_input.0);
            let mut __deku_reader = &mut deku::reader::Reader::new(&mut __deku_cursor);
            if __deku_input.1 != 0 {
                __deku_reader.skip_bits(__deku_input.1)?;
            }
            let __deku_value = Self::from_reader_with_ctx(__deku_reader, ())?;
            let read_whole_byte = (__deku_reader.bits_read % 8) == 0;
            let idx = if read_whole_byte {
                __deku_reader.bits_read / 8
            } else {
                (__deku_reader.bits_read - (__deku_reader.bits_read % 8)) / 8
            };
            Ok(((&__deku_input.0[idx..], __deku_reader.bits_read % 8), __deku_value))
        }
    }
    #[allow(non_snake_case)]
    impl ::deku::DekuReader<'_, ()> for HciEventMsg {
        #[inline]
        fn from_reader_with_ctx<R: ::deku::no_std_io::Read + ::deku::no_std_io::Seek>(
            __deku_reader: &mut ::deku::reader::Reader<R>,
            _: (),
        ) -> core::result::Result<Self, ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            __deku_reader.last_bits_read_amt = 0;
            let __deku_variant_id = <Bytes>::from_reader_with_ctx(__deku_reader, ())?;
            let __deku_value = match &__deku_variant_id {
                &Bytes::Static(&[0x3e, 0x13, 0x01]) => {
                    let __deku___status = {
                        let __deku_value = <HciStatus as ::deku::DekuReader<
                            '_,
                            _,
                        >>::from_reader_with_ctx(__deku_reader, ())?;
                        let __deku_value: HciStatus = core::result::Result::<
                            _,
                            ::deku::DekuError,
                        >::Ok(__deku_value)?;
                        __deku_value
                    };
                    let status = &__deku___status;
                    let __deku___connection_handle = {
                        let __deku_value = <u16 as ::deku::DekuReader<
                            '_,
                            _,
                        >>::from_reader_with_ctx(__deku_reader, ())?;
                        let __deku_value: u16 = core::result::Result::<
                            _,
                            ::deku::DekuError,
                        >::Ok(__deku_value)?;
                        __deku_value
                    };
                    let connection_handle = &__deku___connection_handle;
                    let __deku___role = {
                        let __deku_value = <Role as ::deku::DekuReader<
                            '_,
                            _,
                        >>::from_reader_with_ctx(__deku_reader, ())?;
                        let __deku_value: Role = core::result::Result::<
                            _,
                            ::deku::DekuError,
                        >::Ok(__deku_value)?;
                        __deku_value
                    };
                    let role = &__deku___role;
                    let __deku___peer_address_type = {
                        let __deku_value = <AddressType as ::deku::DekuReader<
                            '_,
                            _,
                        >>::from_reader_with_ctx(__deku_reader, ())?;
                        let __deku_value: AddressType = core::result::Result::<
                            _,
                            ::deku::DekuError,
                        >::Ok(__deku_value)?;
                        __deku_value
                    };
                    let peer_address_type = &__deku___peer_address_type;
                    let __deku___peer_address = {
                        let __deku_value = <[u8; 6] as ::deku::DekuReader<
                            '_,
                            _,
                        >>::from_reader_with_ctx(__deku_reader, ())?;
                        let __deku_value: [u8; 6] = core::result::Result::<
                            _,
                            ::deku::DekuError,
                        >::Ok(__deku_value)?;
                        __deku_value
                    };
                    let peer_address = &__deku___peer_address;
                    let __deku___connection_interval = {
                        let __deku_value = <u16 as ::deku::DekuReader<
                            '_,
                            _,
                        >>::from_reader_with_ctx(__deku_reader, ())?;
                        let __deku_value: u16 = core::result::Result::<
                            _,
                            ::deku::DekuError,
                        >::Ok(__deku_value)?;
                        __deku_value
                    };
                    let connection_interval = &__deku___connection_interval;
                    let __deku___peripheral_latency = {
                        let __deku_value = <u16 as ::deku::DekuReader<
                            '_,
                            _,
                        >>::from_reader_with_ctx(__deku_reader, ())?;
                        let __deku_value: u16 = core::result::Result::<
                            _,
                            ::deku::DekuError,
                        >::Ok(__deku_value)?;
                        __deku_value
                    };
                    let peripheral_latency = &__deku___peripheral_latency;
                    let __deku___supervision_timeout = {
                        let __deku_value = <u16 as ::deku::DekuReader<
                            '_,
                            _,
                        >>::from_reader_with_ctx(__deku_reader, ())?;
                        let __deku_value: u16 = core::result::Result::<
                            _,
                            ::deku::DekuError,
                        >::Ok(__deku_value)?;
                        __deku_value
                    };
                    let supervision_timeout = &__deku___supervision_timeout;
                    let __deku___central_clock_accuracy = {
                        let __deku_value = <ClockAccuracy as ::deku::DekuReader<
                            '_,
                            _,
                        >>::from_reader_with_ctx(__deku_reader, ())?;
                        let __deku_value: ClockAccuracy = core::result::Result::<
                            _,
                            ::deku::DekuError,
                        >::Ok(__deku_value)?;
                        __deku_value
                    };
                    let central_clock_accuracy = &__deku___central_clock_accuracy;
                    Self::LeConnectionComplete {
                        status: __deku___status,
                        connection_handle: __deku___connection_handle,
                        role: __deku___role,
                        peer_address_type: __deku___peer_address_type,
                        peer_address: __deku___peer_address,
                        connection_interval: __deku___connection_interval,
                        peripheral_latency: __deku___peripheral_latency,
                        supervision_timeout: __deku___supervision_timeout,
                        central_clock_accuracy: __deku___central_clock_accuracy,
                    }
                }
                &Bytes::Static(&[0x0E, 0x04]) => {
                    let __deku___num_hci_command_packets = {
                        let __deku_value = <u8 as ::deku::DekuReader<
                            '_,
                            _,
                        >>::from_reader_with_ctx(__deku_reader, ())?;
                        let __deku_value: u8 = core::result::Result::<
                            _,
                            ::deku::DekuError,
                        >::Ok(__deku_value)?;
                        __deku_value
                    };
                    let num_hci_command_packets = &__deku___num_hci_command_packets;
                    let __deku___command_opcode = {
                        let __deku_value = <u16 as ::deku::DekuReader<
                            '_,
                            _,
                        >>::from_reader_with_ctx(__deku_reader, ())?;
                        let __deku_value: u16 = core::result::Result::<
                            _,
                            ::deku::DekuError,
                        >::Ok(__deku_value)?;
                        __deku_value
                    };
                    let command_opcode = &__deku___command_opcode;
                    let __deku___status = {
                        let __deku_value = <HciStatus as ::deku::DekuReader<
                            '_,
                            _,
                        >>::from_reader_with_ctx(__deku_reader, ())?;
                        let __deku_value: HciStatus = core::result::Result::<
                            _,
                            ::deku::DekuError,
                        >::Ok(__deku_value)?;
                        __deku_value
                    };
                    let status = &__deku___status;
                    Self::CommandComplete {
                        num_hci_command_packets: __deku___num_hci_command_packets,
                        command_opcode: __deku___command_opcode,
                        status: __deku___status,
                    }
                }
                &Bytes::Static(&[0x0F, 0x04]) => {
                    let __deku___status = {
                        let __deku_value = <HciStatus as ::deku::DekuReader<
                            '_,
                            _,
                        >>::from_reader_with_ctx(__deku_reader, ())?;
                        let __deku_value: HciStatus = core::result::Result::<
                            _,
                            ::deku::DekuError,
                        >::Ok(__deku_value)?;
                        __deku_value
                    };
                    let status = &__deku___status;
                    let __deku___num_hci_command_packets = {
                        let __deku_value = <u8 as ::deku::DekuReader<
                            '_,
                            _,
                        >>::from_reader_with_ctx(__deku_reader, ())?;
                        let __deku_value: u8 = core::result::Result::<
                            _,
                            ::deku::DekuError,
                        >::Ok(__deku_value)?;
                        __deku_value
                    };
                    let num_hci_command_packets = &__deku___num_hci_command_packets;
                    let __deku___command_opcode = {
                        let __deku_value = <u16 as ::deku::DekuReader<
                            '_,
                            _,
                        >>::from_reader_with_ctx(__deku_reader, ())?;
                        let __deku_value: u16 = core::result::Result::<
                            _,
                            ::deku::DekuError,
                        >::Ok(__deku_value)?;
                        __deku_value
                    };
                    let command_opcode = &__deku___command_opcode;
                    Self::CommandStatus {
                        status: __deku___status,
                        num_hci_command_packets: __deku___num_hci_command_packets,
                        command_opcode: __deku___command_opcode,
                    }
                }
                _ => {
                    extern crate alloc;
                    use alloc::borrow::Cow;
                    return Err(
                        ::deku::DekuError::Parse(
                            Cow::from(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Could not match enum variant id = {0:?} on enum `{1}`",
                                            __deku_variant_id, "HciEventMsg"
                                        ),
                                    );
                                    res
                                }),
                            ),
                        ),
                    );
                }
            };
            Ok(__deku_value)
        }
    }
    impl<'__deku> ::deku::DekuEnumExt<'_, (Bytes)> for HciEventMsg {
        #[inline]
        fn deku_id(&self) -> core::result::Result<(Bytes), ::deku::DekuError> {
            match self {
                Self::LeConnectionComplete {
                    status: __deku___status,
                    connection_handle: __deku___connection_handle,
                    role: __deku___role,
                    peer_address_type: __deku___peer_address_type,
                    peer_address: __deku___peer_address,
                    connection_interval: __deku___connection_interval,
                    peripheral_latency: __deku___peripheral_latency,
                    supervision_timeout: __deku___supervision_timeout,
                    central_clock_accuracy: __deku___central_clock_accuracy,
                } => Ok(Bytes::Static(&[0x3e, 0x13, 0x01])),
                Self::CommandComplete {
                    num_hci_command_packets: __deku___num_hci_command_packets,
                    command_opcode: __deku___command_opcode,
                    status: __deku___status,
                } => Ok(Bytes::Static(&[0x0E, 0x04])),
                Self::CommandStatus {
                    status: __deku___status,
                    num_hci_command_packets: __deku___num_hci_command_packets,
                    command_opcode: __deku___command_opcode,
                } => Ok(Bytes::Static(&[0x0F, 0x04])),
                _ => Err(::deku::DekuError::IdVariantNotFound),
            }
        }
    }
    impl core::convert::TryFrom<HciEventMsg>
    for ::deku::bitvec::BitVec<u8, ::deku::bitvec::Msb0> {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: HciEventMsg) -> core::result::Result<Self, Self::Error> {
            use ::deku::DekuContainerWrite as _;
            input.to_bits()
        }
    }
    impl core::convert::TryFrom<HciEventMsg> for Vec<u8> {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: HciEventMsg) -> core::result::Result<Self, Self::Error> {
            ::deku::DekuContainerWrite::to_bytes(&input)
        }
    }
    impl ::deku::DekuContainerWrite for HciEventMsg {}
    impl ::deku::DekuUpdate for HciEventMsg {
        #[inline]
        fn update(&mut self) -> core::result::Result<(), ::deku::DekuError> {
            use core::convert::TryInto;
            match self {
                Self::LeConnectionComplete {
                    status,
                    connection_handle,
                    role,
                    peer_address_type,
                    peer_address,
                    connection_interval,
                    peripheral_latency,
                    supervision_timeout,
                    central_clock_accuracy,
                } => {}
                Self::CommandComplete {
                    num_hci_command_packets,
                    command_opcode,
                    status,
                } => {}
                Self::CommandStatus {
                    status,
                    num_hci_command_packets,
                    command_opcode,
                } => {}
            }
            Ok(())
        }
    }
    impl ::deku::DekuWriter<()> for HciEventMsg {
        #[allow(unused_variables)]
        #[inline]
        fn to_writer<W: ::deku::no_std_io::Write + ::deku::no_std_io::Seek>(
            &self,
            __deku_writer: &mut ::deku::writer::Writer<W>,
            _: (),
        ) -> core::result::Result<(), ::deku::DekuError> {
            match self {
                Self::LeConnectionComplete {
                    status,
                    connection_handle,
                    role,
                    peer_address_type,
                    peer_address,
                    connection_interval,
                    peripheral_latency,
                    supervision_timeout,
                    central_clock_accuracy,
                } => {
                    let mut __deku_variant_id: Bytes = Bytes::Static(
                        &[0x3e, 0x13, 0x01],
                    );
                    __deku_variant_id.to_writer(__deku_writer, ())?;
                    ::deku::DekuWriter::to_writer(status, __deku_writer, ())?;
                    ::deku::DekuWriter::to_writer(connection_handle, __deku_writer, ())?;
                    ::deku::DekuWriter::to_writer(role, __deku_writer, ())?;
                    ::deku::DekuWriter::to_writer(peer_address_type, __deku_writer, ())?;
                    ::deku::DekuWriter::to_writer(peer_address, __deku_writer, ())?;
                    ::deku::DekuWriter::to_writer(
                        connection_interval,
                        __deku_writer,
                        (),
                    )?;
                    ::deku::DekuWriter::to_writer(
                        peripheral_latency,
                        __deku_writer,
                        (),
                    )?;
                    ::deku::DekuWriter::to_writer(
                        supervision_timeout,
                        __deku_writer,
                        (),
                    )?;
                    ::deku::DekuWriter::to_writer(
                        central_clock_accuracy,
                        __deku_writer,
                        (),
                    )?;
                }
                Self::CommandComplete {
                    num_hci_command_packets,
                    command_opcode,
                    status,
                } => {
                    let mut __deku_variant_id: Bytes = Bytes::Static(&[0x0E, 0x04]);
                    __deku_variant_id.to_writer(__deku_writer, ())?;
                    ::deku::DekuWriter::to_writer(
                        num_hci_command_packets,
                        __deku_writer,
                        (),
                    )?;
                    ::deku::DekuWriter::to_writer(command_opcode, __deku_writer, ())?;
                    ::deku::DekuWriter::to_writer(status, __deku_writer, ())?;
                }
                Self::CommandStatus {
                    status,
                    num_hci_command_packets,
                    command_opcode,
                } => {
                    let mut __deku_variant_id: Bytes = Bytes::Static(&[0x0F, 0x04]);
                    __deku_variant_id.to_writer(__deku_writer, ())?;
                    ::deku::DekuWriter::to_writer(status, __deku_writer, ())?;
                    ::deku::DekuWriter::to_writer(
                        num_hci_command_packets,
                        __deku_writer,
                        (),
                    )?;
                    ::deku::DekuWriter::to_writer(command_opcode, __deku_writer, ())?;
                }
            }
            Ok(())
        }
    }
    #[deku(id_type = "u8")]
    pub enum HciStatus {
        #[deku(id = "0x00")]
        Success,
        #[deku(id_pat = "&id if id > 0")]
        Failure(u8),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for HciStatus {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                HciStatus::Success => ::core::fmt::Formatter::write_str(f, "Success"),
                HciStatus::Failure(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Failure",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for HciStatus {}
    #[automatically_derived]
    impl ::core::clone::Clone for HciStatus {
        #[inline]
        fn clone(&self) -> HciStatus {
            let _: ::core::clone::AssertParamIsClone<u8>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for HciStatus {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for HciStatus {
        #[inline]
        fn eq(&self, other: &HciStatus) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (HciStatus::Failure(__self_0), HciStatus::Failure(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for HciStatus {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u8>;
        }
    }
    impl core::convert::TryFrom<&'_ [u8]> for HciStatus {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: &'_ [u8]) -> core::result::Result<Self, Self::Error> {
            let total_len = input.len();
            let mut cursor = ::deku::no_std_io::Cursor::new(input);
            let (amt_read, res) = <Self as ::deku::DekuContainerRead>::from_reader((
                &mut cursor,
                0,
            ))?;
            if (amt_read / 8) != total_len {
                extern crate alloc;
                use alloc::borrow::Cow;
                return Err(::deku::DekuError::Parse(Cow::from("Too much data")));
            }
            Ok(res)
        }
    }
    impl ::deku::DekuContainerRead<'_> for HciStatus {
        #[allow(non_snake_case)]
        #[inline]
        fn from_reader<'a, R: ::deku::no_std_io::Read + ::deku::no_std_io::Seek>(
            __deku_input: (&'a mut R, usize),
        ) -> core::result::Result<(usize, Self), ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            let __deku_reader = &mut deku::reader::Reader::new(__deku_input.0);
            if __deku_input.1 != 0 {
                __deku_reader.skip_bits(__deku_input.1)?;
            }
            let __deku_value = Self::from_reader_with_ctx(__deku_reader, ())?;
            Ok((__deku_reader.bits_read, __deku_value))
        }
        #[allow(non_snake_case)]
        #[inline]
        fn from_bytes(
            __deku_input: (&'_ [u8], usize),
        ) -> core::result::Result<((&'_ [u8], usize), Self), ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            let mut __deku_cursor = deku::no_std_io::Cursor::new(__deku_input.0);
            let mut __deku_reader = &mut deku::reader::Reader::new(&mut __deku_cursor);
            if __deku_input.1 != 0 {
                __deku_reader.skip_bits(__deku_input.1)?;
            }
            let __deku_value = Self::from_reader_with_ctx(__deku_reader, ())?;
            let read_whole_byte = (__deku_reader.bits_read % 8) == 0;
            let idx = if read_whole_byte {
                __deku_reader.bits_read / 8
            } else {
                (__deku_reader.bits_read - (__deku_reader.bits_read % 8)) / 8
            };
            Ok(((&__deku_input.0[idx..], __deku_reader.bits_read % 8), __deku_value))
        }
    }
    #[allow(non_snake_case)]
    impl ::deku::DekuReader<'_, ()> for HciStatus {
        #[inline]
        fn from_reader_with_ctx<R: ::deku::no_std_io::Read + ::deku::no_std_io::Seek>(
            __deku_reader: &mut ::deku::reader::Reader<R>,
            _: (),
        ) -> core::result::Result<Self, ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            __deku_reader.last_bits_read_amt = 0;
            let __deku_variant_id = <u8>::from_reader_with_ctx(__deku_reader, ())?;
            let __deku_value = match &__deku_variant_id {
                &0x00 => Self::Success,
                &id if id > 0 => {
                    let __deku___field_0 = {
                        let __deku_value = {
                            if let Err(e) = __deku_reader.seek_last_read() {
                                return Err(::deku::DekuError::Io(e.kind()));
                            }
                            <u8 as ::deku::DekuReader<
                                '_,
                                _,
                            >>::from_reader_with_ctx(__deku_reader, ())?
                        };
                        let __deku_value: u8 = core::result::Result::<
                            _,
                            ::deku::DekuError,
                        >::Ok(__deku_value)?;
                        __deku_value
                    };
                    let field_0 = &__deku___field_0;
                    Self::Failure(__deku___field_0)
                }
                _ => {
                    extern crate alloc;
                    use alloc::borrow::Cow;
                    return Err(
                        ::deku::DekuError::Parse(
                            Cow::from(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Could not match enum variant id = {0:?} on enum `{1}`",
                                            __deku_variant_id, "HciStatus"
                                        ),
                                    );
                                    res
                                }),
                            ),
                        ),
                    );
                }
            };
            Ok(__deku_value)
        }
    }
    impl<'__deku> ::deku::DekuEnumExt<'_, (u8)> for HciStatus {
        #[inline]
        fn deku_id(&self) -> core::result::Result<(u8), ::deku::DekuError> {
            match self {
                Self::Success => Ok(0x00),
                _ => Err(::deku::DekuError::IdVariantNotFound),
            }
        }
    }
    impl core::convert::TryFrom<HciStatus>
    for ::deku::bitvec::BitVec<u8, ::deku::bitvec::Msb0> {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: HciStatus) -> core::result::Result<Self, Self::Error> {
            use ::deku::DekuContainerWrite as _;
            input.to_bits()
        }
    }
    impl core::convert::TryFrom<HciStatus> for Vec<u8> {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: HciStatus) -> core::result::Result<Self, Self::Error> {
            ::deku::DekuContainerWrite::to_bytes(&input)
        }
    }
    impl ::deku::DekuContainerWrite for HciStatus {}
    impl ::deku::DekuUpdate for HciStatus {
        #[inline]
        fn update(&mut self) -> core::result::Result<(), ::deku::DekuError> {
            use core::convert::TryInto;
            match self {
                Self::Success => {}
                Self::Failure(field_0) => {}
            }
            Ok(())
        }
    }
    impl ::deku::DekuWriter<()> for HciStatus {
        #[allow(unused_variables)]
        #[inline]
        fn to_writer<W: ::deku::no_std_io::Write + ::deku::no_std_io::Seek>(
            &self,
            __deku_writer: &mut ::deku::writer::Writer<W>,
            _: (),
        ) -> core::result::Result<(), ::deku::DekuError> {
            match self {
                Self::Success => {
                    let mut __deku_variant_id: u8 = 0x00;
                    __deku_variant_id.to_writer(__deku_writer, ())?;
                }
                Self::Failure(field_0) => {
                    ::deku::DekuWriter::to_writer(field_0, __deku_writer, ())?;
                }
            }
            Ok(())
        }
    }
    #[deku(id_type = "u8")]
    pub enum Role {
        Central = 0x00,
        Peripheral = 0x01,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Role {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Role::Central => "Central",
                    Role::Peripheral => "Peripheral",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Role {}
    #[automatically_derived]
    impl ::core::clone::Clone for Role {
        #[inline]
        fn clone(&self) -> Role {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Role {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Role {
        #[inline]
        fn eq(&self, other: &Role) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Role {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl core::convert::TryFrom<&'_ [u8]> for Role {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: &'_ [u8]) -> core::result::Result<Self, Self::Error> {
            let total_len = input.len();
            let mut cursor = ::deku::no_std_io::Cursor::new(input);
            let (amt_read, res) = <Self as ::deku::DekuContainerRead>::from_reader((
                &mut cursor,
                0,
            ))?;
            if (amt_read / 8) != total_len {
                extern crate alloc;
                use alloc::borrow::Cow;
                return Err(::deku::DekuError::Parse(Cow::from("Too much data")));
            }
            Ok(res)
        }
    }
    impl ::deku::DekuContainerRead<'_> for Role {
        #[allow(non_snake_case)]
        #[inline]
        fn from_reader<'a, R: ::deku::no_std_io::Read + ::deku::no_std_io::Seek>(
            __deku_input: (&'a mut R, usize),
        ) -> core::result::Result<(usize, Self), ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            let __deku_reader = &mut deku::reader::Reader::new(__deku_input.0);
            if __deku_input.1 != 0 {
                __deku_reader.skip_bits(__deku_input.1)?;
            }
            let __deku_value = Self::from_reader_with_ctx(__deku_reader, ())?;
            Ok((__deku_reader.bits_read, __deku_value))
        }
        #[allow(non_snake_case)]
        #[inline]
        fn from_bytes(
            __deku_input: (&'_ [u8], usize),
        ) -> core::result::Result<((&'_ [u8], usize), Self), ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            let mut __deku_cursor = deku::no_std_io::Cursor::new(__deku_input.0);
            let mut __deku_reader = &mut deku::reader::Reader::new(&mut __deku_cursor);
            if __deku_input.1 != 0 {
                __deku_reader.skip_bits(__deku_input.1)?;
            }
            let __deku_value = Self::from_reader_with_ctx(__deku_reader, ())?;
            let read_whole_byte = (__deku_reader.bits_read % 8) == 0;
            let idx = if read_whole_byte {
                __deku_reader.bits_read / 8
            } else {
                (__deku_reader.bits_read - (__deku_reader.bits_read % 8)) / 8
            };
            Ok(((&__deku_input.0[idx..], __deku_reader.bits_read % 8), __deku_value))
        }
    }
    #[allow(non_snake_case)]
    impl ::deku::DekuReader<'_, ()> for Role {
        #[inline]
        fn from_reader_with_ctx<R: ::deku::no_std_io::Read + ::deku::no_std_io::Seek>(
            __deku_reader: &mut ::deku::reader::Reader<R>,
            _: (),
        ) -> core::result::Result<Self, ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            __deku_reader.last_bits_read_amt = 0;
            let __deku_variant_id = <u8>::from_reader_with_ctx(__deku_reader, ())?;
            let __deku___Central = <u8>::try_from(Self::Central as isize)?;
            let __deku___Peripheral = <u8>::try_from(Self::Peripheral as isize)?;
            let __deku_value = match &__deku_variant_id {
                _ if __deku_variant_id == __deku___Central => Self::Central,
                _ if __deku_variant_id == __deku___Peripheral => Self::Peripheral,
                _ => {
                    extern crate alloc;
                    use alloc::borrow::Cow;
                    return Err(
                        ::deku::DekuError::Parse(
                            Cow::from(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Could not match enum variant id = {0:?} on enum `{1}`",
                                            __deku_variant_id, "Role"
                                        ),
                                    );
                                    res
                                }),
                            ),
                        ),
                    );
                }
            };
            Ok(__deku_value)
        }
    }
    impl<'__deku> ::deku::DekuEnumExt<'_, (u8)> for Role {
        #[inline]
        fn deku_id(&self) -> core::result::Result<(u8), ::deku::DekuError> {
            match self {
                _ => Err(::deku::DekuError::IdVariantNotFound),
            }
        }
    }
    impl core::convert::TryFrom<Role>
    for ::deku::bitvec::BitVec<u8, ::deku::bitvec::Msb0> {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: Role) -> core::result::Result<Self, Self::Error> {
            use ::deku::DekuContainerWrite as _;
            input.to_bits()
        }
    }
    impl core::convert::TryFrom<Role> for Vec<u8> {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: Role) -> core::result::Result<Self, Self::Error> {
            ::deku::DekuContainerWrite::to_bytes(&input)
        }
    }
    impl ::deku::DekuContainerWrite for Role {}
    impl ::deku::DekuUpdate for Role {
        #[inline]
        fn update(&mut self) -> core::result::Result<(), ::deku::DekuError> {
            use core::convert::TryInto;
            match self {
                Self::Central => {}
                Self::Peripheral => {}
            }
            Ok(())
        }
    }
    impl ::deku::DekuWriter<()> for Role {
        #[allow(unused_variables)]
        #[inline]
        fn to_writer<W: ::deku::no_std_io::Write + ::deku::no_std_io::Seek>(
            &self,
            __deku_writer: &mut ::deku::writer::Writer<W>,
            _: (),
        ) -> core::result::Result<(), ::deku::DekuError> {
            match self {
                Self::Central => {
                    let mut __deku_variant_id: u8 = Self::Central as u8;
                    __deku_variant_id.to_writer(__deku_writer, ())?;
                }
                Self::Peripheral => {
                    let mut __deku_variant_id: u8 = Self::Peripheral as u8;
                    __deku_variant_id.to_writer(__deku_writer, ())?;
                }
            }
            Ok(())
        }
    }
    #[deku(id_type = "u8")]
    pub enum AddressType {
        Public = 0x00,
        Random = 0x01,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AddressType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    AddressType::Public => "Public",
                    AddressType::Random => "Random",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for AddressType {}
    #[automatically_derived]
    impl ::core::clone::Clone for AddressType {
        #[inline]
        fn clone(&self) -> AddressType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AddressType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AddressType {
        #[inline]
        fn eq(&self, other: &AddressType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for AddressType {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl core::convert::TryFrom<&'_ [u8]> for AddressType {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: &'_ [u8]) -> core::result::Result<Self, Self::Error> {
            let total_len = input.len();
            let mut cursor = ::deku::no_std_io::Cursor::new(input);
            let (amt_read, res) = <Self as ::deku::DekuContainerRead>::from_reader((
                &mut cursor,
                0,
            ))?;
            if (amt_read / 8) != total_len {
                extern crate alloc;
                use alloc::borrow::Cow;
                return Err(::deku::DekuError::Parse(Cow::from("Too much data")));
            }
            Ok(res)
        }
    }
    impl ::deku::DekuContainerRead<'_> for AddressType {
        #[allow(non_snake_case)]
        #[inline]
        fn from_reader<'a, R: ::deku::no_std_io::Read + ::deku::no_std_io::Seek>(
            __deku_input: (&'a mut R, usize),
        ) -> core::result::Result<(usize, Self), ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            let __deku_reader = &mut deku::reader::Reader::new(__deku_input.0);
            if __deku_input.1 != 0 {
                __deku_reader.skip_bits(__deku_input.1)?;
            }
            let __deku_value = Self::from_reader_with_ctx(__deku_reader, ())?;
            Ok((__deku_reader.bits_read, __deku_value))
        }
        #[allow(non_snake_case)]
        #[inline]
        fn from_bytes(
            __deku_input: (&'_ [u8], usize),
        ) -> core::result::Result<((&'_ [u8], usize), Self), ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            let mut __deku_cursor = deku::no_std_io::Cursor::new(__deku_input.0);
            let mut __deku_reader = &mut deku::reader::Reader::new(&mut __deku_cursor);
            if __deku_input.1 != 0 {
                __deku_reader.skip_bits(__deku_input.1)?;
            }
            let __deku_value = Self::from_reader_with_ctx(__deku_reader, ())?;
            let read_whole_byte = (__deku_reader.bits_read % 8) == 0;
            let idx = if read_whole_byte {
                __deku_reader.bits_read / 8
            } else {
                (__deku_reader.bits_read - (__deku_reader.bits_read % 8)) / 8
            };
            Ok(((&__deku_input.0[idx..], __deku_reader.bits_read % 8), __deku_value))
        }
    }
    #[allow(non_snake_case)]
    impl ::deku::DekuReader<'_, ()> for AddressType {
        #[inline]
        fn from_reader_with_ctx<R: ::deku::no_std_io::Read + ::deku::no_std_io::Seek>(
            __deku_reader: &mut ::deku::reader::Reader<R>,
            _: (),
        ) -> core::result::Result<Self, ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            __deku_reader.last_bits_read_amt = 0;
            let __deku_variant_id = <u8>::from_reader_with_ctx(__deku_reader, ())?;
            let __deku___Public = <u8>::try_from(Self::Public as isize)?;
            let __deku___Random = <u8>::try_from(Self::Random as isize)?;
            let __deku_value = match &__deku_variant_id {
                _ if __deku_variant_id == __deku___Public => Self::Public,
                _ if __deku_variant_id == __deku___Random => Self::Random,
                _ => {
                    extern crate alloc;
                    use alloc::borrow::Cow;
                    return Err(
                        ::deku::DekuError::Parse(
                            Cow::from(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Could not match enum variant id = {0:?} on enum `{1}`",
                                            __deku_variant_id, "AddressType"
                                        ),
                                    );
                                    res
                                }),
                            ),
                        ),
                    );
                }
            };
            Ok(__deku_value)
        }
    }
    impl<'__deku> ::deku::DekuEnumExt<'_, (u8)> for AddressType {
        #[inline]
        fn deku_id(&self) -> core::result::Result<(u8), ::deku::DekuError> {
            match self {
                _ => Err(::deku::DekuError::IdVariantNotFound),
            }
        }
    }
    impl core::convert::TryFrom<AddressType>
    for ::deku::bitvec::BitVec<u8, ::deku::bitvec::Msb0> {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: AddressType) -> core::result::Result<Self, Self::Error> {
            use ::deku::DekuContainerWrite as _;
            input.to_bits()
        }
    }
    impl core::convert::TryFrom<AddressType> for Vec<u8> {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: AddressType) -> core::result::Result<Self, Self::Error> {
            ::deku::DekuContainerWrite::to_bytes(&input)
        }
    }
    impl ::deku::DekuContainerWrite for AddressType {}
    impl ::deku::DekuUpdate for AddressType {
        #[inline]
        fn update(&mut self) -> core::result::Result<(), ::deku::DekuError> {
            use core::convert::TryInto;
            match self {
                Self::Public => {}
                Self::Random => {}
            }
            Ok(())
        }
    }
    impl ::deku::DekuWriter<()> for AddressType {
        #[allow(unused_variables)]
        #[inline]
        fn to_writer<W: ::deku::no_std_io::Write + ::deku::no_std_io::Seek>(
            &self,
            __deku_writer: &mut ::deku::writer::Writer<W>,
            _: (),
        ) -> core::result::Result<(), ::deku::DekuError> {
            match self {
                Self::Public => {
                    let mut __deku_variant_id: u8 = Self::Public as u8;
                    __deku_variant_id.to_writer(__deku_writer, ())?;
                }
                Self::Random => {
                    let mut __deku_variant_id: u8 = Self::Random as u8;
                    __deku_variant_id.to_writer(__deku_writer, ())?;
                }
            }
            Ok(())
        }
    }
    #[deku(id_type = "u8")]
    pub enum ClockAccuracy {
        Ppm500 = 0x00,
        Ppm250 = 0x01,
        Ppm150 = 0x02,
        Ppm100 = 0x03,
        Ppm75 = 0x04,
        Ppm50 = 0x05,
        Ppm30 = 0x06,
        Ppm20 = 0x07,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ClockAccuracy {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ClockAccuracy::Ppm500 => "Ppm500",
                    ClockAccuracy::Ppm250 => "Ppm250",
                    ClockAccuracy::Ppm150 => "Ppm150",
                    ClockAccuracy::Ppm100 => "Ppm100",
                    ClockAccuracy::Ppm75 => "Ppm75",
                    ClockAccuracy::Ppm50 => "Ppm50",
                    ClockAccuracy::Ppm30 => "Ppm30",
                    ClockAccuracy::Ppm20 => "Ppm20",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ClockAccuracy {}
    #[automatically_derived]
    impl ::core::clone::Clone for ClockAccuracy {
        #[inline]
        fn clone(&self) -> ClockAccuracy {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ClockAccuracy {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ClockAccuracy {
        #[inline]
        fn eq(&self, other: &ClockAccuracy) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ClockAccuracy {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl core::convert::TryFrom<&'_ [u8]> for ClockAccuracy {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: &'_ [u8]) -> core::result::Result<Self, Self::Error> {
            let total_len = input.len();
            let mut cursor = ::deku::no_std_io::Cursor::new(input);
            let (amt_read, res) = <Self as ::deku::DekuContainerRead>::from_reader((
                &mut cursor,
                0,
            ))?;
            if (amt_read / 8) != total_len {
                extern crate alloc;
                use alloc::borrow::Cow;
                return Err(::deku::DekuError::Parse(Cow::from("Too much data")));
            }
            Ok(res)
        }
    }
    impl ::deku::DekuContainerRead<'_> for ClockAccuracy {
        #[allow(non_snake_case)]
        #[inline]
        fn from_reader<'a, R: ::deku::no_std_io::Read + ::deku::no_std_io::Seek>(
            __deku_input: (&'a mut R, usize),
        ) -> core::result::Result<(usize, Self), ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            let __deku_reader = &mut deku::reader::Reader::new(__deku_input.0);
            if __deku_input.1 != 0 {
                __deku_reader.skip_bits(__deku_input.1)?;
            }
            let __deku_value = Self::from_reader_with_ctx(__deku_reader, ())?;
            Ok((__deku_reader.bits_read, __deku_value))
        }
        #[allow(non_snake_case)]
        #[inline]
        fn from_bytes(
            __deku_input: (&'_ [u8], usize),
        ) -> core::result::Result<((&'_ [u8], usize), Self), ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            let mut __deku_cursor = deku::no_std_io::Cursor::new(__deku_input.0);
            let mut __deku_reader = &mut deku::reader::Reader::new(&mut __deku_cursor);
            if __deku_input.1 != 0 {
                __deku_reader.skip_bits(__deku_input.1)?;
            }
            let __deku_value = Self::from_reader_with_ctx(__deku_reader, ())?;
            let read_whole_byte = (__deku_reader.bits_read % 8) == 0;
            let idx = if read_whole_byte {
                __deku_reader.bits_read / 8
            } else {
                (__deku_reader.bits_read - (__deku_reader.bits_read % 8)) / 8
            };
            Ok(((&__deku_input.0[idx..], __deku_reader.bits_read % 8), __deku_value))
        }
    }
    #[allow(non_snake_case)]
    impl ::deku::DekuReader<'_, ()> for ClockAccuracy {
        #[inline]
        fn from_reader_with_ctx<R: ::deku::no_std_io::Read + ::deku::no_std_io::Seek>(
            __deku_reader: &mut ::deku::reader::Reader<R>,
            _: (),
        ) -> core::result::Result<Self, ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            __deku_reader.last_bits_read_amt = 0;
            let __deku_variant_id = <u8>::from_reader_with_ctx(__deku_reader, ())?;
            let __deku___Ppm500 = <u8>::try_from(Self::Ppm500 as isize)?;
            let __deku___Ppm250 = <u8>::try_from(Self::Ppm250 as isize)?;
            let __deku___Ppm150 = <u8>::try_from(Self::Ppm150 as isize)?;
            let __deku___Ppm100 = <u8>::try_from(Self::Ppm100 as isize)?;
            let __deku___Ppm75 = <u8>::try_from(Self::Ppm75 as isize)?;
            let __deku___Ppm50 = <u8>::try_from(Self::Ppm50 as isize)?;
            let __deku___Ppm30 = <u8>::try_from(Self::Ppm30 as isize)?;
            let __deku___Ppm20 = <u8>::try_from(Self::Ppm20 as isize)?;
            let __deku_value = match &__deku_variant_id {
                _ if __deku_variant_id == __deku___Ppm500 => Self::Ppm500,
                _ if __deku_variant_id == __deku___Ppm250 => Self::Ppm250,
                _ if __deku_variant_id == __deku___Ppm150 => Self::Ppm150,
                _ if __deku_variant_id == __deku___Ppm100 => Self::Ppm100,
                _ if __deku_variant_id == __deku___Ppm75 => Self::Ppm75,
                _ if __deku_variant_id == __deku___Ppm50 => Self::Ppm50,
                _ if __deku_variant_id == __deku___Ppm30 => Self::Ppm30,
                _ if __deku_variant_id == __deku___Ppm20 => Self::Ppm20,
                _ => {
                    extern crate alloc;
                    use alloc::borrow::Cow;
                    return Err(
                        ::deku::DekuError::Parse(
                            Cow::from(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Could not match enum variant id = {0:?} on enum `{1}`",
                                            __deku_variant_id, "ClockAccuracy"
                                        ),
                                    );
                                    res
                                }),
                            ),
                        ),
                    );
                }
            };
            Ok(__deku_value)
        }
    }
    impl<'__deku> ::deku::DekuEnumExt<'_, (u8)> for ClockAccuracy {
        #[inline]
        fn deku_id(&self) -> core::result::Result<(u8), ::deku::DekuError> {
            match self {
                _ => Err(::deku::DekuError::IdVariantNotFound),
            }
        }
    }
    impl core::convert::TryFrom<ClockAccuracy>
    for ::deku::bitvec::BitVec<u8, ::deku::bitvec::Msb0> {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: ClockAccuracy) -> core::result::Result<Self, Self::Error> {
            use ::deku::DekuContainerWrite as _;
            input.to_bits()
        }
    }
    impl core::convert::TryFrom<ClockAccuracy> for Vec<u8> {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: ClockAccuracy) -> core::result::Result<Self, Self::Error> {
            ::deku::DekuContainerWrite::to_bytes(&input)
        }
    }
    impl ::deku::DekuContainerWrite for ClockAccuracy {}
    impl ::deku::DekuUpdate for ClockAccuracy {
        #[inline]
        fn update(&mut self) -> core::result::Result<(), ::deku::DekuError> {
            use core::convert::TryInto;
            match self {
                Self::Ppm500 => {}
                Self::Ppm250 => {}
                Self::Ppm150 => {}
                Self::Ppm100 => {}
                Self::Ppm75 => {}
                Self::Ppm50 => {}
                Self::Ppm30 => {}
                Self::Ppm20 => {}
            }
            Ok(())
        }
    }
    impl ::deku::DekuWriter<()> for ClockAccuracy {
        #[allow(unused_variables)]
        #[inline]
        fn to_writer<W: ::deku::no_std_io::Write + ::deku::no_std_io::Seek>(
            &self,
            __deku_writer: &mut ::deku::writer::Writer<W>,
            _: (),
        ) -> core::result::Result<(), ::deku::DekuError> {
            match self {
                Self::Ppm500 => {
                    let mut __deku_variant_id: u8 = Self::Ppm500 as u8;
                    __deku_variant_id.to_writer(__deku_writer, ())?;
                }
                Self::Ppm250 => {
                    let mut __deku_variant_id: u8 = Self::Ppm250 as u8;
                    __deku_variant_id.to_writer(__deku_writer, ())?;
                }
                Self::Ppm150 => {
                    let mut __deku_variant_id: u8 = Self::Ppm150 as u8;
                    __deku_variant_id.to_writer(__deku_writer, ())?;
                }
                Self::Ppm100 => {
                    let mut __deku_variant_id: u8 = Self::Ppm100 as u8;
                    __deku_variant_id.to_writer(__deku_writer, ())?;
                }
                Self::Ppm75 => {
                    let mut __deku_variant_id: u8 = Self::Ppm75 as u8;
                    __deku_variant_id.to_writer(__deku_writer, ())?;
                }
                Self::Ppm50 => {
                    let mut __deku_variant_id: u8 = Self::Ppm50 as u8;
                    __deku_variant_id.to_writer(__deku_writer, ())?;
                }
                Self::Ppm30 => {
                    let mut __deku_variant_id: u8 = Self::Ppm30 as u8;
                    __deku_variant_id.to_writer(__deku_writer, ())?;
                }
                Self::Ppm20 => {
                    let mut __deku_variant_id: u8 = Self::Ppm20 as u8;
                    __deku_variant_id.to_writer(__deku_writer, ())?;
                }
            }
            Ok(())
        }
    }
    #[deku(endian = "little")]
    struct ConnectionHandle(u16);
    #[automatically_derived]
    impl ::core::fmt::Debug for ConnectionHandle {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "ConnectionHandle",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConnectionHandle {
        #[inline]
        fn clone(&self) -> ConnectionHandle {
            ConnectionHandle(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ConnectionHandle {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ConnectionHandle {
        #[inline]
        fn eq(&self, other: &ConnectionHandle) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ConnectionHandle {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u16>;
        }
    }
    impl core::convert::TryFrom<&'_ [u8]> for ConnectionHandle {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: &'_ [u8]) -> core::result::Result<Self, Self::Error> {
            let total_len = input.len();
            let mut cursor = ::deku::no_std_io::Cursor::new(input);
            let (amt_read, res) = <Self as ::deku::DekuContainerRead>::from_reader((
                &mut cursor,
                0,
            ))?;
            if (amt_read / 8) != total_len {
                extern crate alloc;
                use alloc::borrow::Cow;
                return Err(::deku::DekuError::Parse(Cow::from("Too much data")));
            }
            Ok(res)
        }
    }
    impl ::deku::DekuContainerRead<'_> for ConnectionHandle {
        #[allow(non_snake_case)]
        #[inline]
        fn from_reader<'a, R: ::deku::no_std_io::Read + ::deku::no_std_io::Seek>(
            __deku_input: (&'a mut R, usize),
        ) -> core::result::Result<(usize, Self), ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            let __deku_reader = &mut deku::reader::Reader::new(__deku_input.0);
            if __deku_input.1 != 0 {
                __deku_reader.skip_bits(__deku_input.1)?;
            }
            let __deku_value = Self::from_reader_with_ctx(__deku_reader, ())?;
            Ok((__deku_reader.bits_read, __deku_value))
        }
        #[allow(non_snake_case)]
        #[inline]
        fn from_bytes(
            __deku_input: (&'_ [u8], usize),
        ) -> core::result::Result<((&'_ [u8], usize), Self), ::deku::DekuError> {
            use core::convert::TryFrom;
            use ::deku::DekuReader as _;
            let mut __deku_cursor = deku::no_std_io::Cursor::new(__deku_input.0);
            let mut __deku_reader = &mut deku::reader::Reader::new(&mut __deku_cursor);
            if __deku_input.1 != 0 {
                __deku_reader.skip_bits(__deku_input.1)?;
            }
            let __deku_value = Self::from_reader_with_ctx(__deku_reader, ())?;
            let read_whole_byte = (__deku_reader.bits_read % 8) == 0;
            let idx = if read_whole_byte {
                __deku_reader.bits_read / 8
            } else {
                (__deku_reader.bits_read - (__deku_reader.bits_read % 8)) / 8
            };
            Ok(((&__deku_input.0[idx..], __deku_reader.bits_read % 8), __deku_value))
        }
    }
    impl ::deku::DekuReader<'_, ()> for ConnectionHandle {
        #[inline]
        fn from_reader_with_ctx<R: ::deku::no_std_io::Read + ::deku::no_std_io::Seek>(
            __deku_reader: &mut ::deku::reader::Reader<R>,
            _: (),
        ) -> core::result::Result<Self, ::deku::DekuError> {
            use core::convert::TryFrom;
            let __deku___field_0 = {
                let __deku_value = <u16 as ::deku::DekuReader<
                    '_,
                    _,
                >>::from_reader_with_ctx(__deku_reader, (::deku::ctx::Endian::Little))?;
                let __deku_value: u16 = core::result::Result::<
                    _,
                    ::deku::DekuError,
                >::Ok(__deku_value)?;
                __deku_value
            };
            let field_0 = &__deku___field_0;
            let __deku_value = Self(__deku___field_0);
            Ok(__deku_value)
        }
    }
    impl core::convert::TryFrom<ConnectionHandle>
    for ::deku::bitvec::BitVec<u8, ::deku::bitvec::Msb0> {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: ConnectionHandle) -> core::result::Result<Self, Self::Error> {
            use ::deku::DekuContainerWrite as _;
            input.to_bits()
        }
    }
    impl core::convert::TryFrom<ConnectionHandle> for Vec<u8> {
        type Error = ::deku::DekuError;
        #[inline]
        fn try_from(input: ConnectionHandle) -> core::result::Result<Self, Self::Error> {
            ::deku::DekuContainerWrite::to_bytes(&input)
        }
    }
    impl ::deku::DekuContainerWrite for ConnectionHandle {}
    impl ::deku::DekuUpdate for ConnectionHandle {
        #[inline]
        fn update(&mut self) -> core::result::Result<(), ::deku::DekuError> {
            Ok(())
        }
    }
    impl ::deku::DekuWriter<()> for ConnectionHandle {
        #[allow(unused_variables)]
        #[inline]
        fn to_writer<W: ::deku::no_std_io::Write + ::deku::no_std_io::Seek>(
            &self,
            __deku_writer: &mut ::deku::writer::Writer<W>,
            _: (),
        ) -> core::result::Result<(), ::deku::DekuError> {
            match *self {
                ConnectionHandle(ref field_0) => {
                    ::deku::DekuWriter::to_writer(
                        field_0,
                        __deku_writer,
                        (::deku::ctx::Endian::Little),
                    )?;
                    Ok(())
                }
            }
        }
    }
}
