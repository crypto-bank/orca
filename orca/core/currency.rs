// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct CurrencyPair {
    // message fields
    pub quote: Currency,
    pub base: Currency,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CurrencyPair {}

impl CurrencyPair {
    pub fn new() -> CurrencyPair {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CurrencyPair {
        static mut instance: ::protobuf::lazy::Lazy<CurrencyPair> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CurrencyPair,
        };
        unsafe {
            instance.get(CurrencyPair::new)
        }
    }

    // .orca.core.Currency quote = 1;

    pub fn clear_quote(&mut self) {
        self.quote = Currency::NONE;
    }

    // Param is passed by value, moved
    pub fn set_quote(&mut self, v: Currency) {
        self.quote = v;
    }

    pub fn get_quote(&self) -> Currency {
        self.quote
    }

    fn get_quote_for_reflect(&self) -> &Currency {
        &self.quote
    }

    fn mut_quote_for_reflect(&mut self) -> &mut Currency {
        &mut self.quote
    }

    // .orca.core.Currency base = 2;

    pub fn clear_base(&mut self) {
        self.base = Currency::NONE;
    }

    // Param is passed by value, moved
    pub fn set_base(&mut self, v: Currency) {
        self.base = v;
    }

    pub fn get_base(&self) -> Currency {
        self.base
    }

    fn get_base_for_reflect(&self) -> &Currency {
        &self.base
    }

    fn mut_base_for_reflect(&mut self) -> &mut Currency {
        &mut self.base
    }
}

impl ::protobuf::Message for CurrencyPair {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.quote = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.base = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.quote != Currency::NONE {
            my_size += ::protobuf::rt::enum_size(1, self.quote);
        }
        if self.base != Currency::NONE {
            my_size += ::protobuf::rt::enum_size(2, self.base);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.quote != Currency::NONE {
            os.write_enum(1, self.quote.value())?;
        }
        if self.base != Currency::NONE {
            os.write_enum(2, self.base.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CurrencyPair {
    fn new() -> CurrencyPair {
        CurrencyPair::new()
    }

    fn descriptor_static(_: ::std::option::Option<CurrencyPair>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Currency>>(
                    "quote",
                    CurrencyPair::get_quote_for_reflect,
                    CurrencyPair::mut_quote_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Currency>>(
                    "base",
                    CurrencyPair::get_base_for_reflect,
                    CurrencyPair::mut_base_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CurrencyPair>(
                    "CurrencyPair",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CurrencyPair {
    fn clear(&mut self) {
        self.clear_quote();
        self.clear_base();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CurrencyPair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CurrencyPair {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CurrencyVolume {
    // message fields
    pub currency: Currency,
    pub amount: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CurrencyVolume {}

impl CurrencyVolume {
    pub fn new() -> CurrencyVolume {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CurrencyVolume {
        static mut instance: ::protobuf::lazy::Lazy<CurrencyVolume> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CurrencyVolume,
        };
        unsafe {
            instance.get(CurrencyVolume::new)
        }
    }

    // .orca.core.Currency currency = 1;

    pub fn clear_currency(&mut self) {
        self.currency = Currency::NONE;
    }

    // Param is passed by value, moved
    pub fn set_currency(&mut self, v: Currency) {
        self.currency = v;
    }

    pub fn get_currency(&self) -> Currency {
        self.currency
    }

    fn get_currency_for_reflect(&self) -> &Currency {
        &self.currency
    }

    fn mut_currency_for_reflect(&mut self) -> &mut Currency {
        &mut self.currency
    }

    // uint64 amount = 2;

    pub fn clear_amount(&mut self) {
        self.amount = 0;
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: u64) {
        self.amount = v;
    }

    pub fn get_amount(&self) -> u64 {
        self.amount
    }

    fn get_amount_for_reflect(&self) -> &u64 {
        &self.amount
    }

    fn mut_amount_for_reflect(&mut self) -> &mut u64 {
        &mut self.amount
    }
}

impl ::protobuf::Message for CurrencyVolume {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.currency = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.amount = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.currency != Currency::NONE {
            my_size += ::protobuf::rt::enum_size(1, self.currency);
        }
        if self.amount != 0 {
            my_size += ::protobuf::rt::value_size(2, self.amount, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.currency != Currency::NONE {
            os.write_enum(1, self.currency.value())?;
        }
        if self.amount != 0 {
            os.write_uint64(2, self.amount)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CurrencyVolume {
    fn new() -> CurrencyVolume {
        CurrencyVolume::new()
    }

    fn descriptor_static(_: ::std::option::Option<CurrencyVolume>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Currency>>(
                    "currency",
                    CurrencyVolume::get_currency_for_reflect,
                    CurrencyVolume::mut_currency_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "amount",
                    CurrencyVolume::get_amount_for_reflect,
                    CurrencyVolume::mut_amount_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CurrencyVolume>(
                    "CurrencyVolume",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CurrencyVolume {
    fn clear(&mut self) {
        self.clear_currency();
        self.clear_amount();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CurrencyVolume {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CurrencyVolume {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Currency {
    NONE = 0,
    EUR = 1,
    USD = 2,
    BTC = 3,
    ETH = 4,
    ETC = 5,
    XRP = 6,
    ZEC = 7,
    BCH = 8,
    LTC = 9,
    ABC = 10,
    ABN = 11,
    ABY = 12,
    AC = 13,
    ACC = 14,
    ACES = 15,
    ACN = 16,
    ACOIN = 17,
    ACP = 18,
    ACT = 19,
    ADA = 20,
    ADC = 21,
    ADCN = 22,
    ADK = 23,
    ADL = 24,
    ADST = 25,
    ADT = 26,
    ADX = 27,
    ADZ = 28,
    AE = 29,
    AEON = 30,
    AGLC = 31,
    AGRS = 32,
    AHT = 33,
    AIB = 34,
    AION = 35,
    AIR = 36,
    AKY = 37,
    ALIS = 38,
    ALL = 39,
    ALT = 40,
    ALTC = 41,
    ALTCOM = 42,
    AMB = 43,
    AMBER = 44,
    AMIS = 45,
    AMMO = 46,
    AMP = 47,
    AMS = 48,
    ANC = 49,
    ANI = 50,
    ANT = 51,
    ANTI = 52,
    ANTX = 53,
    APC = 54,
    APW = 55,
    APX = 56,
    ARB = 57,
    ARCO = 58,
    ARDR = 59,
    ARG = 60,
    ARGUS = 61,
    ARI = 62,
    ARK = 63,
    ART = 64,
    ASAFE2 = 65,
    ASC = 66,
    ASN = 67,
    AST = 68,
    ATB = 69,
    ATCC = 70,
    ATL = 71,
    ATM = 72,
    ATMC = 73,
    ATMS = 74,
    ATOM = 75,
    ATS = 76,
    ATX = 77,
    AU = 78,
    AUR = 79,
    AURS = 80,
    AV = 81,
    AVT = 82,
    AXF = 83,
    AXIOM = 84,
    B2X = 85,
    B3 = 86,
    BAC = 87,
    BAS = 88,
    BASH = 89,
    BAY = 90,
    BBC = 91,
    BBP = 92,
    BCAP = 93,
    BCC = 94,
    BCCS = 95,
    BCF = 96,
    BCN = 97,
    BCO = 98,
    BCPT = 99,
    BCY = 100,
    BDL = 101,
    BELA = 102,
    BENJI = 103,
    BERN = 104,
    BEST = 105,
    BGR = 106,
    BIGUP = 107,
    BIOB = 108,
    BIOS = 109,
    BIP = 110,
    BIRDS = 111,
    BIS = 112,
    BIT = 113,
    BITB = 114,
    BITBTC = 115,
    BITCF = 116,
    BITCNY = 117,
    BITEUR = 118,
    BITGOLD = 119,
    BITOK = 120,
    BITS = 121,
    BITSILVER = 122,
    BITUSD = 123,
    BITZ = 124,
    BLAS = 125,
    BLAZR = 126,
    BLC = 127,
    BLITZ = 128,
    BLK = 129,
    BLN = 130,
    BLOCK = 131,
    BLOCKPAY = 132,
    BLRY = 133,
    BLU = 134,
    BLUE = 135,
    BLX = 136,
    BLZ = 137,
    BMC = 138,
    BNB = 139,
    BNT = 140,
    BNX = 141,
    BOAT = 142,
    BOLI = 143,
    BOS = 144,
    BOST = 145,
    BPC = 146,
    BQ = 147,
    BQC = 148,
    BQX = 149,
    BRAIN = 150,
    BRAT = 151,
    BRIA = 152,
    BRIT = 153,
    BRK = 154,
    BRO = 155,
    BRX = 156,
    BSC = 157,
    BSD = 158,
    BSN = 159,
    BSR = 160,
    BSTAR = 161,
    BSTY = 162,
    BT1 = 163,
    BT2 = 164,
    BTA = 165,
    BTB = 166,
    BTBc = 167,
    BTCD = 168,
    BTCM = 169,
    BTCR = 170,
    BTCRED = 171,
    BTCZ = 172,
    BTDX = 173,
    BTPL = 174,
    BTQ = 175,
    BTS = 176,
    BTSR = 177,
    BTU = 178,
    BTWTY = 179,
    BTX = 180,
    BUB = 181,
    BUCKS = 182,
    BUMBA = 183,
    BUN = 184,
    BURST = 185,
    BUZZ = 186,
    BVC = 187,
    BXC = 188,
    BXT = 189,
    BYC = 190,
    C2 = 191,
    CAB = 192,
    CACH = 193,
    CADASTRAL = 194,
    CAG = 195,
    CAGE = 196,
    CALC = 197,
    CANN = 198,
    CAP = 199,
    CARBON = 200,
    CASINO = 201,
    CBD = 202,
    CBX = 203,
    CC = 204,
    CCM100 = 205,
    CCN = 206,
    CCRB = 207,
    CCT = 208,
    CDN = 209,
    CDT = 210,
    CESC = 211,
    CF = 212,
    CFI = 213,
    CFT = 214,
    CHC = 215,
    CHEAP = 216,
    CHESS = 217,
    CHIPS = 218,
    CJ = 219,
    CLAM = 220,
    CLINT = 221,
    CLOAK = 222,
    CLUB = 223,
    CME = 224,
    CMP = 225,
    CMPCO = 226,
    CMT = 227,
    CNC = 228,
    CND = 229,
    CNNC = 230,
    CNO = 231,
    CNT = 232,
    CNX = 233,
    COAL = 234,
    COB = 235,
    COLX = 236,
    CON = 237,
    CONX = 238,
    COR = 239,
    CORG = 240,
    COSS = 241,
    COUPE = 242,
    COVAL = 243,
    COXST = 244,
    CPC = 245,
    CPN = 246,
    CRAVE = 247,
    CRB = 248,
    CREA = 249,
    CREDO = 250,
    CREVA = 251,
    CRM = 252,
    CRT = 253,
    CRW = 254,
    CRX = 255,
    CRYPT = 256,
    CSC = 257,
    CSNO = 258,
    CTIC2 = 259,
    CTIC3 = 260,
    CTO = 261,
    CTR = 262,
    CUBE = 263,
    CURE = 264,
    CV2 = 265,
    CVC = 266,
    CVCOIN = 267,
    CWXT = 268,
    CXT = 269,
    CYC = 270,
    CYDER = 271,
    CYP = 272,
    DALC = 273,
    DAR = 274,
    DAS = 275,
    DASH = 276,
    DASHS = 277,
    DATA = 278,
    DAXX = 279,
    DAY = 280,
    DBG = 281,
    DBIX = 282,
    DBTC = 283,
    DCN = 284,
    DCR = 285,
    DCRE = 286,
    DCT = 287,
    DCY = 288,
    DDF = 289,
    DEM = 290,
    DENT = 291,
    DES = 292,
    DEUS = 293,
    DFS = 294,
    DFT = 295,
    DGB = 296,
    DGC = 297,
    DGCS = 298,
    DGD = 299,
    DIBC = 300,
    DICE = 301,
    DIME = 302,
    DISK = 303,
    DIX = 304,
    DLC = 305,
    DLISK = 306,
    DLT = 307,
    DMB = 308,
    DMC = 309,
    DMD = 310,
    DNR = 311,
    DNT = 312,
    DOGE = 313,
    DOLLAR = 314,
    DON = 315,
    DOPE = 316,
    DOT = 317,
    DOVU = 318,
    DP = 319,
    DPAY = 320,
    DRACO = 321,
    DRM = 322,
    DRS = 323,
    DRT = 324,
    DRXNE = 325,
    DSH = 326,
    DTB = 327,
    DUB = 328,
    DUO = 329,
    DUTCH = 330,
    DVC = 331,
    DYN = 332,
    E4ROW = 333,
    EAC = 334,
    EBET = 335,
    EBIT = 336,
    EBST = 337,
    EBT = 338,
    ECA = 339,
    ECASH = 340,
    ECC = 341,
    ECN = 342,
    ECO = 343,
    ECOB = 344,
    EDG = 345,
    EDO = 346,
    EDOGE = 347,
    EDR = 348,
    EDRC = 349,
    EFL = 350,
    EFYT = 351,
    EGAS = 352,
    EGC = 353,
    EGG = 354,
    EGO = 355,
    EGOLD = 356,
    EL = 357,
    ELC = 358,
    ELE = 359,
    ELITE = 360,
    ELIX = 361,
    ELLA = 362,
    ELS = 363,
    ELTC2 = 364,
    EMB = 365,
    EMC = 366,
    EMC2 = 367,
    EMD = 368,
    EMP = 369,
    EMV = 370,
    ENG = 371,
    ENJ = 372,
    ENRG = 373,
    ENT = 374,
    ENV = 375,
    EOS = 376,
    EOT = 377,
    EQT = 378,
    ERC = 379,
    EREAL = 380,
    ERY = 381,
    ESP = 382,
    ETBS = 383,
    ETG = 384,
    ETHD = 385,
    ETN = 386,
    ETP = 387,
    ETX = 388,
    EUC = 389,
    EUSD = 390,
    EVIL = 391,
    EVO = 392,
    EVR = 393,
    EVX = 394,
    EXCL = 395,
    EXL = 396,
    EXN = 397,
    EXP = 398,
    EXRN = 399,
    FAIR = 400,
    FAL = 401,
    FAP = 402,
    FAZZ = 403,
    FC = 404,
    FC2 = 405,
    FCN = 406,
    FCT = 407,
    FDC = 408,
    FFC = 409,
    FID = 410,
    FIMK = 411,
    FIRE = 412,
    FJC = 413,
    FLAP = 414,
    FLASH = 415,
    FLAX = 416,
    FLDC = 417,
    FLIK = 418,
    FLO = 419,
    FLT = 420,
    FLVR = 421,
    FLY = 422,
    FNC = 423,
    FONZ = 424,
    FOR = 425,
    FRAZ = 426,
    FRC = 427,
    FRGC = 428,
    FRK = 429,
    FRN = 430,
    FRST = 431,
    FRWC = 432,
    FST = 433,
    FTC = 434,
    FUCK = 435,
    FUEL = 436,
    FUN = 437,
    FUNC = 438,
    FUNK = 439,
    FUTC = 440,
    FUZZ = 441,
    FXE = 442,
    FYN = 443,
    FYP = 444,
    G3N = 445,
    GAIA = 446,
    GAIN = 447,
    GAM = 448,
    GAME = 449,
    GAP = 450,
    GARY = 451,
    GAS = 452,
    GAY = 453,
    GB = 454,
    GBC = 455,
    GBG = 456,
    GBRC = 457,
    GBT = 458,
    GBYTE = 459,
    GCN = 460,
    GCR = 461,
    GEERT = 462,
    GEO = 463,
    GIM = 464,
    GLC = 465,
    GLD = 466,
    GLT = 467,
    GML = 468,
    GMT = 469,
    GMX = 470,
    GNO = 471,
    GNT = 472,
    GOLF = 473,
    GOLOS = 474,
    GOOD = 475,
    GP = 476,
    GPL = 477,
    GPU = 478,
    GRC = 479,
    GRE = 480,
    GRID = 481,
    GRN = 482,
    GRS = 483,
    GRT = 484,
    GRWI = 485,
    GSR = 486,
    GTC = 487,
    GUC = 488,
    GUN = 489,
    GUP = 490,
    GXS = 491,
    HAL = 492,
    HALLO = 493,
    HBN = 494,
    HBT = 495,
    HCC = 496,
    HDG = 497,
    HDLB = 498,
    HEAT = 499,
    HERO = 500,
    HGT = 501,
    HIGH = 502,
    HKG = 503,
    HMC = 504,
    HMP = 505,
    HMQ = 506,
    HODL = 507,
    HONEY = 508,
    HPC = 509,
    HSR = 510,
    HTC = 511,
    HTML5 = 512,
    HUC = 513,
    HUSH = 514,
    HVCO = 515,
    HVN = 516,
    HXX = 517,
    HYP = 518,
    HYPER = 519,
    I0C = 520,
    IBANK = 521,
    IBTC = 522,
    ICE = 523,
    ICOB = 524,
    ICON = 525,
    ICOO = 526,
    ICOS = 527,
    ICX = 528,
    IETH = 529,
    IFC = 530,
    IFLT = 531,
    IFT = 532,
    IMPS = 533,
    IMS = 534,
    IMX = 535,
    INCNT = 536,
    IND = 537,
    INDIA = 538,
    INF = 539,
    INFX = 540,
    INPAY = 541,
    INSN = 542,
    INXT = 543,
    IOC = 544,
    ION = 545,
    IOP = 546,
    IQT = 547,
    IRL = 548,
    ISL = 549,
    ITI = 550,
    ITT = 551,
    ITZ = 552,
    IVZ = 553,
    IXC = 554,
    IXT = 555,
    J = 556,
    JET = 557,
    JIN = 558,
    JINN = 559,
    JIO = 560,
    JNS = 561,
    JOBS = 562,
    JS = 563,
    JWL = 564,
    KARMA = 565,
    KASHH = 566,
    KAYI = 567,
    KCS = 568,
    KED = 569,
    KEK = 570,
    KEXCOIN = 571,
    KIC = 572,
    KICK = 573,
    KIN = 574,
    KLC = 575,
    KLN = 576,
    KMD = 577,
    KOBO = 578,
    KORE = 579,
    KRB = 580,
    KRONE = 581,
    KURT = 582,
    KUSH = 583,
    LA = 584,
    LANA = 585,
    LAZ = 586,
    LBC = 587,
    LBTC = 588,
    LCP = 589,
    LDCN = 590,
    LDOGE = 591,
    LEA = 592,
    LEO = 593,
    LEPEN = 594,
    LEX = 595,
    LGD = 596,
    LIFE = 597,
    LINDA = 598,
    LINK = 599,
    LINX = 600,
    LIR = 601,
    LKC = 602,
    LKK = 603,
    LLT = 604,
    LMC = 605,
    LNK = 606,
    LOG = 607,
    LOT = 608,
    LRC = 609,
    LSK = 610,
    LTB = 611,
    LTBC = 612,
    LTCR = 613,
    LTCU = 614,
    LTG = 615,
    LTH = 616,
    LUN = 617,
    LUNA = 618,
    LUX = 619,
    LVPS = 620,
    MAC = 621,
    MAD = 622,
    MAGN = 623,
    MAID = 624,
    MANA = 625,
    MAO = 626,
    MAR = 627,
    MARS = 628,
    MARX = 629,
    MAVRO = 630,
    MAX = 631,
    MAY = 632,
    MBI = 633,
    MBL = 634,
    MBRS = 635,
    MCAP = 636,
    MCI = 637,
    MCO = 638,
    MCR = 639,
    MCRN = 640,
    MDA = 641,
    MEC = 642,
    MEME = 643,
    MEN = 644,
    MEOW = 645,
    MER = 646,
    METAL = 647,
    MG = 648,
    MGM = 649,
    MGO = 650,
    MILO = 651,
    MINEX = 652,
    MINT = 653,
    MIOTA = 654,
    MIU = 655,
    MLN = 656,
    MMXVI = 657,
    MND = 658,
    MNE = 659,
    MNM = 660,
    MNX = 661,
    MOD = 662,
    MOIN = 663,
    MOJO = 664,
    MONA = 665,
    MONETA = 666,
    MONEY = 667,
    MOON = 668,
    MOTO = 669,
    MRC = 670,
    MRJA = 671,
    MRNG = 672,
    MRT = 673,
    MSCN = 674,
    MSD = 675,
    MSP = 676,
    MST = 677,
    MTH = 678,
    MTL = 679,
    MTLMC3 = 680,
    MTM = 681,
    MTNC = 682,
    MUE = 683,
    MUG = 684,
    MUSIC = 685,
    MXT = 686,
    MYB = 687,
    MYST = 688,
    MZC = 689,
    NAMO = 690,
    NANOX = 691,
    NAS = 692,
    NAUT = 693,
    NAV = 694,
    NBE = 695,
    NBIT = 696,
    NDAO = 697,
    NDC = 698,
    NEBL = 699,
    NEO = 700,
    NEOS = 701,
    NETKO = 702,
    NEVA = 703,
    NEWB = 704,
    NKA = 705,
    NLC2 = 706,
    NLG = 707,
    NMC = 708,
    NMR = 709,
    NOBL = 710,
    NODC = 711,
    NOTE = 712,
    NRO = 713,
    NSR = 714,
    NTC = 715,
    NTCC = 716,
    NTO = 717,
    NTRN = 718,
    NTWK = 719,
    NULS = 720,
    NVC = 721,
    NVST = 722,
    NXC = 723,
    NXS = 724,
    NXT = 725,
    NXX = 726,
    NYAN = 727,
    NYC = 728,
    OAX = 729,
    OBITS = 730,
    OCEAN = 731,
    OCL = 732,
    OCOW = 733,
    OCT = 734,
    ODN = 735,
    OFF = 736,
    OHM = 737,
    OK = 738,
    OMC = 739,
    OMG = 740,
    OMNI = 741,
    ONION = 742,
    ONX = 743,
    OP = 744,
    OPAL = 745,
    OPES = 746,
    OPT = 747,
    ORB = 748,
    ORLY = 749,
    ORME = 750,
    OS76 = 751,
    OTN = 752,
    OX = 753,
    P7C = 754,
    PAC = 755,
    PAK = 756,
    PART = 757,
    PASC = 758,
    PASL = 759,
    PAY = 760,
    PAYP = 761,
    PBT = 762,
    PCN = 763,
    PCS = 764,
    PDC = 765,
    PDG = 766,
    PEC = 767,
    PEPECASH = 768,
    PEX = 769,
    PGL = 770,
    PHO = 771,
    PHS = 772,
    PI = 773,
    PIE = 774,
    PIGGY = 775,
    PING = 776,
    PINK = 777,
    PIPL = 778,
    PIRL = 779,
    PIVX = 780,
    PIX = 781,
    PIZZA = 782,
    PKB = 783,
    PLACO = 784,
    PLBT = 785,
    PLNC = 786,
    PLR = 787,
    PLU = 788,
    PND = 789,
    POE = 790,
    POKE = 791,
    POLL = 792,
    PONZI = 793,
    POP = 794,
    POS = 795,
    POST = 796,
    POSW = 797,
    POT = 798,
    POWR = 799,
    PPC = 800,
    PPP = 801,
    PPT = 802,
    PPY = 803,
    PR = 804,
    PRC = 805,
    PRES = 806,
    PRG = 807,
    PRIMU = 808,
    PRM = 809,
    PRN = 810,
    PRO = 811,
    PROC = 812,
    PRX = 813,
    PSB = 814,
    PST = 815,
    PSY = 816,
    PTC = 817,
    PTOY = 818,
    PULSE = 819,
    PURA = 820,
    PUT = 821,
    PWR = 822,
    PX = 823,
    PXC = 824,
    PXI = 825,
    PZM = 826,
    Q2C = 827,
    QAU = 828,
    QBC = 829,
    QBK = 830,
    QBT = 831,
    QCN = 832,
    QORA = 833,
    QRK = 834,
    QRL = 835,
    QTL = 836,
    QTUM = 837,
    QVT = 838,
    QWARK = 839,
    R = 840,
    RADS = 841,
    RAIN = 842,
    RBBT = 843,
    RBIES = 844,
    RBT = 845,
    RBX = 846,
    RBY = 847,
    RC = 848,
    RDD = 849,
    REAL = 850,
    REC = 851,
    RED = 852,
    REE = 853,
    REGA = 854,
    REP = 855,
    REQ = 856,
    REV = 857,
    REX = 858,
    RHFC = 859,
    RHOC = 860,
    RIC = 861,
    RICHX = 862,
    RIDE = 863,
    RISE = 864,
    RIYA = 865,
    RKC = 866,
    RLC = 867,
    RLT = 868,
    RNS = 869,
    ROOFS = 870,
    ROYAL = 871,
    RPC = 872,
    RPX = 873,
    RSGP = 874,
    RUBIT = 875,
    RUNNERS = 876,
    RUP = 877,
    RUPX = 878,
    RUSTBITS = 879,
    RVT = 880,
    SAC = 881,
    SAFEX = 882,
    SAK = 883,
    SALT = 884,
    SAN = 885,
    SANDG = 886,
    SBD = 887,
    SC = 888,
    SCL = 889,
    SCORE = 890,
    SCRT = 891,
    SCS = 892,
    SDC = 893,
    SDP = 894,
    SDRN = 895,
    SEQ = 896,
    SFC = 897,
    SFE = 898,
    SH = 899,
    SHA = 900,
    SHDW = 901,
    SHELL = 902,
    SHIFT = 903,
    SHND = 904,
    SHORTY = 905,
    SIB = 906,
    SIC = 907,
    SIFT = 908,
    SIGMA = 909,
    SIGT = 910,
    SJCX = 911,
    SKC = 912,
    SKIN = 913,
    SKR = 914,
    SKULL = 915,
    SKY = 916,
    SLEVIN = 917,
    SLFI = 918,
    SLG = 919,
    SLING = 920,
    SLM = 921,
    SLR = 922,
    SLS = 923,
    SMART = 924,
    SMC = 925,
    SMLY = 926,
    SMOKE = 927,
    SNAKE = 928,
    SNC = 929,
    SND = 930,
    SNGLS = 931,
    SNM = 932,
    SNRG = 933,
    SNT = 934,
    SOAR = 935,
    SOCC = 936,
    SOIL = 937,
    SOJ = 938,
    SONG = 939,
    SOON = 940,
    SOUL = 941,
    SPACE = 942,
    SPEX = 943,
    SPHR = 944,
    SPORT = 945,
    SPR = 946,
    SPRTS = 947,
    SPT = 948,
    SRC = 949,
    STA = 950,
    START = 951,
    STCN = 952,
    STEEM = 953,
    STEPS = 954,
    STEX = 955,
    STORJ = 956,
    STRAT = 957,
    STRC = 958,
    STS = 959,
    STV = 960,
    STX = 961,
    SUB = 962,
    SUMO = 963,
    SUPER = 964,
    SUR = 965,
    SWIFT = 966,
    SWING = 967,
    SWP = 968,
    SWT = 969,
    SXC = 970,
    SYNC = 971,
    SYNX = 972,
    SYS = 973,
    TAAS = 974,
    TAG = 975,
    TAGR = 976,
    TAJ = 977,
    TALK = 978,
    TCC = 979,
    TCOIN = 980,
    TCR = 981,
    TEAM = 982,
    TEK = 983,
    TELL = 984,
    TER = 985,
    TERA = 986,
    TES = 987,
    TESLA = 988,
    TFL = 989,
    TGC = 990,
    TGT = 991,
    THC = 992,
    THS = 993,
    TIME = 994,
    TIPS = 995,
    TIT = 996,
    TKN = 997,
    TKR = 998,
    TKS = 999,
    TLE = 1000,
    TNT = 1001,
    TOA = 1002,
    TODAY = 1003,
    TOKEN = 1004,
    TOP = 1005,
    TOPAZ = 1006,
    TOR = 1007,
    TRADE = 1008,
    TRC = 1009,
    TRCT = 1010,
    TRI = 1011,
    TRICK = 1012,
    TRIG = 1013,
    TRK = 1014,
    TROLL = 1015,
    TRST = 1016,
    TRUMP = 1017,
    TRUST = 1018,
    TRX = 1019,
    TSE = 1020,
    TSTR = 1021,
    TTC = 1022,
    TURBO = 1023,
    TX = 1024,
    TYC = 1025,
    TYCHO = 1026,
    TZC = 1027,
    UBQ = 1028,
    UET = 1029,
    UFO = 1030,
    UGT = 1031,
    UIS = 1032,
    ULA = 1033,
    UNB = 1034,
    UNC = 1035,
    UNI = 1036,
    UNIC = 1037,
    UNIFY = 1038,
    UNIT = 1039,
    UNITS = 1040,
    UNITY = 1041,
    UNO = 1042,
    UNRC = 1043,
    UNY = 1044,
    UR = 1045,
    URC = 1046,
    URO = 1047,
    USC = 1048,
    USDE = 1049,
    USDT = 1050,
    USNBT = 1051,
    UTA = 1052,
    UTC = 1053,
    V = 1054,
    VAL = 1055,
    VASH = 1056,
    VC = 1057,
    VEC2 = 1058,
    VEN = 1059,
    VERI = 1060,
    VGC = 1061,
    VIA = 1062,
    VIB = 1063,
    VIBE = 1064,
    VIDZ = 1065,
    VIP = 1066,
    VISIO = 1067,
    VIVO = 1068,
    VLT = 1069,
    VLTC = 1070,
    VOISE = 1071,
    VOLT = 1072,
    VOX = 1073,
    VOYA = 1074,
    VPRC = 1075,
    VRC = 1076,
    VRM = 1077,
    VRS = 1078,
    VSL = 1079,
    VSX = 1080,
    VTA = 1081,
    VTC = 1082,
    VTR = 1083,
    VUC = 1084,
    VULC = 1085,
    WA = 1086,
    WARP = 1087,
    WAVES = 1088,
    WAY = 1089,
    WBB = 1090,
    WBC = 1091,
    WCT = 1092,
    WDC = 1093,
    WEC = 1094,
    WEX = 1095,
    WGO = 1096,
    WGR = 1097,
    WHL = 1098,
    WIC = 1099,
    WILD = 1100,
    WINGS = 1101,
    WINK = 1102,
    WMC = 1103,
    WOMEN = 1104,
    WORM = 1105,
    WOW = 1106,
    WSX = 1107,
    WTC = 1108,
    WTT = 1109,
    WYV = 1110,
    X2 = 1111,
    XAS = 1112,
    XAU = 1113,
    XAUR = 1114,
    XBC = 1115,
    XBL = 1116,
    XBTC21 = 1117,
    XBTS = 1118,
    XBY = 1119,
    XC = 1120,
    XCN = 1121,
    XCO = 1122,
    XCP = 1123,
    XCRE = 1124,
    XCS = 1125,
    XCT = 1126,
    XCXT = 1127,
    XDE2 = 1128,
    XDN = 1129,
    XEL = 1130,
    XEM = 1131,
    XFT = 1132,
    XGOX = 1133,
    XGR = 1134,
    XHI = 1135,
    XIN = 1136,
    XIOS = 1137,
    XJO = 1138,
    XLC = 1139,
    XLM = 1140,
    XLR = 1141,
    XMCC = 1142,
    XMG = 1143,
    XMR = 1144,
    XMY = 1145,
    XNG = 1146,
    XNN = 1147,
    XOC = 1148,
    XOT = 1149,
    XP = 1150,
    XPA = 1151,
    XPD = 1152,
    XPM = 1153,
    XPTX = 1154,
    XPY = 1155,
    XQN = 1156,
    XRA = 1157,
    XRB = 1158,
    XRC = 1159,
    XRE = 1160,
    XRL = 1161,
    XRY = 1162,
    XSH = 1163,
    XSPEC = 1164,
    XST = 1165,
    XSTC = 1166,
    XTC = 1167,
    XTD = 1168,
    XTO = 1169,
    XTZ = 1170,
    XUC = 1171,
    XVC = 1172,
    XVE = 1173,
    XVG = 1174,
    XVP = 1175,
    XWC = 1176,
    XZC = 1177,
    YAC = 1178,
    YASH = 1179,
    YES = 1180,
    YOC = 1181,
    YOYOW = 1182,
    ZBC = 1183,
    ZCC = 1184,
    ZCL = 1185,
    ZEIT = 1186,
    ZEN = 1187,
    ZENGOLD = 1188,
    ZENI = 1189,
    ZER = 1190,
    ZET = 1191,
    ZMC = 1192,
    ZNE = 1193,
    ZNY = 1194,
    ZOI = 1195,
    ZRC = 1196,
    ZRX = 1197,
    ZSC = 1198,
    ZSE = 1199,
    ZUR = 1200,
    ZYD = 1201,
}

impl ::protobuf::ProtobufEnum for Currency {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Currency> {
        match value {
            0 => ::std::option::Option::Some(Currency::NONE),
            1 => ::std::option::Option::Some(Currency::EUR),
            2 => ::std::option::Option::Some(Currency::USD),
            3 => ::std::option::Option::Some(Currency::BTC),
            4 => ::std::option::Option::Some(Currency::ETH),
            5 => ::std::option::Option::Some(Currency::ETC),
            6 => ::std::option::Option::Some(Currency::XRP),
            7 => ::std::option::Option::Some(Currency::ZEC),
            8 => ::std::option::Option::Some(Currency::BCH),
            9 => ::std::option::Option::Some(Currency::LTC),
            10 => ::std::option::Option::Some(Currency::ABC),
            11 => ::std::option::Option::Some(Currency::ABN),
            12 => ::std::option::Option::Some(Currency::ABY),
            13 => ::std::option::Option::Some(Currency::AC),
            14 => ::std::option::Option::Some(Currency::ACC),
            15 => ::std::option::Option::Some(Currency::ACES),
            16 => ::std::option::Option::Some(Currency::ACN),
            17 => ::std::option::Option::Some(Currency::ACOIN),
            18 => ::std::option::Option::Some(Currency::ACP),
            19 => ::std::option::Option::Some(Currency::ACT),
            20 => ::std::option::Option::Some(Currency::ADA),
            21 => ::std::option::Option::Some(Currency::ADC),
            22 => ::std::option::Option::Some(Currency::ADCN),
            23 => ::std::option::Option::Some(Currency::ADK),
            24 => ::std::option::Option::Some(Currency::ADL),
            25 => ::std::option::Option::Some(Currency::ADST),
            26 => ::std::option::Option::Some(Currency::ADT),
            27 => ::std::option::Option::Some(Currency::ADX),
            28 => ::std::option::Option::Some(Currency::ADZ),
            29 => ::std::option::Option::Some(Currency::AE),
            30 => ::std::option::Option::Some(Currency::AEON),
            31 => ::std::option::Option::Some(Currency::AGLC),
            32 => ::std::option::Option::Some(Currency::AGRS),
            33 => ::std::option::Option::Some(Currency::AHT),
            34 => ::std::option::Option::Some(Currency::AIB),
            35 => ::std::option::Option::Some(Currency::AION),
            36 => ::std::option::Option::Some(Currency::AIR),
            37 => ::std::option::Option::Some(Currency::AKY),
            38 => ::std::option::Option::Some(Currency::ALIS),
            39 => ::std::option::Option::Some(Currency::ALL),
            40 => ::std::option::Option::Some(Currency::ALT),
            41 => ::std::option::Option::Some(Currency::ALTC),
            42 => ::std::option::Option::Some(Currency::ALTCOM),
            43 => ::std::option::Option::Some(Currency::AMB),
            44 => ::std::option::Option::Some(Currency::AMBER),
            45 => ::std::option::Option::Some(Currency::AMIS),
            46 => ::std::option::Option::Some(Currency::AMMO),
            47 => ::std::option::Option::Some(Currency::AMP),
            48 => ::std::option::Option::Some(Currency::AMS),
            49 => ::std::option::Option::Some(Currency::ANC),
            50 => ::std::option::Option::Some(Currency::ANI),
            51 => ::std::option::Option::Some(Currency::ANT),
            52 => ::std::option::Option::Some(Currency::ANTI),
            53 => ::std::option::Option::Some(Currency::ANTX),
            54 => ::std::option::Option::Some(Currency::APC),
            55 => ::std::option::Option::Some(Currency::APW),
            56 => ::std::option::Option::Some(Currency::APX),
            57 => ::std::option::Option::Some(Currency::ARB),
            58 => ::std::option::Option::Some(Currency::ARCO),
            59 => ::std::option::Option::Some(Currency::ARDR),
            60 => ::std::option::Option::Some(Currency::ARG),
            61 => ::std::option::Option::Some(Currency::ARGUS),
            62 => ::std::option::Option::Some(Currency::ARI),
            63 => ::std::option::Option::Some(Currency::ARK),
            64 => ::std::option::Option::Some(Currency::ART),
            65 => ::std::option::Option::Some(Currency::ASAFE2),
            66 => ::std::option::Option::Some(Currency::ASC),
            67 => ::std::option::Option::Some(Currency::ASN),
            68 => ::std::option::Option::Some(Currency::AST),
            69 => ::std::option::Option::Some(Currency::ATB),
            70 => ::std::option::Option::Some(Currency::ATCC),
            71 => ::std::option::Option::Some(Currency::ATL),
            72 => ::std::option::Option::Some(Currency::ATM),
            73 => ::std::option::Option::Some(Currency::ATMC),
            74 => ::std::option::Option::Some(Currency::ATMS),
            75 => ::std::option::Option::Some(Currency::ATOM),
            76 => ::std::option::Option::Some(Currency::ATS),
            77 => ::std::option::Option::Some(Currency::ATX),
            78 => ::std::option::Option::Some(Currency::AU),
            79 => ::std::option::Option::Some(Currency::AUR),
            80 => ::std::option::Option::Some(Currency::AURS),
            81 => ::std::option::Option::Some(Currency::AV),
            82 => ::std::option::Option::Some(Currency::AVT),
            83 => ::std::option::Option::Some(Currency::AXF),
            84 => ::std::option::Option::Some(Currency::AXIOM),
            85 => ::std::option::Option::Some(Currency::B2X),
            86 => ::std::option::Option::Some(Currency::B3),
            87 => ::std::option::Option::Some(Currency::BAC),
            88 => ::std::option::Option::Some(Currency::BAS),
            89 => ::std::option::Option::Some(Currency::BASH),
            90 => ::std::option::Option::Some(Currency::BAY),
            91 => ::std::option::Option::Some(Currency::BBC),
            92 => ::std::option::Option::Some(Currency::BBP),
            93 => ::std::option::Option::Some(Currency::BCAP),
            94 => ::std::option::Option::Some(Currency::BCC),
            95 => ::std::option::Option::Some(Currency::BCCS),
            96 => ::std::option::Option::Some(Currency::BCF),
            97 => ::std::option::Option::Some(Currency::BCN),
            98 => ::std::option::Option::Some(Currency::BCO),
            99 => ::std::option::Option::Some(Currency::BCPT),
            100 => ::std::option::Option::Some(Currency::BCY),
            101 => ::std::option::Option::Some(Currency::BDL),
            102 => ::std::option::Option::Some(Currency::BELA),
            103 => ::std::option::Option::Some(Currency::BENJI),
            104 => ::std::option::Option::Some(Currency::BERN),
            105 => ::std::option::Option::Some(Currency::BEST),
            106 => ::std::option::Option::Some(Currency::BGR),
            107 => ::std::option::Option::Some(Currency::BIGUP),
            108 => ::std::option::Option::Some(Currency::BIOB),
            109 => ::std::option::Option::Some(Currency::BIOS),
            110 => ::std::option::Option::Some(Currency::BIP),
            111 => ::std::option::Option::Some(Currency::BIRDS),
            112 => ::std::option::Option::Some(Currency::BIS),
            113 => ::std::option::Option::Some(Currency::BIT),
            114 => ::std::option::Option::Some(Currency::BITB),
            115 => ::std::option::Option::Some(Currency::BITBTC),
            116 => ::std::option::Option::Some(Currency::BITCF),
            117 => ::std::option::Option::Some(Currency::BITCNY),
            118 => ::std::option::Option::Some(Currency::BITEUR),
            119 => ::std::option::Option::Some(Currency::BITGOLD),
            120 => ::std::option::Option::Some(Currency::BITOK),
            121 => ::std::option::Option::Some(Currency::BITS),
            122 => ::std::option::Option::Some(Currency::BITSILVER),
            123 => ::std::option::Option::Some(Currency::BITUSD),
            124 => ::std::option::Option::Some(Currency::BITZ),
            125 => ::std::option::Option::Some(Currency::BLAS),
            126 => ::std::option::Option::Some(Currency::BLAZR),
            127 => ::std::option::Option::Some(Currency::BLC),
            128 => ::std::option::Option::Some(Currency::BLITZ),
            129 => ::std::option::Option::Some(Currency::BLK),
            130 => ::std::option::Option::Some(Currency::BLN),
            131 => ::std::option::Option::Some(Currency::BLOCK),
            132 => ::std::option::Option::Some(Currency::BLOCKPAY),
            133 => ::std::option::Option::Some(Currency::BLRY),
            134 => ::std::option::Option::Some(Currency::BLU),
            135 => ::std::option::Option::Some(Currency::BLUE),
            136 => ::std::option::Option::Some(Currency::BLX),
            137 => ::std::option::Option::Some(Currency::BLZ),
            138 => ::std::option::Option::Some(Currency::BMC),
            139 => ::std::option::Option::Some(Currency::BNB),
            140 => ::std::option::Option::Some(Currency::BNT),
            141 => ::std::option::Option::Some(Currency::BNX),
            142 => ::std::option::Option::Some(Currency::BOAT),
            143 => ::std::option::Option::Some(Currency::BOLI),
            144 => ::std::option::Option::Some(Currency::BOS),
            145 => ::std::option::Option::Some(Currency::BOST),
            146 => ::std::option::Option::Some(Currency::BPC),
            147 => ::std::option::Option::Some(Currency::BQ),
            148 => ::std::option::Option::Some(Currency::BQC),
            149 => ::std::option::Option::Some(Currency::BQX),
            150 => ::std::option::Option::Some(Currency::BRAIN),
            151 => ::std::option::Option::Some(Currency::BRAT),
            152 => ::std::option::Option::Some(Currency::BRIA),
            153 => ::std::option::Option::Some(Currency::BRIT),
            154 => ::std::option::Option::Some(Currency::BRK),
            155 => ::std::option::Option::Some(Currency::BRO),
            156 => ::std::option::Option::Some(Currency::BRX),
            157 => ::std::option::Option::Some(Currency::BSC),
            158 => ::std::option::Option::Some(Currency::BSD),
            159 => ::std::option::Option::Some(Currency::BSN),
            160 => ::std::option::Option::Some(Currency::BSR),
            161 => ::std::option::Option::Some(Currency::BSTAR),
            162 => ::std::option::Option::Some(Currency::BSTY),
            163 => ::std::option::Option::Some(Currency::BT1),
            164 => ::std::option::Option::Some(Currency::BT2),
            165 => ::std::option::Option::Some(Currency::BTA),
            166 => ::std::option::Option::Some(Currency::BTB),
            167 => ::std::option::Option::Some(Currency::BTBc),
            168 => ::std::option::Option::Some(Currency::BTCD),
            169 => ::std::option::Option::Some(Currency::BTCM),
            170 => ::std::option::Option::Some(Currency::BTCR),
            171 => ::std::option::Option::Some(Currency::BTCRED),
            172 => ::std::option::Option::Some(Currency::BTCZ),
            173 => ::std::option::Option::Some(Currency::BTDX),
            174 => ::std::option::Option::Some(Currency::BTPL),
            175 => ::std::option::Option::Some(Currency::BTQ),
            176 => ::std::option::Option::Some(Currency::BTS),
            177 => ::std::option::Option::Some(Currency::BTSR),
            178 => ::std::option::Option::Some(Currency::BTU),
            179 => ::std::option::Option::Some(Currency::BTWTY),
            180 => ::std::option::Option::Some(Currency::BTX),
            181 => ::std::option::Option::Some(Currency::BUB),
            182 => ::std::option::Option::Some(Currency::BUCKS),
            183 => ::std::option::Option::Some(Currency::BUMBA),
            184 => ::std::option::Option::Some(Currency::BUN),
            185 => ::std::option::Option::Some(Currency::BURST),
            186 => ::std::option::Option::Some(Currency::BUZZ),
            187 => ::std::option::Option::Some(Currency::BVC),
            188 => ::std::option::Option::Some(Currency::BXC),
            189 => ::std::option::Option::Some(Currency::BXT),
            190 => ::std::option::Option::Some(Currency::BYC),
            191 => ::std::option::Option::Some(Currency::C2),
            192 => ::std::option::Option::Some(Currency::CAB),
            193 => ::std::option::Option::Some(Currency::CACH),
            194 => ::std::option::Option::Some(Currency::CADASTRAL),
            195 => ::std::option::Option::Some(Currency::CAG),
            196 => ::std::option::Option::Some(Currency::CAGE),
            197 => ::std::option::Option::Some(Currency::CALC),
            198 => ::std::option::Option::Some(Currency::CANN),
            199 => ::std::option::Option::Some(Currency::CAP),
            200 => ::std::option::Option::Some(Currency::CARBON),
            201 => ::std::option::Option::Some(Currency::CASINO),
            202 => ::std::option::Option::Some(Currency::CBD),
            203 => ::std::option::Option::Some(Currency::CBX),
            204 => ::std::option::Option::Some(Currency::CC),
            205 => ::std::option::Option::Some(Currency::CCM100),
            206 => ::std::option::Option::Some(Currency::CCN),
            207 => ::std::option::Option::Some(Currency::CCRB),
            208 => ::std::option::Option::Some(Currency::CCT),
            209 => ::std::option::Option::Some(Currency::CDN),
            210 => ::std::option::Option::Some(Currency::CDT),
            211 => ::std::option::Option::Some(Currency::CESC),
            212 => ::std::option::Option::Some(Currency::CF),
            213 => ::std::option::Option::Some(Currency::CFI),
            214 => ::std::option::Option::Some(Currency::CFT),
            215 => ::std::option::Option::Some(Currency::CHC),
            216 => ::std::option::Option::Some(Currency::CHEAP),
            217 => ::std::option::Option::Some(Currency::CHESS),
            218 => ::std::option::Option::Some(Currency::CHIPS),
            219 => ::std::option::Option::Some(Currency::CJ),
            220 => ::std::option::Option::Some(Currency::CLAM),
            221 => ::std::option::Option::Some(Currency::CLINT),
            222 => ::std::option::Option::Some(Currency::CLOAK),
            223 => ::std::option::Option::Some(Currency::CLUB),
            224 => ::std::option::Option::Some(Currency::CME),
            225 => ::std::option::Option::Some(Currency::CMP),
            226 => ::std::option::Option::Some(Currency::CMPCO),
            227 => ::std::option::Option::Some(Currency::CMT),
            228 => ::std::option::Option::Some(Currency::CNC),
            229 => ::std::option::Option::Some(Currency::CND),
            230 => ::std::option::Option::Some(Currency::CNNC),
            231 => ::std::option::Option::Some(Currency::CNO),
            232 => ::std::option::Option::Some(Currency::CNT),
            233 => ::std::option::Option::Some(Currency::CNX),
            234 => ::std::option::Option::Some(Currency::COAL),
            235 => ::std::option::Option::Some(Currency::COB),
            236 => ::std::option::Option::Some(Currency::COLX),
            237 => ::std::option::Option::Some(Currency::CON),
            238 => ::std::option::Option::Some(Currency::CONX),
            239 => ::std::option::Option::Some(Currency::COR),
            240 => ::std::option::Option::Some(Currency::CORG),
            241 => ::std::option::Option::Some(Currency::COSS),
            242 => ::std::option::Option::Some(Currency::COUPE),
            243 => ::std::option::Option::Some(Currency::COVAL),
            244 => ::std::option::Option::Some(Currency::COXST),
            245 => ::std::option::Option::Some(Currency::CPC),
            246 => ::std::option::Option::Some(Currency::CPN),
            247 => ::std::option::Option::Some(Currency::CRAVE),
            248 => ::std::option::Option::Some(Currency::CRB),
            249 => ::std::option::Option::Some(Currency::CREA),
            250 => ::std::option::Option::Some(Currency::CREDO),
            251 => ::std::option::Option::Some(Currency::CREVA),
            252 => ::std::option::Option::Some(Currency::CRM),
            253 => ::std::option::Option::Some(Currency::CRT),
            254 => ::std::option::Option::Some(Currency::CRW),
            255 => ::std::option::Option::Some(Currency::CRX),
            256 => ::std::option::Option::Some(Currency::CRYPT),
            257 => ::std::option::Option::Some(Currency::CSC),
            258 => ::std::option::Option::Some(Currency::CSNO),
            259 => ::std::option::Option::Some(Currency::CTIC2),
            260 => ::std::option::Option::Some(Currency::CTIC3),
            261 => ::std::option::Option::Some(Currency::CTO),
            262 => ::std::option::Option::Some(Currency::CTR),
            263 => ::std::option::Option::Some(Currency::CUBE),
            264 => ::std::option::Option::Some(Currency::CURE),
            265 => ::std::option::Option::Some(Currency::CV2),
            266 => ::std::option::Option::Some(Currency::CVC),
            267 => ::std::option::Option::Some(Currency::CVCOIN),
            268 => ::std::option::Option::Some(Currency::CWXT),
            269 => ::std::option::Option::Some(Currency::CXT),
            270 => ::std::option::Option::Some(Currency::CYC),
            271 => ::std::option::Option::Some(Currency::CYDER),
            272 => ::std::option::Option::Some(Currency::CYP),
            273 => ::std::option::Option::Some(Currency::DALC),
            274 => ::std::option::Option::Some(Currency::DAR),
            275 => ::std::option::Option::Some(Currency::DAS),
            276 => ::std::option::Option::Some(Currency::DASH),
            277 => ::std::option::Option::Some(Currency::DASHS),
            278 => ::std::option::Option::Some(Currency::DATA),
            279 => ::std::option::Option::Some(Currency::DAXX),
            280 => ::std::option::Option::Some(Currency::DAY),
            281 => ::std::option::Option::Some(Currency::DBG),
            282 => ::std::option::Option::Some(Currency::DBIX),
            283 => ::std::option::Option::Some(Currency::DBTC),
            284 => ::std::option::Option::Some(Currency::DCN),
            285 => ::std::option::Option::Some(Currency::DCR),
            286 => ::std::option::Option::Some(Currency::DCRE),
            287 => ::std::option::Option::Some(Currency::DCT),
            288 => ::std::option::Option::Some(Currency::DCY),
            289 => ::std::option::Option::Some(Currency::DDF),
            290 => ::std::option::Option::Some(Currency::DEM),
            291 => ::std::option::Option::Some(Currency::DENT),
            292 => ::std::option::Option::Some(Currency::DES),
            293 => ::std::option::Option::Some(Currency::DEUS),
            294 => ::std::option::Option::Some(Currency::DFS),
            295 => ::std::option::Option::Some(Currency::DFT),
            296 => ::std::option::Option::Some(Currency::DGB),
            297 => ::std::option::Option::Some(Currency::DGC),
            298 => ::std::option::Option::Some(Currency::DGCS),
            299 => ::std::option::Option::Some(Currency::DGD),
            300 => ::std::option::Option::Some(Currency::DIBC),
            301 => ::std::option::Option::Some(Currency::DICE),
            302 => ::std::option::Option::Some(Currency::DIME),
            303 => ::std::option::Option::Some(Currency::DISK),
            304 => ::std::option::Option::Some(Currency::DIX),
            305 => ::std::option::Option::Some(Currency::DLC),
            306 => ::std::option::Option::Some(Currency::DLISK),
            307 => ::std::option::Option::Some(Currency::DLT),
            308 => ::std::option::Option::Some(Currency::DMB),
            309 => ::std::option::Option::Some(Currency::DMC),
            310 => ::std::option::Option::Some(Currency::DMD),
            311 => ::std::option::Option::Some(Currency::DNR),
            312 => ::std::option::Option::Some(Currency::DNT),
            313 => ::std::option::Option::Some(Currency::DOGE),
            314 => ::std::option::Option::Some(Currency::DOLLAR),
            315 => ::std::option::Option::Some(Currency::DON),
            316 => ::std::option::Option::Some(Currency::DOPE),
            317 => ::std::option::Option::Some(Currency::DOT),
            318 => ::std::option::Option::Some(Currency::DOVU),
            319 => ::std::option::Option::Some(Currency::DP),
            320 => ::std::option::Option::Some(Currency::DPAY),
            321 => ::std::option::Option::Some(Currency::DRACO),
            322 => ::std::option::Option::Some(Currency::DRM),
            323 => ::std::option::Option::Some(Currency::DRS),
            324 => ::std::option::Option::Some(Currency::DRT),
            325 => ::std::option::Option::Some(Currency::DRXNE),
            326 => ::std::option::Option::Some(Currency::DSH),
            327 => ::std::option::Option::Some(Currency::DTB),
            328 => ::std::option::Option::Some(Currency::DUB),
            329 => ::std::option::Option::Some(Currency::DUO),
            330 => ::std::option::Option::Some(Currency::DUTCH),
            331 => ::std::option::Option::Some(Currency::DVC),
            332 => ::std::option::Option::Some(Currency::DYN),
            333 => ::std::option::Option::Some(Currency::E4ROW),
            334 => ::std::option::Option::Some(Currency::EAC),
            335 => ::std::option::Option::Some(Currency::EBET),
            336 => ::std::option::Option::Some(Currency::EBIT),
            337 => ::std::option::Option::Some(Currency::EBST),
            338 => ::std::option::Option::Some(Currency::EBT),
            339 => ::std::option::Option::Some(Currency::ECA),
            340 => ::std::option::Option::Some(Currency::ECASH),
            341 => ::std::option::Option::Some(Currency::ECC),
            342 => ::std::option::Option::Some(Currency::ECN),
            343 => ::std::option::Option::Some(Currency::ECO),
            344 => ::std::option::Option::Some(Currency::ECOB),
            345 => ::std::option::Option::Some(Currency::EDG),
            346 => ::std::option::Option::Some(Currency::EDO),
            347 => ::std::option::Option::Some(Currency::EDOGE),
            348 => ::std::option::Option::Some(Currency::EDR),
            349 => ::std::option::Option::Some(Currency::EDRC),
            350 => ::std::option::Option::Some(Currency::EFL),
            351 => ::std::option::Option::Some(Currency::EFYT),
            352 => ::std::option::Option::Some(Currency::EGAS),
            353 => ::std::option::Option::Some(Currency::EGC),
            354 => ::std::option::Option::Some(Currency::EGG),
            355 => ::std::option::Option::Some(Currency::EGO),
            356 => ::std::option::Option::Some(Currency::EGOLD),
            357 => ::std::option::Option::Some(Currency::EL),
            358 => ::std::option::Option::Some(Currency::ELC),
            359 => ::std::option::Option::Some(Currency::ELE),
            360 => ::std::option::Option::Some(Currency::ELITE),
            361 => ::std::option::Option::Some(Currency::ELIX),
            362 => ::std::option::Option::Some(Currency::ELLA),
            363 => ::std::option::Option::Some(Currency::ELS),
            364 => ::std::option::Option::Some(Currency::ELTC2),
            365 => ::std::option::Option::Some(Currency::EMB),
            366 => ::std::option::Option::Some(Currency::EMC),
            367 => ::std::option::Option::Some(Currency::EMC2),
            368 => ::std::option::Option::Some(Currency::EMD),
            369 => ::std::option::Option::Some(Currency::EMP),
            370 => ::std::option::Option::Some(Currency::EMV),
            371 => ::std::option::Option::Some(Currency::ENG),
            372 => ::std::option::Option::Some(Currency::ENJ),
            373 => ::std::option::Option::Some(Currency::ENRG),
            374 => ::std::option::Option::Some(Currency::ENT),
            375 => ::std::option::Option::Some(Currency::ENV),
            376 => ::std::option::Option::Some(Currency::EOS),
            377 => ::std::option::Option::Some(Currency::EOT),
            378 => ::std::option::Option::Some(Currency::EQT),
            379 => ::std::option::Option::Some(Currency::ERC),
            380 => ::std::option::Option::Some(Currency::EREAL),
            381 => ::std::option::Option::Some(Currency::ERY),
            382 => ::std::option::Option::Some(Currency::ESP),
            383 => ::std::option::Option::Some(Currency::ETBS),
            384 => ::std::option::Option::Some(Currency::ETG),
            385 => ::std::option::Option::Some(Currency::ETHD),
            386 => ::std::option::Option::Some(Currency::ETN),
            387 => ::std::option::Option::Some(Currency::ETP),
            388 => ::std::option::Option::Some(Currency::ETX),
            389 => ::std::option::Option::Some(Currency::EUC),
            390 => ::std::option::Option::Some(Currency::EUSD),
            391 => ::std::option::Option::Some(Currency::EVIL),
            392 => ::std::option::Option::Some(Currency::EVO),
            393 => ::std::option::Option::Some(Currency::EVR),
            394 => ::std::option::Option::Some(Currency::EVX),
            395 => ::std::option::Option::Some(Currency::EXCL),
            396 => ::std::option::Option::Some(Currency::EXL),
            397 => ::std::option::Option::Some(Currency::EXN),
            398 => ::std::option::Option::Some(Currency::EXP),
            399 => ::std::option::Option::Some(Currency::EXRN),
            400 => ::std::option::Option::Some(Currency::FAIR),
            401 => ::std::option::Option::Some(Currency::FAL),
            402 => ::std::option::Option::Some(Currency::FAP),
            403 => ::std::option::Option::Some(Currency::FAZZ),
            404 => ::std::option::Option::Some(Currency::FC),
            405 => ::std::option::Option::Some(Currency::FC2),
            406 => ::std::option::Option::Some(Currency::FCN),
            407 => ::std::option::Option::Some(Currency::FCT),
            408 => ::std::option::Option::Some(Currency::FDC),
            409 => ::std::option::Option::Some(Currency::FFC),
            410 => ::std::option::Option::Some(Currency::FID),
            411 => ::std::option::Option::Some(Currency::FIMK),
            412 => ::std::option::Option::Some(Currency::FIRE),
            413 => ::std::option::Option::Some(Currency::FJC),
            414 => ::std::option::Option::Some(Currency::FLAP),
            415 => ::std::option::Option::Some(Currency::FLASH),
            416 => ::std::option::Option::Some(Currency::FLAX),
            417 => ::std::option::Option::Some(Currency::FLDC),
            418 => ::std::option::Option::Some(Currency::FLIK),
            419 => ::std::option::Option::Some(Currency::FLO),
            420 => ::std::option::Option::Some(Currency::FLT),
            421 => ::std::option::Option::Some(Currency::FLVR),
            422 => ::std::option::Option::Some(Currency::FLY),
            423 => ::std::option::Option::Some(Currency::FNC),
            424 => ::std::option::Option::Some(Currency::FONZ),
            425 => ::std::option::Option::Some(Currency::FOR),
            426 => ::std::option::Option::Some(Currency::FRAZ),
            427 => ::std::option::Option::Some(Currency::FRC),
            428 => ::std::option::Option::Some(Currency::FRGC),
            429 => ::std::option::Option::Some(Currency::FRK),
            430 => ::std::option::Option::Some(Currency::FRN),
            431 => ::std::option::Option::Some(Currency::FRST),
            432 => ::std::option::Option::Some(Currency::FRWC),
            433 => ::std::option::Option::Some(Currency::FST),
            434 => ::std::option::Option::Some(Currency::FTC),
            435 => ::std::option::Option::Some(Currency::FUCK),
            436 => ::std::option::Option::Some(Currency::FUEL),
            437 => ::std::option::Option::Some(Currency::FUN),
            438 => ::std::option::Option::Some(Currency::FUNC),
            439 => ::std::option::Option::Some(Currency::FUNK),
            440 => ::std::option::Option::Some(Currency::FUTC),
            441 => ::std::option::Option::Some(Currency::FUZZ),
            442 => ::std::option::Option::Some(Currency::FXE),
            443 => ::std::option::Option::Some(Currency::FYN),
            444 => ::std::option::Option::Some(Currency::FYP),
            445 => ::std::option::Option::Some(Currency::G3N),
            446 => ::std::option::Option::Some(Currency::GAIA),
            447 => ::std::option::Option::Some(Currency::GAIN),
            448 => ::std::option::Option::Some(Currency::GAM),
            449 => ::std::option::Option::Some(Currency::GAME),
            450 => ::std::option::Option::Some(Currency::GAP),
            451 => ::std::option::Option::Some(Currency::GARY),
            452 => ::std::option::Option::Some(Currency::GAS),
            453 => ::std::option::Option::Some(Currency::GAY),
            454 => ::std::option::Option::Some(Currency::GB),
            455 => ::std::option::Option::Some(Currency::GBC),
            456 => ::std::option::Option::Some(Currency::GBG),
            457 => ::std::option::Option::Some(Currency::GBRC),
            458 => ::std::option::Option::Some(Currency::GBT),
            459 => ::std::option::Option::Some(Currency::GBYTE),
            460 => ::std::option::Option::Some(Currency::GCN),
            461 => ::std::option::Option::Some(Currency::GCR),
            462 => ::std::option::Option::Some(Currency::GEERT),
            463 => ::std::option::Option::Some(Currency::GEO),
            464 => ::std::option::Option::Some(Currency::GIM),
            465 => ::std::option::Option::Some(Currency::GLC),
            466 => ::std::option::Option::Some(Currency::GLD),
            467 => ::std::option::Option::Some(Currency::GLT),
            468 => ::std::option::Option::Some(Currency::GML),
            469 => ::std::option::Option::Some(Currency::GMT),
            470 => ::std::option::Option::Some(Currency::GMX),
            471 => ::std::option::Option::Some(Currency::GNO),
            472 => ::std::option::Option::Some(Currency::GNT),
            473 => ::std::option::Option::Some(Currency::GOLF),
            474 => ::std::option::Option::Some(Currency::GOLOS),
            475 => ::std::option::Option::Some(Currency::GOOD),
            476 => ::std::option::Option::Some(Currency::GP),
            477 => ::std::option::Option::Some(Currency::GPL),
            478 => ::std::option::Option::Some(Currency::GPU),
            479 => ::std::option::Option::Some(Currency::GRC),
            480 => ::std::option::Option::Some(Currency::GRE),
            481 => ::std::option::Option::Some(Currency::GRID),
            482 => ::std::option::Option::Some(Currency::GRN),
            483 => ::std::option::Option::Some(Currency::GRS),
            484 => ::std::option::Option::Some(Currency::GRT),
            485 => ::std::option::Option::Some(Currency::GRWI),
            486 => ::std::option::Option::Some(Currency::GSR),
            487 => ::std::option::Option::Some(Currency::GTC),
            488 => ::std::option::Option::Some(Currency::GUC),
            489 => ::std::option::Option::Some(Currency::GUN),
            490 => ::std::option::Option::Some(Currency::GUP),
            491 => ::std::option::Option::Some(Currency::GXS),
            492 => ::std::option::Option::Some(Currency::HAL),
            493 => ::std::option::Option::Some(Currency::HALLO),
            494 => ::std::option::Option::Some(Currency::HBN),
            495 => ::std::option::Option::Some(Currency::HBT),
            496 => ::std::option::Option::Some(Currency::HCC),
            497 => ::std::option::Option::Some(Currency::HDG),
            498 => ::std::option::Option::Some(Currency::HDLB),
            499 => ::std::option::Option::Some(Currency::HEAT),
            500 => ::std::option::Option::Some(Currency::HERO),
            501 => ::std::option::Option::Some(Currency::HGT),
            502 => ::std::option::Option::Some(Currency::HIGH),
            503 => ::std::option::Option::Some(Currency::HKG),
            504 => ::std::option::Option::Some(Currency::HMC),
            505 => ::std::option::Option::Some(Currency::HMP),
            506 => ::std::option::Option::Some(Currency::HMQ),
            507 => ::std::option::Option::Some(Currency::HODL),
            508 => ::std::option::Option::Some(Currency::HONEY),
            509 => ::std::option::Option::Some(Currency::HPC),
            510 => ::std::option::Option::Some(Currency::HSR),
            511 => ::std::option::Option::Some(Currency::HTC),
            512 => ::std::option::Option::Some(Currency::HTML5),
            513 => ::std::option::Option::Some(Currency::HUC),
            514 => ::std::option::Option::Some(Currency::HUSH),
            515 => ::std::option::Option::Some(Currency::HVCO),
            516 => ::std::option::Option::Some(Currency::HVN),
            517 => ::std::option::Option::Some(Currency::HXX),
            518 => ::std::option::Option::Some(Currency::HYP),
            519 => ::std::option::Option::Some(Currency::HYPER),
            520 => ::std::option::Option::Some(Currency::I0C),
            521 => ::std::option::Option::Some(Currency::IBANK),
            522 => ::std::option::Option::Some(Currency::IBTC),
            523 => ::std::option::Option::Some(Currency::ICE),
            524 => ::std::option::Option::Some(Currency::ICOB),
            525 => ::std::option::Option::Some(Currency::ICON),
            526 => ::std::option::Option::Some(Currency::ICOO),
            527 => ::std::option::Option::Some(Currency::ICOS),
            528 => ::std::option::Option::Some(Currency::ICX),
            529 => ::std::option::Option::Some(Currency::IETH),
            530 => ::std::option::Option::Some(Currency::IFC),
            531 => ::std::option::Option::Some(Currency::IFLT),
            532 => ::std::option::Option::Some(Currency::IFT),
            533 => ::std::option::Option::Some(Currency::IMPS),
            534 => ::std::option::Option::Some(Currency::IMS),
            535 => ::std::option::Option::Some(Currency::IMX),
            536 => ::std::option::Option::Some(Currency::INCNT),
            537 => ::std::option::Option::Some(Currency::IND),
            538 => ::std::option::Option::Some(Currency::INDIA),
            539 => ::std::option::Option::Some(Currency::INF),
            540 => ::std::option::Option::Some(Currency::INFX),
            541 => ::std::option::Option::Some(Currency::INPAY),
            542 => ::std::option::Option::Some(Currency::INSN),
            543 => ::std::option::Option::Some(Currency::INXT),
            544 => ::std::option::Option::Some(Currency::IOC),
            545 => ::std::option::Option::Some(Currency::ION),
            546 => ::std::option::Option::Some(Currency::IOP),
            547 => ::std::option::Option::Some(Currency::IQT),
            548 => ::std::option::Option::Some(Currency::IRL),
            549 => ::std::option::Option::Some(Currency::ISL),
            550 => ::std::option::Option::Some(Currency::ITI),
            551 => ::std::option::Option::Some(Currency::ITT),
            552 => ::std::option::Option::Some(Currency::ITZ),
            553 => ::std::option::Option::Some(Currency::IVZ),
            554 => ::std::option::Option::Some(Currency::IXC),
            555 => ::std::option::Option::Some(Currency::IXT),
            556 => ::std::option::Option::Some(Currency::J),
            557 => ::std::option::Option::Some(Currency::JET),
            558 => ::std::option::Option::Some(Currency::JIN),
            559 => ::std::option::Option::Some(Currency::JINN),
            560 => ::std::option::Option::Some(Currency::JIO),
            561 => ::std::option::Option::Some(Currency::JNS),
            562 => ::std::option::Option::Some(Currency::JOBS),
            563 => ::std::option::Option::Some(Currency::JS),
            564 => ::std::option::Option::Some(Currency::JWL),
            565 => ::std::option::Option::Some(Currency::KARMA),
            566 => ::std::option::Option::Some(Currency::KASHH),
            567 => ::std::option::Option::Some(Currency::KAYI),
            568 => ::std::option::Option::Some(Currency::KCS),
            569 => ::std::option::Option::Some(Currency::KED),
            570 => ::std::option::Option::Some(Currency::KEK),
            571 => ::std::option::Option::Some(Currency::KEXCOIN),
            572 => ::std::option::Option::Some(Currency::KIC),
            573 => ::std::option::Option::Some(Currency::KICK),
            574 => ::std::option::Option::Some(Currency::KIN),
            575 => ::std::option::Option::Some(Currency::KLC),
            576 => ::std::option::Option::Some(Currency::KLN),
            577 => ::std::option::Option::Some(Currency::KMD),
            578 => ::std::option::Option::Some(Currency::KOBO),
            579 => ::std::option::Option::Some(Currency::KORE),
            580 => ::std::option::Option::Some(Currency::KRB),
            581 => ::std::option::Option::Some(Currency::KRONE),
            582 => ::std::option::Option::Some(Currency::KURT),
            583 => ::std::option::Option::Some(Currency::KUSH),
            584 => ::std::option::Option::Some(Currency::LA),
            585 => ::std::option::Option::Some(Currency::LANA),
            586 => ::std::option::Option::Some(Currency::LAZ),
            587 => ::std::option::Option::Some(Currency::LBC),
            588 => ::std::option::Option::Some(Currency::LBTC),
            589 => ::std::option::Option::Some(Currency::LCP),
            590 => ::std::option::Option::Some(Currency::LDCN),
            591 => ::std::option::Option::Some(Currency::LDOGE),
            592 => ::std::option::Option::Some(Currency::LEA),
            593 => ::std::option::Option::Some(Currency::LEO),
            594 => ::std::option::Option::Some(Currency::LEPEN),
            595 => ::std::option::Option::Some(Currency::LEX),
            596 => ::std::option::Option::Some(Currency::LGD),
            597 => ::std::option::Option::Some(Currency::LIFE),
            598 => ::std::option::Option::Some(Currency::LINDA),
            599 => ::std::option::Option::Some(Currency::LINK),
            600 => ::std::option::Option::Some(Currency::LINX),
            601 => ::std::option::Option::Some(Currency::LIR),
            602 => ::std::option::Option::Some(Currency::LKC),
            603 => ::std::option::Option::Some(Currency::LKK),
            604 => ::std::option::Option::Some(Currency::LLT),
            605 => ::std::option::Option::Some(Currency::LMC),
            606 => ::std::option::Option::Some(Currency::LNK),
            607 => ::std::option::Option::Some(Currency::LOG),
            608 => ::std::option::Option::Some(Currency::LOT),
            609 => ::std::option::Option::Some(Currency::LRC),
            610 => ::std::option::Option::Some(Currency::LSK),
            611 => ::std::option::Option::Some(Currency::LTB),
            612 => ::std::option::Option::Some(Currency::LTBC),
            613 => ::std::option::Option::Some(Currency::LTCR),
            614 => ::std::option::Option::Some(Currency::LTCU),
            615 => ::std::option::Option::Some(Currency::LTG),
            616 => ::std::option::Option::Some(Currency::LTH),
            617 => ::std::option::Option::Some(Currency::LUN),
            618 => ::std::option::Option::Some(Currency::LUNA),
            619 => ::std::option::Option::Some(Currency::LUX),
            620 => ::std::option::Option::Some(Currency::LVPS),
            621 => ::std::option::Option::Some(Currency::MAC),
            622 => ::std::option::Option::Some(Currency::MAD),
            623 => ::std::option::Option::Some(Currency::MAGN),
            624 => ::std::option::Option::Some(Currency::MAID),
            625 => ::std::option::Option::Some(Currency::MANA),
            626 => ::std::option::Option::Some(Currency::MAO),
            627 => ::std::option::Option::Some(Currency::MAR),
            628 => ::std::option::Option::Some(Currency::MARS),
            629 => ::std::option::Option::Some(Currency::MARX),
            630 => ::std::option::Option::Some(Currency::MAVRO),
            631 => ::std::option::Option::Some(Currency::MAX),
            632 => ::std::option::Option::Some(Currency::MAY),
            633 => ::std::option::Option::Some(Currency::MBI),
            634 => ::std::option::Option::Some(Currency::MBL),
            635 => ::std::option::Option::Some(Currency::MBRS),
            636 => ::std::option::Option::Some(Currency::MCAP),
            637 => ::std::option::Option::Some(Currency::MCI),
            638 => ::std::option::Option::Some(Currency::MCO),
            639 => ::std::option::Option::Some(Currency::MCR),
            640 => ::std::option::Option::Some(Currency::MCRN),
            641 => ::std::option::Option::Some(Currency::MDA),
            642 => ::std::option::Option::Some(Currency::MEC),
            643 => ::std::option::Option::Some(Currency::MEME),
            644 => ::std::option::Option::Some(Currency::MEN),
            645 => ::std::option::Option::Some(Currency::MEOW),
            646 => ::std::option::Option::Some(Currency::MER),
            647 => ::std::option::Option::Some(Currency::METAL),
            648 => ::std::option::Option::Some(Currency::MG),
            649 => ::std::option::Option::Some(Currency::MGM),
            650 => ::std::option::Option::Some(Currency::MGO),
            651 => ::std::option::Option::Some(Currency::MILO),
            652 => ::std::option::Option::Some(Currency::MINEX),
            653 => ::std::option::Option::Some(Currency::MINT),
            654 => ::std::option::Option::Some(Currency::MIOTA),
            655 => ::std::option::Option::Some(Currency::MIU),
            656 => ::std::option::Option::Some(Currency::MLN),
            657 => ::std::option::Option::Some(Currency::MMXVI),
            658 => ::std::option::Option::Some(Currency::MND),
            659 => ::std::option::Option::Some(Currency::MNE),
            660 => ::std::option::Option::Some(Currency::MNM),
            661 => ::std::option::Option::Some(Currency::MNX),
            662 => ::std::option::Option::Some(Currency::MOD),
            663 => ::std::option::Option::Some(Currency::MOIN),
            664 => ::std::option::Option::Some(Currency::MOJO),
            665 => ::std::option::Option::Some(Currency::MONA),
            666 => ::std::option::Option::Some(Currency::MONETA),
            667 => ::std::option::Option::Some(Currency::MONEY),
            668 => ::std::option::Option::Some(Currency::MOON),
            669 => ::std::option::Option::Some(Currency::MOTO),
            670 => ::std::option::Option::Some(Currency::MRC),
            671 => ::std::option::Option::Some(Currency::MRJA),
            672 => ::std::option::Option::Some(Currency::MRNG),
            673 => ::std::option::Option::Some(Currency::MRT),
            674 => ::std::option::Option::Some(Currency::MSCN),
            675 => ::std::option::Option::Some(Currency::MSD),
            676 => ::std::option::Option::Some(Currency::MSP),
            677 => ::std::option::Option::Some(Currency::MST),
            678 => ::std::option::Option::Some(Currency::MTH),
            679 => ::std::option::Option::Some(Currency::MTL),
            680 => ::std::option::Option::Some(Currency::MTLMC3),
            681 => ::std::option::Option::Some(Currency::MTM),
            682 => ::std::option::Option::Some(Currency::MTNC),
            683 => ::std::option::Option::Some(Currency::MUE),
            684 => ::std::option::Option::Some(Currency::MUG),
            685 => ::std::option::Option::Some(Currency::MUSIC),
            686 => ::std::option::Option::Some(Currency::MXT),
            687 => ::std::option::Option::Some(Currency::MYB),
            688 => ::std::option::Option::Some(Currency::MYST),
            689 => ::std::option::Option::Some(Currency::MZC),
            690 => ::std::option::Option::Some(Currency::NAMO),
            691 => ::std::option::Option::Some(Currency::NANOX),
            692 => ::std::option::Option::Some(Currency::NAS),
            693 => ::std::option::Option::Some(Currency::NAUT),
            694 => ::std::option::Option::Some(Currency::NAV),
            695 => ::std::option::Option::Some(Currency::NBE),
            696 => ::std::option::Option::Some(Currency::NBIT),
            697 => ::std::option::Option::Some(Currency::NDAO),
            698 => ::std::option::Option::Some(Currency::NDC),
            699 => ::std::option::Option::Some(Currency::NEBL),
            700 => ::std::option::Option::Some(Currency::NEO),
            701 => ::std::option::Option::Some(Currency::NEOS),
            702 => ::std::option::Option::Some(Currency::NETKO),
            703 => ::std::option::Option::Some(Currency::NEVA),
            704 => ::std::option::Option::Some(Currency::NEWB),
            705 => ::std::option::Option::Some(Currency::NKA),
            706 => ::std::option::Option::Some(Currency::NLC2),
            707 => ::std::option::Option::Some(Currency::NLG),
            708 => ::std::option::Option::Some(Currency::NMC),
            709 => ::std::option::Option::Some(Currency::NMR),
            710 => ::std::option::Option::Some(Currency::NOBL),
            711 => ::std::option::Option::Some(Currency::NODC),
            712 => ::std::option::Option::Some(Currency::NOTE),
            713 => ::std::option::Option::Some(Currency::NRO),
            714 => ::std::option::Option::Some(Currency::NSR),
            715 => ::std::option::Option::Some(Currency::NTC),
            716 => ::std::option::Option::Some(Currency::NTCC),
            717 => ::std::option::Option::Some(Currency::NTO),
            718 => ::std::option::Option::Some(Currency::NTRN),
            719 => ::std::option::Option::Some(Currency::NTWK),
            720 => ::std::option::Option::Some(Currency::NULS),
            721 => ::std::option::Option::Some(Currency::NVC),
            722 => ::std::option::Option::Some(Currency::NVST),
            723 => ::std::option::Option::Some(Currency::NXC),
            724 => ::std::option::Option::Some(Currency::NXS),
            725 => ::std::option::Option::Some(Currency::NXT),
            726 => ::std::option::Option::Some(Currency::NXX),
            727 => ::std::option::Option::Some(Currency::NYAN),
            728 => ::std::option::Option::Some(Currency::NYC),
            729 => ::std::option::Option::Some(Currency::OAX),
            730 => ::std::option::Option::Some(Currency::OBITS),
            731 => ::std::option::Option::Some(Currency::OCEAN),
            732 => ::std::option::Option::Some(Currency::OCL),
            733 => ::std::option::Option::Some(Currency::OCOW),
            734 => ::std::option::Option::Some(Currency::OCT),
            735 => ::std::option::Option::Some(Currency::ODN),
            736 => ::std::option::Option::Some(Currency::OFF),
            737 => ::std::option::Option::Some(Currency::OHM),
            738 => ::std::option::Option::Some(Currency::OK),
            739 => ::std::option::Option::Some(Currency::OMC),
            740 => ::std::option::Option::Some(Currency::OMG),
            741 => ::std::option::Option::Some(Currency::OMNI),
            742 => ::std::option::Option::Some(Currency::ONION),
            743 => ::std::option::Option::Some(Currency::ONX),
            744 => ::std::option::Option::Some(Currency::OP),
            745 => ::std::option::Option::Some(Currency::OPAL),
            746 => ::std::option::Option::Some(Currency::OPES),
            747 => ::std::option::Option::Some(Currency::OPT),
            748 => ::std::option::Option::Some(Currency::ORB),
            749 => ::std::option::Option::Some(Currency::ORLY),
            750 => ::std::option::Option::Some(Currency::ORME),
            751 => ::std::option::Option::Some(Currency::OS76),
            752 => ::std::option::Option::Some(Currency::OTN),
            753 => ::std::option::Option::Some(Currency::OX),
            754 => ::std::option::Option::Some(Currency::P7C),
            755 => ::std::option::Option::Some(Currency::PAC),
            756 => ::std::option::Option::Some(Currency::PAK),
            757 => ::std::option::Option::Some(Currency::PART),
            758 => ::std::option::Option::Some(Currency::PASC),
            759 => ::std::option::Option::Some(Currency::PASL),
            760 => ::std::option::Option::Some(Currency::PAY),
            761 => ::std::option::Option::Some(Currency::PAYP),
            762 => ::std::option::Option::Some(Currency::PBT),
            763 => ::std::option::Option::Some(Currency::PCN),
            764 => ::std::option::Option::Some(Currency::PCS),
            765 => ::std::option::Option::Some(Currency::PDC),
            766 => ::std::option::Option::Some(Currency::PDG),
            767 => ::std::option::Option::Some(Currency::PEC),
            768 => ::std::option::Option::Some(Currency::PEPECASH),
            769 => ::std::option::Option::Some(Currency::PEX),
            770 => ::std::option::Option::Some(Currency::PGL),
            771 => ::std::option::Option::Some(Currency::PHO),
            772 => ::std::option::Option::Some(Currency::PHS),
            773 => ::std::option::Option::Some(Currency::PI),
            774 => ::std::option::Option::Some(Currency::PIE),
            775 => ::std::option::Option::Some(Currency::PIGGY),
            776 => ::std::option::Option::Some(Currency::PING),
            777 => ::std::option::Option::Some(Currency::PINK),
            778 => ::std::option::Option::Some(Currency::PIPL),
            779 => ::std::option::Option::Some(Currency::PIRL),
            780 => ::std::option::Option::Some(Currency::PIVX),
            781 => ::std::option::Option::Some(Currency::PIX),
            782 => ::std::option::Option::Some(Currency::PIZZA),
            783 => ::std::option::Option::Some(Currency::PKB),
            784 => ::std::option::Option::Some(Currency::PLACO),
            785 => ::std::option::Option::Some(Currency::PLBT),
            786 => ::std::option::Option::Some(Currency::PLNC),
            787 => ::std::option::Option::Some(Currency::PLR),
            788 => ::std::option::Option::Some(Currency::PLU),
            789 => ::std::option::Option::Some(Currency::PND),
            790 => ::std::option::Option::Some(Currency::POE),
            791 => ::std::option::Option::Some(Currency::POKE),
            792 => ::std::option::Option::Some(Currency::POLL),
            793 => ::std::option::Option::Some(Currency::PONZI),
            794 => ::std::option::Option::Some(Currency::POP),
            795 => ::std::option::Option::Some(Currency::POS),
            796 => ::std::option::Option::Some(Currency::POST),
            797 => ::std::option::Option::Some(Currency::POSW),
            798 => ::std::option::Option::Some(Currency::POT),
            799 => ::std::option::Option::Some(Currency::POWR),
            800 => ::std::option::Option::Some(Currency::PPC),
            801 => ::std::option::Option::Some(Currency::PPP),
            802 => ::std::option::Option::Some(Currency::PPT),
            803 => ::std::option::Option::Some(Currency::PPY),
            804 => ::std::option::Option::Some(Currency::PR),
            805 => ::std::option::Option::Some(Currency::PRC),
            806 => ::std::option::Option::Some(Currency::PRES),
            807 => ::std::option::Option::Some(Currency::PRG),
            808 => ::std::option::Option::Some(Currency::PRIMU),
            809 => ::std::option::Option::Some(Currency::PRM),
            810 => ::std::option::Option::Some(Currency::PRN),
            811 => ::std::option::Option::Some(Currency::PRO),
            812 => ::std::option::Option::Some(Currency::PROC),
            813 => ::std::option::Option::Some(Currency::PRX),
            814 => ::std::option::Option::Some(Currency::PSB),
            815 => ::std::option::Option::Some(Currency::PST),
            816 => ::std::option::Option::Some(Currency::PSY),
            817 => ::std::option::Option::Some(Currency::PTC),
            818 => ::std::option::Option::Some(Currency::PTOY),
            819 => ::std::option::Option::Some(Currency::PULSE),
            820 => ::std::option::Option::Some(Currency::PURA),
            821 => ::std::option::Option::Some(Currency::PUT),
            822 => ::std::option::Option::Some(Currency::PWR),
            823 => ::std::option::Option::Some(Currency::PX),
            824 => ::std::option::Option::Some(Currency::PXC),
            825 => ::std::option::Option::Some(Currency::PXI),
            826 => ::std::option::Option::Some(Currency::PZM),
            827 => ::std::option::Option::Some(Currency::Q2C),
            828 => ::std::option::Option::Some(Currency::QAU),
            829 => ::std::option::Option::Some(Currency::QBC),
            830 => ::std::option::Option::Some(Currency::QBK),
            831 => ::std::option::Option::Some(Currency::QBT),
            832 => ::std::option::Option::Some(Currency::QCN),
            833 => ::std::option::Option::Some(Currency::QORA),
            834 => ::std::option::Option::Some(Currency::QRK),
            835 => ::std::option::Option::Some(Currency::QRL),
            836 => ::std::option::Option::Some(Currency::QTL),
            837 => ::std::option::Option::Some(Currency::QTUM),
            838 => ::std::option::Option::Some(Currency::QVT),
            839 => ::std::option::Option::Some(Currency::QWARK),
            840 => ::std::option::Option::Some(Currency::R),
            841 => ::std::option::Option::Some(Currency::RADS),
            842 => ::std::option::Option::Some(Currency::RAIN),
            843 => ::std::option::Option::Some(Currency::RBBT),
            844 => ::std::option::Option::Some(Currency::RBIES),
            845 => ::std::option::Option::Some(Currency::RBT),
            846 => ::std::option::Option::Some(Currency::RBX),
            847 => ::std::option::Option::Some(Currency::RBY),
            848 => ::std::option::Option::Some(Currency::RC),
            849 => ::std::option::Option::Some(Currency::RDD),
            850 => ::std::option::Option::Some(Currency::REAL),
            851 => ::std::option::Option::Some(Currency::REC),
            852 => ::std::option::Option::Some(Currency::RED),
            853 => ::std::option::Option::Some(Currency::REE),
            854 => ::std::option::Option::Some(Currency::REGA),
            855 => ::std::option::Option::Some(Currency::REP),
            856 => ::std::option::Option::Some(Currency::REQ),
            857 => ::std::option::Option::Some(Currency::REV),
            858 => ::std::option::Option::Some(Currency::REX),
            859 => ::std::option::Option::Some(Currency::RHFC),
            860 => ::std::option::Option::Some(Currency::RHOC),
            861 => ::std::option::Option::Some(Currency::RIC),
            862 => ::std::option::Option::Some(Currency::RICHX),
            863 => ::std::option::Option::Some(Currency::RIDE),
            864 => ::std::option::Option::Some(Currency::RISE),
            865 => ::std::option::Option::Some(Currency::RIYA),
            866 => ::std::option::Option::Some(Currency::RKC),
            867 => ::std::option::Option::Some(Currency::RLC),
            868 => ::std::option::Option::Some(Currency::RLT),
            869 => ::std::option::Option::Some(Currency::RNS),
            870 => ::std::option::Option::Some(Currency::ROOFS),
            871 => ::std::option::Option::Some(Currency::ROYAL),
            872 => ::std::option::Option::Some(Currency::RPC),
            873 => ::std::option::Option::Some(Currency::RPX),
            874 => ::std::option::Option::Some(Currency::RSGP),
            875 => ::std::option::Option::Some(Currency::RUBIT),
            876 => ::std::option::Option::Some(Currency::RUNNERS),
            877 => ::std::option::Option::Some(Currency::RUP),
            878 => ::std::option::Option::Some(Currency::RUPX),
            879 => ::std::option::Option::Some(Currency::RUSTBITS),
            880 => ::std::option::Option::Some(Currency::RVT),
            881 => ::std::option::Option::Some(Currency::SAC),
            882 => ::std::option::Option::Some(Currency::SAFEX),
            883 => ::std::option::Option::Some(Currency::SAK),
            884 => ::std::option::Option::Some(Currency::SALT),
            885 => ::std::option::Option::Some(Currency::SAN),
            886 => ::std::option::Option::Some(Currency::SANDG),
            887 => ::std::option::Option::Some(Currency::SBD),
            888 => ::std::option::Option::Some(Currency::SC),
            889 => ::std::option::Option::Some(Currency::SCL),
            890 => ::std::option::Option::Some(Currency::SCORE),
            891 => ::std::option::Option::Some(Currency::SCRT),
            892 => ::std::option::Option::Some(Currency::SCS),
            893 => ::std::option::Option::Some(Currency::SDC),
            894 => ::std::option::Option::Some(Currency::SDP),
            895 => ::std::option::Option::Some(Currency::SDRN),
            896 => ::std::option::Option::Some(Currency::SEQ),
            897 => ::std::option::Option::Some(Currency::SFC),
            898 => ::std::option::Option::Some(Currency::SFE),
            899 => ::std::option::Option::Some(Currency::SH),
            900 => ::std::option::Option::Some(Currency::SHA),
            901 => ::std::option::Option::Some(Currency::SHDW),
            902 => ::std::option::Option::Some(Currency::SHELL),
            903 => ::std::option::Option::Some(Currency::SHIFT),
            904 => ::std::option::Option::Some(Currency::SHND),
            905 => ::std::option::Option::Some(Currency::SHORTY),
            906 => ::std::option::Option::Some(Currency::SIB),
            907 => ::std::option::Option::Some(Currency::SIC),
            908 => ::std::option::Option::Some(Currency::SIFT),
            909 => ::std::option::Option::Some(Currency::SIGMA),
            910 => ::std::option::Option::Some(Currency::SIGT),
            911 => ::std::option::Option::Some(Currency::SJCX),
            912 => ::std::option::Option::Some(Currency::SKC),
            913 => ::std::option::Option::Some(Currency::SKIN),
            914 => ::std::option::Option::Some(Currency::SKR),
            915 => ::std::option::Option::Some(Currency::SKULL),
            916 => ::std::option::Option::Some(Currency::SKY),
            917 => ::std::option::Option::Some(Currency::SLEVIN),
            918 => ::std::option::Option::Some(Currency::SLFI),
            919 => ::std::option::Option::Some(Currency::SLG),
            920 => ::std::option::Option::Some(Currency::SLING),
            921 => ::std::option::Option::Some(Currency::SLM),
            922 => ::std::option::Option::Some(Currency::SLR),
            923 => ::std::option::Option::Some(Currency::SLS),
            924 => ::std::option::Option::Some(Currency::SMART),
            925 => ::std::option::Option::Some(Currency::SMC),
            926 => ::std::option::Option::Some(Currency::SMLY),
            927 => ::std::option::Option::Some(Currency::SMOKE),
            928 => ::std::option::Option::Some(Currency::SNAKE),
            929 => ::std::option::Option::Some(Currency::SNC),
            930 => ::std::option::Option::Some(Currency::SND),
            931 => ::std::option::Option::Some(Currency::SNGLS),
            932 => ::std::option::Option::Some(Currency::SNM),
            933 => ::std::option::Option::Some(Currency::SNRG),
            934 => ::std::option::Option::Some(Currency::SNT),
            935 => ::std::option::Option::Some(Currency::SOAR),
            936 => ::std::option::Option::Some(Currency::SOCC),
            937 => ::std::option::Option::Some(Currency::SOIL),
            938 => ::std::option::Option::Some(Currency::SOJ),
            939 => ::std::option::Option::Some(Currency::SONG),
            940 => ::std::option::Option::Some(Currency::SOON),
            941 => ::std::option::Option::Some(Currency::SOUL),
            942 => ::std::option::Option::Some(Currency::SPACE),
            943 => ::std::option::Option::Some(Currency::SPEX),
            944 => ::std::option::Option::Some(Currency::SPHR),
            945 => ::std::option::Option::Some(Currency::SPORT),
            946 => ::std::option::Option::Some(Currency::SPR),
            947 => ::std::option::Option::Some(Currency::SPRTS),
            948 => ::std::option::Option::Some(Currency::SPT),
            949 => ::std::option::Option::Some(Currency::SRC),
            950 => ::std::option::Option::Some(Currency::STA),
            951 => ::std::option::Option::Some(Currency::START),
            952 => ::std::option::Option::Some(Currency::STCN),
            953 => ::std::option::Option::Some(Currency::STEEM),
            954 => ::std::option::Option::Some(Currency::STEPS),
            955 => ::std::option::Option::Some(Currency::STEX),
            956 => ::std::option::Option::Some(Currency::STORJ),
            957 => ::std::option::Option::Some(Currency::STRAT),
            958 => ::std::option::Option::Some(Currency::STRC),
            959 => ::std::option::Option::Some(Currency::STS),
            960 => ::std::option::Option::Some(Currency::STV),
            961 => ::std::option::Option::Some(Currency::STX),
            962 => ::std::option::Option::Some(Currency::SUB),
            963 => ::std::option::Option::Some(Currency::SUMO),
            964 => ::std::option::Option::Some(Currency::SUPER),
            965 => ::std::option::Option::Some(Currency::SUR),
            966 => ::std::option::Option::Some(Currency::SWIFT),
            967 => ::std::option::Option::Some(Currency::SWING),
            968 => ::std::option::Option::Some(Currency::SWP),
            969 => ::std::option::Option::Some(Currency::SWT),
            970 => ::std::option::Option::Some(Currency::SXC),
            971 => ::std::option::Option::Some(Currency::SYNC),
            972 => ::std::option::Option::Some(Currency::SYNX),
            973 => ::std::option::Option::Some(Currency::SYS),
            974 => ::std::option::Option::Some(Currency::TAAS),
            975 => ::std::option::Option::Some(Currency::TAG),
            976 => ::std::option::Option::Some(Currency::TAGR),
            977 => ::std::option::Option::Some(Currency::TAJ),
            978 => ::std::option::Option::Some(Currency::TALK),
            979 => ::std::option::Option::Some(Currency::TCC),
            980 => ::std::option::Option::Some(Currency::TCOIN),
            981 => ::std::option::Option::Some(Currency::TCR),
            982 => ::std::option::Option::Some(Currency::TEAM),
            983 => ::std::option::Option::Some(Currency::TEK),
            984 => ::std::option::Option::Some(Currency::TELL),
            985 => ::std::option::Option::Some(Currency::TER),
            986 => ::std::option::Option::Some(Currency::TERA),
            987 => ::std::option::Option::Some(Currency::TES),
            988 => ::std::option::Option::Some(Currency::TESLA),
            989 => ::std::option::Option::Some(Currency::TFL),
            990 => ::std::option::Option::Some(Currency::TGC),
            991 => ::std::option::Option::Some(Currency::TGT),
            992 => ::std::option::Option::Some(Currency::THC),
            993 => ::std::option::Option::Some(Currency::THS),
            994 => ::std::option::Option::Some(Currency::TIME),
            995 => ::std::option::Option::Some(Currency::TIPS),
            996 => ::std::option::Option::Some(Currency::TIT),
            997 => ::std::option::Option::Some(Currency::TKN),
            998 => ::std::option::Option::Some(Currency::TKR),
            999 => ::std::option::Option::Some(Currency::TKS),
            1000 => ::std::option::Option::Some(Currency::TLE),
            1001 => ::std::option::Option::Some(Currency::TNT),
            1002 => ::std::option::Option::Some(Currency::TOA),
            1003 => ::std::option::Option::Some(Currency::TODAY),
            1004 => ::std::option::Option::Some(Currency::TOKEN),
            1005 => ::std::option::Option::Some(Currency::TOP),
            1006 => ::std::option::Option::Some(Currency::TOPAZ),
            1007 => ::std::option::Option::Some(Currency::TOR),
            1008 => ::std::option::Option::Some(Currency::TRADE),
            1009 => ::std::option::Option::Some(Currency::TRC),
            1010 => ::std::option::Option::Some(Currency::TRCT),
            1011 => ::std::option::Option::Some(Currency::TRI),
            1012 => ::std::option::Option::Some(Currency::TRICK),
            1013 => ::std::option::Option::Some(Currency::TRIG),
            1014 => ::std::option::Option::Some(Currency::TRK),
            1015 => ::std::option::Option::Some(Currency::TROLL),
            1016 => ::std::option::Option::Some(Currency::TRST),
            1017 => ::std::option::Option::Some(Currency::TRUMP),
            1018 => ::std::option::Option::Some(Currency::TRUST),
            1019 => ::std::option::Option::Some(Currency::TRX),
            1020 => ::std::option::Option::Some(Currency::TSE),
            1021 => ::std::option::Option::Some(Currency::TSTR),
            1022 => ::std::option::Option::Some(Currency::TTC),
            1023 => ::std::option::Option::Some(Currency::TURBO),
            1024 => ::std::option::Option::Some(Currency::TX),
            1025 => ::std::option::Option::Some(Currency::TYC),
            1026 => ::std::option::Option::Some(Currency::TYCHO),
            1027 => ::std::option::Option::Some(Currency::TZC),
            1028 => ::std::option::Option::Some(Currency::UBQ),
            1029 => ::std::option::Option::Some(Currency::UET),
            1030 => ::std::option::Option::Some(Currency::UFO),
            1031 => ::std::option::Option::Some(Currency::UGT),
            1032 => ::std::option::Option::Some(Currency::UIS),
            1033 => ::std::option::Option::Some(Currency::ULA),
            1034 => ::std::option::Option::Some(Currency::UNB),
            1035 => ::std::option::Option::Some(Currency::UNC),
            1036 => ::std::option::Option::Some(Currency::UNI),
            1037 => ::std::option::Option::Some(Currency::UNIC),
            1038 => ::std::option::Option::Some(Currency::UNIFY),
            1039 => ::std::option::Option::Some(Currency::UNIT),
            1040 => ::std::option::Option::Some(Currency::UNITS),
            1041 => ::std::option::Option::Some(Currency::UNITY),
            1042 => ::std::option::Option::Some(Currency::UNO),
            1043 => ::std::option::Option::Some(Currency::UNRC),
            1044 => ::std::option::Option::Some(Currency::UNY),
            1045 => ::std::option::Option::Some(Currency::UR),
            1046 => ::std::option::Option::Some(Currency::URC),
            1047 => ::std::option::Option::Some(Currency::URO),
            1048 => ::std::option::Option::Some(Currency::USC),
            1049 => ::std::option::Option::Some(Currency::USDE),
            1050 => ::std::option::Option::Some(Currency::USDT),
            1051 => ::std::option::Option::Some(Currency::USNBT),
            1052 => ::std::option::Option::Some(Currency::UTA),
            1053 => ::std::option::Option::Some(Currency::UTC),
            1054 => ::std::option::Option::Some(Currency::V),
            1055 => ::std::option::Option::Some(Currency::VAL),
            1056 => ::std::option::Option::Some(Currency::VASH),
            1057 => ::std::option::Option::Some(Currency::VC),
            1058 => ::std::option::Option::Some(Currency::VEC2),
            1059 => ::std::option::Option::Some(Currency::VEN),
            1060 => ::std::option::Option::Some(Currency::VERI),
            1061 => ::std::option::Option::Some(Currency::VGC),
            1062 => ::std::option::Option::Some(Currency::VIA),
            1063 => ::std::option::Option::Some(Currency::VIB),
            1064 => ::std::option::Option::Some(Currency::VIBE),
            1065 => ::std::option::Option::Some(Currency::VIDZ),
            1066 => ::std::option::Option::Some(Currency::VIP),
            1067 => ::std::option::Option::Some(Currency::VISIO),
            1068 => ::std::option::Option::Some(Currency::VIVO),
            1069 => ::std::option::Option::Some(Currency::VLT),
            1070 => ::std::option::Option::Some(Currency::VLTC),
            1071 => ::std::option::Option::Some(Currency::VOISE),
            1072 => ::std::option::Option::Some(Currency::VOLT),
            1073 => ::std::option::Option::Some(Currency::VOX),
            1074 => ::std::option::Option::Some(Currency::VOYA),
            1075 => ::std::option::Option::Some(Currency::VPRC),
            1076 => ::std::option::Option::Some(Currency::VRC),
            1077 => ::std::option::Option::Some(Currency::VRM),
            1078 => ::std::option::Option::Some(Currency::VRS),
            1079 => ::std::option::Option::Some(Currency::VSL),
            1080 => ::std::option::Option::Some(Currency::VSX),
            1081 => ::std::option::Option::Some(Currency::VTA),
            1082 => ::std::option::Option::Some(Currency::VTC),
            1083 => ::std::option::Option::Some(Currency::VTR),
            1084 => ::std::option::Option::Some(Currency::VUC),
            1085 => ::std::option::Option::Some(Currency::VULC),
            1086 => ::std::option::Option::Some(Currency::WA),
            1087 => ::std::option::Option::Some(Currency::WARP),
            1088 => ::std::option::Option::Some(Currency::WAVES),
            1089 => ::std::option::Option::Some(Currency::WAY),
            1090 => ::std::option::Option::Some(Currency::WBB),
            1091 => ::std::option::Option::Some(Currency::WBC),
            1092 => ::std::option::Option::Some(Currency::WCT),
            1093 => ::std::option::Option::Some(Currency::WDC),
            1094 => ::std::option::Option::Some(Currency::WEC),
            1095 => ::std::option::Option::Some(Currency::WEX),
            1096 => ::std::option::Option::Some(Currency::WGO),
            1097 => ::std::option::Option::Some(Currency::WGR),
            1098 => ::std::option::Option::Some(Currency::WHL),
            1099 => ::std::option::Option::Some(Currency::WIC),
            1100 => ::std::option::Option::Some(Currency::WILD),
            1101 => ::std::option::Option::Some(Currency::WINGS),
            1102 => ::std::option::Option::Some(Currency::WINK),
            1103 => ::std::option::Option::Some(Currency::WMC),
            1104 => ::std::option::Option::Some(Currency::WOMEN),
            1105 => ::std::option::Option::Some(Currency::WORM),
            1106 => ::std::option::Option::Some(Currency::WOW),
            1107 => ::std::option::Option::Some(Currency::WSX),
            1108 => ::std::option::Option::Some(Currency::WTC),
            1109 => ::std::option::Option::Some(Currency::WTT),
            1110 => ::std::option::Option::Some(Currency::WYV),
            1111 => ::std::option::Option::Some(Currency::X2),
            1112 => ::std::option::Option::Some(Currency::XAS),
            1113 => ::std::option::Option::Some(Currency::XAU),
            1114 => ::std::option::Option::Some(Currency::XAUR),
            1115 => ::std::option::Option::Some(Currency::XBC),
            1116 => ::std::option::Option::Some(Currency::XBL),
            1117 => ::std::option::Option::Some(Currency::XBTC21),
            1118 => ::std::option::Option::Some(Currency::XBTS),
            1119 => ::std::option::Option::Some(Currency::XBY),
            1120 => ::std::option::Option::Some(Currency::XC),
            1121 => ::std::option::Option::Some(Currency::XCN),
            1122 => ::std::option::Option::Some(Currency::XCO),
            1123 => ::std::option::Option::Some(Currency::XCP),
            1124 => ::std::option::Option::Some(Currency::XCRE),
            1125 => ::std::option::Option::Some(Currency::XCS),
            1126 => ::std::option::Option::Some(Currency::XCT),
            1127 => ::std::option::Option::Some(Currency::XCXT),
            1128 => ::std::option::Option::Some(Currency::XDE2),
            1129 => ::std::option::Option::Some(Currency::XDN),
            1130 => ::std::option::Option::Some(Currency::XEL),
            1131 => ::std::option::Option::Some(Currency::XEM),
            1132 => ::std::option::Option::Some(Currency::XFT),
            1133 => ::std::option::Option::Some(Currency::XGOX),
            1134 => ::std::option::Option::Some(Currency::XGR),
            1135 => ::std::option::Option::Some(Currency::XHI),
            1136 => ::std::option::Option::Some(Currency::XIN),
            1137 => ::std::option::Option::Some(Currency::XIOS),
            1138 => ::std::option::Option::Some(Currency::XJO),
            1139 => ::std::option::Option::Some(Currency::XLC),
            1140 => ::std::option::Option::Some(Currency::XLM),
            1141 => ::std::option::Option::Some(Currency::XLR),
            1142 => ::std::option::Option::Some(Currency::XMCC),
            1143 => ::std::option::Option::Some(Currency::XMG),
            1144 => ::std::option::Option::Some(Currency::XMR),
            1145 => ::std::option::Option::Some(Currency::XMY),
            1146 => ::std::option::Option::Some(Currency::XNG),
            1147 => ::std::option::Option::Some(Currency::XNN),
            1148 => ::std::option::Option::Some(Currency::XOC),
            1149 => ::std::option::Option::Some(Currency::XOT),
            1150 => ::std::option::Option::Some(Currency::XP),
            1151 => ::std::option::Option::Some(Currency::XPA),
            1152 => ::std::option::Option::Some(Currency::XPD),
            1153 => ::std::option::Option::Some(Currency::XPM),
            1154 => ::std::option::Option::Some(Currency::XPTX),
            1155 => ::std::option::Option::Some(Currency::XPY),
            1156 => ::std::option::Option::Some(Currency::XQN),
            1157 => ::std::option::Option::Some(Currency::XRA),
            1158 => ::std::option::Option::Some(Currency::XRB),
            1159 => ::std::option::Option::Some(Currency::XRC),
            1160 => ::std::option::Option::Some(Currency::XRE),
            1161 => ::std::option::Option::Some(Currency::XRL),
            1162 => ::std::option::Option::Some(Currency::XRY),
            1163 => ::std::option::Option::Some(Currency::XSH),
            1164 => ::std::option::Option::Some(Currency::XSPEC),
            1165 => ::std::option::Option::Some(Currency::XST),
            1166 => ::std::option::Option::Some(Currency::XSTC),
            1167 => ::std::option::Option::Some(Currency::XTC),
            1168 => ::std::option::Option::Some(Currency::XTD),
            1169 => ::std::option::Option::Some(Currency::XTO),
            1170 => ::std::option::Option::Some(Currency::XTZ),
            1171 => ::std::option::Option::Some(Currency::XUC),
            1172 => ::std::option::Option::Some(Currency::XVC),
            1173 => ::std::option::Option::Some(Currency::XVE),
            1174 => ::std::option::Option::Some(Currency::XVG),
            1175 => ::std::option::Option::Some(Currency::XVP),
            1176 => ::std::option::Option::Some(Currency::XWC),
            1177 => ::std::option::Option::Some(Currency::XZC),
            1178 => ::std::option::Option::Some(Currency::YAC),
            1179 => ::std::option::Option::Some(Currency::YASH),
            1180 => ::std::option::Option::Some(Currency::YES),
            1181 => ::std::option::Option::Some(Currency::YOC),
            1182 => ::std::option::Option::Some(Currency::YOYOW),
            1183 => ::std::option::Option::Some(Currency::ZBC),
            1184 => ::std::option::Option::Some(Currency::ZCC),
            1185 => ::std::option::Option::Some(Currency::ZCL),
            1186 => ::std::option::Option::Some(Currency::ZEIT),
            1187 => ::std::option::Option::Some(Currency::ZEN),
            1188 => ::std::option::Option::Some(Currency::ZENGOLD),
            1189 => ::std::option::Option::Some(Currency::ZENI),
            1190 => ::std::option::Option::Some(Currency::ZER),
            1191 => ::std::option::Option::Some(Currency::ZET),
            1192 => ::std::option::Option::Some(Currency::ZMC),
            1193 => ::std::option::Option::Some(Currency::ZNE),
            1194 => ::std::option::Option::Some(Currency::ZNY),
            1195 => ::std::option::Option::Some(Currency::ZOI),
            1196 => ::std::option::Option::Some(Currency::ZRC),
            1197 => ::std::option::Option::Some(Currency::ZRX),
            1198 => ::std::option::Option::Some(Currency::ZSC),
            1199 => ::std::option::Option::Some(Currency::ZSE),
            1200 => ::std::option::Option::Some(Currency::ZUR),
            1201 => ::std::option::Option::Some(Currency::ZYD),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Currency] = &[
            Currency::NONE,
            Currency::EUR,
            Currency::USD,
            Currency::BTC,
            Currency::ETH,
            Currency::ETC,
            Currency::XRP,
            Currency::ZEC,
            Currency::BCH,
            Currency::LTC,
            Currency::ABC,
            Currency::ABN,
            Currency::ABY,
            Currency::AC,
            Currency::ACC,
            Currency::ACES,
            Currency::ACN,
            Currency::ACOIN,
            Currency::ACP,
            Currency::ACT,
            Currency::ADA,
            Currency::ADC,
            Currency::ADCN,
            Currency::ADK,
            Currency::ADL,
            Currency::ADST,
            Currency::ADT,
            Currency::ADX,
            Currency::ADZ,
            Currency::AE,
            Currency::AEON,
            Currency::AGLC,
            Currency::AGRS,
            Currency::AHT,
            Currency::AIB,
            Currency::AION,
            Currency::AIR,
            Currency::AKY,
            Currency::ALIS,
            Currency::ALL,
            Currency::ALT,
            Currency::ALTC,
            Currency::ALTCOM,
            Currency::AMB,
            Currency::AMBER,
            Currency::AMIS,
            Currency::AMMO,
            Currency::AMP,
            Currency::AMS,
            Currency::ANC,
            Currency::ANI,
            Currency::ANT,
            Currency::ANTI,
            Currency::ANTX,
            Currency::APC,
            Currency::APW,
            Currency::APX,
            Currency::ARB,
            Currency::ARCO,
            Currency::ARDR,
            Currency::ARG,
            Currency::ARGUS,
            Currency::ARI,
            Currency::ARK,
            Currency::ART,
            Currency::ASAFE2,
            Currency::ASC,
            Currency::ASN,
            Currency::AST,
            Currency::ATB,
            Currency::ATCC,
            Currency::ATL,
            Currency::ATM,
            Currency::ATMC,
            Currency::ATMS,
            Currency::ATOM,
            Currency::ATS,
            Currency::ATX,
            Currency::AU,
            Currency::AUR,
            Currency::AURS,
            Currency::AV,
            Currency::AVT,
            Currency::AXF,
            Currency::AXIOM,
            Currency::B2X,
            Currency::B3,
            Currency::BAC,
            Currency::BAS,
            Currency::BASH,
            Currency::BAY,
            Currency::BBC,
            Currency::BBP,
            Currency::BCAP,
            Currency::BCC,
            Currency::BCCS,
            Currency::BCF,
            Currency::BCN,
            Currency::BCO,
            Currency::BCPT,
            Currency::BCY,
            Currency::BDL,
            Currency::BELA,
            Currency::BENJI,
            Currency::BERN,
            Currency::BEST,
            Currency::BGR,
            Currency::BIGUP,
            Currency::BIOB,
            Currency::BIOS,
            Currency::BIP,
            Currency::BIRDS,
            Currency::BIS,
            Currency::BIT,
            Currency::BITB,
            Currency::BITBTC,
            Currency::BITCF,
            Currency::BITCNY,
            Currency::BITEUR,
            Currency::BITGOLD,
            Currency::BITOK,
            Currency::BITS,
            Currency::BITSILVER,
            Currency::BITUSD,
            Currency::BITZ,
            Currency::BLAS,
            Currency::BLAZR,
            Currency::BLC,
            Currency::BLITZ,
            Currency::BLK,
            Currency::BLN,
            Currency::BLOCK,
            Currency::BLOCKPAY,
            Currency::BLRY,
            Currency::BLU,
            Currency::BLUE,
            Currency::BLX,
            Currency::BLZ,
            Currency::BMC,
            Currency::BNB,
            Currency::BNT,
            Currency::BNX,
            Currency::BOAT,
            Currency::BOLI,
            Currency::BOS,
            Currency::BOST,
            Currency::BPC,
            Currency::BQ,
            Currency::BQC,
            Currency::BQX,
            Currency::BRAIN,
            Currency::BRAT,
            Currency::BRIA,
            Currency::BRIT,
            Currency::BRK,
            Currency::BRO,
            Currency::BRX,
            Currency::BSC,
            Currency::BSD,
            Currency::BSN,
            Currency::BSR,
            Currency::BSTAR,
            Currency::BSTY,
            Currency::BT1,
            Currency::BT2,
            Currency::BTA,
            Currency::BTB,
            Currency::BTBc,
            Currency::BTCD,
            Currency::BTCM,
            Currency::BTCR,
            Currency::BTCRED,
            Currency::BTCZ,
            Currency::BTDX,
            Currency::BTPL,
            Currency::BTQ,
            Currency::BTS,
            Currency::BTSR,
            Currency::BTU,
            Currency::BTWTY,
            Currency::BTX,
            Currency::BUB,
            Currency::BUCKS,
            Currency::BUMBA,
            Currency::BUN,
            Currency::BURST,
            Currency::BUZZ,
            Currency::BVC,
            Currency::BXC,
            Currency::BXT,
            Currency::BYC,
            Currency::C2,
            Currency::CAB,
            Currency::CACH,
            Currency::CADASTRAL,
            Currency::CAG,
            Currency::CAGE,
            Currency::CALC,
            Currency::CANN,
            Currency::CAP,
            Currency::CARBON,
            Currency::CASINO,
            Currency::CBD,
            Currency::CBX,
            Currency::CC,
            Currency::CCM100,
            Currency::CCN,
            Currency::CCRB,
            Currency::CCT,
            Currency::CDN,
            Currency::CDT,
            Currency::CESC,
            Currency::CF,
            Currency::CFI,
            Currency::CFT,
            Currency::CHC,
            Currency::CHEAP,
            Currency::CHESS,
            Currency::CHIPS,
            Currency::CJ,
            Currency::CLAM,
            Currency::CLINT,
            Currency::CLOAK,
            Currency::CLUB,
            Currency::CME,
            Currency::CMP,
            Currency::CMPCO,
            Currency::CMT,
            Currency::CNC,
            Currency::CND,
            Currency::CNNC,
            Currency::CNO,
            Currency::CNT,
            Currency::CNX,
            Currency::COAL,
            Currency::COB,
            Currency::COLX,
            Currency::CON,
            Currency::CONX,
            Currency::COR,
            Currency::CORG,
            Currency::COSS,
            Currency::COUPE,
            Currency::COVAL,
            Currency::COXST,
            Currency::CPC,
            Currency::CPN,
            Currency::CRAVE,
            Currency::CRB,
            Currency::CREA,
            Currency::CREDO,
            Currency::CREVA,
            Currency::CRM,
            Currency::CRT,
            Currency::CRW,
            Currency::CRX,
            Currency::CRYPT,
            Currency::CSC,
            Currency::CSNO,
            Currency::CTIC2,
            Currency::CTIC3,
            Currency::CTO,
            Currency::CTR,
            Currency::CUBE,
            Currency::CURE,
            Currency::CV2,
            Currency::CVC,
            Currency::CVCOIN,
            Currency::CWXT,
            Currency::CXT,
            Currency::CYC,
            Currency::CYDER,
            Currency::CYP,
            Currency::DALC,
            Currency::DAR,
            Currency::DAS,
            Currency::DASH,
            Currency::DASHS,
            Currency::DATA,
            Currency::DAXX,
            Currency::DAY,
            Currency::DBG,
            Currency::DBIX,
            Currency::DBTC,
            Currency::DCN,
            Currency::DCR,
            Currency::DCRE,
            Currency::DCT,
            Currency::DCY,
            Currency::DDF,
            Currency::DEM,
            Currency::DENT,
            Currency::DES,
            Currency::DEUS,
            Currency::DFS,
            Currency::DFT,
            Currency::DGB,
            Currency::DGC,
            Currency::DGCS,
            Currency::DGD,
            Currency::DIBC,
            Currency::DICE,
            Currency::DIME,
            Currency::DISK,
            Currency::DIX,
            Currency::DLC,
            Currency::DLISK,
            Currency::DLT,
            Currency::DMB,
            Currency::DMC,
            Currency::DMD,
            Currency::DNR,
            Currency::DNT,
            Currency::DOGE,
            Currency::DOLLAR,
            Currency::DON,
            Currency::DOPE,
            Currency::DOT,
            Currency::DOVU,
            Currency::DP,
            Currency::DPAY,
            Currency::DRACO,
            Currency::DRM,
            Currency::DRS,
            Currency::DRT,
            Currency::DRXNE,
            Currency::DSH,
            Currency::DTB,
            Currency::DUB,
            Currency::DUO,
            Currency::DUTCH,
            Currency::DVC,
            Currency::DYN,
            Currency::E4ROW,
            Currency::EAC,
            Currency::EBET,
            Currency::EBIT,
            Currency::EBST,
            Currency::EBT,
            Currency::ECA,
            Currency::ECASH,
            Currency::ECC,
            Currency::ECN,
            Currency::ECO,
            Currency::ECOB,
            Currency::EDG,
            Currency::EDO,
            Currency::EDOGE,
            Currency::EDR,
            Currency::EDRC,
            Currency::EFL,
            Currency::EFYT,
            Currency::EGAS,
            Currency::EGC,
            Currency::EGG,
            Currency::EGO,
            Currency::EGOLD,
            Currency::EL,
            Currency::ELC,
            Currency::ELE,
            Currency::ELITE,
            Currency::ELIX,
            Currency::ELLA,
            Currency::ELS,
            Currency::ELTC2,
            Currency::EMB,
            Currency::EMC,
            Currency::EMC2,
            Currency::EMD,
            Currency::EMP,
            Currency::EMV,
            Currency::ENG,
            Currency::ENJ,
            Currency::ENRG,
            Currency::ENT,
            Currency::ENV,
            Currency::EOS,
            Currency::EOT,
            Currency::EQT,
            Currency::ERC,
            Currency::EREAL,
            Currency::ERY,
            Currency::ESP,
            Currency::ETBS,
            Currency::ETG,
            Currency::ETHD,
            Currency::ETN,
            Currency::ETP,
            Currency::ETX,
            Currency::EUC,
            Currency::EUSD,
            Currency::EVIL,
            Currency::EVO,
            Currency::EVR,
            Currency::EVX,
            Currency::EXCL,
            Currency::EXL,
            Currency::EXN,
            Currency::EXP,
            Currency::EXRN,
            Currency::FAIR,
            Currency::FAL,
            Currency::FAP,
            Currency::FAZZ,
            Currency::FC,
            Currency::FC2,
            Currency::FCN,
            Currency::FCT,
            Currency::FDC,
            Currency::FFC,
            Currency::FID,
            Currency::FIMK,
            Currency::FIRE,
            Currency::FJC,
            Currency::FLAP,
            Currency::FLASH,
            Currency::FLAX,
            Currency::FLDC,
            Currency::FLIK,
            Currency::FLO,
            Currency::FLT,
            Currency::FLVR,
            Currency::FLY,
            Currency::FNC,
            Currency::FONZ,
            Currency::FOR,
            Currency::FRAZ,
            Currency::FRC,
            Currency::FRGC,
            Currency::FRK,
            Currency::FRN,
            Currency::FRST,
            Currency::FRWC,
            Currency::FST,
            Currency::FTC,
            Currency::FUCK,
            Currency::FUEL,
            Currency::FUN,
            Currency::FUNC,
            Currency::FUNK,
            Currency::FUTC,
            Currency::FUZZ,
            Currency::FXE,
            Currency::FYN,
            Currency::FYP,
            Currency::G3N,
            Currency::GAIA,
            Currency::GAIN,
            Currency::GAM,
            Currency::GAME,
            Currency::GAP,
            Currency::GARY,
            Currency::GAS,
            Currency::GAY,
            Currency::GB,
            Currency::GBC,
            Currency::GBG,
            Currency::GBRC,
            Currency::GBT,
            Currency::GBYTE,
            Currency::GCN,
            Currency::GCR,
            Currency::GEERT,
            Currency::GEO,
            Currency::GIM,
            Currency::GLC,
            Currency::GLD,
            Currency::GLT,
            Currency::GML,
            Currency::GMT,
            Currency::GMX,
            Currency::GNO,
            Currency::GNT,
            Currency::GOLF,
            Currency::GOLOS,
            Currency::GOOD,
            Currency::GP,
            Currency::GPL,
            Currency::GPU,
            Currency::GRC,
            Currency::GRE,
            Currency::GRID,
            Currency::GRN,
            Currency::GRS,
            Currency::GRT,
            Currency::GRWI,
            Currency::GSR,
            Currency::GTC,
            Currency::GUC,
            Currency::GUN,
            Currency::GUP,
            Currency::GXS,
            Currency::HAL,
            Currency::HALLO,
            Currency::HBN,
            Currency::HBT,
            Currency::HCC,
            Currency::HDG,
            Currency::HDLB,
            Currency::HEAT,
            Currency::HERO,
            Currency::HGT,
            Currency::HIGH,
            Currency::HKG,
            Currency::HMC,
            Currency::HMP,
            Currency::HMQ,
            Currency::HODL,
            Currency::HONEY,
            Currency::HPC,
            Currency::HSR,
            Currency::HTC,
            Currency::HTML5,
            Currency::HUC,
            Currency::HUSH,
            Currency::HVCO,
            Currency::HVN,
            Currency::HXX,
            Currency::HYP,
            Currency::HYPER,
            Currency::I0C,
            Currency::IBANK,
            Currency::IBTC,
            Currency::ICE,
            Currency::ICOB,
            Currency::ICON,
            Currency::ICOO,
            Currency::ICOS,
            Currency::ICX,
            Currency::IETH,
            Currency::IFC,
            Currency::IFLT,
            Currency::IFT,
            Currency::IMPS,
            Currency::IMS,
            Currency::IMX,
            Currency::INCNT,
            Currency::IND,
            Currency::INDIA,
            Currency::INF,
            Currency::INFX,
            Currency::INPAY,
            Currency::INSN,
            Currency::INXT,
            Currency::IOC,
            Currency::ION,
            Currency::IOP,
            Currency::IQT,
            Currency::IRL,
            Currency::ISL,
            Currency::ITI,
            Currency::ITT,
            Currency::ITZ,
            Currency::IVZ,
            Currency::IXC,
            Currency::IXT,
            Currency::J,
            Currency::JET,
            Currency::JIN,
            Currency::JINN,
            Currency::JIO,
            Currency::JNS,
            Currency::JOBS,
            Currency::JS,
            Currency::JWL,
            Currency::KARMA,
            Currency::KASHH,
            Currency::KAYI,
            Currency::KCS,
            Currency::KED,
            Currency::KEK,
            Currency::KEXCOIN,
            Currency::KIC,
            Currency::KICK,
            Currency::KIN,
            Currency::KLC,
            Currency::KLN,
            Currency::KMD,
            Currency::KOBO,
            Currency::KORE,
            Currency::KRB,
            Currency::KRONE,
            Currency::KURT,
            Currency::KUSH,
            Currency::LA,
            Currency::LANA,
            Currency::LAZ,
            Currency::LBC,
            Currency::LBTC,
            Currency::LCP,
            Currency::LDCN,
            Currency::LDOGE,
            Currency::LEA,
            Currency::LEO,
            Currency::LEPEN,
            Currency::LEX,
            Currency::LGD,
            Currency::LIFE,
            Currency::LINDA,
            Currency::LINK,
            Currency::LINX,
            Currency::LIR,
            Currency::LKC,
            Currency::LKK,
            Currency::LLT,
            Currency::LMC,
            Currency::LNK,
            Currency::LOG,
            Currency::LOT,
            Currency::LRC,
            Currency::LSK,
            Currency::LTB,
            Currency::LTBC,
            Currency::LTCR,
            Currency::LTCU,
            Currency::LTG,
            Currency::LTH,
            Currency::LUN,
            Currency::LUNA,
            Currency::LUX,
            Currency::LVPS,
            Currency::MAC,
            Currency::MAD,
            Currency::MAGN,
            Currency::MAID,
            Currency::MANA,
            Currency::MAO,
            Currency::MAR,
            Currency::MARS,
            Currency::MARX,
            Currency::MAVRO,
            Currency::MAX,
            Currency::MAY,
            Currency::MBI,
            Currency::MBL,
            Currency::MBRS,
            Currency::MCAP,
            Currency::MCI,
            Currency::MCO,
            Currency::MCR,
            Currency::MCRN,
            Currency::MDA,
            Currency::MEC,
            Currency::MEME,
            Currency::MEN,
            Currency::MEOW,
            Currency::MER,
            Currency::METAL,
            Currency::MG,
            Currency::MGM,
            Currency::MGO,
            Currency::MILO,
            Currency::MINEX,
            Currency::MINT,
            Currency::MIOTA,
            Currency::MIU,
            Currency::MLN,
            Currency::MMXVI,
            Currency::MND,
            Currency::MNE,
            Currency::MNM,
            Currency::MNX,
            Currency::MOD,
            Currency::MOIN,
            Currency::MOJO,
            Currency::MONA,
            Currency::MONETA,
            Currency::MONEY,
            Currency::MOON,
            Currency::MOTO,
            Currency::MRC,
            Currency::MRJA,
            Currency::MRNG,
            Currency::MRT,
            Currency::MSCN,
            Currency::MSD,
            Currency::MSP,
            Currency::MST,
            Currency::MTH,
            Currency::MTL,
            Currency::MTLMC3,
            Currency::MTM,
            Currency::MTNC,
            Currency::MUE,
            Currency::MUG,
            Currency::MUSIC,
            Currency::MXT,
            Currency::MYB,
            Currency::MYST,
            Currency::MZC,
            Currency::NAMO,
            Currency::NANOX,
            Currency::NAS,
            Currency::NAUT,
            Currency::NAV,
            Currency::NBE,
            Currency::NBIT,
            Currency::NDAO,
            Currency::NDC,
            Currency::NEBL,
            Currency::NEO,
            Currency::NEOS,
            Currency::NETKO,
            Currency::NEVA,
            Currency::NEWB,
            Currency::NKA,
            Currency::NLC2,
            Currency::NLG,
            Currency::NMC,
            Currency::NMR,
            Currency::NOBL,
            Currency::NODC,
            Currency::NOTE,
            Currency::NRO,
            Currency::NSR,
            Currency::NTC,
            Currency::NTCC,
            Currency::NTO,
            Currency::NTRN,
            Currency::NTWK,
            Currency::NULS,
            Currency::NVC,
            Currency::NVST,
            Currency::NXC,
            Currency::NXS,
            Currency::NXT,
            Currency::NXX,
            Currency::NYAN,
            Currency::NYC,
            Currency::OAX,
            Currency::OBITS,
            Currency::OCEAN,
            Currency::OCL,
            Currency::OCOW,
            Currency::OCT,
            Currency::ODN,
            Currency::OFF,
            Currency::OHM,
            Currency::OK,
            Currency::OMC,
            Currency::OMG,
            Currency::OMNI,
            Currency::ONION,
            Currency::ONX,
            Currency::OP,
            Currency::OPAL,
            Currency::OPES,
            Currency::OPT,
            Currency::ORB,
            Currency::ORLY,
            Currency::ORME,
            Currency::OS76,
            Currency::OTN,
            Currency::OX,
            Currency::P7C,
            Currency::PAC,
            Currency::PAK,
            Currency::PART,
            Currency::PASC,
            Currency::PASL,
            Currency::PAY,
            Currency::PAYP,
            Currency::PBT,
            Currency::PCN,
            Currency::PCS,
            Currency::PDC,
            Currency::PDG,
            Currency::PEC,
            Currency::PEPECASH,
            Currency::PEX,
            Currency::PGL,
            Currency::PHO,
            Currency::PHS,
            Currency::PI,
            Currency::PIE,
            Currency::PIGGY,
            Currency::PING,
            Currency::PINK,
            Currency::PIPL,
            Currency::PIRL,
            Currency::PIVX,
            Currency::PIX,
            Currency::PIZZA,
            Currency::PKB,
            Currency::PLACO,
            Currency::PLBT,
            Currency::PLNC,
            Currency::PLR,
            Currency::PLU,
            Currency::PND,
            Currency::POE,
            Currency::POKE,
            Currency::POLL,
            Currency::PONZI,
            Currency::POP,
            Currency::POS,
            Currency::POST,
            Currency::POSW,
            Currency::POT,
            Currency::POWR,
            Currency::PPC,
            Currency::PPP,
            Currency::PPT,
            Currency::PPY,
            Currency::PR,
            Currency::PRC,
            Currency::PRES,
            Currency::PRG,
            Currency::PRIMU,
            Currency::PRM,
            Currency::PRN,
            Currency::PRO,
            Currency::PROC,
            Currency::PRX,
            Currency::PSB,
            Currency::PST,
            Currency::PSY,
            Currency::PTC,
            Currency::PTOY,
            Currency::PULSE,
            Currency::PURA,
            Currency::PUT,
            Currency::PWR,
            Currency::PX,
            Currency::PXC,
            Currency::PXI,
            Currency::PZM,
            Currency::Q2C,
            Currency::QAU,
            Currency::QBC,
            Currency::QBK,
            Currency::QBT,
            Currency::QCN,
            Currency::QORA,
            Currency::QRK,
            Currency::QRL,
            Currency::QTL,
            Currency::QTUM,
            Currency::QVT,
            Currency::QWARK,
            Currency::R,
            Currency::RADS,
            Currency::RAIN,
            Currency::RBBT,
            Currency::RBIES,
            Currency::RBT,
            Currency::RBX,
            Currency::RBY,
            Currency::RC,
            Currency::RDD,
            Currency::REAL,
            Currency::REC,
            Currency::RED,
            Currency::REE,
            Currency::REGA,
            Currency::REP,
            Currency::REQ,
            Currency::REV,
            Currency::REX,
            Currency::RHFC,
            Currency::RHOC,
            Currency::RIC,
            Currency::RICHX,
            Currency::RIDE,
            Currency::RISE,
            Currency::RIYA,
            Currency::RKC,
            Currency::RLC,
            Currency::RLT,
            Currency::RNS,
            Currency::ROOFS,
            Currency::ROYAL,
            Currency::RPC,
            Currency::RPX,
            Currency::RSGP,
            Currency::RUBIT,
            Currency::RUNNERS,
            Currency::RUP,
            Currency::RUPX,
            Currency::RUSTBITS,
            Currency::RVT,
            Currency::SAC,
            Currency::SAFEX,
            Currency::SAK,
            Currency::SALT,
            Currency::SAN,
            Currency::SANDG,
            Currency::SBD,
            Currency::SC,
            Currency::SCL,
            Currency::SCORE,
            Currency::SCRT,
            Currency::SCS,
            Currency::SDC,
            Currency::SDP,
            Currency::SDRN,
            Currency::SEQ,
            Currency::SFC,
            Currency::SFE,
            Currency::SH,
            Currency::SHA,
            Currency::SHDW,
            Currency::SHELL,
            Currency::SHIFT,
            Currency::SHND,
            Currency::SHORTY,
            Currency::SIB,
            Currency::SIC,
            Currency::SIFT,
            Currency::SIGMA,
            Currency::SIGT,
            Currency::SJCX,
            Currency::SKC,
            Currency::SKIN,
            Currency::SKR,
            Currency::SKULL,
            Currency::SKY,
            Currency::SLEVIN,
            Currency::SLFI,
            Currency::SLG,
            Currency::SLING,
            Currency::SLM,
            Currency::SLR,
            Currency::SLS,
            Currency::SMART,
            Currency::SMC,
            Currency::SMLY,
            Currency::SMOKE,
            Currency::SNAKE,
            Currency::SNC,
            Currency::SND,
            Currency::SNGLS,
            Currency::SNM,
            Currency::SNRG,
            Currency::SNT,
            Currency::SOAR,
            Currency::SOCC,
            Currency::SOIL,
            Currency::SOJ,
            Currency::SONG,
            Currency::SOON,
            Currency::SOUL,
            Currency::SPACE,
            Currency::SPEX,
            Currency::SPHR,
            Currency::SPORT,
            Currency::SPR,
            Currency::SPRTS,
            Currency::SPT,
            Currency::SRC,
            Currency::STA,
            Currency::START,
            Currency::STCN,
            Currency::STEEM,
            Currency::STEPS,
            Currency::STEX,
            Currency::STORJ,
            Currency::STRAT,
            Currency::STRC,
            Currency::STS,
            Currency::STV,
            Currency::STX,
            Currency::SUB,
            Currency::SUMO,
            Currency::SUPER,
            Currency::SUR,
            Currency::SWIFT,
            Currency::SWING,
            Currency::SWP,
            Currency::SWT,
            Currency::SXC,
            Currency::SYNC,
            Currency::SYNX,
            Currency::SYS,
            Currency::TAAS,
            Currency::TAG,
            Currency::TAGR,
            Currency::TAJ,
            Currency::TALK,
            Currency::TCC,
            Currency::TCOIN,
            Currency::TCR,
            Currency::TEAM,
            Currency::TEK,
            Currency::TELL,
            Currency::TER,
            Currency::TERA,
            Currency::TES,
            Currency::TESLA,
            Currency::TFL,
            Currency::TGC,
            Currency::TGT,
            Currency::THC,
            Currency::THS,
            Currency::TIME,
            Currency::TIPS,
            Currency::TIT,
            Currency::TKN,
            Currency::TKR,
            Currency::TKS,
            Currency::TLE,
            Currency::TNT,
            Currency::TOA,
            Currency::TODAY,
            Currency::TOKEN,
            Currency::TOP,
            Currency::TOPAZ,
            Currency::TOR,
            Currency::TRADE,
            Currency::TRC,
            Currency::TRCT,
            Currency::TRI,
            Currency::TRICK,
            Currency::TRIG,
            Currency::TRK,
            Currency::TROLL,
            Currency::TRST,
            Currency::TRUMP,
            Currency::TRUST,
            Currency::TRX,
            Currency::TSE,
            Currency::TSTR,
            Currency::TTC,
            Currency::TURBO,
            Currency::TX,
            Currency::TYC,
            Currency::TYCHO,
            Currency::TZC,
            Currency::UBQ,
            Currency::UET,
            Currency::UFO,
            Currency::UGT,
            Currency::UIS,
            Currency::ULA,
            Currency::UNB,
            Currency::UNC,
            Currency::UNI,
            Currency::UNIC,
            Currency::UNIFY,
            Currency::UNIT,
            Currency::UNITS,
            Currency::UNITY,
            Currency::UNO,
            Currency::UNRC,
            Currency::UNY,
            Currency::UR,
            Currency::URC,
            Currency::URO,
            Currency::USC,
            Currency::USDE,
            Currency::USDT,
            Currency::USNBT,
            Currency::UTA,
            Currency::UTC,
            Currency::V,
            Currency::VAL,
            Currency::VASH,
            Currency::VC,
            Currency::VEC2,
            Currency::VEN,
            Currency::VERI,
            Currency::VGC,
            Currency::VIA,
            Currency::VIB,
            Currency::VIBE,
            Currency::VIDZ,
            Currency::VIP,
            Currency::VISIO,
            Currency::VIVO,
            Currency::VLT,
            Currency::VLTC,
            Currency::VOISE,
            Currency::VOLT,
            Currency::VOX,
            Currency::VOYA,
            Currency::VPRC,
            Currency::VRC,
            Currency::VRM,
            Currency::VRS,
            Currency::VSL,
            Currency::VSX,
            Currency::VTA,
            Currency::VTC,
            Currency::VTR,
            Currency::VUC,
            Currency::VULC,
            Currency::WA,
            Currency::WARP,
            Currency::WAVES,
            Currency::WAY,
            Currency::WBB,
            Currency::WBC,
            Currency::WCT,
            Currency::WDC,
            Currency::WEC,
            Currency::WEX,
            Currency::WGO,
            Currency::WGR,
            Currency::WHL,
            Currency::WIC,
            Currency::WILD,
            Currency::WINGS,
            Currency::WINK,
            Currency::WMC,
            Currency::WOMEN,
            Currency::WORM,
            Currency::WOW,
            Currency::WSX,
            Currency::WTC,
            Currency::WTT,
            Currency::WYV,
            Currency::X2,
            Currency::XAS,
            Currency::XAU,
            Currency::XAUR,
            Currency::XBC,
            Currency::XBL,
            Currency::XBTC21,
            Currency::XBTS,
            Currency::XBY,
            Currency::XC,
            Currency::XCN,
            Currency::XCO,
            Currency::XCP,
            Currency::XCRE,
            Currency::XCS,
            Currency::XCT,
            Currency::XCXT,
            Currency::XDE2,
            Currency::XDN,
            Currency::XEL,
            Currency::XEM,
            Currency::XFT,
            Currency::XGOX,
            Currency::XGR,
            Currency::XHI,
            Currency::XIN,
            Currency::XIOS,
            Currency::XJO,
            Currency::XLC,
            Currency::XLM,
            Currency::XLR,
            Currency::XMCC,
            Currency::XMG,
            Currency::XMR,
            Currency::XMY,
            Currency::XNG,
            Currency::XNN,
            Currency::XOC,
            Currency::XOT,
            Currency::XP,
            Currency::XPA,
            Currency::XPD,
            Currency::XPM,
            Currency::XPTX,
            Currency::XPY,
            Currency::XQN,
            Currency::XRA,
            Currency::XRB,
            Currency::XRC,
            Currency::XRE,
            Currency::XRL,
            Currency::XRY,
            Currency::XSH,
            Currency::XSPEC,
            Currency::XST,
            Currency::XSTC,
            Currency::XTC,
            Currency::XTD,
            Currency::XTO,
            Currency::XTZ,
            Currency::XUC,
            Currency::XVC,
            Currency::XVE,
            Currency::XVG,
            Currency::XVP,
            Currency::XWC,
            Currency::XZC,
            Currency::YAC,
            Currency::YASH,
            Currency::YES,
            Currency::YOC,
            Currency::YOYOW,
            Currency::ZBC,
            Currency::ZCC,
            Currency::ZCL,
            Currency::ZEIT,
            Currency::ZEN,
            Currency::ZENGOLD,
            Currency::ZENI,
            Currency::ZER,
            Currency::ZET,
            Currency::ZMC,
            Currency::ZNE,
            Currency::ZNY,
            Currency::ZOI,
            Currency::ZRC,
            Currency::ZRX,
            Currency::ZSC,
            Currency::ZSE,
            Currency::ZUR,
            Currency::ZYD,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Currency>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Currency", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Currency {
}

impl ::std::default::Default for Currency {
    fn default() -> Self {
        Currency::NONE
    }
}

impl ::protobuf::reflect::ProtobufValue for Currency {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18orca/core/currency.proto\x12\torca.core\"b\n\x0cCurrencyPair\x12)\
    \n\x05quote\x18\x01\x20\x01(\x0e2\x13.orca.core.CurrencyR\x05quote\x12'\
    \n\x04base\x18\x02\x20\x01(\x0e2\x13.orca.core.CurrencyR\x04base\"Y\n\
    \x0eCurrencyVolume\x12/\n\x08currency\x18\x01\x20\x01(\x0e2\x13.orca.cor\
    e.CurrencyR\x08currency\x12\x16\n\x06amount\x18\x02\x20\x01(\x04R\x06amo\
    unt*\x84b\n\x08Currency\x12\x08\n\x04NONE\x10\0\x12\x07\n\x03EUR\x10\x01\
    \x12\x07\n\x03USD\x10\x02\x12\x07\n\x03BTC\x10\x03\x12\x07\n\x03ETH\x10\
    \x04\x12\x07\n\x03ETC\x10\x05\x12\x07\n\x03XRP\x10\x06\x12\x07\n\x03ZEC\
    \x10\x07\x12\x07\n\x03BCH\x10\x08\x12\x07\n\x03LTC\x10\t\x12\x07\n\x03AB\
    C\x10\n\x12\x07\n\x03ABN\x10\x0b\x12\x07\n\x03ABY\x10\x0c\x12\x06\n\x02A\
    C\x10\r\x12\x07\n\x03ACC\x10\x0e\x12\x08\n\x04ACES\x10\x0f\x12\x07\n\x03\
    ACN\x10\x10\x12\t\n\x05ACOIN\x10\x11\x12\x07\n\x03ACP\x10\x12\x12\x07\n\
    \x03ACT\x10\x13\x12\x07\n\x03ADA\x10\x14\x12\x07\n\x03ADC\x10\x15\x12\
    \x08\n\x04ADCN\x10\x16\x12\x07\n\x03ADK\x10\x17\x12\x07\n\x03ADL\x10\x18\
    \x12\x08\n\x04ADST\x10\x19\x12\x07\n\x03ADT\x10\x1a\x12\x07\n\x03ADX\x10\
    \x1b\x12\x07\n\x03ADZ\x10\x1c\x12\x06\n\x02AE\x10\x1d\x12\x08\n\x04AEON\
    \x10\x1e\x12\x08\n\x04AGLC\x10\x1f\x12\x08\n\x04AGRS\x10\x20\x12\x07\n\
    \x03AHT\x10!\x12\x07\n\x03AIB\x10\"\x12\x08\n\x04AION\x10#\x12\x07\n\x03\
    AIR\x10$\x12\x07\n\x03AKY\x10%\x12\x08\n\x04ALIS\x10&\x12\x07\n\x03ALL\
    \x10'\x12\x07\n\x03ALT\x10(\x12\x08\n\x04ALTC\x10)\x12\n\n\x06ALTCOM\x10\
    *\x12\x07\n\x03AMB\x10+\x12\t\n\x05AMBER\x10,\x12\x08\n\x04AMIS\x10-\x12\
    \x08\n\x04AMMO\x10.\x12\x07\n\x03AMP\x10/\x12\x07\n\x03AMS\x100\x12\x07\
    \n\x03ANC\x101\x12\x07\n\x03ANI\x102\x12\x07\n\x03ANT\x103\x12\x08\n\x04\
    ANTI\x104\x12\x08\n\x04ANTX\x105\x12\x07\n\x03APC\x106\x12\x07\n\x03APW\
    \x107\x12\x07\n\x03APX\x108\x12\x07\n\x03ARB\x109\x12\x08\n\x04ARCO\x10:\
    \x12\x08\n\x04ARDR\x10;\x12\x07\n\x03ARG\x10<\x12\t\n\x05ARGUS\x10=\x12\
    \x07\n\x03ARI\x10>\x12\x07\n\x03ARK\x10?\x12\x07\n\x03ART\x10@\x12\n\n\
    \x06ASAFE2\x10A\x12\x07\n\x03ASC\x10B\x12\x07\n\x03ASN\x10C\x12\x07\n\
    \x03AST\x10D\x12\x07\n\x03ATB\x10E\x12\x08\n\x04ATCC\x10F\x12\x07\n\x03A\
    TL\x10G\x12\x07\n\x03ATM\x10H\x12\x08\n\x04ATMC\x10I\x12\x08\n\x04ATMS\
    \x10J\x12\x08\n\x04ATOM\x10K\x12\x07\n\x03ATS\x10L\x12\x07\n\x03ATX\x10M\
    \x12\x06\n\x02AU\x10N\x12\x07\n\x03AUR\x10O\x12\x08\n\x04AURS\x10P\x12\
    \x06\n\x02AV\x10Q\x12\x07\n\x03AVT\x10R\x12\x07\n\x03AXF\x10S\x12\t\n\
    \x05AXIOM\x10T\x12\x07\n\x03B2X\x10U\x12\x06\n\x02B3\x10V\x12\x07\n\x03B\
    AC\x10W\x12\x07\n\x03BAS\x10X\x12\x08\n\x04BASH\x10Y\x12\x07\n\x03BAY\
    \x10Z\x12\x07\n\x03BBC\x10[\x12\x07\n\x03BBP\x10\\\x12\x08\n\x04BCAP\x10\
    ]\x12\x07\n\x03BCC\x10^\x12\x08\n\x04BCCS\x10_\x12\x07\n\x03BCF\x10`\x12\
    \x07\n\x03BCN\x10a\x12\x07\n\x03BCO\x10b\x12\x08\n\x04BCPT\x10c\x12\x07\
    \n\x03BCY\x10d\x12\x07\n\x03BDL\x10e\x12\x08\n\x04BELA\x10f\x12\t\n\x05B\
    ENJI\x10g\x12\x08\n\x04BERN\x10h\x12\x08\n\x04BEST\x10i\x12\x07\n\x03BGR\
    \x10j\x12\t\n\x05BIGUP\x10k\x12\x08\n\x04BIOB\x10l\x12\x08\n\x04BIOS\x10\
    m\x12\x07\n\x03BIP\x10n\x12\t\n\x05BIRDS\x10o\x12\x07\n\x03BIS\x10p\x12\
    \x07\n\x03BIT\x10q\x12\x08\n\x04BITB\x10r\x12\n\n\x06BITBTC\x10s\x12\t\n\
    \x05BITCF\x10t\x12\n\n\x06BITCNY\x10u\x12\n\n\x06BITEUR\x10v\x12\x0b\n\
    \x07BITGOLD\x10w\x12\t\n\x05BITOK\x10x\x12\x08\n\x04BITS\x10y\x12\r\n\tB\
    ITSILVER\x10z\x12\n\n\x06BITUSD\x10{\x12\x08\n\x04BITZ\x10|\x12\x08\n\
    \x04BLAS\x10}\x12\t\n\x05BLAZR\x10~\x12\x07\n\x03BLC\x10\x7f\x12\n\n\x05\
    BLITZ\x10\x80\x01\x12\x08\n\x03BLK\x10\x81\x01\x12\x08\n\x03BLN\x10\x82\
    \x01\x12\n\n\x05BLOCK\x10\x83\x01\x12\r\n\x08BLOCKPAY\x10\x84\x01\x12\t\
    \n\x04BLRY\x10\x85\x01\x12\x08\n\x03BLU\x10\x86\x01\x12\t\n\x04BLUE\x10\
    \x87\x01\x12\x08\n\x03BLX\x10\x88\x01\x12\x08\n\x03BLZ\x10\x89\x01\x12\
    \x08\n\x03BMC\x10\x8a\x01\x12\x08\n\x03BNB\x10\x8b\x01\x12\x08\n\x03BNT\
    \x10\x8c\x01\x12\x08\n\x03BNX\x10\x8d\x01\x12\t\n\x04BOAT\x10\x8e\x01\
    \x12\t\n\x04BOLI\x10\x8f\x01\x12\x08\n\x03BOS\x10\x90\x01\x12\t\n\x04BOS\
    T\x10\x91\x01\x12\x08\n\x03BPC\x10\x92\x01\x12\x07\n\x02BQ\x10\x93\x01\
    \x12\x08\n\x03BQC\x10\x94\x01\x12\x08\n\x03BQX\x10\x95\x01\x12\n\n\x05BR\
    AIN\x10\x96\x01\x12\t\n\x04BRAT\x10\x97\x01\x12\t\n\x04BRIA\x10\x98\x01\
    \x12\t\n\x04BRIT\x10\x99\x01\x12\x08\n\x03BRK\x10\x9a\x01\x12\x08\n\x03B\
    RO\x10\x9b\x01\x12\x08\n\x03BRX\x10\x9c\x01\x12\x08\n\x03BSC\x10\x9d\x01\
    \x12\x08\n\x03BSD\x10\x9e\x01\x12\x08\n\x03BSN\x10\x9f\x01\x12\x08\n\x03\
    BSR\x10\xa0\x01\x12\n\n\x05BSTAR\x10\xa1\x01\x12\t\n\x04BSTY\x10\xa2\x01\
    \x12\x08\n\x03BT1\x10\xa3\x01\x12\x08\n\x03BT2\x10\xa4\x01\x12\x08\n\x03\
    BTA\x10\xa5\x01\x12\x08\n\x03BTB\x10\xa6\x01\x12\t\n\x04BTBc\x10\xa7\x01\
    \x12\t\n\x04BTCD\x10\xa8\x01\x12\t\n\x04BTCM\x10\xa9\x01\x12\t\n\x04BTCR\
    \x10\xaa\x01\x12\x0b\n\x06BTCRED\x10\xab\x01\x12\t\n\x04BTCZ\x10\xac\x01\
    \x12\t\n\x04BTDX\x10\xad\x01\x12\t\n\x04BTPL\x10\xae\x01\x12\x08\n\x03BT\
    Q\x10\xaf\x01\x12\x08\n\x03BTS\x10\xb0\x01\x12\t\n\x04BTSR\x10\xb1\x01\
    \x12\x08\n\x03BTU\x10\xb2\x01\x12\n\n\x05BTWTY\x10\xb3\x01\x12\x08\n\x03\
    BTX\x10\xb4\x01\x12\x08\n\x03BUB\x10\xb5\x01\x12\n\n\x05BUCKS\x10\xb6\
    \x01\x12\n\n\x05BUMBA\x10\xb7\x01\x12\x08\n\x03BUN\x10\xb8\x01\x12\n\n\
    \x05BURST\x10\xb9\x01\x12\t\n\x04BUZZ\x10\xba\x01\x12\x08\n\x03BVC\x10\
    \xbb\x01\x12\x08\n\x03BXC\x10\xbc\x01\x12\x08\n\x03BXT\x10\xbd\x01\x12\
    \x08\n\x03BYC\x10\xbe\x01\x12\x07\n\x02C2\x10\xbf\x01\x12\x08\n\x03CAB\
    \x10\xc0\x01\x12\t\n\x04CACH\x10\xc1\x01\x12\x0e\n\tCADASTRAL\x10\xc2\
    \x01\x12\x08\n\x03CAG\x10\xc3\x01\x12\t\n\x04CAGE\x10\xc4\x01\x12\t\n\
    \x04CALC\x10\xc5\x01\x12\t\n\x04CANN\x10\xc6\x01\x12\x08\n\x03CAP\x10\
    \xc7\x01\x12\x0b\n\x06CARBON\x10\xc8\x01\x12\x0b\n\x06CASINO\x10\xc9\x01\
    \x12\x08\n\x03CBD\x10\xca\x01\x12\x08\n\x03CBX\x10\xcb\x01\x12\x07\n\x02\
    CC\x10\xcc\x01\x12\x0b\n\x06CCM100\x10\xcd\x01\x12\x08\n\x03CCN\x10\xce\
    \x01\x12\t\n\x04CCRB\x10\xcf\x01\x12\x08\n\x03CCT\x10\xd0\x01\x12\x08\n\
    \x03CDN\x10\xd1\x01\x12\x08\n\x03CDT\x10\xd2\x01\x12\t\n\x04CESC\x10\xd3\
    \x01\x12\x07\n\x02CF\x10\xd4\x01\x12\x08\n\x03CFI\x10\xd5\x01\x12\x08\n\
    \x03CFT\x10\xd6\x01\x12\x08\n\x03CHC\x10\xd7\x01\x12\n\n\x05CHEAP\x10\
    \xd8\x01\x12\n\n\x05CHESS\x10\xd9\x01\x12\n\n\x05CHIPS\x10\xda\x01\x12\
    \x07\n\x02CJ\x10\xdb\x01\x12\t\n\x04CLAM\x10\xdc\x01\x12\n\n\x05CLINT\
    \x10\xdd\x01\x12\n\n\x05CLOAK\x10\xde\x01\x12\t\n\x04CLUB\x10\xdf\x01\
    \x12\x08\n\x03CME\x10\xe0\x01\x12\x08\n\x03CMP\x10\xe1\x01\x12\n\n\x05CM\
    PCO\x10\xe2\x01\x12\x08\n\x03CMT\x10\xe3\x01\x12\x08\n\x03CNC\x10\xe4\
    \x01\x12\x08\n\x03CND\x10\xe5\x01\x12\t\n\x04CNNC\x10\xe6\x01\x12\x08\n\
    \x03CNO\x10\xe7\x01\x12\x08\n\x03CNT\x10\xe8\x01\x12\x08\n\x03CNX\x10\
    \xe9\x01\x12\t\n\x04COAL\x10\xea\x01\x12\x08\n\x03COB\x10\xeb\x01\x12\t\
    \n\x04COLX\x10\xec\x01\x12\x08\n\x03CON\x10\xed\x01\x12\t\n\x04CONX\x10\
    \xee\x01\x12\x08\n\x03COR\x10\xef\x01\x12\t\n\x04CORG\x10\xf0\x01\x12\t\
    \n\x04COSS\x10\xf1\x01\x12\n\n\x05COUPE\x10\xf2\x01\x12\n\n\x05COVAL\x10\
    \xf3\x01\x12\n\n\x05COXST\x10\xf4\x01\x12\x08\n\x03CPC\x10\xf5\x01\x12\
    \x08\n\x03CPN\x10\xf6\x01\x12\n\n\x05CRAVE\x10\xf7\x01\x12\x08\n\x03CRB\
    \x10\xf8\x01\x12\t\n\x04CREA\x10\xf9\x01\x12\n\n\x05CREDO\x10\xfa\x01\
    \x12\n\n\x05CREVA\x10\xfb\x01\x12\x08\n\x03CRM\x10\xfc\x01\x12\x08\n\x03\
    CRT\x10\xfd\x01\x12\x08\n\x03CRW\x10\xfe\x01\x12\x08\n\x03CRX\x10\xff\
    \x01\x12\n\n\x05CRYPT\x10\x80\x02\x12\x08\n\x03CSC\x10\x81\x02\x12\t\n\
    \x04CSNO\x10\x82\x02\x12\n\n\x05CTIC2\x10\x83\x02\x12\n\n\x05CTIC3\x10\
    \x84\x02\x12\x08\n\x03CTO\x10\x85\x02\x12\x08\n\x03CTR\x10\x86\x02\x12\t\
    \n\x04CUBE\x10\x87\x02\x12\t\n\x04CURE\x10\x88\x02\x12\x08\n\x03CV2\x10\
    \x89\x02\x12\x08\n\x03CVC\x10\x8a\x02\x12\x0b\n\x06CVCOIN\x10\x8b\x02\
    \x12\t\n\x04CWXT\x10\x8c\x02\x12\x08\n\x03CXT\x10\x8d\x02\x12\x08\n\x03C\
    YC\x10\x8e\x02\x12\n\n\x05CYDER\x10\x8f\x02\x12\x08\n\x03CYP\x10\x90\x02\
    \x12\t\n\x04DALC\x10\x91\x02\x12\x08\n\x03DAR\x10\x92\x02\x12\x08\n\x03D\
    AS\x10\x93\x02\x12\t\n\x04DASH\x10\x94\x02\x12\n\n\x05DASHS\x10\x95\x02\
    \x12\t\n\x04DATA\x10\x96\x02\x12\t\n\x04DAXX\x10\x97\x02\x12\x08\n\x03DA\
    Y\x10\x98\x02\x12\x08\n\x03DBG\x10\x99\x02\x12\t\n\x04DBIX\x10\x9a\x02\
    \x12\t\n\x04DBTC\x10\x9b\x02\x12\x08\n\x03DCN\x10\x9c\x02\x12\x08\n\x03D\
    CR\x10\x9d\x02\x12\t\n\x04DCRE\x10\x9e\x02\x12\x08\n\x03DCT\x10\x9f\x02\
    \x12\x08\n\x03DCY\x10\xa0\x02\x12\x08\n\x03DDF\x10\xa1\x02\x12\x08\n\x03\
    DEM\x10\xa2\x02\x12\t\n\x04DENT\x10\xa3\x02\x12\x08\n\x03DES\x10\xa4\x02\
    \x12\t\n\x04DEUS\x10\xa5\x02\x12\x08\n\x03DFS\x10\xa6\x02\x12\x08\n\x03D\
    FT\x10\xa7\x02\x12\x08\n\x03DGB\x10\xa8\x02\x12\x08\n\x03DGC\x10\xa9\x02\
    \x12\t\n\x04DGCS\x10\xaa\x02\x12\x08\n\x03DGD\x10\xab\x02\x12\t\n\x04DIB\
    C\x10\xac\x02\x12\t\n\x04DICE\x10\xad\x02\x12\t\n\x04DIME\x10\xae\x02\
    \x12\t\n\x04DISK\x10\xaf\x02\x12\x08\n\x03DIX\x10\xb0\x02\x12\x08\n\x03D\
    LC\x10\xb1\x02\x12\n\n\x05DLISK\x10\xb2\x02\x12\x08\n\x03DLT\x10\xb3\x02\
    \x12\x08\n\x03DMB\x10\xb4\x02\x12\x08\n\x03DMC\x10\xb5\x02\x12\x08\n\x03\
    DMD\x10\xb6\x02\x12\x08\n\x03DNR\x10\xb7\x02\x12\x08\n\x03DNT\x10\xb8\
    \x02\x12\t\n\x04DOGE\x10\xb9\x02\x12\x0b\n\x06DOLLAR\x10\xba\x02\x12\x08\
    \n\x03DON\x10\xbb\x02\x12\t\n\x04DOPE\x10\xbc\x02\x12\x08\n\x03DOT\x10\
    \xbd\x02\x12\t\n\x04DOVU\x10\xbe\x02\x12\x07\n\x02DP\x10\xbf\x02\x12\t\n\
    \x04DPAY\x10\xc0\x02\x12\n\n\x05DRACO\x10\xc1\x02\x12\x08\n\x03DRM\x10\
    \xc2\x02\x12\x08\n\x03DRS\x10\xc3\x02\x12\x08\n\x03DRT\x10\xc4\x02\x12\n\
    \n\x05DRXNE\x10\xc5\x02\x12\x08\n\x03DSH\x10\xc6\x02\x12\x08\n\x03DTB\
    \x10\xc7\x02\x12\x08\n\x03DUB\x10\xc8\x02\x12\x08\n\x03DUO\x10\xc9\x02\
    \x12\n\n\x05DUTCH\x10\xca\x02\x12\x08\n\x03DVC\x10\xcb\x02\x12\x08\n\x03\
    DYN\x10\xcc\x02\x12\n\n\x05E4ROW\x10\xcd\x02\x12\x08\n\x03EAC\x10\xce\
    \x02\x12\t\n\x04EBET\x10\xcf\x02\x12\t\n\x04EBIT\x10\xd0\x02\x12\t\n\x04\
    EBST\x10\xd1\x02\x12\x08\n\x03EBT\x10\xd2\x02\x12\x08\n\x03ECA\x10\xd3\
    \x02\x12\n\n\x05ECASH\x10\xd4\x02\x12\x08\n\x03ECC\x10\xd5\x02\x12\x08\n\
    \x03ECN\x10\xd6\x02\x12\x08\n\x03ECO\x10\xd7\x02\x12\t\n\x04ECOB\x10\xd8\
    \x02\x12\x08\n\x03EDG\x10\xd9\x02\x12\x08\n\x03EDO\x10\xda\x02\x12\n\n\
    \x05EDOGE\x10\xdb\x02\x12\x08\n\x03EDR\x10\xdc\x02\x12\t\n\x04EDRC\x10\
    \xdd\x02\x12\x08\n\x03EFL\x10\xde\x02\x12\t\n\x04EFYT\x10\xdf\x02\x12\t\
    \n\x04EGAS\x10\xe0\x02\x12\x08\n\x03EGC\x10\xe1\x02\x12\x08\n\x03EGG\x10\
    \xe2\x02\x12\x08\n\x03EGO\x10\xe3\x02\x12\n\n\x05EGOLD\x10\xe4\x02\x12\
    \x07\n\x02EL\x10\xe5\x02\x12\x08\n\x03ELC\x10\xe6\x02\x12\x08\n\x03ELE\
    \x10\xe7\x02\x12\n\n\x05ELITE\x10\xe8\x02\x12\t\n\x04ELIX\x10\xe9\x02\
    \x12\t\n\x04ELLA\x10\xea\x02\x12\x08\n\x03ELS\x10\xeb\x02\x12\n\n\x05ELT\
    C2\x10\xec\x02\x12\x08\n\x03EMB\x10\xed\x02\x12\x08\n\x03EMC\x10\xee\x02\
    \x12\t\n\x04EMC2\x10\xef\x02\x12\x08\n\x03EMD\x10\xf0\x02\x12\x08\n\x03E\
    MP\x10\xf1\x02\x12\x08\n\x03EMV\x10\xf2\x02\x12\x08\n\x03ENG\x10\xf3\x02\
    \x12\x08\n\x03ENJ\x10\xf4\x02\x12\t\n\x04ENRG\x10\xf5\x02\x12\x08\n\x03E\
    NT\x10\xf6\x02\x12\x08\n\x03ENV\x10\xf7\x02\x12\x08\n\x03EOS\x10\xf8\x02\
    \x12\x08\n\x03EOT\x10\xf9\x02\x12\x08\n\x03EQT\x10\xfa\x02\x12\x08\n\x03\
    ERC\x10\xfb\x02\x12\n\n\x05EREAL\x10\xfc\x02\x12\x08\n\x03ERY\x10\xfd\
    \x02\x12\x08\n\x03ESP\x10\xfe\x02\x12\t\n\x04ETBS\x10\xff\x02\x12\x08\n\
    \x03ETG\x10\x80\x03\x12\t\n\x04ETHD\x10\x81\x03\x12\x08\n\x03ETN\x10\x82\
    \x03\x12\x08\n\x03ETP\x10\x83\x03\x12\x08\n\x03ETX\x10\x84\x03\x12\x08\n\
    \x03EUC\x10\x85\x03\x12\t\n\x04EUSD\x10\x86\x03\x12\t\n\x04EVIL\x10\x87\
    \x03\x12\x08\n\x03EVO\x10\x88\x03\x12\x08\n\x03EVR\x10\x89\x03\x12\x08\n\
    \x03EVX\x10\x8a\x03\x12\t\n\x04EXCL\x10\x8b\x03\x12\x08\n\x03EXL\x10\x8c\
    \x03\x12\x08\n\x03EXN\x10\x8d\x03\x12\x08\n\x03EXP\x10\x8e\x03\x12\t\n\
    \x04EXRN\x10\x8f\x03\x12\t\n\x04FAIR\x10\x90\x03\x12\x08\n\x03FAL\x10\
    \x91\x03\x12\x08\n\x03FAP\x10\x92\x03\x12\t\n\x04FAZZ\x10\x93\x03\x12\
    \x07\n\x02FC\x10\x94\x03\x12\x08\n\x03FC2\x10\x95\x03\x12\x08\n\x03FCN\
    \x10\x96\x03\x12\x08\n\x03FCT\x10\x97\x03\x12\x08\n\x03FDC\x10\x98\x03\
    \x12\x08\n\x03FFC\x10\x99\x03\x12\x08\n\x03FID\x10\x9a\x03\x12\t\n\x04FI\
    MK\x10\x9b\x03\x12\t\n\x04FIRE\x10\x9c\x03\x12\x08\n\x03FJC\x10\x9d\x03\
    \x12\t\n\x04FLAP\x10\x9e\x03\x12\n\n\x05FLASH\x10\x9f\x03\x12\t\n\x04FLA\
    X\x10\xa0\x03\x12\t\n\x04FLDC\x10\xa1\x03\x12\t\n\x04FLIK\x10\xa2\x03\
    \x12\x08\n\x03FLO\x10\xa3\x03\x12\x08\n\x03FLT\x10\xa4\x03\x12\t\n\x04FL\
    VR\x10\xa5\x03\x12\x08\n\x03FLY\x10\xa6\x03\x12\x08\n\x03FNC\x10\xa7\x03\
    \x12\t\n\x04FONZ\x10\xa8\x03\x12\x08\n\x03FOR\x10\xa9\x03\x12\t\n\x04FRA\
    Z\x10\xaa\x03\x12\x08\n\x03FRC\x10\xab\x03\x12\t\n\x04FRGC\x10\xac\x03\
    \x12\x08\n\x03FRK\x10\xad\x03\x12\x08\n\x03FRN\x10\xae\x03\x12\t\n\x04FR\
    ST\x10\xaf\x03\x12\t\n\x04FRWC\x10\xb0\x03\x12\x08\n\x03FST\x10\xb1\x03\
    \x12\x08\n\x03FTC\x10\xb2\x03\x12\t\n\x04FUCK\x10\xb3\x03\x12\t\n\x04FUE\
    L\x10\xb4\x03\x12\x08\n\x03FUN\x10\xb5\x03\x12\t\n\x04FUNC\x10\xb6\x03\
    \x12\t\n\x04FUNK\x10\xb7\x03\x12\t\n\x04FUTC\x10\xb8\x03\x12\t\n\x04FUZZ\
    \x10\xb9\x03\x12\x08\n\x03FXE\x10\xba\x03\x12\x08\n\x03FYN\x10\xbb\x03\
    \x12\x08\n\x03FYP\x10\xbc\x03\x12\x08\n\x03G3N\x10\xbd\x03\x12\t\n\x04GA\
    IA\x10\xbe\x03\x12\t\n\x04GAIN\x10\xbf\x03\x12\x08\n\x03GAM\x10\xc0\x03\
    \x12\t\n\x04GAME\x10\xc1\x03\x12\x08\n\x03GAP\x10\xc2\x03\x12\t\n\x04GAR\
    Y\x10\xc3\x03\x12\x08\n\x03GAS\x10\xc4\x03\x12\x08\n\x03GAY\x10\xc5\x03\
    \x12\x07\n\x02GB\x10\xc6\x03\x12\x08\n\x03GBC\x10\xc7\x03\x12\x08\n\x03G\
    BG\x10\xc8\x03\x12\t\n\x04GBRC\x10\xc9\x03\x12\x08\n\x03GBT\x10\xca\x03\
    \x12\n\n\x05GBYTE\x10\xcb\x03\x12\x08\n\x03GCN\x10\xcc\x03\x12\x08\n\x03\
    GCR\x10\xcd\x03\x12\n\n\x05GEERT\x10\xce\x03\x12\x08\n\x03GEO\x10\xcf\
    \x03\x12\x08\n\x03GIM\x10\xd0\x03\x12\x08\n\x03GLC\x10\xd1\x03\x12\x08\n\
    \x03GLD\x10\xd2\x03\x12\x08\n\x03GLT\x10\xd3\x03\x12\x08\n\x03GML\x10\
    \xd4\x03\x12\x08\n\x03GMT\x10\xd5\x03\x12\x08\n\x03GMX\x10\xd6\x03\x12\
    \x08\n\x03GNO\x10\xd7\x03\x12\x08\n\x03GNT\x10\xd8\x03\x12\t\n\x04GOLF\
    \x10\xd9\x03\x12\n\n\x05GOLOS\x10\xda\x03\x12\t\n\x04GOOD\x10\xdb\x03\
    \x12\x07\n\x02GP\x10\xdc\x03\x12\x08\n\x03GPL\x10\xdd\x03\x12\x08\n\x03G\
    PU\x10\xde\x03\x12\x08\n\x03GRC\x10\xdf\x03\x12\x08\n\x03GRE\x10\xe0\x03\
    \x12\t\n\x04GRID\x10\xe1\x03\x12\x08\n\x03GRN\x10\xe2\x03\x12\x08\n\x03G\
    RS\x10\xe3\x03\x12\x08\n\x03GRT\x10\xe4\x03\x12\t\n\x04GRWI\x10\xe5\x03\
    \x12\x08\n\x03GSR\x10\xe6\x03\x12\x08\n\x03GTC\x10\xe7\x03\x12\x08\n\x03\
    GUC\x10\xe8\x03\x12\x08\n\x03GUN\x10\xe9\x03\x12\x08\n\x03GUP\x10\xea\
    \x03\x12\x08\n\x03GXS\x10\xeb\x03\x12\x08\n\x03HAL\x10\xec\x03\x12\n\n\
    \x05HALLO\x10\xed\x03\x12\x08\n\x03HBN\x10\xee\x03\x12\x08\n\x03HBT\x10\
    \xef\x03\x12\x08\n\x03HCC\x10\xf0\x03\x12\x08\n\x03HDG\x10\xf1\x03\x12\t\
    \n\x04HDLB\x10\xf2\x03\x12\t\n\x04HEAT\x10\xf3\x03\x12\t\n\x04HERO\x10\
    \xf4\x03\x12\x08\n\x03HGT\x10\xf5\x03\x12\t\n\x04HIGH\x10\xf6\x03\x12\
    \x08\n\x03HKG\x10\xf7\x03\x12\x08\n\x03HMC\x10\xf8\x03\x12\x08\n\x03HMP\
    \x10\xf9\x03\x12\x08\n\x03HMQ\x10\xfa\x03\x12\t\n\x04HODL\x10\xfb\x03\
    \x12\n\n\x05HONEY\x10\xfc\x03\x12\x08\n\x03HPC\x10\xfd\x03\x12\x08\n\x03\
    HSR\x10\xfe\x03\x12\x08\n\x03HTC\x10\xff\x03\x12\n\n\x05HTML5\x10\x80\
    \x04\x12\x08\n\x03HUC\x10\x81\x04\x12\t\n\x04HUSH\x10\x82\x04\x12\t\n\
    \x04HVCO\x10\x83\x04\x12\x08\n\x03HVN\x10\x84\x04\x12\x08\n\x03HXX\x10\
    \x85\x04\x12\x08\n\x03HYP\x10\x86\x04\x12\n\n\x05HYPER\x10\x87\x04\x12\
    \x08\n\x03I0C\x10\x88\x04\x12\n\n\x05IBANK\x10\x89\x04\x12\t\n\x04IBTC\
    \x10\x8a\x04\x12\x08\n\x03ICE\x10\x8b\x04\x12\t\n\x04ICOB\x10\x8c\x04\
    \x12\t\n\x04ICON\x10\x8d\x04\x12\t\n\x04ICOO\x10\x8e\x04\x12\t\n\x04ICOS\
    \x10\x8f\x04\x12\x08\n\x03ICX\x10\x90\x04\x12\t\n\x04IETH\x10\x91\x04\
    \x12\x08\n\x03IFC\x10\x92\x04\x12\t\n\x04IFLT\x10\x93\x04\x12\x08\n\x03I\
    FT\x10\x94\x04\x12\t\n\x04IMPS\x10\x95\x04\x12\x08\n\x03IMS\x10\x96\x04\
    \x12\x08\n\x03IMX\x10\x97\x04\x12\n\n\x05INCNT\x10\x98\x04\x12\x08\n\x03\
    IND\x10\x99\x04\x12\n\n\x05INDIA\x10\x9a\x04\x12\x08\n\x03INF\x10\x9b\
    \x04\x12\t\n\x04INFX\x10\x9c\x04\x12\n\n\x05INPAY\x10\x9d\x04\x12\t\n\
    \x04INSN\x10\x9e\x04\x12\t\n\x04INXT\x10\x9f\x04\x12\x08\n\x03IOC\x10\
    \xa0\x04\x12\x08\n\x03ION\x10\xa1\x04\x12\x08\n\x03IOP\x10\xa2\x04\x12\
    \x08\n\x03IQT\x10\xa3\x04\x12\x08\n\x03IRL\x10\xa4\x04\x12\x08\n\x03ISL\
    \x10\xa5\x04\x12\x08\n\x03ITI\x10\xa6\x04\x12\x08\n\x03ITT\x10\xa7\x04\
    \x12\x08\n\x03ITZ\x10\xa8\x04\x12\x08\n\x03IVZ\x10\xa9\x04\x12\x08\n\x03\
    IXC\x10\xaa\x04\x12\x08\n\x03IXT\x10\xab\x04\x12\x06\n\x01J\x10\xac\x04\
    \x12\x08\n\x03JET\x10\xad\x04\x12\x08\n\x03JIN\x10\xae\x04\x12\t\n\x04JI\
    NN\x10\xaf\x04\x12\x08\n\x03JIO\x10\xb0\x04\x12\x08\n\x03JNS\x10\xb1\x04\
    \x12\t\n\x04JOBS\x10\xb2\x04\x12\x07\n\x02JS\x10\xb3\x04\x12\x08\n\x03JW\
    L\x10\xb4\x04\x12\n\n\x05KARMA\x10\xb5\x04\x12\n\n\x05KASHH\x10\xb6\x04\
    \x12\t\n\x04KAYI\x10\xb7\x04\x12\x08\n\x03KCS\x10\xb8\x04\x12\x08\n\x03K\
    ED\x10\xb9\x04\x12\x08\n\x03KEK\x10\xba\x04\x12\x0c\n\x07KEXCOIN\x10\xbb\
    \x04\x12\x08\n\x03KIC\x10\xbc\x04\x12\t\n\x04KICK\x10\xbd\x04\x12\x08\n\
    \x03KIN\x10\xbe\x04\x12\x08\n\x03KLC\x10\xbf\x04\x12\x08\n\x03KLN\x10\
    \xc0\x04\x12\x08\n\x03KMD\x10\xc1\x04\x12\t\n\x04KOBO\x10\xc2\x04\x12\t\
    \n\x04KORE\x10\xc3\x04\x12\x08\n\x03KRB\x10\xc4\x04\x12\n\n\x05KRONE\x10\
    \xc5\x04\x12\t\n\x04KURT\x10\xc6\x04\x12\t\n\x04KUSH\x10\xc7\x04\x12\x07\
    \n\x02LA\x10\xc8\x04\x12\t\n\x04LANA\x10\xc9\x04\x12\x08\n\x03LAZ\x10\
    \xca\x04\x12\x08\n\x03LBC\x10\xcb\x04\x12\t\n\x04LBTC\x10\xcc\x04\x12\
    \x08\n\x03LCP\x10\xcd\x04\x12\t\n\x04LDCN\x10\xce\x04\x12\n\n\x05LDOGE\
    \x10\xcf\x04\x12\x08\n\x03LEA\x10\xd0\x04\x12\x08\n\x03LEO\x10\xd1\x04\
    \x12\n\n\x05LEPEN\x10\xd2\x04\x12\x08\n\x03LEX\x10\xd3\x04\x12\x08\n\x03\
    LGD\x10\xd4\x04\x12\t\n\x04LIFE\x10\xd5\x04\x12\n\n\x05LINDA\x10\xd6\x04\
    \x12\t\n\x04LINK\x10\xd7\x04\x12\t\n\x04LINX\x10\xd8\x04\x12\x08\n\x03LI\
    R\x10\xd9\x04\x12\x08\n\x03LKC\x10\xda\x04\x12\x08\n\x03LKK\x10\xdb\x04\
    \x12\x08\n\x03LLT\x10\xdc\x04\x12\x08\n\x03LMC\x10\xdd\x04\x12\x08\n\x03\
    LNK\x10\xde\x04\x12\x08\n\x03LOG\x10\xdf\x04\x12\x08\n\x03LOT\x10\xe0\
    \x04\x12\x08\n\x03LRC\x10\xe1\x04\x12\x08\n\x03LSK\x10\xe2\x04\x12\x08\n\
    \x03LTB\x10\xe3\x04\x12\t\n\x04LTBC\x10\xe4\x04\x12\t\n\x04LTCR\x10\xe5\
    \x04\x12\t\n\x04LTCU\x10\xe6\x04\x12\x08\n\x03LTG\x10\xe7\x04\x12\x08\n\
    \x03LTH\x10\xe8\x04\x12\x08\n\x03LUN\x10\xe9\x04\x12\t\n\x04LUNA\x10\xea\
    \x04\x12\x08\n\x03LUX\x10\xeb\x04\x12\t\n\x04LVPS\x10\xec\x04\x12\x08\n\
    \x03MAC\x10\xed\x04\x12\x08\n\x03MAD\x10\xee\x04\x12\t\n\x04MAGN\x10\xef\
    \x04\x12\t\n\x04MAID\x10\xf0\x04\x12\t\n\x04MANA\x10\xf1\x04\x12\x08\n\
    \x03MAO\x10\xf2\x04\x12\x08\n\x03MAR\x10\xf3\x04\x12\t\n\x04MARS\x10\xf4\
    \x04\x12\t\n\x04MARX\x10\xf5\x04\x12\n\n\x05MAVRO\x10\xf6\x04\x12\x08\n\
    \x03MAX\x10\xf7\x04\x12\x08\n\x03MAY\x10\xf8\x04\x12\x08\n\x03MBI\x10\
    \xf9\x04\x12\x08\n\x03MBL\x10\xfa\x04\x12\t\n\x04MBRS\x10\xfb\x04\x12\t\
    \n\x04MCAP\x10\xfc\x04\x12\x08\n\x03MCI\x10\xfd\x04\x12\x08\n\x03MCO\x10\
    \xfe\x04\x12\x08\n\x03MCR\x10\xff\x04\x12\t\n\x04MCRN\x10\x80\x05\x12\
    \x08\n\x03MDA\x10\x81\x05\x12\x08\n\x03MEC\x10\x82\x05\x12\t\n\x04MEME\
    \x10\x83\x05\x12\x08\n\x03MEN\x10\x84\x05\x12\t\n\x04MEOW\x10\x85\x05\
    \x12\x08\n\x03MER\x10\x86\x05\x12\n\n\x05METAL\x10\x87\x05\x12\x07\n\x02\
    MG\x10\x88\x05\x12\x08\n\x03MGM\x10\x89\x05\x12\x08\n\x03MGO\x10\x8a\x05\
    \x12\t\n\x04MILO\x10\x8b\x05\x12\n\n\x05MINEX\x10\x8c\x05\x12\t\n\x04MIN\
    T\x10\x8d\x05\x12\n\n\x05MIOTA\x10\x8e\x05\x12\x08\n\x03MIU\x10\x8f\x05\
    \x12\x08\n\x03MLN\x10\x90\x05\x12\n\n\x05MMXVI\x10\x91\x05\x12\x08\n\x03\
    MND\x10\x92\x05\x12\x08\n\x03MNE\x10\x93\x05\x12\x08\n\x03MNM\x10\x94\
    \x05\x12\x08\n\x03MNX\x10\x95\x05\x12\x08\n\x03MOD\x10\x96\x05\x12\t\n\
    \x04MOIN\x10\x97\x05\x12\t\n\x04MOJO\x10\x98\x05\x12\t\n\x04MONA\x10\x99\
    \x05\x12\x0b\n\x06MONETA\x10\x9a\x05\x12\n\n\x05MONEY\x10\x9b\x05\x12\t\
    \n\x04MOON\x10\x9c\x05\x12\t\n\x04MOTO\x10\x9d\x05\x12\x08\n\x03MRC\x10\
    \x9e\x05\x12\t\n\x04MRJA\x10\x9f\x05\x12\t\n\x04MRNG\x10\xa0\x05\x12\x08\
    \n\x03MRT\x10\xa1\x05\x12\t\n\x04MSCN\x10\xa2\x05\x12\x08\n\x03MSD\x10\
    \xa3\x05\x12\x08\n\x03MSP\x10\xa4\x05\x12\x08\n\x03MST\x10\xa5\x05\x12\
    \x08\n\x03MTH\x10\xa6\x05\x12\x08\n\x03MTL\x10\xa7\x05\x12\x0b\n\x06MTLM\
    C3\x10\xa8\x05\x12\x08\n\x03MTM\x10\xa9\x05\x12\t\n\x04MTNC\x10\xaa\x05\
    \x12\x08\n\x03MUE\x10\xab\x05\x12\x08\n\x03MUG\x10\xac\x05\x12\n\n\x05MU\
    SIC\x10\xad\x05\x12\x08\n\x03MXT\x10\xae\x05\x12\x08\n\x03MYB\x10\xaf\
    \x05\x12\t\n\x04MYST\x10\xb0\x05\x12\x08\n\x03MZC\x10\xb1\x05\x12\t\n\
    \x04NAMO\x10\xb2\x05\x12\n\n\x05NANOX\x10\xb3\x05\x12\x08\n\x03NAS\x10\
    \xb4\x05\x12\t\n\x04NAUT\x10\xb5\x05\x12\x08\n\x03NAV\x10\xb6\x05\x12\
    \x08\n\x03NBE\x10\xb7\x05\x12\t\n\x04NBIT\x10\xb8\x05\x12\t\n\x04NDAO\
    \x10\xb9\x05\x12\x08\n\x03NDC\x10\xba\x05\x12\t\n\x04NEBL\x10\xbb\x05\
    \x12\x08\n\x03NEO\x10\xbc\x05\x12\t\n\x04NEOS\x10\xbd\x05\x12\n\n\x05NET\
    KO\x10\xbe\x05\x12\t\n\x04NEVA\x10\xbf\x05\x12\t\n\x04NEWB\x10\xc0\x05\
    \x12\x08\n\x03NKA\x10\xc1\x05\x12\t\n\x04NLC2\x10\xc2\x05\x12\x08\n\x03N\
    LG\x10\xc3\x05\x12\x08\n\x03NMC\x10\xc4\x05\x12\x08\n\x03NMR\x10\xc5\x05\
    \x12\t\n\x04NOBL\x10\xc6\x05\x12\t\n\x04NODC\x10\xc7\x05\x12\t\n\x04NOTE\
    \x10\xc8\x05\x12\x08\n\x03NRO\x10\xc9\x05\x12\x08\n\x03NSR\x10\xca\x05\
    \x12\x08\n\x03NTC\x10\xcb\x05\x12\t\n\x04NTCC\x10\xcc\x05\x12\x08\n\x03N\
    TO\x10\xcd\x05\x12\t\n\x04NTRN\x10\xce\x05\x12\t\n\x04NTWK\x10\xcf\x05\
    \x12\t\n\x04NULS\x10\xd0\x05\x12\x08\n\x03NVC\x10\xd1\x05\x12\t\n\x04NVS\
    T\x10\xd2\x05\x12\x08\n\x03NXC\x10\xd3\x05\x12\x08\n\x03NXS\x10\xd4\x05\
    \x12\x08\n\x03NXT\x10\xd5\x05\x12\x08\n\x03NXX\x10\xd6\x05\x12\t\n\x04NY\
    AN\x10\xd7\x05\x12\x08\n\x03NYC\x10\xd8\x05\x12\x08\n\x03OAX\x10\xd9\x05\
    \x12\n\n\x05OBITS\x10\xda\x05\x12\n\n\x05OCEAN\x10\xdb\x05\x12\x08\n\x03\
    OCL\x10\xdc\x05\x12\t\n\x04OCOW\x10\xdd\x05\x12\x08\n\x03OCT\x10\xde\x05\
    \x12\x08\n\x03ODN\x10\xdf\x05\x12\x08\n\x03OFF\x10\xe0\x05\x12\x08\n\x03\
    OHM\x10\xe1\x05\x12\x07\n\x02OK\x10\xe2\x05\x12\x08\n\x03OMC\x10\xe3\x05\
    \x12\x08\n\x03OMG\x10\xe4\x05\x12\t\n\x04OMNI\x10\xe5\x05\x12\n\n\x05ONI\
    ON\x10\xe6\x05\x12\x08\n\x03ONX\x10\xe7\x05\x12\x07\n\x02OP\x10\xe8\x05\
    \x12\t\n\x04OPAL\x10\xe9\x05\x12\t\n\x04OPES\x10\xea\x05\x12\x08\n\x03OP\
    T\x10\xeb\x05\x12\x08\n\x03ORB\x10\xec\x05\x12\t\n\x04ORLY\x10\xed\x05\
    \x12\t\n\x04ORME\x10\xee\x05\x12\t\n\x04OS76\x10\xef\x05\x12\x08\n\x03OT\
    N\x10\xf0\x05\x12\x07\n\x02OX\x10\xf1\x05\x12\x08\n\x03P7C\x10\xf2\x05\
    \x12\x08\n\x03PAC\x10\xf3\x05\x12\x08\n\x03PAK\x10\xf4\x05\x12\t\n\x04PA\
    RT\x10\xf5\x05\x12\t\n\x04PASC\x10\xf6\x05\x12\t\n\x04PASL\x10\xf7\x05\
    \x12\x08\n\x03PAY\x10\xf8\x05\x12\t\n\x04PAYP\x10\xf9\x05\x12\x08\n\x03P\
    BT\x10\xfa\x05\x12\x08\n\x03PCN\x10\xfb\x05\x12\x08\n\x03PCS\x10\xfc\x05\
    \x12\x08\n\x03PDC\x10\xfd\x05\x12\x08\n\x03PDG\x10\xfe\x05\x12\x08\n\x03\
    PEC\x10\xff\x05\x12\r\n\x08PEPECASH\x10\x80\x06\x12\x08\n\x03PEX\x10\x81\
    \x06\x12\x08\n\x03PGL\x10\x82\x06\x12\x08\n\x03PHO\x10\x83\x06\x12\x08\n\
    \x03PHS\x10\x84\x06\x12\x07\n\x02PI\x10\x85\x06\x12\x08\n\x03PIE\x10\x86\
    \x06\x12\n\n\x05PIGGY\x10\x87\x06\x12\t\n\x04PING\x10\x88\x06\x12\t\n\
    \x04PINK\x10\x89\x06\x12\t\n\x04PIPL\x10\x8a\x06\x12\t\n\x04PIRL\x10\x8b\
    \x06\x12\t\n\x04PIVX\x10\x8c\x06\x12\x08\n\x03PIX\x10\x8d\x06\x12\n\n\
    \x05PIZZA\x10\x8e\x06\x12\x08\n\x03PKB\x10\x8f\x06\x12\n\n\x05PLACO\x10\
    \x90\x06\x12\t\n\x04PLBT\x10\x91\x06\x12\t\n\x04PLNC\x10\x92\x06\x12\x08\
    \n\x03PLR\x10\x93\x06\x12\x08\n\x03PLU\x10\x94\x06\x12\x08\n\x03PND\x10\
    \x95\x06\x12\x08\n\x03POE\x10\x96\x06\x12\t\n\x04POKE\x10\x97\x06\x12\t\
    \n\x04POLL\x10\x98\x06\x12\n\n\x05PONZI\x10\x99\x06\x12\x08\n\x03POP\x10\
    \x9a\x06\x12\x08\n\x03POS\x10\x9b\x06\x12\t\n\x04POST\x10\x9c\x06\x12\t\
    \n\x04POSW\x10\x9d\x06\x12\x08\n\x03POT\x10\x9e\x06\x12\t\n\x04POWR\x10\
    \x9f\x06\x12\x08\n\x03PPC\x10\xa0\x06\x12\x08\n\x03PPP\x10\xa1\x06\x12\
    \x08\n\x03PPT\x10\xa2\x06\x12\x08\n\x03PPY\x10\xa3\x06\x12\x07\n\x02PR\
    \x10\xa4\x06\x12\x08\n\x03PRC\x10\xa5\x06\x12\t\n\x04PRES\x10\xa6\x06\
    \x12\x08\n\x03PRG\x10\xa7\x06\x12\n\n\x05PRIMU\x10\xa8\x06\x12\x08\n\x03\
    PRM\x10\xa9\x06\x12\x08\n\x03PRN\x10\xaa\x06\x12\x08\n\x03PRO\x10\xab\
    \x06\x12\t\n\x04PROC\x10\xac\x06\x12\x08\n\x03PRX\x10\xad\x06\x12\x08\n\
    \x03PSB\x10\xae\x06\x12\x08\n\x03PST\x10\xaf\x06\x12\x08\n\x03PSY\x10\
    \xb0\x06\x12\x08\n\x03PTC\x10\xb1\x06\x12\t\n\x04PTOY\x10\xb2\x06\x12\n\
    \n\x05PULSE\x10\xb3\x06\x12\t\n\x04PURA\x10\xb4\x06\x12\x08\n\x03PUT\x10\
    \xb5\x06\x12\x08\n\x03PWR\x10\xb6\x06\x12\x07\n\x02PX\x10\xb7\x06\x12\
    \x08\n\x03PXC\x10\xb8\x06\x12\x08\n\x03PXI\x10\xb9\x06\x12\x08\n\x03PZM\
    \x10\xba\x06\x12\x08\n\x03Q2C\x10\xbb\x06\x12\x08\n\x03QAU\x10\xbc\x06\
    \x12\x08\n\x03QBC\x10\xbd\x06\x12\x08\n\x03QBK\x10\xbe\x06\x12\x08\n\x03\
    QBT\x10\xbf\x06\x12\x08\n\x03QCN\x10\xc0\x06\x12\t\n\x04QORA\x10\xc1\x06\
    \x12\x08\n\x03QRK\x10\xc2\x06\x12\x08\n\x03QRL\x10\xc3\x06\x12\x08\n\x03\
    QTL\x10\xc4\x06\x12\t\n\x04QTUM\x10\xc5\x06\x12\x08\n\x03QVT\x10\xc6\x06\
    \x12\n\n\x05QWARK\x10\xc7\x06\x12\x06\n\x01R\x10\xc8\x06\x12\t\n\x04RADS\
    \x10\xc9\x06\x12\t\n\x04RAIN\x10\xca\x06\x12\t\n\x04RBBT\x10\xcb\x06\x12\
    \n\n\x05RBIES\x10\xcc\x06\x12\x08\n\x03RBT\x10\xcd\x06\x12\x08\n\x03RBX\
    \x10\xce\x06\x12\x08\n\x03RBY\x10\xcf\x06\x12\x07\n\x02RC\x10\xd0\x06\
    \x12\x08\n\x03RDD\x10\xd1\x06\x12\t\n\x04REAL\x10\xd2\x06\x12\x08\n\x03R\
    EC\x10\xd3\x06\x12\x08\n\x03RED\x10\xd4\x06\x12\x08\n\x03REE\x10\xd5\x06\
    \x12\t\n\x04REGA\x10\xd6\x06\x12\x08\n\x03REP\x10\xd7\x06\x12\x08\n\x03R\
    EQ\x10\xd8\x06\x12\x08\n\x03REV\x10\xd9\x06\x12\x08\n\x03REX\x10\xda\x06\
    \x12\t\n\x04RHFC\x10\xdb\x06\x12\t\n\x04RHOC\x10\xdc\x06\x12\x08\n\x03RI\
    C\x10\xdd\x06\x12\n\n\x05RICHX\x10\xde\x06\x12\t\n\x04RIDE\x10\xdf\x06\
    \x12\t\n\x04RISE\x10\xe0\x06\x12\t\n\x04RIYA\x10\xe1\x06\x12\x08\n\x03RK\
    C\x10\xe2\x06\x12\x08\n\x03RLC\x10\xe3\x06\x12\x08\n\x03RLT\x10\xe4\x06\
    \x12\x08\n\x03RNS\x10\xe5\x06\x12\n\n\x05ROOFS\x10\xe6\x06\x12\n\n\x05RO\
    YAL\x10\xe7\x06\x12\x08\n\x03RPC\x10\xe8\x06\x12\x08\n\x03RPX\x10\xe9\
    \x06\x12\t\n\x04RSGP\x10\xea\x06\x12\n\n\x05RUBIT\x10\xeb\x06\x12\x0c\n\
    \x07RUNNERS\x10\xec\x06\x12\x08\n\x03RUP\x10\xed\x06\x12\t\n\x04RUPX\x10\
    \xee\x06\x12\r\n\x08RUSTBITS\x10\xef\x06\x12\x08\n\x03RVT\x10\xf0\x06\
    \x12\x08\n\x03SAC\x10\xf1\x06\x12\n\n\x05SAFEX\x10\xf2\x06\x12\x08\n\x03\
    SAK\x10\xf3\x06\x12\t\n\x04SALT\x10\xf4\x06\x12\x08\n\x03SAN\x10\xf5\x06\
    \x12\n\n\x05SANDG\x10\xf6\x06\x12\x08\n\x03SBD\x10\xf7\x06\x12\x07\n\x02\
    SC\x10\xf8\x06\x12\x08\n\x03SCL\x10\xf9\x06\x12\n\n\x05SCORE\x10\xfa\x06\
    \x12\t\n\x04SCRT\x10\xfb\x06\x12\x08\n\x03SCS\x10\xfc\x06\x12\x08\n\x03S\
    DC\x10\xfd\x06\x12\x08\n\x03SDP\x10\xfe\x06\x12\t\n\x04SDRN\x10\xff\x06\
    \x12\x08\n\x03SEQ\x10\x80\x07\x12\x08\n\x03SFC\x10\x81\x07\x12\x08\n\x03\
    SFE\x10\x82\x07\x12\x07\n\x02SH\x10\x83\x07\x12\x08\n\x03SHA\x10\x84\x07\
    \x12\t\n\x04SHDW\x10\x85\x07\x12\n\n\x05SHELL\x10\x86\x07\x12\n\n\x05SHI\
    FT\x10\x87\x07\x12\t\n\x04SHND\x10\x88\x07\x12\x0b\n\x06SHORTY\x10\x89\
    \x07\x12\x08\n\x03SIB\x10\x8a\x07\x12\x08\n\x03SIC\x10\x8b\x07\x12\t\n\
    \x04SIFT\x10\x8c\x07\x12\n\n\x05SIGMA\x10\x8d\x07\x12\t\n\x04SIGT\x10\
    \x8e\x07\x12\t\n\x04SJCX\x10\x8f\x07\x12\x08\n\x03SKC\x10\x90\x07\x12\t\
    \n\x04SKIN\x10\x91\x07\x12\x08\n\x03SKR\x10\x92\x07\x12\n\n\x05SKULL\x10\
    \x93\x07\x12\x08\n\x03SKY\x10\x94\x07\x12\x0b\n\x06SLEVIN\x10\x95\x07\
    \x12\t\n\x04SLFI\x10\x96\x07\x12\x08\n\x03SLG\x10\x97\x07\x12\n\n\x05SLI\
    NG\x10\x98\x07\x12\x08\n\x03SLM\x10\x99\x07\x12\x08\n\x03SLR\x10\x9a\x07\
    \x12\x08\n\x03SLS\x10\x9b\x07\x12\n\n\x05SMART\x10\x9c\x07\x12\x08\n\x03\
    SMC\x10\x9d\x07\x12\t\n\x04SMLY\x10\x9e\x07\x12\n\n\x05SMOKE\x10\x9f\x07\
    \x12\n\n\x05SNAKE\x10\xa0\x07\x12\x08\n\x03SNC\x10\xa1\x07\x12\x08\n\x03\
    SND\x10\xa2\x07\x12\n\n\x05SNGLS\x10\xa3\x07\x12\x08\n\x03SNM\x10\xa4\
    \x07\x12\t\n\x04SNRG\x10\xa5\x07\x12\x08\n\x03SNT\x10\xa6\x07\x12\t\n\
    \x04SOAR\x10\xa7\x07\x12\t\n\x04SOCC\x10\xa8\x07\x12\t\n\x04SOIL\x10\xa9\
    \x07\x12\x08\n\x03SOJ\x10\xaa\x07\x12\t\n\x04SONG\x10\xab\x07\x12\t\n\
    \x04SOON\x10\xac\x07\x12\t\n\x04SOUL\x10\xad\x07\x12\n\n\x05SPACE\x10\
    \xae\x07\x12\t\n\x04SPEX\x10\xaf\x07\x12\t\n\x04SPHR\x10\xb0\x07\x12\n\n\
    \x05SPORT\x10\xb1\x07\x12\x08\n\x03SPR\x10\xb2\x07\x12\n\n\x05SPRTS\x10\
    \xb3\x07\x12\x08\n\x03SPT\x10\xb4\x07\x12\x08\n\x03SRC\x10\xb5\x07\x12\
    \x08\n\x03STA\x10\xb6\x07\x12\n\n\x05START\x10\xb7\x07\x12\t\n\x04STCN\
    \x10\xb8\x07\x12\n\n\x05STEEM\x10\xb9\x07\x12\n\n\x05STEPS\x10\xba\x07\
    \x12\t\n\x04STEX\x10\xbb\x07\x12\n\n\x05STORJ\x10\xbc\x07\x12\n\n\x05STR\
    AT\x10\xbd\x07\x12\t\n\x04STRC\x10\xbe\x07\x12\x08\n\x03STS\x10\xbf\x07\
    \x12\x08\n\x03STV\x10\xc0\x07\x12\x08\n\x03STX\x10\xc1\x07\x12\x08\n\x03\
    SUB\x10\xc2\x07\x12\t\n\x04SUMO\x10\xc3\x07\x12\n\n\x05SUPER\x10\xc4\x07\
    \x12\x08\n\x03SUR\x10\xc5\x07\x12\n\n\x05SWIFT\x10\xc6\x07\x12\n\n\x05SW\
    ING\x10\xc7\x07\x12\x08\n\x03SWP\x10\xc8\x07\x12\x08\n\x03SWT\x10\xc9\
    \x07\x12\x08\n\x03SXC\x10\xca\x07\x12\t\n\x04SYNC\x10\xcb\x07\x12\t\n\
    \x04SYNX\x10\xcc\x07\x12\x08\n\x03SYS\x10\xcd\x07\x12\t\n\x04TAAS\x10\
    \xce\x07\x12\x08\n\x03TAG\x10\xcf\x07\x12\t\n\x04TAGR\x10\xd0\x07\x12\
    \x08\n\x03TAJ\x10\xd1\x07\x12\t\n\x04TALK\x10\xd2\x07\x12\x08\n\x03TCC\
    \x10\xd3\x07\x12\n\n\x05TCOIN\x10\xd4\x07\x12\x08\n\x03TCR\x10\xd5\x07\
    \x12\t\n\x04TEAM\x10\xd6\x07\x12\x08\n\x03TEK\x10\xd7\x07\x12\t\n\x04TEL\
    L\x10\xd8\x07\x12\x08\n\x03TER\x10\xd9\x07\x12\t\n\x04TERA\x10\xda\x07\
    \x12\x08\n\x03TES\x10\xdb\x07\x12\n\n\x05TESLA\x10\xdc\x07\x12\x08\n\x03\
    TFL\x10\xdd\x07\x12\x08\n\x03TGC\x10\xde\x07\x12\x08\n\x03TGT\x10\xdf\
    \x07\x12\x08\n\x03THC\x10\xe0\x07\x12\x08\n\x03THS\x10\xe1\x07\x12\t\n\
    \x04TIME\x10\xe2\x07\x12\t\n\x04TIPS\x10\xe3\x07\x12\x08\n\x03TIT\x10\
    \xe4\x07\x12\x08\n\x03TKN\x10\xe5\x07\x12\x08\n\x03TKR\x10\xe6\x07\x12\
    \x08\n\x03TKS\x10\xe7\x07\x12\x08\n\x03TLE\x10\xe8\x07\x12\x08\n\x03TNT\
    \x10\xe9\x07\x12\x08\n\x03TOA\x10\xea\x07\x12\n\n\x05TODAY\x10\xeb\x07\
    \x12\n\n\x05TOKEN\x10\xec\x07\x12\x08\n\x03TOP\x10\xed\x07\x12\n\n\x05TO\
    PAZ\x10\xee\x07\x12\x08\n\x03TOR\x10\xef\x07\x12\n\n\x05TRADE\x10\xf0\
    \x07\x12\x08\n\x03TRC\x10\xf1\x07\x12\t\n\x04TRCT\x10\xf2\x07\x12\x08\n\
    \x03TRI\x10\xf3\x07\x12\n\n\x05TRICK\x10\xf4\x07\x12\t\n\x04TRIG\x10\xf5\
    \x07\x12\x08\n\x03TRK\x10\xf6\x07\x12\n\n\x05TROLL\x10\xf7\x07\x12\t\n\
    \x04TRST\x10\xf8\x07\x12\n\n\x05TRUMP\x10\xf9\x07\x12\n\n\x05TRUST\x10\
    \xfa\x07\x12\x08\n\x03TRX\x10\xfb\x07\x12\x08\n\x03TSE\x10\xfc\x07\x12\t\
    \n\x04TSTR\x10\xfd\x07\x12\x08\n\x03TTC\x10\xfe\x07\x12\n\n\x05TURBO\x10\
    \xff\x07\x12\x07\n\x02TX\x10\x80\x08\x12\x08\n\x03TYC\x10\x81\x08\x12\n\
    \n\x05TYCHO\x10\x82\x08\x12\x08\n\x03TZC\x10\x83\x08\x12\x08\n\x03UBQ\
    \x10\x84\x08\x12\x08\n\x03UET\x10\x85\x08\x12\x08\n\x03UFO\x10\x86\x08\
    \x12\x08\n\x03UGT\x10\x87\x08\x12\x08\n\x03UIS\x10\x88\x08\x12\x08\n\x03\
    ULA\x10\x89\x08\x12\x08\n\x03UNB\x10\x8a\x08\x12\x08\n\x03UNC\x10\x8b\
    \x08\x12\x08\n\x03UNI\x10\x8c\x08\x12\t\n\x04UNIC\x10\x8d\x08\x12\n\n\
    \x05UNIFY\x10\x8e\x08\x12\t\n\x04UNIT\x10\x8f\x08\x12\n\n\x05UNITS\x10\
    \x90\x08\x12\n\n\x05UNITY\x10\x91\x08\x12\x08\n\x03UNO\x10\x92\x08\x12\t\
    \n\x04UNRC\x10\x93\x08\x12\x08\n\x03UNY\x10\x94\x08\x12\x07\n\x02UR\x10\
    \x95\x08\x12\x08\n\x03URC\x10\x96\x08\x12\x08\n\x03URO\x10\x97\x08\x12\
    \x08\n\x03USC\x10\x98\x08\x12\t\n\x04USDE\x10\x99\x08\x12\t\n\x04USDT\
    \x10\x9a\x08\x12\n\n\x05USNBT\x10\x9b\x08\x12\x08\n\x03UTA\x10\x9c\x08\
    \x12\x08\n\x03UTC\x10\x9d\x08\x12\x06\n\x01V\x10\x9e\x08\x12\x08\n\x03VA\
    L\x10\x9f\x08\x12\t\n\x04VASH\x10\xa0\x08\x12\x07\n\x02VC\x10\xa1\x08\
    \x12\t\n\x04VEC2\x10\xa2\x08\x12\x08\n\x03VEN\x10\xa3\x08\x12\t\n\x04VER\
    I\x10\xa4\x08\x12\x08\n\x03VGC\x10\xa5\x08\x12\x08\n\x03VIA\x10\xa6\x08\
    \x12\x08\n\x03VIB\x10\xa7\x08\x12\t\n\x04VIBE\x10\xa8\x08\x12\t\n\x04VID\
    Z\x10\xa9\x08\x12\x08\n\x03VIP\x10\xaa\x08\x12\n\n\x05VISIO\x10\xab\x08\
    \x12\t\n\x04VIVO\x10\xac\x08\x12\x08\n\x03VLT\x10\xad\x08\x12\t\n\x04VLT\
    C\x10\xae\x08\x12\n\n\x05VOISE\x10\xaf\x08\x12\t\n\x04VOLT\x10\xb0\x08\
    \x12\x08\n\x03VOX\x10\xb1\x08\x12\t\n\x04VOYA\x10\xb2\x08\x12\t\n\x04VPR\
    C\x10\xb3\x08\x12\x08\n\x03VRC\x10\xb4\x08\x12\x08\n\x03VRM\x10\xb5\x08\
    \x12\x08\n\x03VRS\x10\xb6\x08\x12\x08\n\x03VSL\x10\xb7\x08\x12\x08\n\x03\
    VSX\x10\xb8\x08\x12\x08\n\x03VTA\x10\xb9\x08\x12\x08\n\x03VTC\x10\xba\
    \x08\x12\x08\n\x03VTR\x10\xbb\x08\x12\x08\n\x03VUC\x10\xbc\x08\x12\t\n\
    \x04VULC\x10\xbd\x08\x12\x07\n\x02WA\x10\xbe\x08\x12\t\n\x04WARP\x10\xbf\
    \x08\x12\n\n\x05WAVES\x10\xc0\x08\x12\x08\n\x03WAY\x10\xc1\x08\x12\x08\n\
    \x03WBB\x10\xc2\x08\x12\x08\n\x03WBC\x10\xc3\x08\x12\x08\n\x03WCT\x10\
    \xc4\x08\x12\x08\n\x03WDC\x10\xc5\x08\x12\x08\n\x03WEC\x10\xc6\x08\x12\
    \x08\n\x03WEX\x10\xc7\x08\x12\x08\n\x03WGO\x10\xc8\x08\x12\x08\n\x03WGR\
    \x10\xc9\x08\x12\x08\n\x03WHL\x10\xca\x08\x12\x08\n\x03WIC\x10\xcb\x08\
    \x12\t\n\x04WILD\x10\xcc\x08\x12\n\n\x05WINGS\x10\xcd\x08\x12\t\n\x04WIN\
    K\x10\xce\x08\x12\x08\n\x03WMC\x10\xcf\x08\x12\n\n\x05WOMEN\x10\xd0\x08\
    \x12\t\n\x04WORM\x10\xd1\x08\x12\x08\n\x03WOW\x10\xd2\x08\x12\x08\n\x03W\
    SX\x10\xd3\x08\x12\x08\n\x03WTC\x10\xd4\x08\x12\x08\n\x03WTT\x10\xd5\x08\
    \x12\x08\n\x03WYV\x10\xd6\x08\x12\x07\n\x02X2\x10\xd7\x08\x12\x08\n\x03X\
    AS\x10\xd8\x08\x12\x08\n\x03XAU\x10\xd9\x08\x12\t\n\x04XAUR\x10\xda\x08\
    \x12\x08\n\x03XBC\x10\xdb\x08\x12\x08\n\x03XBL\x10\xdc\x08\x12\x0b\n\x06\
    XBTC21\x10\xdd\x08\x12\t\n\x04XBTS\x10\xde\x08\x12\x08\n\x03XBY\x10\xdf\
    \x08\x12\x07\n\x02XC\x10\xe0\x08\x12\x08\n\x03XCN\x10\xe1\x08\x12\x08\n\
    \x03XCO\x10\xe2\x08\x12\x08\n\x03XCP\x10\xe3\x08\x12\t\n\x04XCRE\x10\xe4\
    \x08\x12\x08\n\x03XCS\x10\xe5\x08\x12\x08\n\x03XCT\x10\xe6\x08\x12\t\n\
    \x04XCXT\x10\xe7\x08\x12\t\n\x04XDE2\x10\xe8\x08\x12\x08\n\x03XDN\x10\
    \xe9\x08\x12\x08\n\x03XEL\x10\xea\x08\x12\x08\n\x03XEM\x10\xeb\x08\x12\
    \x08\n\x03XFT\x10\xec\x08\x12\t\n\x04XGOX\x10\xed\x08\x12\x08\n\x03XGR\
    \x10\xee\x08\x12\x08\n\x03XHI\x10\xef\x08\x12\x08\n\x03XIN\x10\xf0\x08\
    \x12\t\n\x04XIOS\x10\xf1\x08\x12\x08\n\x03XJO\x10\xf2\x08\x12\x08\n\x03X\
    LC\x10\xf3\x08\x12\x08\n\x03XLM\x10\xf4\x08\x12\x08\n\x03XLR\x10\xf5\x08\
    \x12\t\n\x04XMCC\x10\xf6\x08\x12\x08\n\x03XMG\x10\xf7\x08\x12\x08\n\x03X\
    MR\x10\xf8\x08\x12\x08\n\x03XMY\x10\xf9\x08\x12\x08\n\x03XNG\x10\xfa\x08\
    \x12\x08\n\x03XNN\x10\xfb\x08\x12\x08\n\x03XOC\x10\xfc\x08\x12\x08\n\x03\
    XOT\x10\xfd\x08\x12\x07\n\x02XP\x10\xfe\x08\x12\x08\n\x03XPA\x10\xff\x08\
    \x12\x08\n\x03XPD\x10\x80\t\x12\x08\n\x03XPM\x10\x81\t\x12\t\n\x04XPTX\
    \x10\x82\t\x12\x08\n\x03XPY\x10\x83\t\x12\x08\n\x03XQN\x10\x84\t\x12\x08\
    \n\x03XRA\x10\x85\t\x12\x08\n\x03XRB\x10\x86\t\x12\x08\n\x03XRC\x10\x87\
    \t\x12\x08\n\x03XRE\x10\x88\t\x12\x08\n\x03XRL\x10\x89\t\x12\x08\n\x03XR\
    Y\x10\x8a\t\x12\x08\n\x03XSH\x10\x8b\t\x12\n\n\x05XSPEC\x10\x8c\t\x12\
    \x08\n\x03XST\x10\x8d\t\x12\t\n\x04XSTC\x10\x8e\t\x12\x08\n\x03XTC\x10\
    \x8f\t\x12\x08\n\x03XTD\x10\x90\t\x12\x08\n\x03XTO\x10\x91\t\x12\x08\n\
    \x03XTZ\x10\x92\t\x12\x08\n\x03XUC\x10\x93\t\x12\x08\n\x03XVC\x10\x94\t\
    \x12\x08\n\x03XVE\x10\x95\t\x12\x08\n\x03XVG\x10\x96\t\x12\x08\n\x03XVP\
    \x10\x97\t\x12\x08\n\x03XWC\x10\x98\t\x12\x08\n\x03XZC\x10\x99\t\x12\x08\
    \n\x03YAC\x10\x9a\t\x12\t\n\x04YASH\x10\x9b\t\x12\x08\n\x03YES\x10\x9c\t\
    \x12\x08\n\x03YOC\x10\x9d\t\x12\n\n\x05YOYOW\x10\x9e\t\x12\x08\n\x03ZBC\
    \x10\x9f\t\x12\x08\n\x03ZCC\x10\xa0\t\x12\x08\n\x03ZCL\x10\xa1\t\x12\t\n\
    \x04ZEIT\x10\xa2\t\x12\x08\n\x03ZEN\x10\xa3\t\x12\x0c\n\x07ZENGOLD\x10\
    \xa4\t\x12\t\n\x04ZENI\x10\xa5\t\x12\x08\n\x03ZER\x10\xa6\t\x12\x08\n\
    \x03ZET\x10\xa7\t\x12\x08\n\x03ZMC\x10\xa8\t\x12\x08\n\x03ZNE\x10\xa9\t\
    \x12\x08\n\x03ZNY\x10\xaa\t\x12\x08\n\x03ZOI\x10\xab\t\x12\x08\n\x03ZRC\
    \x10\xac\t\x12\x08\n\x03ZRX\x10\xad\t\x12\x08\n\x03ZSC\x10\xae\t\x12\x08\
    \n\x03ZSE\x10\xaf\t\x12\x08\n\x03ZUR\x10\xb0\t\x12\x08\n\x03ZYD\x10\xb1\
    \tb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
