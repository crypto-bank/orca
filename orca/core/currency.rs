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

    // .orca.Currency quote = 1;

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

    // .orca.Currency base = 2;

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

    // .orca.Currency currency = 1;

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
    AMB = 42,
    AMBER = 43,
    AMIS = 44,
    AMMO = 45,
    AMP = 46,
    AMS = 47,
    ANC = 48,
    ANI = 49,
    ANT = 50,
    ANTI = 51,
    ANTX = 52,
    APC = 53,
    APW = 54,
    APX = 55,
    ARB = 56,
    ARCO = 57,
    ARDR = 58,
    ARG = 59,
    ARGUS = 60,
    ARI = 61,
    ARK = 62,
    ART = 63,
    ASAFE2 = 64,
    ASC = 65,
    ASN = 66,
    AST = 67,
    ATB = 68,
    ATCC = 69,
    ATM = 70,
    ATMS = 71,
    ATOM = 72,
    ATS = 73,
    ATX = 74,
    AU = 75,
    AUR = 76,
    AURS = 77,
    AV = 78,
    AVT = 79,
    AXF = 80,
    AXIOM = 81,
    B2X = 82,
    B3 = 83,
    BAC = 84,
    BAS = 85,
    BASH = 86,
    BAY = 87,
    BBC = 88,
    BBP = 89,
    BCAP = 90,
    BCC = 91,
    BCCS = 92,
    BCF = 93,
    BCN = 94,
    BCO = 95,
    BCPT = 96,
    BCY = 97,
    BDL = 98,
    BELA = 99,
    BENJI = 100,
    BERN = 101,
    BEST = 102,
    BGR = 103,
    BIGUP = 104,
    BIOB = 105,
    BIOS = 106,
    BIP = 107,
    BIRDS = 108,
    BIS = 109,
    BIT = 110,
    BITB = 111,
    BITBTC = 112,
    BITCF = 113,
    BITCNY = 114,
    BITEUR = 115,
    BITGOLD = 116,
    BITOK = 117,
    BITS = 118,
    BITSILVER = 119,
    BITUSD = 120,
    BITZ = 121,
    BLAS = 122,
    BLAZR = 123,
    BLC = 124,
    BLITZ = 125,
    BLK = 126,
    BLN = 127,
    BLOCK = 128,
    BLOCKPAY = 129,
    BLRY = 130,
    BLU = 131,
    BLUE = 132,
    BLX = 133,
    BLZ = 134,
    BMC = 135,
    BNB = 136,
    BNT = 137,
    BNX = 138,
    BOAT = 139,
    BOLI = 140,
    BOS = 141,
    BOST = 142,
    BPC = 143,
    BQ = 144,
    BQC = 145,
    BQX = 146,
    BRAIN = 147,
    BRAT = 148,
    BRIA = 149,
    BRIT = 150,
    BRK = 151,
    BRO = 152,
    BRX = 153,
    BSC = 154,
    BSD = 155,
    BSN = 156,
    BSR = 157,
    BSTAR = 158,
    BSTY = 159,
    BT1 = 160,
    BT2 = 161,
    BTA = 162,
    BTB = 163,
    BTCD = 164,
    BTCM = 165,
    BTCR = 166,
    BTCRED = 167,
    BTCZ = 168,
    BTDX = 169,
    BTPL = 170,
    BTQ = 171,
    BTS = 172,
    BTSR = 173,
    BTU = 174,
    BTWTY = 175,
    BTX = 176,
    BUB = 177,
    BUCKS = 178,
    BUMBA = 179,
    BUN = 180,
    BURST = 181,
    BUZZ = 182,
    BVC = 183,
    BXC = 184,
    BXT = 185,
    BYC = 186,
    C2 = 187,
    CAB = 188,
    CACH = 189,
    CADASTRAL = 190,
    CAG = 191,
    CAGE = 192,
    CALC = 193,
    CANN = 194,
    CAP = 195,
    CARBON = 196,
    CASINO = 197,
    CBD = 198,
    CBX = 199,
    CC = 200,
    CCM100 = 201,
    CCN = 202,
    CCRB = 203,
    CCT = 204,
    CDN = 205,
    CDT = 206,
    CESC = 207,
    CF = 208,
    CFI = 209,
    CFT = 210,
    CHC = 211,
    CHEAP = 212,
    CHESS = 213,
    CHIPS = 214,
    CJ = 215,
    CLAM = 216,
    CLINT = 217,
    CLOAK = 218,
    CLUB = 219,
    CME = 220,
    CMP = 221,
    CMPCO = 222,
    CMT = 223,
    CNC = 224,
    CND = 225,
    CNNC = 226,
    CNO = 227,
    CNT = 228,
    CNX = 229,
    COAL = 230,
    COB = 231,
    COLX = 232,
    CON = 233,
    CONX = 234,
    COR = 235,
    CORG = 236,
    COSS = 237,
    COUPE = 238,
    COVAL = 239,
    COXST = 240,
    CPC = 241,
    CPN = 242,
    CRAVE = 243,
    CRB = 244,
    CREA = 245,
    CREDO = 246,
    CREVA = 247,
    CRM = 248,
    CRT = 249,
    CRW = 250,
    CRX = 251,
    CRYPT = 252,
    CSC = 253,
    CSNO = 254,
    CTIC2 = 255,
    CTIC3 = 256,
    CTO = 257,
    CTR = 258,
    CUBE = 259,
    CURE = 260,
    CV2 = 261,
    CVC = 262,
    CVCOIN = 263,
    CWXT = 264,
    CXT = 265,
    CYC = 266,
    CYDER = 267,
    CYP = 268,
    DALC = 269,
    DAR = 270,
    DAS = 271,
    DASH = 272,
    DASHS = 273,
    DAXX = 274,
    DAY = 275,
    DBG = 276,
    DBIX = 277,
    DBTC = 278,
    DCN = 279,
    DCR = 280,
    DCRE = 281,
    DCT = 282,
    DCY = 283,
    DDF = 284,
    DEM = 285,
    DENT = 286,
    DES = 287,
    DEUS = 288,
    DFS = 289,
    DFT = 290,
    DGB = 291,
    DGC = 292,
    DGCS = 293,
    DGD = 294,
    DIBC = 295,
    DICE = 296,
    DIME = 297,
    DISK = 298,
    DIX = 299,
    DLC = 300,
    DLISK = 301,
    DLT = 302,
    DMB = 303,
    DMC = 304,
    DMD = 305,
    DNR = 306,
    DNT = 307,
    DOGE = 308,
    DOLLAR = 309,
    DON = 310,
    DOPE = 311,
    DOT = 312,
    DOVU = 313,
    DP = 314,
    DPAY = 315,
    DRACO = 316,
    DRM = 317,
    DRS = 318,
    DRT = 319,
    DRXNE = 320,
    DSH = 321,
    DTB = 322,
    DUB = 323,
    DUO = 324,
    DUTCH = 325,
    DVC = 326,
    DYN = 327,
    E4ROW = 328,
    EAC = 329,
    EBET = 330,
    EBST = 331,
    EBT = 332,
    EBTC = 333,
    ECA = 334,
    ECASH = 335,
    ECC = 336,
    ECN = 337,
    ECO = 338,
    ECOB = 339,
    EDG = 340,
    EDO = 341,
    EDOGE = 342,
    EDR = 343,
    EDRC = 344,
    EFL = 345,
    EFYT = 346,
    EGC = 347,
    EGG = 348,
    EGO = 349,
    EL = 350,
    ELC = 351,
    ELE = 352,
    ELITE = 353,
    ELIX = 354,
    ELLA = 355,
    ELS = 356,
    ELTC2 = 357,
    EMB = 358,
    EMC = 359,
    EMC2 = 360,
    EMD = 361,
    EMP = 362,
    EMV = 363,
    ENG = 364,
    ENRG = 365,
    ENT = 366,
    ENV = 367,
    EOS = 368,
    EOT = 369,
    EQT = 370,
    ERC = 371,
    EREAL = 372,
    ERY = 373,
    ESP = 374,
    ETBS = 375,
    ETG = 376,
    ETHD = 377,
    ETP = 378,
    ETX = 379,
    EUC = 380,
    EUSD = 381,
    EVIL = 382,
    EVO = 383,
    EVR = 384,
    EVX = 385,
    EXCL = 386,
    EXL = 387,
    EXN = 388,
    EXP = 389,
    EXRN = 390,
    FAIR = 391,
    FAL = 392,
    FAP = 393,
    FAZZ = 394,
    FC = 395,
    FC2 = 396,
    FCN = 397,
    FCT = 398,
    FDC = 399,
    FFC = 400,
    FID = 401,
    FIMK = 402,
    FIRE = 403,
    FJC = 404,
    FLAP = 405,
    FLASH = 406,
    FLAX = 407,
    FLDC = 408,
    FLIK = 409,
    FLO = 410,
    FLT = 411,
    FLVR = 412,
    FLY = 413,
    FNC = 414,
    FONZ = 415,
    FRAZ = 416,
    FRC = 417,
    FRGC = 418,
    FRK = 419,
    FRN = 420,
    FRST = 421,
    FRWC = 422,
    FST = 423,
    FTC = 424,
    FUCK = 425,
    FUEL = 426,
    FUN = 427,
    FUNC = 428,
    FUNK = 429,
    FUTC = 430,
    FUZZ = 431,
    FXE = 432,
    FYN = 433,
    G3N = 434,
    GAIA = 435,
    GAIN = 436,
    GAM = 437,
    GAME = 438,
    GAP = 439,
    GARY = 440,
    GAS = 441,
    GAY = 442,
    GB = 443,
    GBC = 444,
    GBG = 445,
    GBRC = 446,
    GBT = 447,
    GBYTE = 448,
    GCN = 449,
    GCR = 450,
    GEERT = 451,
    GEO = 452,
    GLC = 453,
    GLD = 454,
    GLT = 455,
    GML = 456,
    GMT = 457,
    GMX = 458,
    GNO = 459,
    GNT = 460,
    GOLF = 461,
    GOLOS = 462,
    GOOD = 463,
    GP = 464,
    GPL = 465,
    GPU = 466,
    GRC = 467,
    GRE = 468,
    GRN = 469,
    GRS = 470,
    GRT = 471,
    GRWI = 472,
    GSR = 473,
    GTC = 474,
    GUC = 475,
    GUN = 476,
    GUP = 477,
    GXS = 478,
    HAL = 479,
    HALLO = 480,
    HBN = 481,
    HBT = 482,
    HCC = 483,
    HDG = 484,
    HDLB = 485,
    HEAT = 486,
    HERO = 487,
    HGT = 488,
    HKG = 489,
    HMC = 490,
    HMP = 491,
    HMQ = 492,
    HODL = 493,
    HONEY = 494,
    HPC = 495,
    HSR = 496,
    HTC = 497,
    HTML5 = 498,
    HUC = 499,
    HUSH = 500,
    HVCO = 501,
    HVN = 502,
    HXX = 503,
    HYP = 504,
    HYPER = 505,
    I0C = 506,
    IBANK = 507,
    ICE = 508,
    ICOB = 509,
    ICON = 510,
    ICOO = 511,
    ICOS = 512,
    ICX = 513,
    IETH = 514,
    IFC = 515,
    IFLT = 516,
    IFT = 517,
    IMPS = 518,
    IMS = 519,
    IMX = 520,
    INCNT = 521,
    IND = 522,
    INDIA = 523,
    INF = 524,
    INFX = 525,
    INPAY = 526,
    INSN = 527,
    INXT = 528,
    IOC = 529,
    ION = 530,
    IOP = 531,
    IQT = 532,
    IRL = 533,
    ISL = 534,
    ITI = 535,
    ITT = 536,
    ITZ = 537,
    IVZ = 538,
    IXC = 539,
    IXT = 540,
    J = 541,
    JET = 542,
    JIN = 543,
    JINN = 544,
    JIO = 545,
    JNS = 546,
    JOBS = 547,
    JS = 548,
    JWL = 549,
    KARMA = 550,
    KASHH = 551,
    KAYI = 552,
    KCS = 553,
    KED = 554,
    KEK = 555,
    KEXCOIN = 556,
    KIC = 557,
    KICK = 558,
    KIN = 559,
    KLC = 560,
    KLN = 561,
    KMD = 562,
    KOBO = 563,
    KORE = 564,
    KRB = 565,
    KRONE = 566,
    KURT = 567,
    KUSH = 568,
    LA = 569,
    LANA = 570,
    LAZ = 571,
    LBC = 572,
    LBTC = 573,
    LCP = 574,
    LDCN = 575,
    LDOGE = 576,
    LEA = 577,
    LEO = 578,
    LEPEN = 579,
    LEX = 580,
    LGD = 581,
    LIFE = 582,
    LINDA = 583,
    LINK = 584,
    LINX = 585,
    LIR = 586,
    LKC = 587,
    LKK = 588,
    LLT = 589,
    LMC = 590,
    LNK = 591,
    LOG = 592,
    LOT = 593,
    LRC = 594,
    LSK = 595,
    LTB = 596,
    LTBC = 597,
    LTCR = 598,
    LTCU = 599,
    LTG = 600,
    LTH = 601,
    LUN = 602,
    LUNA = 603,
    LUX = 604,
    LVPS = 605,
    MAC = 606,
    MAD = 607,
    MAGN = 608,
    MAID = 609,
    MANA = 610,
    MAO = 611,
    MAR = 612,
    MARS = 613,
    MARX = 614,
    MAVRO = 615,
    MAX = 616,
    MAY = 617,
    MBI = 618,
    MBL = 619,
    MBRS = 620,
    MCAP = 621,
    MCI = 622,
    MCO = 623,
    MCR = 624,
    MCRN = 625,
    MDA = 626,
    MEC = 627,
    MEME = 628,
    MEN = 629,
    MEOW = 630,
    MER = 631,
    METAL = 632,
    MG = 633,
    MGM = 634,
    MGO = 635,
    MILO = 636,
    MINEX = 637,
    MINT = 638,
    MIOTA = 639,
    MIU = 640,
    MLN = 641,
    MMXVI = 642,
    MND = 643,
    MNE = 644,
    MNM = 645,
    MOD = 646,
    MOIN = 647,
    MOJO = 648,
    MONA = 649,
    MONETA = 650,
    MONEY = 651,
    MOON = 652,
    MOTO = 653,
    MRC = 654,
    MRJA = 655,
    MRNG = 656,
    MRT = 657,
    MSCN = 658,
    MSD = 659,
    MSP = 660,
    MST = 661,
    MTH = 662,
    MTL = 663,
    MTLMC3 = 664,
    MTM = 665,
    MTNC = 666,
    MUE = 667,
    MUG = 668,
    MUSIC = 669,
    MXT = 670,
    MYB = 671,
    MYST = 672,
    MZC = 673,
    NAMO = 674,
    NANOX = 675,
    NAS = 676,
    NAUT = 677,
    NAV = 678,
    NBE = 679,
    NBIT = 680,
    NDAO = 681,
    NDC = 682,
    NEBL = 683,
    NEO = 684,
    NEOS = 685,
    NETKO = 686,
    NEVA = 687,
    NEWB = 688,
    NKA = 689,
    NLC2 = 690,
    NLG = 691,
    NMC = 692,
    NMR = 693,
    NOBL = 694,
    NODC = 695,
    NOTE = 696,
    NRO = 697,
    NSR = 698,
    NTC = 699,
    NTCC = 700,
    NTO = 701,
    NTRN = 702,
    NTWK = 703,
    NULS = 704,
    NVC = 705,
    NVST = 706,
    NXC = 707,
    NXS = 708,
    NXT = 709,
    NXX = 710,
    NYAN = 711,
    NYC = 712,
    OAX = 713,
    OBITS = 714,
    OCEAN = 715,
    OCL = 716,
    OCOW = 717,
    OCT = 718,
    ODN = 719,
    OFF = 720,
    OHM = 721,
    OK = 722,
    OMC = 723,
    OMG = 724,
    OMNI = 725,
    ONION = 726,
    ONX = 727,
    OP = 728,
    OPAL = 729,
    OPES = 730,
    OPT = 731,
    ORB = 732,
    ORLY = 733,
    ORME = 734,
    OS76 = 735,
    OTN = 736,
    OX = 737,
    P7C = 738,
    PAC = 739,
    PAK = 740,
    PART = 741,
    PASC = 742,
    PASL = 743,
    PAY = 744,
    PAYP = 745,
    PBT = 746,
    PCN = 747,
    PCS = 748,
    PDC = 749,
    PDG = 750,
    PEC = 751,
    PEPECASH = 752,
    PEX = 753,
    PGL = 754,
    PHO = 755,
    PHS = 756,
    PI = 757,
    PIE = 758,
    PIGGY = 759,
    PING = 760,
    PINK = 761,
    PIPL = 762,
    PIRL = 763,
    PIVX = 764,
    PIX = 765,
    PIZZA = 766,
    PKB = 767,
    PLACO = 768,
    PLBT = 769,
    PLNC = 770,
    PLR = 771,
    PLU = 772,
    PND = 773,
    POE = 774,
    POKE = 775,
    POLL = 776,
    PONZI = 777,
    POP = 778,
    POS = 779,
    POST = 780,
    POSW = 781,
    POT = 782,
    PPC = 783,
    PPP = 784,
    PPT = 785,
    PPY = 786,
    PR = 787,
    PRC = 788,
    PRES = 789,
    PRG = 790,
    PRIMU = 791,
    PRM = 792,
    PRN = 793,
    PRO = 794,
    PROC = 795,
    PRX = 796,
    PSB = 797,
    PST = 798,
    PSY = 799,
    PTC = 800,
    PTOY = 801,
    PULSE = 802,
    PURA = 803,
    PUT = 804,
    PWR = 805,
    PX = 806,
    PXC = 807,
    PXI = 808,
    PZM = 809,
    Q2C = 810,
    QAU = 811,
    QBC = 812,
    QBK = 813,
    QBT = 814,
    QCN = 815,
    QORA = 816,
    QRK = 817,
    QRL = 818,
    QTL = 819,
    QTUM = 820,
    QWARK = 821,
    RADS = 822,
    RAIN = 823,
    RBBT = 824,
    RBIES = 825,
    RBT = 826,
    RBX = 827,
    RBY = 828,
    RC = 829,
    RDD = 830,
    REAL = 831,
    REC = 832,
    RED = 833,
    REE = 834,
    REGA = 835,
    REP = 836,
    REQ = 837,
    REV = 838,
    REX = 839,
    RHFC = 840,
    RHOC = 841,
    RIC = 842,
    RICHX = 843,
    RIDE = 844,
    RISE = 845,
    RIYA = 846,
    RKC = 847,
    RLC = 848,
    RLT = 849,
    RMC = 850,
    RNS = 851,
    ROOFS = 852,
    ROYAL = 853,
    RPC = 854,
    RPX = 855,
    RSGP = 856,
    RUBIT = 857,
    RUNNERS = 858,
    RUP = 859,
    RUPX = 860,
    RUSTBITS = 861,
    RVT = 862,
    SAC = 863,
    SAFEX = 864,
    SAK = 865,
    SALT = 866,
    SAN = 867,
    SANDG = 868,
    SBD = 869,
    SC = 870,
    SCL = 871,
    SCORE = 872,
    SCRT = 873,
    SCS = 874,
    SDC = 875,
    SDP = 876,
    SDRN = 877,
    SEQ = 878,
    SFC = 879,
    SFE = 880,
    SH = 881,
    SHA = 882,
    SHDW = 883,
    SHELL = 884,
    SHIFT = 885,
    SHND = 886,
    SHORTY = 887,
    SIB = 888,
    SIC = 889,
    SIFT = 890,
    SIGMA = 891,
    SIGT = 892,
    SJCX = 893,
    SKC = 894,
    SKIN = 895,
    SKR = 896,
    SKULL = 897,
    SKY = 898,
    SLEVIN = 899,
    SLFI = 900,
    SLG = 901,
    SLING = 902,
    SLM = 903,
    SLR = 904,
    SLS = 905,
    SMART = 906,
    SMC = 907,
    SMLY = 908,
    SMOKE = 909,
    SNAKE = 910,
    SNC = 911,
    SND = 912,
    SNGLS = 913,
    SNM = 914,
    SNRG = 915,
    SNT = 916,
    SOAR = 917,
    SOCC = 918,
    SOIL = 919,
    SOJ = 920,
    SONG = 921,
    SOON = 922,
    SOUL = 923,
    SPACE = 924,
    SPEX = 925,
    SPHR = 926,
    SPORT = 927,
    SPR = 928,
    SPRTS = 929,
    SPT = 930,
    SRC = 931,
    STA = 932,
    START = 933,
    STCN = 934,
    STEEM = 935,
    STEPS = 936,
    STEX = 937,
    STORJ = 938,
    STRAT = 939,
    STRC = 940,
    STS = 941,
    STV = 942,
    STX = 943,
    SUB = 944,
    SUMO = 945,
    SUPER = 946,
    SUR = 947,
    SWIFT = 948,
    SWING = 949,
    SWP = 950,
    SWT = 951,
    SXC = 952,
    SYNC = 953,
    SYNX = 954,
    SYS = 955,
    TAAS = 956,
    TAG = 957,
    TAGR = 958,
    TAJ = 959,
    TALK = 960,
    TCC = 961,
    TCOIN = 962,
    TCR = 963,
    TEAM = 964,
    TEK = 965,
    TELL = 966,
    TER = 967,
    TERA = 968,
    TES = 969,
    TESLA = 970,
    TFL = 971,
    TGC = 972,
    TGT = 973,
    THC = 974,
    THS = 975,
    TIME = 976,
    TIPS = 977,
    TIT = 978,
    TKN = 979,
    TKR = 980,
    TKS = 981,
    TLE = 982,
    TNT = 983,
    TOA = 984,
    TODAY = 985,
    TOKEN = 986,
    TOP = 987,
    TOPAZ = 988,
    TOR = 989,
    TRADE = 990,
    TRC = 991,
    TRCT = 992,
    TRI = 993,
    TRICK = 994,
    TRIG = 995,
    TRK = 996,
    TROLL = 997,
    TRST = 998,
    TRUMP = 999,
    TRUST = 1000,
    TRX = 1001,
    TSE = 1002,
    TSTR = 1003,
    TTC = 1004,
    TURBO = 1005,
    TX = 1006,
    TYC = 1007,
    TYCHO = 1008,
    TZC = 1009,
    UBQ = 1010,
    UET = 1011,
    UFO = 1012,
    UGT = 1013,
    UIS = 1014,
    ULA = 1015,
    UNB = 1016,
    UNC = 1017,
    UNI = 1018,
    UNIC = 1019,
    UNIFY = 1020,
    UNIT = 1021,
    UNITS = 1022,
    UNITY = 1023,
    UNO = 1024,
    UNRC = 1025,
    UNY = 1026,
    UR = 1027,
    URC = 1028,
    URO = 1029,
    USC = 1030,
    USDE = 1031,
    USDT = 1032,
    USNBT = 1033,
    UTA = 1034,
    UTC = 1035,
    V = 1036,
    VAL = 1037,
    VASH = 1038,
    VC = 1039,
    VEC2 = 1040,
    VEN = 1041,
    VERI = 1042,
    VGC = 1043,
    VIA = 1044,
    VIB = 1045,
    VIBE = 1046,
    VIDZ = 1047,
    VIP = 1048,
    VISIO = 1049,
    VIVO = 1050,
    VLT = 1051,
    VLTC = 1052,
    VOISE = 1053,
    VOLT = 1054,
    VOX = 1055,
    VOYA = 1056,
    VPRC = 1057,
    VRC = 1058,
    VRM = 1059,
    VRS = 1060,
    VSL = 1061,
    VSX = 1062,
    VTA = 1063,
    VTC = 1064,
    VTR = 1065,
    VUC = 1066,
    VULC = 1067,
    WA = 1068,
    WARP = 1069,
    WAVES = 1070,
    WAY = 1071,
    WBB = 1072,
    WBC = 1073,
    WCT = 1074,
    WDC = 1075,
    WEC = 1076,
    WEX = 1077,
    WGO = 1078,
    WGR = 1079,
    WHL = 1080,
    WIC = 1081,
    WILD = 1082,
    WINGS = 1083,
    WINK = 1084,
    WMC = 1085,
    WOMEN = 1086,
    WORM = 1087,
    WOW = 1088,
    WSX = 1089,
    WTC = 1090,
    WTT = 1091,
    WYV = 1092,
    X2 = 1093,
    XAS = 1094,
    XAU = 1095,
    XAUR = 1096,
    XBC = 1097,
    XBG = 1098,
    XBL = 1099,
    XBTC21 = 1100,
    XBTS = 1101,
    XBY = 1102,
    XC = 1103,
    XCN = 1104,
    XCO = 1105,
    XCP = 1106,
    XCRE = 1107,
    XCS = 1108,
    XCT = 1109,
    XCXT = 1110,
    XDE2 = 1111,
    XDN = 1112,
    XEL = 1113,
    XEM = 1114,
    XFT = 1115,
    XGOX = 1116,
    XGR = 1117,
    XHI = 1118,
    XIN = 1119,
    XIOS = 1120,
    XJO = 1121,
    XLC = 1122,
    XLM = 1123,
    XLR = 1124,
    XMCC = 1125,
    XMG = 1126,
    XMR = 1127,
    XMY = 1128,
    XNG = 1129,
    XNN = 1130,
    XOC = 1131,
    XOT = 1132,
    XP = 1133,
    XPA = 1134,
    XPD = 1135,
    XPM = 1136,
    XPTX = 1137,
    XPY = 1138,
    XQN = 1139,
    XRA = 1140,
    XRB = 1141,
    XRC = 1142,
    XRE = 1143,
    XRL = 1144,
    XRY = 1145,
    XSPEC = 1146,
    XST = 1147,
    XSTC = 1148,
    XTC = 1149,
    XTD = 1150,
    XTO = 1151,
    XTZ = 1152,
    XUC = 1153,
    XVC = 1154,
    XVE = 1155,
    XVG = 1156,
    XVP = 1157,
    XWC = 1158,
    XZC = 1159,
    YAC = 1160,
    YASH = 1161,
    YES = 1162,
    YOC = 1163,
    YOYOW = 1164,
    ZBC = 1165,
    ZCC = 1166,
    ZCL = 1167,
    ZEIT = 1168,
    ZEN = 1169,
    ZENGOLD = 1170,
    ZENI = 1171,
    ZER = 1172,
    ZET = 1173,
    ZMC = 1174,
    ZNE = 1175,
    ZNY = 1176,
    ZOI = 1177,
    ZRC = 1178,
    ZRX = 1179,
    ZSC = 1180,
    ZSE = 1181,
    ZUR = 1182,
    ZYD = 1183,
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
            42 => ::std::option::Option::Some(Currency::AMB),
            43 => ::std::option::Option::Some(Currency::AMBER),
            44 => ::std::option::Option::Some(Currency::AMIS),
            45 => ::std::option::Option::Some(Currency::AMMO),
            46 => ::std::option::Option::Some(Currency::AMP),
            47 => ::std::option::Option::Some(Currency::AMS),
            48 => ::std::option::Option::Some(Currency::ANC),
            49 => ::std::option::Option::Some(Currency::ANI),
            50 => ::std::option::Option::Some(Currency::ANT),
            51 => ::std::option::Option::Some(Currency::ANTI),
            52 => ::std::option::Option::Some(Currency::ANTX),
            53 => ::std::option::Option::Some(Currency::APC),
            54 => ::std::option::Option::Some(Currency::APW),
            55 => ::std::option::Option::Some(Currency::APX),
            56 => ::std::option::Option::Some(Currency::ARB),
            57 => ::std::option::Option::Some(Currency::ARCO),
            58 => ::std::option::Option::Some(Currency::ARDR),
            59 => ::std::option::Option::Some(Currency::ARG),
            60 => ::std::option::Option::Some(Currency::ARGUS),
            61 => ::std::option::Option::Some(Currency::ARI),
            62 => ::std::option::Option::Some(Currency::ARK),
            63 => ::std::option::Option::Some(Currency::ART),
            64 => ::std::option::Option::Some(Currency::ASAFE2),
            65 => ::std::option::Option::Some(Currency::ASC),
            66 => ::std::option::Option::Some(Currency::ASN),
            67 => ::std::option::Option::Some(Currency::AST),
            68 => ::std::option::Option::Some(Currency::ATB),
            69 => ::std::option::Option::Some(Currency::ATCC),
            70 => ::std::option::Option::Some(Currency::ATM),
            71 => ::std::option::Option::Some(Currency::ATMS),
            72 => ::std::option::Option::Some(Currency::ATOM),
            73 => ::std::option::Option::Some(Currency::ATS),
            74 => ::std::option::Option::Some(Currency::ATX),
            75 => ::std::option::Option::Some(Currency::AU),
            76 => ::std::option::Option::Some(Currency::AUR),
            77 => ::std::option::Option::Some(Currency::AURS),
            78 => ::std::option::Option::Some(Currency::AV),
            79 => ::std::option::Option::Some(Currency::AVT),
            80 => ::std::option::Option::Some(Currency::AXF),
            81 => ::std::option::Option::Some(Currency::AXIOM),
            82 => ::std::option::Option::Some(Currency::B2X),
            83 => ::std::option::Option::Some(Currency::B3),
            84 => ::std::option::Option::Some(Currency::BAC),
            85 => ::std::option::Option::Some(Currency::BAS),
            86 => ::std::option::Option::Some(Currency::BASH),
            87 => ::std::option::Option::Some(Currency::BAY),
            88 => ::std::option::Option::Some(Currency::BBC),
            89 => ::std::option::Option::Some(Currency::BBP),
            90 => ::std::option::Option::Some(Currency::BCAP),
            91 => ::std::option::Option::Some(Currency::BCC),
            92 => ::std::option::Option::Some(Currency::BCCS),
            93 => ::std::option::Option::Some(Currency::BCF),
            94 => ::std::option::Option::Some(Currency::BCN),
            95 => ::std::option::Option::Some(Currency::BCO),
            96 => ::std::option::Option::Some(Currency::BCPT),
            97 => ::std::option::Option::Some(Currency::BCY),
            98 => ::std::option::Option::Some(Currency::BDL),
            99 => ::std::option::Option::Some(Currency::BELA),
            100 => ::std::option::Option::Some(Currency::BENJI),
            101 => ::std::option::Option::Some(Currency::BERN),
            102 => ::std::option::Option::Some(Currency::BEST),
            103 => ::std::option::Option::Some(Currency::BGR),
            104 => ::std::option::Option::Some(Currency::BIGUP),
            105 => ::std::option::Option::Some(Currency::BIOB),
            106 => ::std::option::Option::Some(Currency::BIOS),
            107 => ::std::option::Option::Some(Currency::BIP),
            108 => ::std::option::Option::Some(Currency::BIRDS),
            109 => ::std::option::Option::Some(Currency::BIS),
            110 => ::std::option::Option::Some(Currency::BIT),
            111 => ::std::option::Option::Some(Currency::BITB),
            112 => ::std::option::Option::Some(Currency::BITBTC),
            113 => ::std::option::Option::Some(Currency::BITCF),
            114 => ::std::option::Option::Some(Currency::BITCNY),
            115 => ::std::option::Option::Some(Currency::BITEUR),
            116 => ::std::option::Option::Some(Currency::BITGOLD),
            117 => ::std::option::Option::Some(Currency::BITOK),
            118 => ::std::option::Option::Some(Currency::BITS),
            119 => ::std::option::Option::Some(Currency::BITSILVER),
            120 => ::std::option::Option::Some(Currency::BITUSD),
            121 => ::std::option::Option::Some(Currency::BITZ),
            122 => ::std::option::Option::Some(Currency::BLAS),
            123 => ::std::option::Option::Some(Currency::BLAZR),
            124 => ::std::option::Option::Some(Currency::BLC),
            125 => ::std::option::Option::Some(Currency::BLITZ),
            126 => ::std::option::Option::Some(Currency::BLK),
            127 => ::std::option::Option::Some(Currency::BLN),
            128 => ::std::option::Option::Some(Currency::BLOCK),
            129 => ::std::option::Option::Some(Currency::BLOCKPAY),
            130 => ::std::option::Option::Some(Currency::BLRY),
            131 => ::std::option::Option::Some(Currency::BLU),
            132 => ::std::option::Option::Some(Currency::BLUE),
            133 => ::std::option::Option::Some(Currency::BLX),
            134 => ::std::option::Option::Some(Currency::BLZ),
            135 => ::std::option::Option::Some(Currency::BMC),
            136 => ::std::option::Option::Some(Currency::BNB),
            137 => ::std::option::Option::Some(Currency::BNT),
            138 => ::std::option::Option::Some(Currency::BNX),
            139 => ::std::option::Option::Some(Currency::BOAT),
            140 => ::std::option::Option::Some(Currency::BOLI),
            141 => ::std::option::Option::Some(Currency::BOS),
            142 => ::std::option::Option::Some(Currency::BOST),
            143 => ::std::option::Option::Some(Currency::BPC),
            144 => ::std::option::Option::Some(Currency::BQ),
            145 => ::std::option::Option::Some(Currency::BQC),
            146 => ::std::option::Option::Some(Currency::BQX),
            147 => ::std::option::Option::Some(Currency::BRAIN),
            148 => ::std::option::Option::Some(Currency::BRAT),
            149 => ::std::option::Option::Some(Currency::BRIA),
            150 => ::std::option::Option::Some(Currency::BRIT),
            151 => ::std::option::Option::Some(Currency::BRK),
            152 => ::std::option::Option::Some(Currency::BRO),
            153 => ::std::option::Option::Some(Currency::BRX),
            154 => ::std::option::Option::Some(Currency::BSC),
            155 => ::std::option::Option::Some(Currency::BSD),
            156 => ::std::option::Option::Some(Currency::BSN),
            157 => ::std::option::Option::Some(Currency::BSR),
            158 => ::std::option::Option::Some(Currency::BSTAR),
            159 => ::std::option::Option::Some(Currency::BSTY),
            160 => ::std::option::Option::Some(Currency::BT1),
            161 => ::std::option::Option::Some(Currency::BT2),
            162 => ::std::option::Option::Some(Currency::BTA),
            163 => ::std::option::Option::Some(Currency::BTB),
            164 => ::std::option::Option::Some(Currency::BTCD),
            165 => ::std::option::Option::Some(Currency::BTCM),
            166 => ::std::option::Option::Some(Currency::BTCR),
            167 => ::std::option::Option::Some(Currency::BTCRED),
            168 => ::std::option::Option::Some(Currency::BTCZ),
            169 => ::std::option::Option::Some(Currency::BTDX),
            170 => ::std::option::Option::Some(Currency::BTPL),
            171 => ::std::option::Option::Some(Currency::BTQ),
            172 => ::std::option::Option::Some(Currency::BTS),
            173 => ::std::option::Option::Some(Currency::BTSR),
            174 => ::std::option::Option::Some(Currency::BTU),
            175 => ::std::option::Option::Some(Currency::BTWTY),
            176 => ::std::option::Option::Some(Currency::BTX),
            177 => ::std::option::Option::Some(Currency::BUB),
            178 => ::std::option::Option::Some(Currency::BUCKS),
            179 => ::std::option::Option::Some(Currency::BUMBA),
            180 => ::std::option::Option::Some(Currency::BUN),
            181 => ::std::option::Option::Some(Currency::BURST),
            182 => ::std::option::Option::Some(Currency::BUZZ),
            183 => ::std::option::Option::Some(Currency::BVC),
            184 => ::std::option::Option::Some(Currency::BXC),
            185 => ::std::option::Option::Some(Currency::BXT),
            186 => ::std::option::Option::Some(Currency::BYC),
            187 => ::std::option::Option::Some(Currency::C2),
            188 => ::std::option::Option::Some(Currency::CAB),
            189 => ::std::option::Option::Some(Currency::CACH),
            190 => ::std::option::Option::Some(Currency::CADASTRAL),
            191 => ::std::option::Option::Some(Currency::CAG),
            192 => ::std::option::Option::Some(Currency::CAGE),
            193 => ::std::option::Option::Some(Currency::CALC),
            194 => ::std::option::Option::Some(Currency::CANN),
            195 => ::std::option::Option::Some(Currency::CAP),
            196 => ::std::option::Option::Some(Currency::CARBON),
            197 => ::std::option::Option::Some(Currency::CASINO),
            198 => ::std::option::Option::Some(Currency::CBD),
            199 => ::std::option::Option::Some(Currency::CBX),
            200 => ::std::option::Option::Some(Currency::CC),
            201 => ::std::option::Option::Some(Currency::CCM100),
            202 => ::std::option::Option::Some(Currency::CCN),
            203 => ::std::option::Option::Some(Currency::CCRB),
            204 => ::std::option::Option::Some(Currency::CCT),
            205 => ::std::option::Option::Some(Currency::CDN),
            206 => ::std::option::Option::Some(Currency::CDT),
            207 => ::std::option::Option::Some(Currency::CESC),
            208 => ::std::option::Option::Some(Currency::CF),
            209 => ::std::option::Option::Some(Currency::CFI),
            210 => ::std::option::Option::Some(Currency::CFT),
            211 => ::std::option::Option::Some(Currency::CHC),
            212 => ::std::option::Option::Some(Currency::CHEAP),
            213 => ::std::option::Option::Some(Currency::CHESS),
            214 => ::std::option::Option::Some(Currency::CHIPS),
            215 => ::std::option::Option::Some(Currency::CJ),
            216 => ::std::option::Option::Some(Currency::CLAM),
            217 => ::std::option::Option::Some(Currency::CLINT),
            218 => ::std::option::Option::Some(Currency::CLOAK),
            219 => ::std::option::Option::Some(Currency::CLUB),
            220 => ::std::option::Option::Some(Currency::CME),
            221 => ::std::option::Option::Some(Currency::CMP),
            222 => ::std::option::Option::Some(Currency::CMPCO),
            223 => ::std::option::Option::Some(Currency::CMT),
            224 => ::std::option::Option::Some(Currency::CNC),
            225 => ::std::option::Option::Some(Currency::CND),
            226 => ::std::option::Option::Some(Currency::CNNC),
            227 => ::std::option::Option::Some(Currency::CNO),
            228 => ::std::option::Option::Some(Currency::CNT),
            229 => ::std::option::Option::Some(Currency::CNX),
            230 => ::std::option::Option::Some(Currency::COAL),
            231 => ::std::option::Option::Some(Currency::COB),
            232 => ::std::option::Option::Some(Currency::COLX),
            233 => ::std::option::Option::Some(Currency::CON),
            234 => ::std::option::Option::Some(Currency::CONX),
            235 => ::std::option::Option::Some(Currency::COR),
            236 => ::std::option::Option::Some(Currency::CORG),
            237 => ::std::option::Option::Some(Currency::COSS),
            238 => ::std::option::Option::Some(Currency::COUPE),
            239 => ::std::option::Option::Some(Currency::COVAL),
            240 => ::std::option::Option::Some(Currency::COXST),
            241 => ::std::option::Option::Some(Currency::CPC),
            242 => ::std::option::Option::Some(Currency::CPN),
            243 => ::std::option::Option::Some(Currency::CRAVE),
            244 => ::std::option::Option::Some(Currency::CRB),
            245 => ::std::option::Option::Some(Currency::CREA),
            246 => ::std::option::Option::Some(Currency::CREDO),
            247 => ::std::option::Option::Some(Currency::CREVA),
            248 => ::std::option::Option::Some(Currency::CRM),
            249 => ::std::option::Option::Some(Currency::CRT),
            250 => ::std::option::Option::Some(Currency::CRW),
            251 => ::std::option::Option::Some(Currency::CRX),
            252 => ::std::option::Option::Some(Currency::CRYPT),
            253 => ::std::option::Option::Some(Currency::CSC),
            254 => ::std::option::Option::Some(Currency::CSNO),
            255 => ::std::option::Option::Some(Currency::CTIC2),
            256 => ::std::option::Option::Some(Currency::CTIC3),
            257 => ::std::option::Option::Some(Currency::CTO),
            258 => ::std::option::Option::Some(Currency::CTR),
            259 => ::std::option::Option::Some(Currency::CUBE),
            260 => ::std::option::Option::Some(Currency::CURE),
            261 => ::std::option::Option::Some(Currency::CV2),
            262 => ::std::option::Option::Some(Currency::CVC),
            263 => ::std::option::Option::Some(Currency::CVCOIN),
            264 => ::std::option::Option::Some(Currency::CWXT),
            265 => ::std::option::Option::Some(Currency::CXT),
            266 => ::std::option::Option::Some(Currency::CYC),
            267 => ::std::option::Option::Some(Currency::CYDER),
            268 => ::std::option::Option::Some(Currency::CYP),
            269 => ::std::option::Option::Some(Currency::DALC),
            270 => ::std::option::Option::Some(Currency::DAR),
            271 => ::std::option::Option::Some(Currency::DAS),
            272 => ::std::option::Option::Some(Currency::DASH),
            273 => ::std::option::Option::Some(Currency::DASHS),
            274 => ::std::option::Option::Some(Currency::DAXX),
            275 => ::std::option::Option::Some(Currency::DAY),
            276 => ::std::option::Option::Some(Currency::DBG),
            277 => ::std::option::Option::Some(Currency::DBIX),
            278 => ::std::option::Option::Some(Currency::DBTC),
            279 => ::std::option::Option::Some(Currency::DCN),
            280 => ::std::option::Option::Some(Currency::DCR),
            281 => ::std::option::Option::Some(Currency::DCRE),
            282 => ::std::option::Option::Some(Currency::DCT),
            283 => ::std::option::Option::Some(Currency::DCY),
            284 => ::std::option::Option::Some(Currency::DDF),
            285 => ::std::option::Option::Some(Currency::DEM),
            286 => ::std::option::Option::Some(Currency::DENT),
            287 => ::std::option::Option::Some(Currency::DES),
            288 => ::std::option::Option::Some(Currency::DEUS),
            289 => ::std::option::Option::Some(Currency::DFS),
            290 => ::std::option::Option::Some(Currency::DFT),
            291 => ::std::option::Option::Some(Currency::DGB),
            292 => ::std::option::Option::Some(Currency::DGC),
            293 => ::std::option::Option::Some(Currency::DGCS),
            294 => ::std::option::Option::Some(Currency::DGD),
            295 => ::std::option::Option::Some(Currency::DIBC),
            296 => ::std::option::Option::Some(Currency::DICE),
            297 => ::std::option::Option::Some(Currency::DIME),
            298 => ::std::option::Option::Some(Currency::DISK),
            299 => ::std::option::Option::Some(Currency::DIX),
            300 => ::std::option::Option::Some(Currency::DLC),
            301 => ::std::option::Option::Some(Currency::DLISK),
            302 => ::std::option::Option::Some(Currency::DLT),
            303 => ::std::option::Option::Some(Currency::DMB),
            304 => ::std::option::Option::Some(Currency::DMC),
            305 => ::std::option::Option::Some(Currency::DMD),
            306 => ::std::option::Option::Some(Currency::DNR),
            307 => ::std::option::Option::Some(Currency::DNT),
            308 => ::std::option::Option::Some(Currency::DOGE),
            309 => ::std::option::Option::Some(Currency::DOLLAR),
            310 => ::std::option::Option::Some(Currency::DON),
            311 => ::std::option::Option::Some(Currency::DOPE),
            312 => ::std::option::Option::Some(Currency::DOT),
            313 => ::std::option::Option::Some(Currency::DOVU),
            314 => ::std::option::Option::Some(Currency::DP),
            315 => ::std::option::Option::Some(Currency::DPAY),
            316 => ::std::option::Option::Some(Currency::DRACO),
            317 => ::std::option::Option::Some(Currency::DRM),
            318 => ::std::option::Option::Some(Currency::DRS),
            319 => ::std::option::Option::Some(Currency::DRT),
            320 => ::std::option::Option::Some(Currency::DRXNE),
            321 => ::std::option::Option::Some(Currency::DSH),
            322 => ::std::option::Option::Some(Currency::DTB),
            323 => ::std::option::Option::Some(Currency::DUB),
            324 => ::std::option::Option::Some(Currency::DUO),
            325 => ::std::option::Option::Some(Currency::DUTCH),
            326 => ::std::option::Option::Some(Currency::DVC),
            327 => ::std::option::Option::Some(Currency::DYN),
            328 => ::std::option::Option::Some(Currency::E4ROW),
            329 => ::std::option::Option::Some(Currency::EAC),
            330 => ::std::option::Option::Some(Currency::EBET),
            331 => ::std::option::Option::Some(Currency::EBST),
            332 => ::std::option::Option::Some(Currency::EBT),
            333 => ::std::option::Option::Some(Currency::EBTC),
            334 => ::std::option::Option::Some(Currency::ECA),
            335 => ::std::option::Option::Some(Currency::ECASH),
            336 => ::std::option::Option::Some(Currency::ECC),
            337 => ::std::option::Option::Some(Currency::ECN),
            338 => ::std::option::Option::Some(Currency::ECO),
            339 => ::std::option::Option::Some(Currency::ECOB),
            340 => ::std::option::Option::Some(Currency::EDG),
            341 => ::std::option::Option::Some(Currency::EDO),
            342 => ::std::option::Option::Some(Currency::EDOGE),
            343 => ::std::option::Option::Some(Currency::EDR),
            344 => ::std::option::Option::Some(Currency::EDRC),
            345 => ::std::option::Option::Some(Currency::EFL),
            346 => ::std::option::Option::Some(Currency::EFYT),
            347 => ::std::option::Option::Some(Currency::EGC),
            348 => ::std::option::Option::Some(Currency::EGG),
            349 => ::std::option::Option::Some(Currency::EGO),
            350 => ::std::option::Option::Some(Currency::EL),
            351 => ::std::option::Option::Some(Currency::ELC),
            352 => ::std::option::Option::Some(Currency::ELE),
            353 => ::std::option::Option::Some(Currency::ELITE),
            354 => ::std::option::Option::Some(Currency::ELIX),
            355 => ::std::option::Option::Some(Currency::ELLA),
            356 => ::std::option::Option::Some(Currency::ELS),
            357 => ::std::option::Option::Some(Currency::ELTC2),
            358 => ::std::option::Option::Some(Currency::EMB),
            359 => ::std::option::Option::Some(Currency::EMC),
            360 => ::std::option::Option::Some(Currency::EMC2),
            361 => ::std::option::Option::Some(Currency::EMD),
            362 => ::std::option::Option::Some(Currency::EMP),
            363 => ::std::option::Option::Some(Currency::EMV),
            364 => ::std::option::Option::Some(Currency::ENG),
            365 => ::std::option::Option::Some(Currency::ENRG),
            366 => ::std::option::Option::Some(Currency::ENT),
            367 => ::std::option::Option::Some(Currency::ENV),
            368 => ::std::option::Option::Some(Currency::EOS),
            369 => ::std::option::Option::Some(Currency::EOT),
            370 => ::std::option::Option::Some(Currency::EQT),
            371 => ::std::option::Option::Some(Currency::ERC),
            372 => ::std::option::Option::Some(Currency::EREAL),
            373 => ::std::option::Option::Some(Currency::ERY),
            374 => ::std::option::Option::Some(Currency::ESP),
            375 => ::std::option::Option::Some(Currency::ETBS),
            376 => ::std::option::Option::Some(Currency::ETG),
            377 => ::std::option::Option::Some(Currency::ETHD),
            378 => ::std::option::Option::Some(Currency::ETP),
            379 => ::std::option::Option::Some(Currency::ETX),
            380 => ::std::option::Option::Some(Currency::EUC),
            381 => ::std::option::Option::Some(Currency::EUSD),
            382 => ::std::option::Option::Some(Currency::EVIL),
            383 => ::std::option::Option::Some(Currency::EVO),
            384 => ::std::option::Option::Some(Currency::EVR),
            385 => ::std::option::Option::Some(Currency::EVX),
            386 => ::std::option::Option::Some(Currency::EXCL),
            387 => ::std::option::Option::Some(Currency::EXL),
            388 => ::std::option::Option::Some(Currency::EXN),
            389 => ::std::option::Option::Some(Currency::EXP),
            390 => ::std::option::Option::Some(Currency::EXRN),
            391 => ::std::option::Option::Some(Currency::FAIR),
            392 => ::std::option::Option::Some(Currency::FAL),
            393 => ::std::option::Option::Some(Currency::FAP),
            394 => ::std::option::Option::Some(Currency::FAZZ),
            395 => ::std::option::Option::Some(Currency::FC),
            396 => ::std::option::Option::Some(Currency::FC2),
            397 => ::std::option::Option::Some(Currency::FCN),
            398 => ::std::option::Option::Some(Currency::FCT),
            399 => ::std::option::Option::Some(Currency::FDC),
            400 => ::std::option::Option::Some(Currency::FFC),
            401 => ::std::option::Option::Some(Currency::FID),
            402 => ::std::option::Option::Some(Currency::FIMK),
            403 => ::std::option::Option::Some(Currency::FIRE),
            404 => ::std::option::Option::Some(Currency::FJC),
            405 => ::std::option::Option::Some(Currency::FLAP),
            406 => ::std::option::Option::Some(Currency::FLASH),
            407 => ::std::option::Option::Some(Currency::FLAX),
            408 => ::std::option::Option::Some(Currency::FLDC),
            409 => ::std::option::Option::Some(Currency::FLIK),
            410 => ::std::option::Option::Some(Currency::FLO),
            411 => ::std::option::Option::Some(Currency::FLT),
            412 => ::std::option::Option::Some(Currency::FLVR),
            413 => ::std::option::Option::Some(Currency::FLY),
            414 => ::std::option::Option::Some(Currency::FNC),
            415 => ::std::option::Option::Some(Currency::FONZ),
            416 => ::std::option::Option::Some(Currency::FRAZ),
            417 => ::std::option::Option::Some(Currency::FRC),
            418 => ::std::option::Option::Some(Currency::FRGC),
            419 => ::std::option::Option::Some(Currency::FRK),
            420 => ::std::option::Option::Some(Currency::FRN),
            421 => ::std::option::Option::Some(Currency::FRST),
            422 => ::std::option::Option::Some(Currency::FRWC),
            423 => ::std::option::Option::Some(Currency::FST),
            424 => ::std::option::Option::Some(Currency::FTC),
            425 => ::std::option::Option::Some(Currency::FUCK),
            426 => ::std::option::Option::Some(Currency::FUEL),
            427 => ::std::option::Option::Some(Currency::FUN),
            428 => ::std::option::Option::Some(Currency::FUNC),
            429 => ::std::option::Option::Some(Currency::FUNK),
            430 => ::std::option::Option::Some(Currency::FUTC),
            431 => ::std::option::Option::Some(Currency::FUZZ),
            432 => ::std::option::Option::Some(Currency::FXE),
            433 => ::std::option::Option::Some(Currency::FYN),
            434 => ::std::option::Option::Some(Currency::G3N),
            435 => ::std::option::Option::Some(Currency::GAIA),
            436 => ::std::option::Option::Some(Currency::GAIN),
            437 => ::std::option::Option::Some(Currency::GAM),
            438 => ::std::option::Option::Some(Currency::GAME),
            439 => ::std::option::Option::Some(Currency::GAP),
            440 => ::std::option::Option::Some(Currency::GARY),
            441 => ::std::option::Option::Some(Currency::GAS),
            442 => ::std::option::Option::Some(Currency::GAY),
            443 => ::std::option::Option::Some(Currency::GB),
            444 => ::std::option::Option::Some(Currency::GBC),
            445 => ::std::option::Option::Some(Currency::GBG),
            446 => ::std::option::Option::Some(Currency::GBRC),
            447 => ::std::option::Option::Some(Currency::GBT),
            448 => ::std::option::Option::Some(Currency::GBYTE),
            449 => ::std::option::Option::Some(Currency::GCN),
            450 => ::std::option::Option::Some(Currency::GCR),
            451 => ::std::option::Option::Some(Currency::GEERT),
            452 => ::std::option::Option::Some(Currency::GEO),
            453 => ::std::option::Option::Some(Currency::GLC),
            454 => ::std::option::Option::Some(Currency::GLD),
            455 => ::std::option::Option::Some(Currency::GLT),
            456 => ::std::option::Option::Some(Currency::GML),
            457 => ::std::option::Option::Some(Currency::GMT),
            458 => ::std::option::Option::Some(Currency::GMX),
            459 => ::std::option::Option::Some(Currency::GNO),
            460 => ::std::option::Option::Some(Currency::GNT),
            461 => ::std::option::Option::Some(Currency::GOLF),
            462 => ::std::option::Option::Some(Currency::GOLOS),
            463 => ::std::option::Option::Some(Currency::GOOD),
            464 => ::std::option::Option::Some(Currency::GP),
            465 => ::std::option::Option::Some(Currency::GPL),
            466 => ::std::option::Option::Some(Currency::GPU),
            467 => ::std::option::Option::Some(Currency::GRC),
            468 => ::std::option::Option::Some(Currency::GRE),
            469 => ::std::option::Option::Some(Currency::GRN),
            470 => ::std::option::Option::Some(Currency::GRS),
            471 => ::std::option::Option::Some(Currency::GRT),
            472 => ::std::option::Option::Some(Currency::GRWI),
            473 => ::std::option::Option::Some(Currency::GSR),
            474 => ::std::option::Option::Some(Currency::GTC),
            475 => ::std::option::Option::Some(Currency::GUC),
            476 => ::std::option::Option::Some(Currency::GUN),
            477 => ::std::option::Option::Some(Currency::GUP),
            478 => ::std::option::Option::Some(Currency::GXS),
            479 => ::std::option::Option::Some(Currency::HAL),
            480 => ::std::option::Option::Some(Currency::HALLO),
            481 => ::std::option::Option::Some(Currency::HBN),
            482 => ::std::option::Option::Some(Currency::HBT),
            483 => ::std::option::Option::Some(Currency::HCC),
            484 => ::std::option::Option::Some(Currency::HDG),
            485 => ::std::option::Option::Some(Currency::HDLB),
            486 => ::std::option::Option::Some(Currency::HEAT),
            487 => ::std::option::Option::Some(Currency::HERO),
            488 => ::std::option::Option::Some(Currency::HGT),
            489 => ::std::option::Option::Some(Currency::HKG),
            490 => ::std::option::Option::Some(Currency::HMC),
            491 => ::std::option::Option::Some(Currency::HMP),
            492 => ::std::option::Option::Some(Currency::HMQ),
            493 => ::std::option::Option::Some(Currency::HODL),
            494 => ::std::option::Option::Some(Currency::HONEY),
            495 => ::std::option::Option::Some(Currency::HPC),
            496 => ::std::option::Option::Some(Currency::HSR),
            497 => ::std::option::Option::Some(Currency::HTC),
            498 => ::std::option::Option::Some(Currency::HTML5),
            499 => ::std::option::Option::Some(Currency::HUC),
            500 => ::std::option::Option::Some(Currency::HUSH),
            501 => ::std::option::Option::Some(Currency::HVCO),
            502 => ::std::option::Option::Some(Currency::HVN),
            503 => ::std::option::Option::Some(Currency::HXX),
            504 => ::std::option::Option::Some(Currency::HYP),
            505 => ::std::option::Option::Some(Currency::HYPER),
            506 => ::std::option::Option::Some(Currency::I0C),
            507 => ::std::option::Option::Some(Currency::IBANK),
            508 => ::std::option::Option::Some(Currency::ICE),
            509 => ::std::option::Option::Some(Currency::ICOB),
            510 => ::std::option::Option::Some(Currency::ICON),
            511 => ::std::option::Option::Some(Currency::ICOO),
            512 => ::std::option::Option::Some(Currency::ICOS),
            513 => ::std::option::Option::Some(Currency::ICX),
            514 => ::std::option::Option::Some(Currency::IETH),
            515 => ::std::option::Option::Some(Currency::IFC),
            516 => ::std::option::Option::Some(Currency::IFLT),
            517 => ::std::option::Option::Some(Currency::IFT),
            518 => ::std::option::Option::Some(Currency::IMPS),
            519 => ::std::option::Option::Some(Currency::IMS),
            520 => ::std::option::Option::Some(Currency::IMX),
            521 => ::std::option::Option::Some(Currency::INCNT),
            522 => ::std::option::Option::Some(Currency::IND),
            523 => ::std::option::Option::Some(Currency::INDIA),
            524 => ::std::option::Option::Some(Currency::INF),
            525 => ::std::option::Option::Some(Currency::INFX),
            526 => ::std::option::Option::Some(Currency::INPAY),
            527 => ::std::option::Option::Some(Currency::INSN),
            528 => ::std::option::Option::Some(Currency::INXT),
            529 => ::std::option::Option::Some(Currency::IOC),
            530 => ::std::option::Option::Some(Currency::ION),
            531 => ::std::option::Option::Some(Currency::IOP),
            532 => ::std::option::Option::Some(Currency::IQT),
            533 => ::std::option::Option::Some(Currency::IRL),
            534 => ::std::option::Option::Some(Currency::ISL),
            535 => ::std::option::Option::Some(Currency::ITI),
            536 => ::std::option::Option::Some(Currency::ITT),
            537 => ::std::option::Option::Some(Currency::ITZ),
            538 => ::std::option::Option::Some(Currency::IVZ),
            539 => ::std::option::Option::Some(Currency::IXC),
            540 => ::std::option::Option::Some(Currency::IXT),
            541 => ::std::option::Option::Some(Currency::J),
            542 => ::std::option::Option::Some(Currency::JET),
            543 => ::std::option::Option::Some(Currency::JIN),
            544 => ::std::option::Option::Some(Currency::JINN),
            545 => ::std::option::Option::Some(Currency::JIO),
            546 => ::std::option::Option::Some(Currency::JNS),
            547 => ::std::option::Option::Some(Currency::JOBS),
            548 => ::std::option::Option::Some(Currency::JS),
            549 => ::std::option::Option::Some(Currency::JWL),
            550 => ::std::option::Option::Some(Currency::KARMA),
            551 => ::std::option::Option::Some(Currency::KASHH),
            552 => ::std::option::Option::Some(Currency::KAYI),
            553 => ::std::option::Option::Some(Currency::KCS),
            554 => ::std::option::Option::Some(Currency::KED),
            555 => ::std::option::Option::Some(Currency::KEK),
            556 => ::std::option::Option::Some(Currency::KEXCOIN),
            557 => ::std::option::Option::Some(Currency::KIC),
            558 => ::std::option::Option::Some(Currency::KICK),
            559 => ::std::option::Option::Some(Currency::KIN),
            560 => ::std::option::Option::Some(Currency::KLC),
            561 => ::std::option::Option::Some(Currency::KLN),
            562 => ::std::option::Option::Some(Currency::KMD),
            563 => ::std::option::Option::Some(Currency::KOBO),
            564 => ::std::option::Option::Some(Currency::KORE),
            565 => ::std::option::Option::Some(Currency::KRB),
            566 => ::std::option::Option::Some(Currency::KRONE),
            567 => ::std::option::Option::Some(Currency::KURT),
            568 => ::std::option::Option::Some(Currency::KUSH),
            569 => ::std::option::Option::Some(Currency::LA),
            570 => ::std::option::Option::Some(Currency::LANA),
            571 => ::std::option::Option::Some(Currency::LAZ),
            572 => ::std::option::Option::Some(Currency::LBC),
            573 => ::std::option::Option::Some(Currency::LBTC),
            574 => ::std::option::Option::Some(Currency::LCP),
            575 => ::std::option::Option::Some(Currency::LDCN),
            576 => ::std::option::Option::Some(Currency::LDOGE),
            577 => ::std::option::Option::Some(Currency::LEA),
            578 => ::std::option::Option::Some(Currency::LEO),
            579 => ::std::option::Option::Some(Currency::LEPEN),
            580 => ::std::option::Option::Some(Currency::LEX),
            581 => ::std::option::Option::Some(Currency::LGD),
            582 => ::std::option::Option::Some(Currency::LIFE),
            583 => ::std::option::Option::Some(Currency::LINDA),
            584 => ::std::option::Option::Some(Currency::LINK),
            585 => ::std::option::Option::Some(Currency::LINX),
            586 => ::std::option::Option::Some(Currency::LIR),
            587 => ::std::option::Option::Some(Currency::LKC),
            588 => ::std::option::Option::Some(Currency::LKK),
            589 => ::std::option::Option::Some(Currency::LLT),
            590 => ::std::option::Option::Some(Currency::LMC),
            591 => ::std::option::Option::Some(Currency::LNK),
            592 => ::std::option::Option::Some(Currency::LOG),
            593 => ::std::option::Option::Some(Currency::LOT),
            594 => ::std::option::Option::Some(Currency::LRC),
            595 => ::std::option::Option::Some(Currency::LSK),
            596 => ::std::option::Option::Some(Currency::LTB),
            597 => ::std::option::Option::Some(Currency::LTBC),
            598 => ::std::option::Option::Some(Currency::LTCR),
            599 => ::std::option::Option::Some(Currency::LTCU),
            600 => ::std::option::Option::Some(Currency::LTG),
            601 => ::std::option::Option::Some(Currency::LTH),
            602 => ::std::option::Option::Some(Currency::LUN),
            603 => ::std::option::Option::Some(Currency::LUNA),
            604 => ::std::option::Option::Some(Currency::LUX),
            605 => ::std::option::Option::Some(Currency::LVPS),
            606 => ::std::option::Option::Some(Currency::MAC),
            607 => ::std::option::Option::Some(Currency::MAD),
            608 => ::std::option::Option::Some(Currency::MAGN),
            609 => ::std::option::Option::Some(Currency::MAID),
            610 => ::std::option::Option::Some(Currency::MANA),
            611 => ::std::option::Option::Some(Currency::MAO),
            612 => ::std::option::Option::Some(Currency::MAR),
            613 => ::std::option::Option::Some(Currency::MARS),
            614 => ::std::option::Option::Some(Currency::MARX),
            615 => ::std::option::Option::Some(Currency::MAVRO),
            616 => ::std::option::Option::Some(Currency::MAX),
            617 => ::std::option::Option::Some(Currency::MAY),
            618 => ::std::option::Option::Some(Currency::MBI),
            619 => ::std::option::Option::Some(Currency::MBL),
            620 => ::std::option::Option::Some(Currency::MBRS),
            621 => ::std::option::Option::Some(Currency::MCAP),
            622 => ::std::option::Option::Some(Currency::MCI),
            623 => ::std::option::Option::Some(Currency::MCO),
            624 => ::std::option::Option::Some(Currency::MCR),
            625 => ::std::option::Option::Some(Currency::MCRN),
            626 => ::std::option::Option::Some(Currency::MDA),
            627 => ::std::option::Option::Some(Currency::MEC),
            628 => ::std::option::Option::Some(Currency::MEME),
            629 => ::std::option::Option::Some(Currency::MEN),
            630 => ::std::option::Option::Some(Currency::MEOW),
            631 => ::std::option::Option::Some(Currency::MER),
            632 => ::std::option::Option::Some(Currency::METAL),
            633 => ::std::option::Option::Some(Currency::MG),
            634 => ::std::option::Option::Some(Currency::MGM),
            635 => ::std::option::Option::Some(Currency::MGO),
            636 => ::std::option::Option::Some(Currency::MILO),
            637 => ::std::option::Option::Some(Currency::MINEX),
            638 => ::std::option::Option::Some(Currency::MINT),
            639 => ::std::option::Option::Some(Currency::MIOTA),
            640 => ::std::option::Option::Some(Currency::MIU),
            641 => ::std::option::Option::Some(Currency::MLN),
            642 => ::std::option::Option::Some(Currency::MMXVI),
            643 => ::std::option::Option::Some(Currency::MND),
            644 => ::std::option::Option::Some(Currency::MNE),
            645 => ::std::option::Option::Some(Currency::MNM),
            646 => ::std::option::Option::Some(Currency::MOD),
            647 => ::std::option::Option::Some(Currency::MOIN),
            648 => ::std::option::Option::Some(Currency::MOJO),
            649 => ::std::option::Option::Some(Currency::MONA),
            650 => ::std::option::Option::Some(Currency::MONETA),
            651 => ::std::option::Option::Some(Currency::MONEY),
            652 => ::std::option::Option::Some(Currency::MOON),
            653 => ::std::option::Option::Some(Currency::MOTO),
            654 => ::std::option::Option::Some(Currency::MRC),
            655 => ::std::option::Option::Some(Currency::MRJA),
            656 => ::std::option::Option::Some(Currency::MRNG),
            657 => ::std::option::Option::Some(Currency::MRT),
            658 => ::std::option::Option::Some(Currency::MSCN),
            659 => ::std::option::Option::Some(Currency::MSD),
            660 => ::std::option::Option::Some(Currency::MSP),
            661 => ::std::option::Option::Some(Currency::MST),
            662 => ::std::option::Option::Some(Currency::MTH),
            663 => ::std::option::Option::Some(Currency::MTL),
            664 => ::std::option::Option::Some(Currency::MTLMC3),
            665 => ::std::option::Option::Some(Currency::MTM),
            666 => ::std::option::Option::Some(Currency::MTNC),
            667 => ::std::option::Option::Some(Currency::MUE),
            668 => ::std::option::Option::Some(Currency::MUG),
            669 => ::std::option::Option::Some(Currency::MUSIC),
            670 => ::std::option::Option::Some(Currency::MXT),
            671 => ::std::option::Option::Some(Currency::MYB),
            672 => ::std::option::Option::Some(Currency::MYST),
            673 => ::std::option::Option::Some(Currency::MZC),
            674 => ::std::option::Option::Some(Currency::NAMO),
            675 => ::std::option::Option::Some(Currency::NANOX),
            676 => ::std::option::Option::Some(Currency::NAS),
            677 => ::std::option::Option::Some(Currency::NAUT),
            678 => ::std::option::Option::Some(Currency::NAV),
            679 => ::std::option::Option::Some(Currency::NBE),
            680 => ::std::option::Option::Some(Currency::NBIT),
            681 => ::std::option::Option::Some(Currency::NDAO),
            682 => ::std::option::Option::Some(Currency::NDC),
            683 => ::std::option::Option::Some(Currency::NEBL),
            684 => ::std::option::Option::Some(Currency::NEO),
            685 => ::std::option::Option::Some(Currency::NEOS),
            686 => ::std::option::Option::Some(Currency::NETKO),
            687 => ::std::option::Option::Some(Currency::NEVA),
            688 => ::std::option::Option::Some(Currency::NEWB),
            689 => ::std::option::Option::Some(Currency::NKA),
            690 => ::std::option::Option::Some(Currency::NLC2),
            691 => ::std::option::Option::Some(Currency::NLG),
            692 => ::std::option::Option::Some(Currency::NMC),
            693 => ::std::option::Option::Some(Currency::NMR),
            694 => ::std::option::Option::Some(Currency::NOBL),
            695 => ::std::option::Option::Some(Currency::NODC),
            696 => ::std::option::Option::Some(Currency::NOTE),
            697 => ::std::option::Option::Some(Currency::NRO),
            698 => ::std::option::Option::Some(Currency::NSR),
            699 => ::std::option::Option::Some(Currency::NTC),
            700 => ::std::option::Option::Some(Currency::NTCC),
            701 => ::std::option::Option::Some(Currency::NTO),
            702 => ::std::option::Option::Some(Currency::NTRN),
            703 => ::std::option::Option::Some(Currency::NTWK),
            704 => ::std::option::Option::Some(Currency::NULS),
            705 => ::std::option::Option::Some(Currency::NVC),
            706 => ::std::option::Option::Some(Currency::NVST),
            707 => ::std::option::Option::Some(Currency::NXC),
            708 => ::std::option::Option::Some(Currency::NXS),
            709 => ::std::option::Option::Some(Currency::NXT),
            710 => ::std::option::Option::Some(Currency::NXX),
            711 => ::std::option::Option::Some(Currency::NYAN),
            712 => ::std::option::Option::Some(Currency::NYC),
            713 => ::std::option::Option::Some(Currency::OAX),
            714 => ::std::option::Option::Some(Currency::OBITS),
            715 => ::std::option::Option::Some(Currency::OCEAN),
            716 => ::std::option::Option::Some(Currency::OCL),
            717 => ::std::option::Option::Some(Currency::OCOW),
            718 => ::std::option::Option::Some(Currency::OCT),
            719 => ::std::option::Option::Some(Currency::ODN),
            720 => ::std::option::Option::Some(Currency::OFF),
            721 => ::std::option::Option::Some(Currency::OHM),
            722 => ::std::option::Option::Some(Currency::OK),
            723 => ::std::option::Option::Some(Currency::OMC),
            724 => ::std::option::Option::Some(Currency::OMG),
            725 => ::std::option::Option::Some(Currency::OMNI),
            726 => ::std::option::Option::Some(Currency::ONION),
            727 => ::std::option::Option::Some(Currency::ONX),
            728 => ::std::option::Option::Some(Currency::OP),
            729 => ::std::option::Option::Some(Currency::OPAL),
            730 => ::std::option::Option::Some(Currency::OPES),
            731 => ::std::option::Option::Some(Currency::OPT),
            732 => ::std::option::Option::Some(Currency::ORB),
            733 => ::std::option::Option::Some(Currency::ORLY),
            734 => ::std::option::Option::Some(Currency::ORME),
            735 => ::std::option::Option::Some(Currency::OS76),
            736 => ::std::option::Option::Some(Currency::OTN),
            737 => ::std::option::Option::Some(Currency::OX),
            738 => ::std::option::Option::Some(Currency::P7C),
            739 => ::std::option::Option::Some(Currency::PAC),
            740 => ::std::option::Option::Some(Currency::PAK),
            741 => ::std::option::Option::Some(Currency::PART),
            742 => ::std::option::Option::Some(Currency::PASC),
            743 => ::std::option::Option::Some(Currency::PASL),
            744 => ::std::option::Option::Some(Currency::PAY),
            745 => ::std::option::Option::Some(Currency::PAYP),
            746 => ::std::option::Option::Some(Currency::PBT),
            747 => ::std::option::Option::Some(Currency::PCN),
            748 => ::std::option::Option::Some(Currency::PCS),
            749 => ::std::option::Option::Some(Currency::PDC),
            750 => ::std::option::Option::Some(Currency::PDG),
            751 => ::std::option::Option::Some(Currency::PEC),
            752 => ::std::option::Option::Some(Currency::PEPECASH),
            753 => ::std::option::Option::Some(Currency::PEX),
            754 => ::std::option::Option::Some(Currency::PGL),
            755 => ::std::option::Option::Some(Currency::PHO),
            756 => ::std::option::Option::Some(Currency::PHS),
            757 => ::std::option::Option::Some(Currency::PI),
            758 => ::std::option::Option::Some(Currency::PIE),
            759 => ::std::option::Option::Some(Currency::PIGGY),
            760 => ::std::option::Option::Some(Currency::PING),
            761 => ::std::option::Option::Some(Currency::PINK),
            762 => ::std::option::Option::Some(Currency::PIPL),
            763 => ::std::option::Option::Some(Currency::PIRL),
            764 => ::std::option::Option::Some(Currency::PIVX),
            765 => ::std::option::Option::Some(Currency::PIX),
            766 => ::std::option::Option::Some(Currency::PIZZA),
            767 => ::std::option::Option::Some(Currency::PKB),
            768 => ::std::option::Option::Some(Currency::PLACO),
            769 => ::std::option::Option::Some(Currency::PLBT),
            770 => ::std::option::Option::Some(Currency::PLNC),
            771 => ::std::option::Option::Some(Currency::PLR),
            772 => ::std::option::Option::Some(Currency::PLU),
            773 => ::std::option::Option::Some(Currency::PND),
            774 => ::std::option::Option::Some(Currency::POE),
            775 => ::std::option::Option::Some(Currency::POKE),
            776 => ::std::option::Option::Some(Currency::POLL),
            777 => ::std::option::Option::Some(Currency::PONZI),
            778 => ::std::option::Option::Some(Currency::POP),
            779 => ::std::option::Option::Some(Currency::POS),
            780 => ::std::option::Option::Some(Currency::POST),
            781 => ::std::option::Option::Some(Currency::POSW),
            782 => ::std::option::Option::Some(Currency::POT),
            783 => ::std::option::Option::Some(Currency::PPC),
            784 => ::std::option::Option::Some(Currency::PPP),
            785 => ::std::option::Option::Some(Currency::PPT),
            786 => ::std::option::Option::Some(Currency::PPY),
            787 => ::std::option::Option::Some(Currency::PR),
            788 => ::std::option::Option::Some(Currency::PRC),
            789 => ::std::option::Option::Some(Currency::PRES),
            790 => ::std::option::Option::Some(Currency::PRG),
            791 => ::std::option::Option::Some(Currency::PRIMU),
            792 => ::std::option::Option::Some(Currency::PRM),
            793 => ::std::option::Option::Some(Currency::PRN),
            794 => ::std::option::Option::Some(Currency::PRO),
            795 => ::std::option::Option::Some(Currency::PROC),
            796 => ::std::option::Option::Some(Currency::PRX),
            797 => ::std::option::Option::Some(Currency::PSB),
            798 => ::std::option::Option::Some(Currency::PST),
            799 => ::std::option::Option::Some(Currency::PSY),
            800 => ::std::option::Option::Some(Currency::PTC),
            801 => ::std::option::Option::Some(Currency::PTOY),
            802 => ::std::option::Option::Some(Currency::PULSE),
            803 => ::std::option::Option::Some(Currency::PURA),
            804 => ::std::option::Option::Some(Currency::PUT),
            805 => ::std::option::Option::Some(Currency::PWR),
            806 => ::std::option::Option::Some(Currency::PX),
            807 => ::std::option::Option::Some(Currency::PXC),
            808 => ::std::option::Option::Some(Currency::PXI),
            809 => ::std::option::Option::Some(Currency::PZM),
            810 => ::std::option::Option::Some(Currency::Q2C),
            811 => ::std::option::Option::Some(Currency::QAU),
            812 => ::std::option::Option::Some(Currency::QBC),
            813 => ::std::option::Option::Some(Currency::QBK),
            814 => ::std::option::Option::Some(Currency::QBT),
            815 => ::std::option::Option::Some(Currency::QCN),
            816 => ::std::option::Option::Some(Currency::QORA),
            817 => ::std::option::Option::Some(Currency::QRK),
            818 => ::std::option::Option::Some(Currency::QRL),
            819 => ::std::option::Option::Some(Currency::QTL),
            820 => ::std::option::Option::Some(Currency::QTUM),
            821 => ::std::option::Option::Some(Currency::QWARK),
            822 => ::std::option::Option::Some(Currency::RADS),
            823 => ::std::option::Option::Some(Currency::RAIN),
            824 => ::std::option::Option::Some(Currency::RBBT),
            825 => ::std::option::Option::Some(Currency::RBIES),
            826 => ::std::option::Option::Some(Currency::RBT),
            827 => ::std::option::Option::Some(Currency::RBX),
            828 => ::std::option::Option::Some(Currency::RBY),
            829 => ::std::option::Option::Some(Currency::RC),
            830 => ::std::option::Option::Some(Currency::RDD),
            831 => ::std::option::Option::Some(Currency::REAL),
            832 => ::std::option::Option::Some(Currency::REC),
            833 => ::std::option::Option::Some(Currency::RED),
            834 => ::std::option::Option::Some(Currency::REE),
            835 => ::std::option::Option::Some(Currency::REGA),
            836 => ::std::option::Option::Some(Currency::REP),
            837 => ::std::option::Option::Some(Currency::REQ),
            838 => ::std::option::Option::Some(Currency::REV),
            839 => ::std::option::Option::Some(Currency::REX),
            840 => ::std::option::Option::Some(Currency::RHFC),
            841 => ::std::option::Option::Some(Currency::RHOC),
            842 => ::std::option::Option::Some(Currency::RIC),
            843 => ::std::option::Option::Some(Currency::RICHX),
            844 => ::std::option::Option::Some(Currency::RIDE),
            845 => ::std::option::Option::Some(Currency::RISE),
            846 => ::std::option::Option::Some(Currency::RIYA),
            847 => ::std::option::Option::Some(Currency::RKC),
            848 => ::std::option::Option::Some(Currency::RLC),
            849 => ::std::option::Option::Some(Currency::RLT),
            850 => ::std::option::Option::Some(Currency::RMC),
            851 => ::std::option::Option::Some(Currency::RNS),
            852 => ::std::option::Option::Some(Currency::ROOFS),
            853 => ::std::option::Option::Some(Currency::ROYAL),
            854 => ::std::option::Option::Some(Currency::RPC),
            855 => ::std::option::Option::Some(Currency::RPX),
            856 => ::std::option::Option::Some(Currency::RSGP),
            857 => ::std::option::Option::Some(Currency::RUBIT),
            858 => ::std::option::Option::Some(Currency::RUNNERS),
            859 => ::std::option::Option::Some(Currency::RUP),
            860 => ::std::option::Option::Some(Currency::RUPX),
            861 => ::std::option::Option::Some(Currency::RUSTBITS),
            862 => ::std::option::Option::Some(Currency::RVT),
            863 => ::std::option::Option::Some(Currency::SAC),
            864 => ::std::option::Option::Some(Currency::SAFEX),
            865 => ::std::option::Option::Some(Currency::SAK),
            866 => ::std::option::Option::Some(Currency::SALT),
            867 => ::std::option::Option::Some(Currency::SAN),
            868 => ::std::option::Option::Some(Currency::SANDG),
            869 => ::std::option::Option::Some(Currency::SBD),
            870 => ::std::option::Option::Some(Currency::SC),
            871 => ::std::option::Option::Some(Currency::SCL),
            872 => ::std::option::Option::Some(Currency::SCORE),
            873 => ::std::option::Option::Some(Currency::SCRT),
            874 => ::std::option::Option::Some(Currency::SCS),
            875 => ::std::option::Option::Some(Currency::SDC),
            876 => ::std::option::Option::Some(Currency::SDP),
            877 => ::std::option::Option::Some(Currency::SDRN),
            878 => ::std::option::Option::Some(Currency::SEQ),
            879 => ::std::option::Option::Some(Currency::SFC),
            880 => ::std::option::Option::Some(Currency::SFE),
            881 => ::std::option::Option::Some(Currency::SH),
            882 => ::std::option::Option::Some(Currency::SHA),
            883 => ::std::option::Option::Some(Currency::SHDW),
            884 => ::std::option::Option::Some(Currency::SHELL),
            885 => ::std::option::Option::Some(Currency::SHIFT),
            886 => ::std::option::Option::Some(Currency::SHND),
            887 => ::std::option::Option::Some(Currency::SHORTY),
            888 => ::std::option::Option::Some(Currency::SIB),
            889 => ::std::option::Option::Some(Currency::SIC),
            890 => ::std::option::Option::Some(Currency::SIFT),
            891 => ::std::option::Option::Some(Currency::SIGMA),
            892 => ::std::option::Option::Some(Currency::SIGT),
            893 => ::std::option::Option::Some(Currency::SJCX),
            894 => ::std::option::Option::Some(Currency::SKC),
            895 => ::std::option::Option::Some(Currency::SKIN),
            896 => ::std::option::Option::Some(Currency::SKR),
            897 => ::std::option::Option::Some(Currency::SKULL),
            898 => ::std::option::Option::Some(Currency::SKY),
            899 => ::std::option::Option::Some(Currency::SLEVIN),
            900 => ::std::option::Option::Some(Currency::SLFI),
            901 => ::std::option::Option::Some(Currency::SLG),
            902 => ::std::option::Option::Some(Currency::SLING),
            903 => ::std::option::Option::Some(Currency::SLM),
            904 => ::std::option::Option::Some(Currency::SLR),
            905 => ::std::option::Option::Some(Currency::SLS),
            906 => ::std::option::Option::Some(Currency::SMART),
            907 => ::std::option::Option::Some(Currency::SMC),
            908 => ::std::option::Option::Some(Currency::SMLY),
            909 => ::std::option::Option::Some(Currency::SMOKE),
            910 => ::std::option::Option::Some(Currency::SNAKE),
            911 => ::std::option::Option::Some(Currency::SNC),
            912 => ::std::option::Option::Some(Currency::SND),
            913 => ::std::option::Option::Some(Currency::SNGLS),
            914 => ::std::option::Option::Some(Currency::SNM),
            915 => ::std::option::Option::Some(Currency::SNRG),
            916 => ::std::option::Option::Some(Currency::SNT),
            917 => ::std::option::Option::Some(Currency::SOAR),
            918 => ::std::option::Option::Some(Currency::SOCC),
            919 => ::std::option::Option::Some(Currency::SOIL),
            920 => ::std::option::Option::Some(Currency::SOJ),
            921 => ::std::option::Option::Some(Currency::SONG),
            922 => ::std::option::Option::Some(Currency::SOON),
            923 => ::std::option::Option::Some(Currency::SOUL),
            924 => ::std::option::Option::Some(Currency::SPACE),
            925 => ::std::option::Option::Some(Currency::SPEX),
            926 => ::std::option::Option::Some(Currency::SPHR),
            927 => ::std::option::Option::Some(Currency::SPORT),
            928 => ::std::option::Option::Some(Currency::SPR),
            929 => ::std::option::Option::Some(Currency::SPRTS),
            930 => ::std::option::Option::Some(Currency::SPT),
            931 => ::std::option::Option::Some(Currency::SRC),
            932 => ::std::option::Option::Some(Currency::STA),
            933 => ::std::option::Option::Some(Currency::START),
            934 => ::std::option::Option::Some(Currency::STCN),
            935 => ::std::option::Option::Some(Currency::STEEM),
            936 => ::std::option::Option::Some(Currency::STEPS),
            937 => ::std::option::Option::Some(Currency::STEX),
            938 => ::std::option::Option::Some(Currency::STORJ),
            939 => ::std::option::Option::Some(Currency::STRAT),
            940 => ::std::option::Option::Some(Currency::STRC),
            941 => ::std::option::Option::Some(Currency::STS),
            942 => ::std::option::Option::Some(Currency::STV),
            943 => ::std::option::Option::Some(Currency::STX),
            944 => ::std::option::Option::Some(Currency::SUB),
            945 => ::std::option::Option::Some(Currency::SUMO),
            946 => ::std::option::Option::Some(Currency::SUPER),
            947 => ::std::option::Option::Some(Currency::SUR),
            948 => ::std::option::Option::Some(Currency::SWIFT),
            949 => ::std::option::Option::Some(Currency::SWING),
            950 => ::std::option::Option::Some(Currency::SWP),
            951 => ::std::option::Option::Some(Currency::SWT),
            952 => ::std::option::Option::Some(Currency::SXC),
            953 => ::std::option::Option::Some(Currency::SYNC),
            954 => ::std::option::Option::Some(Currency::SYNX),
            955 => ::std::option::Option::Some(Currency::SYS),
            956 => ::std::option::Option::Some(Currency::TAAS),
            957 => ::std::option::Option::Some(Currency::TAG),
            958 => ::std::option::Option::Some(Currency::TAGR),
            959 => ::std::option::Option::Some(Currency::TAJ),
            960 => ::std::option::Option::Some(Currency::TALK),
            961 => ::std::option::Option::Some(Currency::TCC),
            962 => ::std::option::Option::Some(Currency::TCOIN),
            963 => ::std::option::Option::Some(Currency::TCR),
            964 => ::std::option::Option::Some(Currency::TEAM),
            965 => ::std::option::Option::Some(Currency::TEK),
            966 => ::std::option::Option::Some(Currency::TELL),
            967 => ::std::option::Option::Some(Currency::TER),
            968 => ::std::option::Option::Some(Currency::TERA),
            969 => ::std::option::Option::Some(Currency::TES),
            970 => ::std::option::Option::Some(Currency::TESLA),
            971 => ::std::option::Option::Some(Currency::TFL),
            972 => ::std::option::Option::Some(Currency::TGC),
            973 => ::std::option::Option::Some(Currency::TGT),
            974 => ::std::option::Option::Some(Currency::THC),
            975 => ::std::option::Option::Some(Currency::THS),
            976 => ::std::option::Option::Some(Currency::TIME),
            977 => ::std::option::Option::Some(Currency::TIPS),
            978 => ::std::option::Option::Some(Currency::TIT),
            979 => ::std::option::Option::Some(Currency::TKN),
            980 => ::std::option::Option::Some(Currency::TKR),
            981 => ::std::option::Option::Some(Currency::TKS),
            982 => ::std::option::Option::Some(Currency::TLE),
            983 => ::std::option::Option::Some(Currency::TNT),
            984 => ::std::option::Option::Some(Currency::TOA),
            985 => ::std::option::Option::Some(Currency::TODAY),
            986 => ::std::option::Option::Some(Currency::TOKEN),
            987 => ::std::option::Option::Some(Currency::TOP),
            988 => ::std::option::Option::Some(Currency::TOPAZ),
            989 => ::std::option::Option::Some(Currency::TOR),
            990 => ::std::option::Option::Some(Currency::TRADE),
            991 => ::std::option::Option::Some(Currency::TRC),
            992 => ::std::option::Option::Some(Currency::TRCT),
            993 => ::std::option::Option::Some(Currency::TRI),
            994 => ::std::option::Option::Some(Currency::TRICK),
            995 => ::std::option::Option::Some(Currency::TRIG),
            996 => ::std::option::Option::Some(Currency::TRK),
            997 => ::std::option::Option::Some(Currency::TROLL),
            998 => ::std::option::Option::Some(Currency::TRST),
            999 => ::std::option::Option::Some(Currency::TRUMP),
            1000 => ::std::option::Option::Some(Currency::TRUST),
            1001 => ::std::option::Option::Some(Currency::TRX),
            1002 => ::std::option::Option::Some(Currency::TSE),
            1003 => ::std::option::Option::Some(Currency::TSTR),
            1004 => ::std::option::Option::Some(Currency::TTC),
            1005 => ::std::option::Option::Some(Currency::TURBO),
            1006 => ::std::option::Option::Some(Currency::TX),
            1007 => ::std::option::Option::Some(Currency::TYC),
            1008 => ::std::option::Option::Some(Currency::TYCHO),
            1009 => ::std::option::Option::Some(Currency::TZC),
            1010 => ::std::option::Option::Some(Currency::UBQ),
            1011 => ::std::option::Option::Some(Currency::UET),
            1012 => ::std::option::Option::Some(Currency::UFO),
            1013 => ::std::option::Option::Some(Currency::UGT),
            1014 => ::std::option::Option::Some(Currency::UIS),
            1015 => ::std::option::Option::Some(Currency::ULA),
            1016 => ::std::option::Option::Some(Currency::UNB),
            1017 => ::std::option::Option::Some(Currency::UNC),
            1018 => ::std::option::Option::Some(Currency::UNI),
            1019 => ::std::option::Option::Some(Currency::UNIC),
            1020 => ::std::option::Option::Some(Currency::UNIFY),
            1021 => ::std::option::Option::Some(Currency::UNIT),
            1022 => ::std::option::Option::Some(Currency::UNITS),
            1023 => ::std::option::Option::Some(Currency::UNITY),
            1024 => ::std::option::Option::Some(Currency::UNO),
            1025 => ::std::option::Option::Some(Currency::UNRC),
            1026 => ::std::option::Option::Some(Currency::UNY),
            1027 => ::std::option::Option::Some(Currency::UR),
            1028 => ::std::option::Option::Some(Currency::URC),
            1029 => ::std::option::Option::Some(Currency::URO),
            1030 => ::std::option::Option::Some(Currency::USC),
            1031 => ::std::option::Option::Some(Currency::USDE),
            1032 => ::std::option::Option::Some(Currency::USDT),
            1033 => ::std::option::Option::Some(Currency::USNBT),
            1034 => ::std::option::Option::Some(Currency::UTA),
            1035 => ::std::option::Option::Some(Currency::UTC),
            1036 => ::std::option::Option::Some(Currency::V),
            1037 => ::std::option::Option::Some(Currency::VAL),
            1038 => ::std::option::Option::Some(Currency::VASH),
            1039 => ::std::option::Option::Some(Currency::VC),
            1040 => ::std::option::Option::Some(Currency::VEC2),
            1041 => ::std::option::Option::Some(Currency::VEN),
            1042 => ::std::option::Option::Some(Currency::VERI),
            1043 => ::std::option::Option::Some(Currency::VGC),
            1044 => ::std::option::Option::Some(Currency::VIA),
            1045 => ::std::option::Option::Some(Currency::VIB),
            1046 => ::std::option::Option::Some(Currency::VIBE),
            1047 => ::std::option::Option::Some(Currency::VIDZ),
            1048 => ::std::option::Option::Some(Currency::VIP),
            1049 => ::std::option::Option::Some(Currency::VISIO),
            1050 => ::std::option::Option::Some(Currency::VIVO),
            1051 => ::std::option::Option::Some(Currency::VLT),
            1052 => ::std::option::Option::Some(Currency::VLTC),
            1053 => ::std::option::Option::Some(Currency::VOISE),
            1054 => ::std::option::Option::Some(Currency::VOLT),
            1055 => ::std::option::Option::Some(Currency::VOX),
            1056 => ::std::option::Option::Some(Currency::VOYA),
            1057 => ::std::option::Option::Some(Currency::VPRC),
            1058 => ::std::option::Option::Some(Currency::VRC),
            1059 => ::std::option::Option::Some(Currency::VRM),
            1060 => ::std::option::Option::Some(Currency::VRS),
            1061 => ::std::option::Option::Some(Currency::VSL),
            1062 => ::std::option::Option::Some(Currency::VSX),
            1063 => ::std::option::Option::Some(Currency::VTA),
            1064 => ::std::option::Option::Some(Currency::VTC),
            1065 => ::std::option::Option::Some(Currency::VTR),
            1066 => ::std::option::Option::Some(Currency::VUC),
            1067 => ::std::option::Option::Some(Currency::VULC),
            1068 => ::std::option::Option::Some(Currency::WA),
            1069 => ::std::option::Option::Some(Currency::WARP),
            1070 => ::std::option::Option::Some(Currency::WAVES),
            1071 => ::std::option::Option::Some(Currency::WAY),
            1072 => ::std::option::Option::Some(Currency::WBB),
            1073 => ::std::option::Option::Some(Currency::WBC),
            1074 => ::std::option::Option::Some(Currency::WCT),
            1075 => ::std::option::Option::Some(Currency::WDC),
            1076 => ::std::option::Option::Some(Currency::WEC),
            1077 => ::std::option::Option::Some(Currency::WEX),
            1078 => ::std::option::Option::Some(Currency::WGO),
            1079 => ::std::option::Option::Some(Currency::WGR),
            1080 => ::std::option::Option::Some(Currency::WHL),
            1081 => ::std::option::Option::Some(Currency::WIC),
            1082 => ::std::option::Option::Some(Currency::WILD),
            1083 => ::std::option::Option::Some(Currency::WINGS),
            1084 => ::std::option::Option::Some(Currency::WINK),
            1085 => ::std::option::Option::Some(Currency::WMC),
            1086 => ::std::option::Option::Some(Currency::WOMEN),
            1087 => ::std::option::Option::Some(Currency::WORM),
            1088 => ::std::option::Option::Some(Currency::WOW),
            1089 => ::std::option::Option::Some(Currency::WSX),
            1090 => ::std::option::Option::Some(Currency::WTC),
            1091 => ::std::option::Option::Some(Currency::WTT),
            1092 => ::std::option::Option::Some(Currency::WYV),
            1093 => ::std::option::Option::Some(Currency::X2),
            1094 => ::std::option::Option::Some(Currency::XAS),
            1095 => ::std::option::Option::Some(Currency::XAU),
            1096 => ::std::option::Option::Some(Currency::XAUR),
            1097 => ::std::option::Option::Some(Currency::XBC),
            1098 => ::std::option::Option::Some(Currency::XBG),
            1099 => ::std::option::Option::Some(Currency::XBL),
            1100 => ::std::option::Option::Some(Currency::XBTC21),
            1101 => ::std::option::Option::Some(Currency::XBTS),
            1102 => ::std::option::Option::Some(Currency::XBY),
            1103 => ::std::option::Option::Some(Currency::XC),
            1104 => ::std::option::Option::Some(Currency::XCN),
            1105 => ::std::option::Option::Some(Currency::XCO),
            1106 => ::std::option::Option::Some(Currency::XCP),
            1107 => ::std::option::Option::Some(Currency::XCRE),
            1108 => ::std::option::Option::Some(Currency::XCS),
            1109 => ::std::option::Option::Some(Currency::XCT),
            1110 => ::std::option::Option::Some(Currency::XCXT),
            1111 => ::std::option::Option::Some(Currency::XDE2),
            1112 => ::std::option::Option::Some(Currency::XDN),
            1113 => ::std::option::Option::Some(Currency::XEL),
            1114 => ::std::option::Option::Some(Currency::XEM),
            1115 => ::std::option::Option::Some(Currency::XFT),
            1116 => ::std::option::Option::Some(Currency::XGOX),
            1117 => ::std::option::Option::Some(Currency::XGR),
            1118 => ::std::option::Option::Some(Currency::XHI),
            1119 => ::std::option::Option::Some(Currency::XIN),
            1120 => ::std::option::Option::Some(Currency::XIOS),
            1121 => ::std::option::Option::Some(Currency::XJO),
            1122 => ::std::option::Option::Some(Currency::XLC),
            1123 => ::std::option::Option::Some(Currency::XLM),
            1124 => ::std::option::Option::Some(Currency::XLR),
            1125 => ::std::option::Option::Some(Currency::XMCC),
            1126 => ::std::option::Option::Some(Currency::XMG),
            1127 => ::std::option::Option::Some(Currency::XMR),
            1128 => ::std::option::Option::Some(Currency::XMY),
            1129 => ::std::option::Option::Some(Currency::XNG),
            1130 => ::std::option::Option::Some(Currency::XNN),
            1131 => ::std::option::Option::Some(Currency::XOC),
            1132 => ::std::option::Option::Some(Currency::XOT),
            1133 => ::std::option::Option::Some(Currency::XP),
            1134 => ::std::option::Option::Some(Currency::XPA),
            1135 => ::std::option::Option::Some(Currency::XPD),
            1136 => ::std::option::Option::Some(Currency::XPM),
            1137 => ::std::option::Option::Some(Currency::XPTX),
            1138 => ::std::option::Option::Some(Currency::XPY),
            1139 => ::std::option::Option::Some(Currency::XQN),
            1140 => ::std::option::Option::Some(Currency::XRA),
            1141 => ::std::option::Option::Some(Currency::XRB),
            1142 => ::std::option::Option::Some(Currency::XRC),
            1143 => ::std::option::Option::Some(Currency::XRE),
            1144 => ::std::option::Option::Some(Currency::XRL),
            1145 => ::std::option::Option::Some(Currency::XRY),
            1146 => ::std::option::Option::Some(Currency::XSPEC),
            1147 => ::std::option::Option::Some(Currency::XST),
            1148 => ::std::option::Option::Some(Currency::XSTC),
            1149 => ::std::option::Option::Some(Currency::XTC),
            1150 => ::std::option::Option::Some(Currency::XTD),
            1151 => ::std::option::Option::Some(Currency::XTO),
            1152 => ::std::option::Option::Some(Currency::XTZ),
            1153 => ::std::option::Option::Some(Currency::XUC),
            1154 => ::std::option::Option::Some(Currency::XVC),
            1155 => ::std::option::Option::Some(Currency::XVE),
            1156 => ::std::option::Option::Some(Currency::XVG),
            1157 => ::std::option::Option::Some(Currency::XVP),
            1158 => ::std::option::Option::Some(Currency::XWC),
            1159 => ::std::option::Option::Some(Currency::XZC),
            1160 => ::std::option::Option::Some(Currency::YAC),
            1161 => ::std::option::Option::Some(Currency::YASH),
            1162 => ::std::option::Option::Some(Currency::YES),
            1163 => ::std::option::Option::Some(Currency::YOC),
            1164 => ::std::option::Option::Some(Currency::YOYOW),
            1165 => ::std::option::Option::Some(Currency::ZBC),
            1166 => ::std::option::Option::Some(Currency::ZCC),
            1167 => ::std::option::Option::Some(Currency::ZCL),
            1168 => ::std::option::Option::Some(Currency::ZEIT),
            1169 => ::std::option::Option::Some(Currency::ZEN),
            1170 => ::std::option::Option::Some(Currency::ZENGOLD),
            1171 => ::std::option::Option::Some(Currency::ZENI),
            1172 => ::std::option::Option::Some(Currency::ZER),
            1173 => ::std::option::Option::Some(Currency::ZET),
            1174 => ::std::option::Option::Some(Currency::ZMC),
            1175 => ::std::option::Option::Some(Currency::ZNE),
            1176 => ::std::option::Option::Some(Currency::ZNY),
            1177 => ::std::option::Option::Some(Currency::ZOI),
            1178 => ::std::option::Option::Some(Currency::ZRC),
            1179 => ::std::option::Option::Some(Currency::ZRX),
            1180 => ::std::option::Option::Some(Currency::ZSC),
            1181 => ::std::option::Option::Some(Currency::ZSE),
            1182 => ::std::option::Option::Some(Currency::ZUR),
            1183 => ::std::option::Option::Some(Currency::ZYD),
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
            Currency::ATM,
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
            Currency::EBST,
            Currency::EBT,
            Currency::EBTC,
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
            Currency::EGC,
            Currency::EGG,
            Currency::EGO,
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
            Currency::QWARK,
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
            Currency::RMC,
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
            Currency::XBG,
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
    \n\x18orca/core/currency.proto\x12\x04orca\"X\n\x0cCurrencyPair\x12$\n\
    \x05quote\x18\x01\x20\x01(\x0e2\x0e.orca.CurrencyR\x05quote\x12\"\n\x04b\
    ase\x18\x02\x20\x01(\x0e2\x0e.orca.CurrencyR\x04base\"T\n\x0eCurrencyVol\
    ume\x12*\n\x08currency\x18\x01\x20\x01(\x0e2\x0e.orca.CurrencyR\x08curre\
    ncy\x12\x16\n\x06amount\x18\x02\x20\x01(\x04R\x06amount*\xc5`\n\x08Curre\
    ncy\x12\x08\n\x04NONE\x10\0\x12\x07\n\x03EUR\x10\x01\x12\x07\n\x03USD\
    \x10\x02\x12\x07\n\x03BTC\x10\x03\x12\x07\n\x03ETH\x10\x04\x12\x07\n\x03\
    ETC\x10\x05\x12\x07\n\x03XRP\x10\x06\x12\x07\n\x03ZEC\x10\x07\x12\x07\n\
    \x03BCH\x10\x08\x12\x07\n\x03LTC\x10\t\x12\x07\n\x03ABC\x10\n\x12\x07\n\
    \x03ABN\x10\x0b\x12\x07\n\x03ABY\x10\x0c\x12\x06\n\x02AC\x10\r\x12\x07\n\
    \x03ACC\x10\x0e\x12\x08\n\x04ACES\x10\x0f\x12\x07\n\x03ACN\x10\x10\x12\t\
    \n\x05ACOIN\x10\x11\x12\x07\n\x03ACP\x10\x12\x12\x07\n\x03ACT\x10\x13\
    \x12\x07\n\x03ADA\x10\x14\x12\x07\n\x03ADC\x10\x15\x12\x08\n\x04ADCN\x10\
    \x16\x12\x07\n\x03ADK\x10\x17\x12\x07\n\x03ADL\x10\x18\x12\x08\n\x04ADST\
    \x10\x19\x12\x07\n\x03ADT\x10\x1a\x12\x07\n\x03ADX\x10\x1b\x12\x07\n\x03\
    ADZ\x10\x1c\x12\x06\n\x02AE\x10\x1d\x12\x08\n\x04AEON\x10\x1e\x12\x08\n\
    \x04AGLC\x10\x1f\x12\x08\n\x04AGRS\x10\x20\x12\x07\n\x03AHT\x10!\x12\x07\
    \n\x03AIB\x10\"\x12\x08\n\x04AION\x10#\x12\x07\n\x03AIR\x10$\x12\x07\n\
    \x03AKY\x10%\x12\x08\n\x04ALIS\x10&\x12\x07\n\x03ALL\x10'\x12\x07\n\x03A\
    LT\x10(\x12\x08\n\x04ALTC\x10)\x12\x07\n\x03AMB\x10*\x12\t\n\x05AMBER\
    \x10+\x12\x08\n\x04AMIS\x10,\x12\x08\n\x04AMMO\x10-\x12\x07\n\x03AMP\x10\
    .\x12\x07\n\x03AMS\x10/\x12\x07\n\x03ANC\x100\x12\x07\n\x03ANI\x101\x12\
    \x07\n\x03ANT\x102\x12\x08\n\x04ANTI\x103\x12\x08\n\x04ANTX\x104\x12\x07\
    \n\x03APC\x105\x12\x07\n\x03APW\x106\x12\x07\n\x03APX\x107\x12\x07\n\x03\
    ARB\x108\x12\x08\n\x04ARCO\x109\x12\x08\n\x04ARDR\x10:\x12\x07\n\x03ARG\
    \x10;\x12\t\n\x05ARGUS\x10<\x12\x07\n\x03ARI\x10=\x12\x07\n\x03ARK\x10>\
    \x12\x07\n\x03ART\x10?\x12\n\n\x06ASAFE2\x10@\x12\x07\n\x03ASC\x10A\x12\
    \x07\n\x03ASN\x10B\x12\x07\n\x03AST\x10C\x12\x07\n\x03ATB\x10D\x12\x08\n\
    \x04ATCC\x10E\x12\x07\n\x03ATM\x10F\x12\x08\n\x04ATMS\x10G\x12\x08\n\x04\
    ATOM\x10H\x12\x07\n\x03ATS\x10I\x12\x07\n\x03ATX\x10J\x12\x06\n\x02AU\
    \x10K\x12\x07\n\x03AUR\x10L\x12\x08\n\x04AURS\x10M\x12\x06\n\x02AV\x10N\
    \x12\x07\n\x03AVT\x10O\x12\x07\n\x03AXF\x10P\x12\t\n\x05AXIOM\x10Q\x12\
    \x07\n\x03B2X\x10R\x12\x06\n\x02B3\x10S\x12\x07\n\x03BAC\x10T\x12\x07\n\
    \x03BAS\x10U\x12\x08\n\x04BASH\x10V\x12\x07\n\x03BAY\x10W\x12\x07\n\x03B\
    BC\x10X\x12\x07\n\x03BBP\x10Y\x12\x08\n\x04BCAP\x10Z\x12\x07\n\x03BCC\
    \x10[\x12\x08\n\x04BCCS\x10\\\x12\x07\n\x03BCF\x10]\x12\x07\n\x03BCN\x10\
    ^\x12\x07\n\x03BCO\x10_\x12\x08\n\x04BCPT\x10`\x12\x07\n\x03BCY\x10a\x12\
    \x07\n\x03BDL\x10b\x12\x08\n\x04BELA\x10c\x12\t\n\x05BENJI\x10d\x12\x08\
    \n\x04BERN\x10e\x12\x08\n\x04BEST\x10f\x12\x07\n\x03BGR\x10g\x12\t\n\x05\
    BIGUP\x10h\x12\x08\n\x04BIOB\x10i\x12\x08\n\x04BIOS\x10j\x12\x07\n\x03BI\
    P\x10k\x12\t\n\x05BIRDS\x10l\x12\x07\n\x03BIS\x10m\x12\x07\n\x03BIT\x10n\
    \x12\x08\n\x04BITB\x10o\x12\n\n\x06BITBTC\x10p\x12\t\n\x05BITCF\x10q\x12\
    \n\n\x06BITCNY\x10r\x12\n\n\x06BITEUR\x10s\x12\x0b\n\x07BITGOLD\x10t\x12\
    \t\n\x05BITOK\x10u\x12\x08\n\x04BITS\x10v\x12\r\n\tBITSILVER\x10w\x12\n\
    \n\x06BITUSD\x10x\x12\x08\n\x04BITZ\x10y\x12\x08\n\x04BLAS\x10z\x12\t\n\
    \x05BLAZR\x10{\x12\x07\n\x03BLC\x10|\x12\t\n\x05BLITZ\x10}\x12\x07\n\x03\
    BLK\x10~\x12\x07\n\x03BLN\x10\x7f\x12\n\n\x05BLOCK\x10\x80\x01\x12\r\n\
    \x08BLOCKPAY\x10\x81\x01\x12\t\n\x04BLRY\x10\x82\x01\x12\x08\n\x03BLU\
    \x10\x83\x01\x12\t\n\x04BLUE\x10\x84\x01\x12\x08\n\x03BLX\x10\x85\x01\
    \x12\x08\n\x03BLZ\x10\x86\x01\x12\x08\n\x03BMC\x10\x87\x01\x12\x08\n\x03\
    BNB\x10\x88\x01\x12\x08\n\x03BNT\x10\x89\x01\x12\x08\n\x03BNX\x10\x8a\
    \x01\x12\t\n\x04BOAT\x10\x8b\x01\x12\t\n\x04BOLI\x10\x8c\x01\x12\x08\n\
    \x03BOS\x10\x8d\x01\x12\t\n\x04BOST\x10\x8e\x01\x12\x08\n\x03BPC\x10\x8f\
    \x01\x12\x07\n\x02BQ\x10\x90\x01\x12\x08\n\x03BQC\x10\x91\x01\x12\x08\n\
    \x03BQX\x10\x92\x01\x12\n\n\x05BRAIN\x10\x93\x01\x12\t\n\x04BRAT\x10\x94\
    \x01\x12\t\n\x04BRIA\x10\x95\x01\x12\t\n\x04BRIT\x10\x96\x01\x12\x08\n\
    \x03BRK\x10\x97\x01\x12\x08\n\x03BRO\x10\x98\x01\x12\x08\n\x03BRX\x10\
    \x99\x01\x12\x08\n\x03BSC\x10\x9a\x01\x12\x08\n\x03BSD\x10\x9b\x01\x12\
    \x08\n\x03BSN\x10\x9c\x01\x12\x08\n\x03BSR\x10\x9d\x01\x12\n\n\x05BSTAR\
    \x10\x9e\x01\x12\t\n\x04BSTY\x10\x9f\x01\x12\x08\n\x03BT1\x10\xa0\x01\
    \x12\x08\n\x03BT2\x10\xa1\x01\x12\x08\n\x03BTA\x10\xa2\x01\x12\x08\n\x03\
    BTB\x10\xa3\x01\x12\t\n\x04BTCD\x10\xa4\x01\x12\t\n\x04BTCM\x10\xa5\x01\
    \x12\t\n\x04BTCR\x10\xa6\x01\x12\x0b\n\x06BTCRED\x10\xa7\x01\x12\t\n\x04\
    BTCZ\x10\xa8\x01\x12\t\n\x04BTDX\x10\xa9\x01\x12\t\n\x04BTPL\x10\xaa\x01\
    \x12\x08\n\x03BTQ\x10\xab\x01\x12\x08\n\x03BTS\x10\xac\x01\x12\t\n\x04BT\
    SR\x10\xad\x01\x12\x08\n\x03BTU\x10\xae\x01\x12\n\n\x05BTWTY\x10\xaf\x01\
    \x12\x08\n\x03BTX\x10\xb0\x01\x12\x08\n\x03BUB\x10\xb1\x01\x12\n\n\x05BU\
    CKS\x10\xb2\x01\x12\n\n\x05BUMBA\x10\xb3\x01\x12\x08\n\x03BUN\x10\xb4\
    \x01\x12\n\n\x05BURST\x10\xb5\x01\x12\t\n\x04BUZZ\x10\xb6\x01\x12\x08\n\
    \x03BVC\x10\xb7\x01\x12\x08\n\x03BXC\x10\xb8\x01\x12\x08\n\x03BXT\x10\
    \xb9\x01\x12\x08\n\x03BYC\x10\xba\x01\x12\x07\n\x02C2\x10\xbb\x01\x12\
    \x08\n\x03CAB\x10\xbc\x01\x12\t\n\x04CACH\x10\xbd\x01\x12\x0e\n\tCADASTR\
    AL\x10\xbe\x01\x12\x08\n\x03CAG\x10\xbf\x01\x12\t\n\x04CAGE\x10\xc0\x01\
    \x12\t\n\x04CALC\x10\xc1\x01\x12\t\n\x04CANN\x10\xc2\x01\x12\x08\n\x03CA\
    P\x10\xc3\x01\x12\x0b\n\x06CARBON\x10\xc4\x01\x12\x0b\n\x06CASINO\x10\
    \xc5\x01\x12\x08\n\x03CBD\x10\xc6\x01\x12\x08\n\x03CBX\x10\xc7\x01\x12\
    \x07\n\x02CC\x10\xc8\x01\x12\x0b\n\x06CCM100\x10\xc9\x01\x12\x08\n\x03CC\
    N\x10\xca\x01\x12\t\n\x04CCRB\x10\xcb\x01\x12\x08\n\x03CCT\x10\xcc\x01\
    \x12\x08\n\x03CDN\x10\xcd\x01\x12\x08\n\x03CDT\x10\xce\x01\x12\t\n\x04CE\
    SC\x10\xcf\x01\x12\x07\n\x02CF\x10\xd0\x01\x12\x08\n\x03CFI\x10\xd1\x01\
    \x12\x08\n\x03CFT\x10\xd2\x01\x12\x08\n\x03CHC\x10\xd3\x01\x12\n\n\x05CH\
    EAP\x10\xd4\x01\x12\n\n\x05CHESS\x10\xd5\x01\x12\n\n\x05CHIPS\x10\xd6\
    \x01\x12\x07\n\x02CJ\x10\xd7\x01\x12\t\n\x04CLAM\x10\xd8\x01\x12\n\n\x05\
    CLINT\x10\xd9\x01\x12\n\n\x05CLOAK\x10\xda\x01\x12\t\n\x04CLUB\x10\xdb\
    \x01\x12\x08\n\x03CME\x10\xdc\x01\x12\x08\n\x03CMP\x10\xdd\x01\x12\n\n\
    \x05CMPCO\x10\xde\x01\x12\x08\n\x03CMT\x10\xdf\x01\x12\x08\n\x03CNC\x10\
    \xe0\x01\x12\x08\n\x03CND\x10\xe1\x01\x12\t\n\x04CNNC\x10\xe2\x01\x12\
    \x08\n\x03CNO\x10\xe3\x01\x12\x08\n\x03CNT\x10\xe4\x01\x12\x08\n\x03CNX\
    \x10\xe5\x01\x12\t\n\x04COAL\x10\xe6\x01\x12\x08\n\x03COB\x10\xe7\x01\
    \x12\t\n\x04COLX\x10\xe8\x01\x12\x08\n\x03CON\x10\xe9\x01\x12\t\n\x04CON\
    X\x10\xea\x01\x12\x08\n\x03COR\x10\xeb\x01\x12\t\n\x04CORG\x10\xec\x01\
    \x12\t\n\x04COSS\x10\xed\x01\x12\n\n\x05COUPE\x10\xee\x01\x12\n\n\x05COV\
    AL\x10\xef\x01\x12\n\n\x05COXST\x10\xf0\x01\x12\x08\n\x03CPC\x10\xf1\x01\
    \x12\x08\n\x03CPN\x10\xf2\x01\x12\n\n\x05CRAVE\x10\xf3\x01\x12\x08\n\x03\
    CRB\x10\xf4\x01\x12\t\n\x04CREA\x10\xf5\x01\x12\n\n\x05CREDO\x10\xf6\x01\
    \x12\n\n\x05CREVA\x10\xf7\x01\x12\x08\n\x03CRM\x10\xf8\x01\x12\x08\n\x03\
    CRT\x10\xf9\x01\x12\x08\n\x03CRW\x10\xfa\x01\x12\x08\n\x03CRX\x10\xfb\
    \x01\x12\n\n\x05CRYPT\x10\xfc\x01\x12\x08\n\x03CSC\x10\xfd\x01\x12\t\n\
    \x04CSNO\x10\xfe\x01\x12\n\n\x05CTIC2\x10\xff\x01\x12\n\n\x05CTIC3\x10\
    \x80\x02\x12\x08\n\x03CTO\x10\x81\x02\x12\x08\n\x03CTR\x10\x82\x02\x12\t\
    \n\x04CUBE\x10\x83\x02\x12\t\n\x04CURE\x10\x84\x02\x12\x08\n\x03CV2\x10\
    \x85\x02\x12\x08\n\x03CVC\x10\x86\x02\x12\x0b\n\x06CVCOIN\x10\x87\x02\
    \x12\t\n\x04CWXT\x10\x88\x02\x12\x08\n\x03CXT\x10\x89\x02\x12\x08\n\x03C\
    YC\x10\x8a\x02\x12\n\n\x05CYDER\x10\x8b\x02\x12\x08\n\x03CYP\x10\x8c\x02\
    \x12\t\n\x04DALC\x10\x8d\x02\x12\x08\n\x03DAR\x10\x8e\x02\x12\x08\n\x03D\
    AS\x10\x8f\x02\x12\t\n\x04DASH\x10\x90\x02\x12\n\n\x05DASHS\x10\x91\x02\
    \x12\t\n\x04DAXX\x10\x92\x02\x12\x08\n\x03DAY\x10\x93\x02\x12\x08\n\x03D\
    BG\x10\x94\x02\x12\t\n\x04DBIX\x10\x95\x02\x12\t\n\x04DBTC\x10\x96\x02\
    \x12\x08\n\x03DCN\x10\x97\x02\x12\x08\n\x03DCR\x10\x98\x02\x12\t\n\x04DC\
    RE\x10\x99\x02\x12\x08\n\x03DCT\x10\x9a\x02\x12\x08\n\x03DCY\x10\x9b\x02\
    \x12\x08\n\x03DDF\x10\x9c\x02\x12\x08\n\x03DEM\x10\x9d\x02\x12\t\n\x04DE\
    NT\x10\x9e\x02\x12\x08\n\x03DES\x10\x9f\x02\x12\t\n\x04DEUS\x10\xa0\x02\
    \x12\x08\n\x03DFS\x10\xa1\x02\x12\x08\n\x03DFT\x10\xa2\x02\x12\x08\n\x03\
    DGB\x10\xa3\x02\x12\x08\n\x03DGC\x10\xa4\x02\x12\t\n\x04DGCS\x10\xa5\x02\
    \x12\x08\n\x03DGD\x10\xa6\x02\x12\t\n\x04DIBC\x10\xa7\x02\x12\t\n\x04DIC\
    E\x10\xa8\x02\x12\t\n\x04DIME\x10\xa9\x02\x12\t\n\x04DISK\x10\xaa\x02\
    \x12\x08\n\x03DIX\x10\xab\x02\x12\x08\n\x03DLC\x10\xac\x02\x12\n\n\x05DL\
    ISK\x10\xad\x02\x12\x08\n\x03DLT\x10\xae\x02\x12\x08\n\x03DMB\x10\xaf\
    \x02\x12\x08\n\x03DMC\x10\xb0\x02\x12\x08\n\x03DMD\x10\xb1\x02\x12\x08\n\
    \x03DNR\x10\xb2\x02\x12\x08\n\x03DNT\x10\xb3\x02\x12\t\n\x04DOGE\x10\xb4\
    \x02\x12\x0b\n\x06DOLLAR\x10\xb5\x02\x12\x08\n\x03DON\x10\xb6\x02\x12\t\
    \n\x04DOPE\x10\xb7\x02\x12\x08\n\x03DOT\x10\xb8\x02\x12\t\n\x04DOVU\x10\
    \xb9\x02\x12\x07\n\x02DP\x10\xba\x02\x12\t\n\x04DPAY\x10\xbb\x02\x12\n\n\
    \x05DRACO\x10\xbc\x02\x12\x08\n\x03DRM\x10\xbd\x02\x12\x08\n\x03DRS\x10\
    \xbe\x02\x12\x08\n\x03DRT\x10\xbf\x02\x12\n\n\x05DRXNE\x10\xc0\x02\x12\
    \x08\n\x03DSH\x10\xc1\x02\x12\x08\n\x03DTB\x10\xc2\x02\x12\x08\n\x03DUB\
    \x10\xc3\x02\x12\x08\n\x03DUO\x10\xc4\x02\x12\n\n\x05DUTCH\x10\xc5\x02\
    \x12\x08\n\x03DVC\x10\xc6\x02\x12\x08\n\x03DYN\x10\xc7\x02\x12\n\n\x05E4\
    ROW\x10\xc8\x02\x12\x08\n\x03EAC\x10\xc9\x02\x12\t\n\x04EBET\x10\xca\x02\
    \x12\t\n\x04EBST\x10\xcb\x02\x12\x08\n\x03EBT\x10\xcc\x02\x12\t\n\x04EBT\
    C\x10\xcd\x02\x12\x08\n\x03ECA\x10\xce\x02\x12\n\n\x05ECASH\x10\xcf\x02\
    \x12\x08\n\x03ECC\x10\xd0\x02\x12\x08\n\x03ECN\x10\xd1\x02\x12\x08\n\x03\
    ECO\x10\xd2\x02\x12\t\n\x04ECOB\x10\xd3\x02\x12\x08\n\x03EDG\x10\xd4\x02\
    \x12\x08\n\x03EDO\x10\xd5\x02\x12\n\n\x05EDOGE\x10\xd6\x02\x12\x08\n\x03\
    EDR\x10\xd7\x02\x12\t\n\x04EDRC\x10\xd8\x02\x12\x08\n\x03EFL\x10\xd9\x02\
    \x12\t\n\x04EFYT\x10\xda\x02\x12\x08\n\x03EGC\x10\xdb\x02\x12\x08\n\x03E\
    GG\x10\xdc\x02\x12\x08\n\x03EGO\x10\xdd\x02\x12\x07\n\x02EL\x10\xde\x02\
    \x12\x08\n\x03ELC\x10\xdf\x02\x12\x08\n\x03ELE\x10\xe0\x02\x12\n\n\x05EL\
    ITE\x10\xe1\x02\x12\t\n\x04ELIX\x10\xe2\x02\x12\t\n\x04ELLA\x10\xe3\x02\
    \x12\x08\n\x03ELS\x10\xe4\x02\x12\n\n\x05ELTC2\x10\xe5\x02\x12\x08\n\x03\
    EMB\x10\xe6\x02\x12\x08\n\x03EMC\x10\xe7\x02\x12\t\n\x04EMC2\x10\xe8\x02\
    \x12\x08\n\x03EMD\x10\xe9\x02\x12\x08\n\x03EMP\x10\xea\x02\x12\x08\n\x03\
    EMV\x10\xeb\x02\x12\x08\n\x03ENG\x10\xec\x02\x12\t\n\x04ENRG\x10\xed\x02\
    \x12\x08\n\x03ENT\x10\xee\x02\x12\x08\n\x03ENV\x10\xef\x02\x12\x08\n\x03\
    EOS\x10\xf0\x02\x12\x08\n\x03EOT\x10\xf1\x02\x12\x08\n\x03EQT\x10\xf2\
    \x02\x12\x08\n\x03ERC\x10\xf3\x02\x12\n\n\x05EREAL\x10\xf4\x02\x12\x08\n\
    \x03ERY\x10\xf5\x02\x12\x08\n\x03ESP\x10\xf6\x02\x12\t\n\x04ETBS\x10\xf7\
    \x02\x12\x08\n\x03ETG\x10\xf8\x02\x12\t\n\x04ETHD\x10\xf9\x02\x12\x08\n\
    \x03ETP\x10\xfa\x02\x12\x08\n\x03ETX\x10\xfb\x02\x12\x08\n\x03EUC\x10\
    \xfc\x02\x12\t\n\x04EUSD\x10\xfd\x02\x12\t\n\x04EVIL\x10\xfe\x02\x12\x08\
    \n\x03EVO\x10\xff\x02\x12\x08\n\x03EVR\x10\x80\x03\x12\x08\n\x03EVX\x10\
    \x81\x03\x12\t\n\x04EXCL\x10\x82\x03\x12\x08\n\x03EXL\x10\x83\x03\x12\
    \x08\n\x03EXN\x10\x84\x03\x12\x08\n\x03EXP\x10\x85\x03\x12\t\n\x04EXRN\
    \x10\x86\x03\x12\t\n\x04FAIR\x10\x87\x03\x12\x08\n\x03FAL\x10\x88\x03\
    \x12\x08\n\x03FAP\x10\x89\x03\x12\t\n\x04FAZZ\x10\x8a\x03\x12\x07\n\x02F\
    C\x10\x8b\x03\x12\x08\n\x03FC2\x10\x8c\x03\x12\x08\n\x03FCN\x10\x8d\x03\
    \x12\x08\n\x03FCT\x10\x8e\x03\x12\x08\n\x03FDC\x10\x8f\x03\x12\x08\n\x03\
    FFC\x10\x90\x03\x12\x08\n\x03FID\x10\x91\x03\x12\t\n\x04FIMK\x10\x92\x03\
    \x12\t\n\x04FIRE\x10\x93\x03\x12\x08\n\x03FJC\x10\x94\x03\x12\t\n\x04FLA\
    P\x10\x95\x03\x12\n\n\x05FLASH\x10\x96\x03\x12\t\n\x04FLAX\x10\x97\x03\
    \x12\t\n\x04FLDC\x10\x98\x03\x12\t\n\x04FLIK\x10\x99\x03\x12\x08\n\x03FL\
    O\x10\x9a\x03\x12\x08\n\x03FLT\x10\x9b\x03\x12\t\n\x04FLVR\x10\x9c\x03\
    \x12\x08\n\x03FLY\x10\x9d\x03\x12\x08\n\x03FNC\x10\x9e\x03\x12\t\n\x04FO\
    NZ\x10\x9f\x03\x12\t\n\x04FRAZ\x10\xa0\x03\x12\x08\n\x03FRC\x10\xa1\x03\
    \x12\t\n\x04FRGC\x10\xa2\x03\x12\x08\n\x03FRK\x10\xa3\x03\x12\x08\n\x03F\
    RN\x10\xa4\x03\x12\t\n\x04FRST\x10\xa5\x03\x12\t\n\x04FRWC\x10\xa6\x03\
    \x12\x08\n\x03FST\x10\xa7\x03\x12\x08\n\x03FTC\x10\xa8\x03\x12\t\n\x04FU\
    CK\x10\xa9\x03\x12\t\n\x04FUEL\x10\xaa\x03\x12\x08\n\x03FUN\x10\xab\x03\
    \x12\t\n\x04FUNC\x10\xac\x03\x12\t\n\x04FUNK\x10\xad\x03\x12\t\n\x04FUTC\
    \x10\xae\x03\x12\t\n\x04FUZZ\x10\xaf\x03\x12\x08\n\x03FXE\x10\xb0\x03\
    \x12\x08\n\x03FYN\x10\xb1\x03\x12\x08\n\x03G3N\x10\xb2\x03\x12\t\n\x04GA\
    IA\x10\xb3\x03\x12\t\n\x04GAIN\x10\xb4\x03\x12\x08\n\x03GAM\x10\xb5\x03\
    \x12\t\n\x04GAME\x10\xb6\x03\x12\x08\n\x03GAP\x10\xb7\x03\x12\t\n\x04GAR\
    Y\x10\xb8\x03\x12\x08\n\x03GAS\x10\xb9\x03\x12\x08\n\x03GAY\x10\xba\x03\
    \x12\x07\n\x02GB\x10\xbb\x03\x12\x08\n\x03GBC\x10\xbc\x03\x12\x08\n\x03G\
    BG\x10\xbd\x03\x12\t\n\x04GBRC\x10\xbe\x03\x12\x08\n\x03GBT\x10\xbf\x03\
    \x12\n\n\x05GBYTE\x10\xc0\x03\x12\x08\n\x03GCN\x10\xc1\x03\x12\x08\n\x03\
    GCR\x10\xc2\x03\x12\n\n\x05GEERT\x10\xc3\x03\x12\x08\n\x03GEO\x10\xc4\
    \x03\x12\x08\n\x03GLC\x10\xc5\x03\x12\x08\n\x03GLD\x10\xc6\x03\x12\x08\n\
    \x03GLT\x10\xc7\x03\x12\x08\n\x03GML\x10\xc8\x03\x12\x08\n\x03GMT\x10\
    \xc9\x03\x12\x08\n\x03GMX\x10\xca\x03\x12\x08\n\x03GNO\x10\xcb\x03\x12\
    \x08\n\x03GNT\x10\xcc\x03\x12\t\n\x04GOLF\x10\xcd\x03\x12\n\n\x05GOLOS\
    \x10\xce\x03\x12\t\n\x04GOOD\x10\xcf\x03\x12\x07\n\x02GP\x10\xd0\x03\x12\
    \x08\n\x03GPL\x10\xd1\x03\x12\x08\n\x03GPU\x10\xd2\x03\x12\x08\n\x03GRC\
    \x10\xd3\x03\x12\x08\n\x03GRE\x10\xd4\x03\x12\x08\n\x03GRN\x10\xd5\x03\
    \x12\x08\n\x03GRS\x10\xd6\x03\x12\x08\n\x03GRT\x10\xd7\x03\x12\t\n\x04GR\
    WI\x10\xd8\x03\x12\x08\n\x03GSR\x10\xd9\x03\x12\x08\n\x03GTC\x10\xda\x03\
    \x12\x08\n\x03GUC\x10\xdb\x03\x12\x08\n\x03GUN\x10\xdc\x03\x12\x08\n\x03\
    GUP\x10\xdd\x03\x12\x08\n\x03GXS\x10\xde\x03\x12\x08\n\x03HAL\x10\xdf\
    \x03\x12\n\n\x05HALLO\x10\xe0\x03\x12\x08\n\x03HBN\x10\xe1\x03\x12\x08\n\
    \x03HBT\x10\xe2\x03\x12\x08\n\x03HCC\x10\xe3\x03\x12\x08\n\x03HDG\x10\
    \xe4\x03\x12\t\n\x04HDLB\x10\xe5\x03\x12\t\n\x04HEAT\x10\xe6\x03\x12\t\n\
    \x04HERO\x10\xe7\x03\x12\x08\n\x03HGT\x10\xe8\x03\x12\x08\n\x03HKG\x10\
    \xe9\x03\x12\x08\n\x03HMC\x10\xea\x03\x12\x08\n\x03HMP\x10\xeb\x03\x12\
    \x08\n\x03HMQ\x10\xec\x03\x12\t\n\x04HODL\x10\xed\x03\x12\n\n\x05HONEY\
    \x10\xee\x03\x12\x08\n\x03HPC\x10\xef\x03\x12\x08\n\x03HSR\x10\xf0\x03\
    \x12\x08\n\x03HTC\x10\xf1\x03\x12\n\n\x05HTML5\x10\xf2\x03\x12\x08\n\x03\
    HUC\x10\xf3\x03\x12\t\n\x04HUSH\x10\xf4\x03\x12\t\n\x04HVCO\x10\xf5\x03\
    \x12\x08\n\x03HVN\x10\xf6\x03\x12\x08\n\x03HXX\x10\xf7\x03\x12\x08\n\x03\
    HYP\x10\xf8\x03\x12\n\n\x05HYPER\x10\xf9\x03\x12\x08\n\x03I0C\x10\xfa\
    \x03\x12\n\n\x05IBANK\x10\xfb\x03\x12\x08\n\x03ICE\x10\xfc\x03\x12\t\n\
    \x04ICOB\x10\xfd\x03\x12\t\n\x04ICON\x10\xfe\x03\x12\t\n\x04ICOO\x10\xff\
    \x03\x12\t\n\x04ICOS\x10\x80\x04\x12\x08\n\x03ICX\x10\x81\x04\x12\t\n\
    \x04IETH\x10\x82\x04\x12\x08\n\x03IFC\x10\x83\x04\x12\t\n\x04IFLT\x10\
    \x84\x04\x12\x08\n\x03IFT\x10\x85\x04\x12\t\n\x04IMPS\x10\x86\x04\x12\
    \x08\n\x03IMS\x10\x87\x04\x12\x08\n\x03IMX\x10\x88\x04\x12\n\n\x05INCNT\
    \x10\x89\x04\x12\x08\n\x03IND\x10\x8a\x04\x12\n\n\x05INDIA\x10\x8b\x04\
    \x12\x08\n\x03INF\x10\x8c\x04\x12\t\n\x04INFX\x10\x8d\x04\x12\n\n\x05INP\
    AY\x10\x8e\x04\x12\t\n\x04INSN\x10\x8f\x04\x12\t\n\x04INXT\x10\x90\x04\
    \x12\x08\n\x03IOC\x10\x91\x04\x12\x08\n\x03ION\x10\x92\x04\x12\x08\n\x03\
    IOP\x10\x93\x04\x12\x08\n\x03IQT\x10\x94\x04\x12\x08\n\x03IRL\x10\x95\
    \x04\x12\x08\n\x03ISL\x10\x96\x04\x12\x08\n\x03ITI\x10\x97\x04\x12\x08\n\
    \x03ITT\x10\x98\x04\x12\x08\n\x03ITZ\x10\x99\x04\x12\x08\n\x03IVZ\x10\
    \x9a\x04\x12\x08\n\x03IXC\x10\x9b\x04\x12\x08\n\x03IXT\x10\x9c\x04\x12\
    \x06\n\x01J\x10\x9d\x04\x12\x08\n\x03JET\x10\x9e\x04\x12\x08\n\x03JIN\
    \x10\x9f\x04\x12\t\n\x04JINN\x10\xa0\x04\x12\x08\n\x03JIO\x10\xa1\x04\
    \x12\x08\n\x03JNS\x10\xa2\x04\x12\t\n\x04JOBS\x10\xa3\x04\x12\x07\n\x02J\
    S\x10\xa4\x04\x12\x08\n\x03JWL\x10\xa5\x04\x12\n\n\x05KARMA\x10\xa6\x04\
    \x12\n\n\x05KASHH\x10\xa7\x04\x12\t\n\x04KAYI\x10\xa8\x04\x12\x08\n\x03K\
    CS\x10\xa9\x04\x12\x08\n\x03KED\x10\xaa\x04\x12\x08\n\x03KEK\x10\xab\x04\
    \x12\x0c\n\x07KEXCOIN\x10\xac\x04\x12\x08\n\x03KIC\x10\xad\x04\x12\t\n\
    \x04KICK\x10\xae\x04\x12\x08\n\x03KIN\x10\xaf\x04\x12\x08\n\x03KLC\x10\
    \xb0\x04\x12\x08\n\x03KLN\x10\xb1\x04\x12\x08\n\x03KMD\x10\xb2\x04\x12\t\
    \n\x04KOBO\x10\xb3\x04\x12\t\n\x04KORE\x10\xb4\x04\x12\x08\n\x03KRB\x10\
    \xb5\x04\x12\n\n\x05KRONE\x10\xb6\x04\x12\t\n\x04KURT\x10\xb7\x04\x12\t\
    \n\x04KUSH\x10\xb8\x04\x12\x07\n\x02LA\x10\xb9\x04\x12\t\n\x04LANA\x10\
    \xba\x04\x12\x08\n\x03LAZ\x10\xbb\x04\x12\x08\n\x03LBC\x10\xbc\x04\x12\t\
    \n\x04LBTC\x10\xbd\x04\x12\x08\n\x03LCP\x10\xbe\x04\x12\t\n\x04LDCN\x10\
    \xbf\x04\x12\n\n\x05LDOGE\x10\xc0\x04\x12\x08\n\x03LEA\x10\xc1\x04\x12\
    \x08\n\x03LEO\x10\xc2\x04\x12\n\n\x05LEPEN\x10\xc3\x04\x12\x08\n\x03LEX\
    \x10\xc4\x04\x12\x08\n\x03LGD\x10\xc5\x04\x12\t\n\x04LIFE\x10\xc6\x04\
    \x12\n\n\x05LINDA\x10\xc7\x04\x12\t\n\x04LINK\x10\xc8\x04\x12\t\n\x04LIN\
    X\x10\xc9\x04\x12\x08\n\x03LIR\x10\xca\x04\x12\x08\n\x03LKC\x10\xcb\x04\
    \x12\x08\n\x03LKK\x10\xcc\x04\x12\x08\n\x03LLT\x10\xcd\x04\x12\x08\n\x03\
    LMC\x10\xce\x04\x12\x08\n\x03LNK\x10\xcf\x04\x12\x08\n\x03LOG\x10\xd0\
    \x04\x12\x08\n\x03LOT\x10\xd1\x04\x12\x08\n\x03LRC\x10\xd2\x04\x12\x08\n\
    \x03LSK\x10\xd3\x04\x12\x08\n\x03LTB\x10\xd4\x04\x12\t\n\x04LTBC\x10\xd5\
    \x04\x12\t\n\x04LTCR\x10\xd6\x04\x12\t\n\x04LTCU\x10\xd7\x04\x12\x08\n\
    \x03LTG\x10\xd8\x04\x12\x08\n\x03LTH\x10\xd9\x04\x12\x08\n\x03LUN\x10\
    \xda\x04\x12\t\n\x04LUNA\x10\xdb\x04\x12\x08\n\x03LUX\x10\xdc\x04\x12\t\
    \n\x04LVPS\x10\xdd\x04\x12\x08\n\x03MAC\x10\xde\x04\x12\x08\n\x03MAD\x10\
    \xdf\x04\x12\t\n\x04MAGN\x10\xe0\x04\x12\t\n\x04MAID\x10\xe1\x04\x12\t\n\
    \x04MANA\x10\xe2\x04\x12\x08\n\x03MAO\x10\xe3\x04\x12\x08\n\x03MAR\x10\
    \xe4\x04\x12\t\n\x04MARS\x10\xe5\x04\x12\t\n\x04MARX\x10\xe6\x04\x12\n\n\
    \x05MAVRO\x10\xe7\x04\x12\x08\n\x03MAX\x10\xe8\x04\x12\x08\n\x03MAY\x10\
    \xe9\x04\x12\x08\n\x03MBI\x10\xea\x04\x12\x08\n\x03MBL\x10\xeb\x04\x12\t\
    \n\x04MBRS\x10\xec\x04\x12\t\n\x04MCAP\x10\xed\x04\x12\x08\n\x03MCI\x10\
    \xee\x04\x12\x08\n\x03MCO\x10\xef\x04\x12\x08\n\x03MCR\x10\xf0\x04\x12\t\
    \n\x04MCRN\x10\xf1\x04\x12\x08\n\x03MDA\x10\xf2\x04\x12\x08\n\x03MEC\x10\
    \xf3\x04\x12\t\n\x04MEME\x10\xf4\x04\x12\x08\n\x03MEN\x10\xf5\x04\x12\t\
    \n\x04MEOW\x10\xf6\x04\x12\x08\n\x03MER\x10\xf7\x04\x12\n\n\x05METAL\x10\
    \xf8\x04\x12\x07\n\x02MG\x10\xf9\x04\x12\x08\n\x03MGM\x10\xfa\x04\x12\
    \x08\n\x03MGO\x10\xfb\x04\x12\t\n\x04MILO\x10\xfc\x04\x12\n\n\x05MINEX\
    \x10\xfd\x04\x12\t\n\x04MINT\x10\xfe\x04\x12\n\n\x05MIOTA\x10\xff\x04\
    \x12\x08\n\x03MIU\x10\x80\x05\x12\x08\n\x03MLN\x10\x81\x05\x12\n\n\x05MM\
    XVI\x10\x82\x05\x12\x08\n\x03MND\x10\x83\x05\x12\x08\n\x03MNE\x10\x84\
    \x05\x12\x08\n\x03MNM\x10\x85\x05\x12\x08\n\x03MOD\x10\x86\x05\x12\t\n\
    \x04MOIN\x10\x87\x05\x12\t\n\x04MOJO\x10\x88\x05\x12\t\n\x04MONA\x10\x89\
    \x05\x12\x0b\n\x06MONETA\x10\x8a\x05\x12\n\n\x05MONEY\x10\x8b\x05\x12\t\
    \n\x04MOON\x10\x8c\x05\x12\t\n\x04MOTO\x10\x8d\x05\x12\x08\n\x03MRC\x10\
    \x8e\x05\x12\t\n\x04MRJA\x10\x8f\x05\x12\t\n\x04MRNG\x10\x90\x05\x12\x08\
    \n\x03MRT\x10\x91\x05\x12\t\n\x04MSCN\x10\x92\x05\x12\x08\n\x03MSD\x10\
    \x93\x05\x12\x08\n\x03MSP\x10\x94\x05\x12\x08\n\x03MST\x10\x95\x05\x12\
    \x08\n\x03MTH\x10\x96\x05\x12\x08\n\x03MTL\x10\x97\x05\x12\x0b\n\x06MTLM\
    C3\x10\x98\x05\x12\x08\n\x03MTM\x10\x99\x05\x12\t\n\x04MTNC\x10\x9a\x05\
    \x12\x08\n\x03MUE\x10\x9b\x05\x12\x08\n\x03MUG\x10\x9c\x05\x12\n\n\x05MU\
    SIC\x10\x9d\x05\x12\x08\n\x03MXT\x10\x9e\x05\x12\x08\n\x03MYB\x10\x9f\
    \x05\x12\t\n\x04MYST\x10\xa0\x05\x12\x08\n\x03MZC\x10\xa1\x05\x12\t\n\
    \x04NAMO\x10\xa2\x05\x12\n\n\x05NANOX\x10\xa3\x05\x12\x08\n\x03NAS\x10\
    \xa4\x05\x12\t\n\x04NAUT\x10\xa5\x05\x12\x08\n\x03NAV\x10\xa6\x05\x12\
    \x08\n\x03NBE\x10\xa7\x05\x12\t\n\x04NBIT\x10\xa8\x05\x12\t\n\x04NDAO\
    \x10\xa9\x05\x12\x08\n\x03NDC\x10\xaa\x05\x12\t\n\x04NEBL\x10\xab\x05\
    \x12\x08\n\x03NEO\x10\xac\x05\x12\t\n\x04NEOS\x10\xad\x05\x12\n\n\x05NET\
    KO\x10\xae\x05\x12\t\n\x04NEVA\x10\xaf\x05\x12\t\n\x04NEWB\x10\xb0\x05\
    \x12\x08\n\x03NKA\x10\xb1\x05\x12\t\n\x04NLC2\x10\xb2\x05\x12\x08\n\x03N\
    LG\x10\xb3\x05\x12\x08\n\x03NMC\x10\xb4\x05\x12\x08\n\x03NMR\x10\xb5\x05\
    \x12\t\n\x04NOBL\x10\xb6\x05\x12\t\n\x04NODC\x10\xb7\x05\x12\t\n\x04NOTE\
    \x10\xb8\x05\x12\x08\n\x03NRO\x10\xb9\x05\x12\x08\n\x03NSR\x10\xba\x05\
    \x12\x08\n\x03NTC\x10\xbb\x05\x12\t\n\x04NTCC\x10\xbc\x05\x12\x08\n\x03N\
    TO\x10\xbd\x05\x12\t\n\x04NTRN\x10\xbe\x05\x12\t\n\x04NTWK\x10\xbf\x05\
    \x12\t\n\x04NULS\x10\xc0\x05\x12\x08\n\x03NVC\x10\xc1\x05\x12\t\n\x04NVS\
    T\x10\xc2\x05\x12\x08\n\x03NXC\x10\xc3\x05\x12\x08\n\x03NXS\x10\xc4\x05\
    \x12\x08\n\x03NXT\x10\xc5\x05\x12\x08\n\x03NXX\x10\xc6\x05\x12\t\n\x04NY\
    AN\x10\xc7\x05\x12\x08\n\x03NYC\x10\xc8\x05\x12\x08\n\x03OAX\x10\xc9\x05\
    \x12\n\n\x05OBITS\x10\xca\x05\x12\n\n\x05OCEAN\x10\xcb\x05\x12\x08\n\x03\
    OCL\x10\xcc\x05\x12\t\n\x04OCOW\x10\xcd\x05\x12\x08\n\x03OCT\x10\xce\x05\
    \x12\x08\n\x03ODN\x10\xcf\x05\x12\x08\n\x03OFF\x10\xd0\x05\x12\x08\n\x03\
    OHM\x10\xd1\x05\x12\x07\n\x02OK\x10\xd2\x05\x12\x08\n\x03OMC\x10\xd3\x05\
    \x12\x08\n\x03OMG\x10\xd4\x05\x12\t\n\x04OMNI\x10\xd5\x05\x12\n\n\x05ONI\
    ON\x10\xd6\x05\x12\x08\n\x03ONX\x10\xd7\x05\x12\x07\n\x02OP\x10\xd8\x05\
    \x12\t\n\x04OPAL\x10\xd9\x05\x12\t\n\x04OPES\x10\xda\x05\x12\x08\n\x03OP\
    T\x10\xdb\x05\x12\x08\n\x03ORB\x10\xdc\x05\x12\t\n\x04ORLY\x10\xdd\x05\
    \x12\t\n\x04ORME\x10\xde\x05\x12\t\n\x04OS76\x10\xdf\x05\x12\x08\n\x03OT\
    N\x10\xe0\x05\x12\x07\n\x02OX\x10\xe1\x05\x12\x08\n\x03P7C\x10\xe2\x05\
    \x12\x08\n\x03PAC\x10\xe3\x05\x12\x08\n\x03PAK\x10\xe4\x05\x12\t\n\x04PA\
    RT\x10\xe5\x05\x12\t\n\x04PASC\x10\xe6\x05\x12\t\n\x04PASL\x10\xe7\x05\
    \x12\x08\n\x03PAY\x10\xe8\x05\x12\t\n\x04PAYP\x10\xe9\x05\x12\x08\n\x03P\
    BT\x10\xea\x05\x12\x08\n\x03PCN\x10\xeb\x05\x12\x08\n\x03PCS\x10\xec\x05\
    \x12\x08\n\x03PDC\x10\xed\x05\x12\x08\n\x03PDG\x10\xee\x05\x12\x08\n\x03\
    PEC\x10\xef\x05\x12\r\n\x08PEPECASH\x10\xf0\x05\x12\x08\n\x03PEX\x10\xf1\
    \x05\x12\x08\n\x03PGL\x10\xf2\x05\x12\x08\n\x03PHO\x10\xf3\x05\x12\x08\n\
    \x03PHS\x10\xf4\x05\x12\x07\n\x02PI\x10\xf5\x05\x12\x08\n\x03PIE\x10\xf6\
    \x05\x12\n\n\x05PIGGY\x10\xf7\x05\x12\t\n\x04PING\x10\xf8\x05\x12\t\n\
    \x04PINK\x10\xf9\x05\x12\t\n\x04PIPL\x10\xfa\x05\x12\t\n\x04PIRL\x10\xfb\
    \x05\x12\t\n\x04PIVX\x10\xfc\x05\x12\x08\n\x03PIX\x10\xfd\x05\x12\n\n\
    \x05PIZZA\x10\xfe\x05\x12\x08\n\x03PKB\x10\xff\x05\x12\n\n\x05PLACO\x10\
    \x80\x06\x12\t\n\x04PLBT\x10\x81\x06\x12\t\n\x04PLNC\x10\x82\x06\x12\x08\
    \n\x03PLR\x10\x83\x06\x12\x08\n\x03PLU\x10\x84\x06\x12\x08\n\x03PND\x10\
    \x85\x06\x12\x08\n\x03POE\x10\x86\x06\x12\t\n\x04POKE\x10\x87\x06\x12\t\
    \n\x04POLL\x10\x88\x06\x12\n\n\x05PONZI\x10\x89\x06\x12\x08\n\x03POP\x10\
    \x8a\x06\x12\x08\n\x03POS\x10\x8b\x06\x12\t\n\x04POST\x10\x8c\x06\x12\t\
    \n\x04POSW\x10\x8d\x06\x12\x08\n\x03POT\x10\x8e\x06\x12\x08\n\x03PPC\x10\
    \x8f\x06\x12\x08\n\x03PPP\x10\x90\x06\x12\x08\n\x03PPT\x10\x91\x06\x12\
    \x08\n\x03PPY\x10\x92\x06\x12\x07\n\x02PR\x10\x93\x06\x12\x08\n\x03PRC\
    \x10\x94\x06\x12\t\n\x04PRES\x10\x95\x06\x12\x08\n\x03PRG\x10\x96\x06\
    \x12\n\n\x05PRIMU\x10\x97\x06\x12\x08\n\x03PRM\x10\x98\x06\x12\x08\n\x03\
    PRN\x10\x99\x06\x12\x08\n\x03PRO\x10\x9a\x06\x12\t\n\x04PROC\x10\x9b\x06\
    \x12\x08\n\x03PRX\x10\x9c\x06\x12\x08\n\x03PSB\x10\x9d\x06\x12\x08\n\x03\
    PST\x10\x9e\x06\x12\x08\n\x03PSY\x10\x9f\x06\x12\x08\n\x03PTC\x10\xa0\
    \x06\x12\t\n\x04PTOY\x10\xa1\x06\x12\n\n\x05PULSE\x10\xa2\x06\x12\t\n\
    \x04PURA\x10\xa3\x06\x12\x08\n\x03PUT\x10\xa4\x06\x12\x08\n\x03PWR\x10\
    \xa5\x06\x12\x07\n\x02PX\x10\xa6\x06\x12\x08\n\x03PXC\x10\xa7\x06\x12\
    \x08\n\x03PXI\x10\xa8\x06\x12\x08\n\x03PZM\x10\xa9\x06\x12\x08\n\x03Q2C\
    \x10\xaa\x06\x12\x08\n\x03QAU\x10\xab\x06\x12\x08\n\x03QBC\x10\xac\x06\
    \x12\x08\n\x03QBK\x10\xad\x06\x12\x08\n\x03QBT\x10\xae\x06\x12\x08\n\x03\
    QCN\x10\xaf\x06\x12\t\n\x04QORA\x10\xb0\x06\x12\x08\n\x03QRK\x10\xb1\x06\
    \x12\x08\n\x03QRL\x10\xb2\x06\x12\x08\n\x03QTL\x10\xb3\x06\x12\t\n\x04QT\
    UM\x10\xb4\x06\x12\n\n\x05QWARK\x10\xb5\x06\x12\t\n\x04RADS\x10\xb6\x06\
    \x12\t\n\x04RAIN\x10\xb7\x06\x12\t\n\x04RBBT\x10\xb8\x06\x12\n\n\x05RBIE\
    S\x10\xb9\x06\x12\x08\n\x03RBT\x10\xba\x06\x12\x08\n\x03RBX\x10\xbb\x06\
    \x12\x08\n\x03RBY\x10\xbc\x06\x12\x07\n\x02RC\x10\xbd\x06\x12\x08\n\x03R\
    DD\x10\xbe\x06\x12\t\n\x04REAL\x10\xbf\x06\x12\x08\n\x03REC\x10\xc0\x06\
    \x12\x08\n\x03RED\x10\xc1\x06\x12\x08\n\x03REE\x10\xc2\x06\x12\t\n\x04RE\
    GA\x10\xc3\x06\x12\x08\n\x03REP\x10\xc4\x06\x12\x08\n\x03REQ\x10\xc5\x06\
    \x12\x08\n\x03REV\x10\xc6\x06\x12\x08\n\x03REX\x10\xc7\x06\x12\t\n\x04RH\
    FC\x10\xc8\x06\x12\t\n\x04RHOC\x10\xc9\x06\x12\x08\n\x03RIC\x10\xca\x06\
    \x12\n\n\x05RICHX\x10\xcb\x06\x12\t\n\x04RIDE\x10\xcc\x06\x12\t\n\x04RIS\
    E\x10\xcd\x06\x12\t\n\x04RIYA\x10\xce\x06\x12\x08\n\x03RKC\x10\xcf\x06\
    \x12\x08\n\x03RLC\x10\xd0\x06\x12\x08\n\x03RLT\x10\xd1\x06\x12\x08\n\x03\
    RMC\x10\xd2\x06\x12\x08\n\x03RNS\x10\xd3\x06\x12\n\n\x05ROOFS\x10\xd4\
    \x06\x12\n\n\x05ROYAL\x10\xd5\x06\x12\x08\n\x03RPC\x10\xd6\x06\x12\x08\n\
    \x03RPX\x10\xd7\x06\x12\t\n\x04RSGP\x10\xd8\x06\x12\n\n\x05RUBIT\x10\xd9\
    \x06\x12\x0c\n\x07RUNNERS\x10\xda\x06\x12\x08\n\x03RUP\x10\xdb\x06\x12\t\
    \n\x04RUPX\x10\xdc\x06\x12\r\n\x08RUSTBITS\x10\xdd\x06\x12\x08\n\x03RVT\
    \x10\xde\x06\x12\x08\n\x03SAC\x10\xdf\x06\x12\n\n\x05SAFEX\x10\xe0\x06\
    \x12\x08\n\x03SAK\x10\xe1\x06\x12\t\n\x04SALT\x10\xe2\x06\x12\x08\n\x03S\
    AN\x10\xe3\x06\x12\n\n\x05SANDG\x10\xe4\x06\x12\x08\n\x03SBD\x10\xe5\x06\
    \x12\x07\n\x02SC\x10\xe6\x06\x12\x08\n\x03SCL\x10\xe7\x06\x12\n\n\x05SCO\
    RE\x10\xe8\x06\x12\t\n\x04SCRT\x10\xe9\x06\x12\x08\n\x03SCS\x10\xea\x06\
    \x12\x08\n\x03SDC\x10\xeb\x06\x12\x08\n\x03SDP\x10\xec\x06\x12\t\n\x04SD\
    RN\x10\xed\x06\x12\x08\n\x03SEQ\x10\xee\x06\x12\x08\n\x03SFC\x10\xef\x06\
    \x12\x08\n\x03SFE\x10\xf0\x06\x12\x07\n\x02SH\x10\xf1\x06\x12\x08\n\x03S\
    HA\x10\xf2\x06\x12\t\n\x04SHDW\x10\xf3\x06\x12\n\n\x05SHELL\x10\xf4\x06\
    \x12\n\n\x05SHIFT\x10\xf5\x06\x12\t\n\x04SHND\x10\xf6\x06\x12\x0b\n\x06S\
    HORTY\x10\xf7\x06\x12\x08\n\x03SIB\x10\xf8\x06\x12\x08\n\x03SIC\x10\xf9\
    \x06\x12\t\n\x04SIFT\x10\xfa\x06\x12\n\n\x05SIGMA\x10\xfb\x06\x12\t\n\
    \x04SIGT\x10\xfc\x06\x12\t\n\x04SJCX\x10\xfd\x06\x12\x08\n\x03SKC\x10\
    \xfe\x06\x12\t\n\x04SKIN\x10\xff\x06\x12\x08\n\x03SKR\x10\x80\x07\x12\n\
    \n\x05SKULL\x10\x81\x07\x12\x08\n\x03SKY\x10\x82\x07\x12\x0b\n\x06SLEVIN\
    \x10\x83\x07\x12\t\n\x04SLFI\x10\x84\x07\x12\x08\n\x03SLG\x10\x85\x07\
    \x12\n\n\x05SLING\x10\x86\x07\x12\x08\n\x03SLM\x10\x87\x07\x12\x08\n\x03\
    SLR\x10\x88\x07\x12\x08\n\x03SLS\x10\x89\x07\x12\n\n\x05SMART\x10\x8a\
    \x07\x12\x08\n\x03SMC\x10\x8b\x07\x12\t\n\x04SMLY\x10\x8c\x07\x12\n\n\
    \x05SMOKE\x10\x8d\x07\x12\n\n\x05SNAKE\x10\x8e\x07\x12\x08\n\x03SNC\x10\
    \x8f\x07\x12\x08\n\x03SND\x10\x90\x07\x12\n\n\x05SNGLS\x10\x91\x07\x12\
    \x08\n\x03SNM\x10\x92\x07\x12\t\n\x04SNRG\x10\x93\x07\x12\x08\n\x03SNT\
    \x10\x94\x07\x12\t\n\x04SOAR\x10\x95\x07\x12\t\n\x04SOCC\x10\x96\x07\x12\
    \t\n\x04SOIL\x10\x97\x07\x12\x08\n\x03SOJ\x10\x98\x07\x12\t\n\x04SONG\
    \x10\x99\x07\x12\t\n\x04SOON\x10\x9a\x07\x12\t\n\x04SOUL\x10\x9b\x07\x12\
    \n\n\x05SPACE\x10\x9c\x07\x12\t\n\x04SPEX\x10\x9d\x07\x12\t\n\x04SPHR\
    \x10\x9e\x07\x12\n\n\x05SPORT\x10\x9f\x07\x12\x08\n\x03SPR\x10\xa0\x07\
    \x12\n\n\x05SPRTS\x10\xa1\x07\x12\x08\n\x03SPT\x10\xa2\x07\x12\x08\n\x03\
    SRC\x10\xa3\x07\x12\x08\n\x03STA\x10\xa4\x07\x12\n\n\x05START\x10\xa5\
    \x07\x12\t\n\x04STCN\x10\xa6\x07\x12\n\n\x05STEEM\x10\xa7\x07\x12\n\n\
    \x05STEPS\x10\xa8\x07\x12\t\n\x04STEX\x10\xa9\x07\x12\n\n\x05STORJ\x10\
    \xaa\x07\x12\n\n\x05STRAT\x10\xab\x07\x12\t\n\x04STRC\x10\xac\x07\x12\
    \x08\n\x03STS\x10\xad\x07\x12\x08\n\x03STV\x10\xae\x07\x12\x08\n\x03STX\
    \x10\xaf\x07\x12\x08\n\x03SUB\x10\xb0\x07\x12\t\n\x04SUMO\x10\xb1\x07\
    \x12\n\n\x05SUPER\x10\xb2\x07\x12\x08\n\x03SUR\x10\xb3\x07\x12\n\n\x05SW\
    IFT\x10\xb4\x07\x12\n\n\x05SWING\x10\xb5\x07\x12\x08\n\x03SWP\x10\xb6\
    \x07\x12\x08\n\x03SWT\x10\xb7\x07\x12\x08\n\x03SXC\x10\xb8\x07\x12\t\n\
    \x04SYNC\x10\xb9\x07\x12\t\n\x04SYNX\x10\xba\x07\x12\x08\n\x03SYS\x10\
    \xbb\x07\x12\t\n\x04TAAS\x10\xbc\x07\x12\x08\n\x03TAG\x10\xbd\x07\x12\t\
    \n\x04TAGR\x10\xbe\x07\x12\x08\n\x03TAJ\x10\xbf\x07\x12\t\n\x04TALK\x10\
    \xc0\x07\x12\x08\n\x03TCC\x10\xc1\x07\x12\n\n\x05TCOIN\x10\xc2\x07\x12\
    \x08\n\x03TCR\x10\xc3\x07\x12\t\n\x04TEAM\x10\xc4\x07\x12\x08\n\x03TEK\
    \x10\xc5\x07\x12\t\n\x04TELL\x10\xc6\x07\x12\x08\n\x03TER\x10\xc7\x07\
    \x12\t\n\x04TERA\x10\xc8\x07\x12\x08\n\x03TES\x10\xc9\x07\x12\n\n\x05TES\
    LA\x10\xca\x07\x12\x08\n\x03TFL\x10\xcb\x07\x12\x08\n\x03TGC\x10\xcc\x07\
    \x12\x08\n\x03TGT\x10\xcd\x07\x12\x08\n\x03THC\x10\xce\x07\x12\x08\n\x03\
    THS\x10\xcf\x07\x12\t\n\x04TIME\x10\xd0\x07\x12\t\n\x04TIPS\x10\xd1\x07\
    \x12\x08\n\x03TIT\x10\xd2\x07\x12\x08\n\x03TKN\x10\xd3\x07\x12\x08\n\x03\
    TKR\x10\xd4\x07\x12\x08\n\x03TKS\x10\xd5\x07\x12\x08\n\x03TLE\x10\xd6\
    \x07\x12\x08\n\x03TNT\x10\xd7\x07\x12\x08\n\x03TOA\x10\xd8\x07\x12\n\n\
    \x05TODAY\x10\xd9\x07\x12\n\n\x05TOKEN\x10\xda\x07\x12\x08\n\x03TOP\x10\
    \xdb\x07\x12\n\n\x05TOPAZ\x10\xdc\x07\x12\x08\n\x03TOR\x10\xdd\x07\x12\n\
    \n\x05TRADE\x10\xde\x07\x12\x08\n\x03TRC\x10\xdf\x07\x12\t\n\x04TRCT\x10\
    \xe0\x07\x12\x08\n\x03TRI\x10\xe1\x07\x12\n\n\x05TRICK\x10\xe2\x07\x12\t\
    \n\x04TRIG\x10\xe3\x07\x12\x08\n\x03TRK\x10\xe4\x07\x12\n\n\x05TROLL\x10\
    \xe5\x07\x12\t\n\x04TRST\x10\xe6\x07\x12\n\n\x05TRUMP\x10\xe7\x07\x12\n\
    \n\x05TRUST\x10\xe8\x07\x12\x08\n\x03TRX\x10\xe9\x07\x12\x08\n\x03TSE\
    \x10\xea\x07\x12\t\n\x04TSTR\x10\xeb\x07\x12\x08\n\x03TTC\x10\xec\x07\
    \x12\n\n\x05TURBO\x10\xed\x07\x12\x07\n\x02TX\x10\xee\x07\x12\x08\n\x03T\
    YC\x10\xef\x07\x12\n\n\x05TYCHO\x10\xf0\x07\x12\x08\n\x03TZC\x10\xf1\x07\
    \x12\x08\n\x03UBQ\x10\xf2\x07\x12\x08\n\x03UET\x10\xf3\x07\x12\x08\n\x03\
    UFO\x10\xf4\x07\x12\x08\n\x03UGT\x10\xf5\x07\x12\x08\n\x03UIS\x10\xf6\
    \x07\x12\x08\n\x03ULA\x10\xf7\x07\x12\x08\n\x03UNB\x10\xf8\x07\x12\x08\n\
    \x03UNC\x10\xf9\x07\x12\x08\n\x03UNI\x10\xfa\x07\x12\t\n\x04UNIC\x10\xfb\
    \x07\x12\n\n\x05UNIFY\x10\xfc\x07\x12\t\n\x04UNIT\x10\xfd\x07\x12\n\n\
    \x05UNITS\x10\xfe\x07\x12\n\n\x05UNITY\x10\xff\x07\x12\x08\n\x03UNO\x10\
    \x80\x08\x12\t\n\x04UNRC\x10\x81\x08\x12\x08\n\x03UNY\x10\x82\x08\x12\
    \x07\n\x02UR\x10\x83\x08\x12\x08\n\x03URC\x10\x84\x08\x12\x08\n\x03URO\
    \x10\x85\x08\x12\x08\n\x03USC\x10\x86\x08\x12\t\n\x04USDE\x10\x87\x08\
    \x12\t\n\x04USDT\x10\x88\x08\x12\n\n\x05USNBT\x10\x89\x08\x12\x08\n\x03U\
    TA\x10\x8a\x08\x12\x08\n\x03UTC\x10\x8b\x08\x12\x06\n\x01V\x10\x8c\x08\
    \x12\x08\n\x03VAL\x10\x8d\x08\x12\t\n\x04VASH\x10\x8e\x08\x12\x07\n\x02V\
    C\x10\x8f\x08\x12\t\n\x04VEC2\x10\x90\x08\x12\x08\n\x03VEN\x10\x91\x08\
    \x12\t\n\x04VERI\x10\x92\x08\x12\x08\n\x03VGC\x10\x93\x08\x12\x08\n\x03V\
    IA\x10\x94\x08\x12\x08\n\x03VIB\x10\x95\x08\x12\t\n\x04VIBE\x10\x96\x08\
    \x12\t\n\x04VIDZ\x10\x97\x08\x12\x08\n\x03VIP\x10\x98\x08\x12\n\n\x05VIS\
    IO\x10\x99\x08\x12\t\n\x04VIVO\x10\x9a\x08\x12\x08\n\x03VLT\x10\x9b\x08\
    \x12\t\n\x04VLTC\x10\x9c\x08\x12\n\n\x05VOISE\x10\x9d\x08\x12\t\n\x04VOL\
    T\x10\x9e\x08\x12\x08\n\x03VOX\x10\x9f\x08\x12\t\n\x04VOYA\x10\xa0\x08\
    \x12\t\n\x04VPRC\x10\xa1\x08\x12\x08\n\x03VRC\x10\xa2\x08\x12\x08\n\x03V\
    RM\x10\xa3\x08\x12\x08\n\x03VRS\x10\xa4\x08\x12\x08\n\x03VSL\x10\xa5\x08\
    \x12\x08\n\x03VSX\x10\xa6\x08\x12\x08\n\x03VTA\x10\xa7\x08\x12\x08\n\x03\
    VTC\x10\xa8\x08\x12\x08\n\x03VTR\x10\xa9\x08\x12\x08\n\x03VUC\x10\xaa\
    \x08\x12\t\n\x04VULC\x10\xab\x08\x12\x07\n\x02WA\x10\xac\x08\x12\t\n\x04\
    WARP\x10\xad\x08\x12\n\n\x05WAVES\x10\xae\x08\x12\x08\n\x03WAY\x10\xaf\
    \x08\x12\x08\n\x03WBB\x10\xb0\x08\x12\x08\n\x03WBC\x10\xb1\x08\x12\x08\n\
    \x03WCT\x10\xb2\x08\x12\x08\n\x03WDC\x10\xb3\x08\x12\x08\n\x03WEC\x10\
    \xb4\x08\x12\x08\n\x03WEX\x10\xb5\x08\x12\x08\n\x03WGO\x10\xb6\x08\x12\
    \x08\n\x03WGR\x10\xb7\x08\x12\x08\n\x03WHL\x10\xb8\x08\x12\x08\n\x03WIC\
    \x10\xb9\x08\x12\t\n\x04WILD\x10\xba\x08\x12\n\n\x05WINGS\x10\xbb\x08\
    \x12\t\n\x04WINK\x10\xbc\x08\x12\x08\n\x03WMC\x10\xbd\x08\x12\n\n\x05WOM\
    EN\x10\xbe\x08\x12\t\n\x04WORM\x10\xbf\x08\x12\x08\n\x03WOW\x10\xc0\x08\
    \x12\x08\n\x03WSX\x10\xc1\x08\x12\x08\n\x03WTC\x10\xc2\x08\x12\x08\n\x03\
    WTT\x10\xc3\x08\x12\x08\n\x03WYV\x10\xc4\x08\x12\x07\n\x02X2\x10\xc5\x08\
    \x12\x08\n\x03XAS\x10\xc6\x08\x12\x08\n\x03XAU\x10\xc7\x08\x12\t\n\x04XA\
    UR\x10\xc8\x08\x12\x08\n\x03XBC\x10\xc9\x08\x12\x08\n\x03XBG\x10\xca\x08\
    \x12\x08\n\x03XBL\x10\xcb\x08\x12\x0b\n\x06XBTC21\x10\xcc\x08\x12\t\n\
    \x04XBTS\x10\xcd\x08\x12\x08\n\x03XBY\x10\xce\x08\x12\x07\n\x02XC\x10\
    \xcf\x08\x12\x08\n\x03XCN\x10\xd0\x08\x12\x08\n\x03XCO\x10\xd1\x08\x12\
    \x08\n\x03XCP\x10\xd2\x08\x12\t\n\x04XCRE\x10\xd3\x08\x12\x08\n\x03XCS\
    \x10\xd4\x08\x12\x08\n\x03XCT\x10\xd5\x08\x12\t\n\x04XCXT\x10\xd6\x08\
    \x12\t\n\x04XDE2\x10\xd7\x08\x12\x08\n\x03XDN\x10\xd8\x08\x12\x08\n\x03X\
    EL\x10\xd9\x08\x12\x08\n\x03XEM\x10\xda\x08\x12\x08\n\x03XFT\x10\xdb\x08\
    \x12\t\n\x04XGOX\x10\xdc\x08\x12\x08\n\x03XGR\x10\xdd\x08\x12\x08\n\x03X\
    HI\x10\xde\x08\x12\x08\n\x03XIN\x10\xdf\x08\x12\t\n\x04XIOS\x10\xe0\x08\
    \x12\x08\n\x03XJO\x10\xe1\x08\x12\x08\n\x03XLC\x10\xe2\x08\x12\x08\n\x03\
    XLM\x10\xe3\x08\x12\x08\n\x03XLR\x10\xe4\x08\x12\t\n\x04XMCC\x10\xe5\x08\
    \x12\x08\n\x03XMG\x10\xe6\x08\x12\x08\n\x03XMR\x10\xe7\x08\x12\x08\n\x03\
    XMY\x10\xe8\x08\x12\x08\n\x03XNG\x10\xe9\x08\x12\x08\n\x03XNN\x10\xea\
    \x08\x12\x08\n\x03XOC\x10\xeb\x08\x12\x08\n\x03XOT\x10\xec\x08\x12\x07\n\
    \x02XP\x10\xed\x08\x12\x08\n\x03XPA\x10\xee\x08\x12\x08\n\x03XPD\x10\xef\
    \x08\x12\x08\n\x03XPM\x10\xf0\x08\x12\t\n\x04XPTX\x10\xf1\x08\x12\x08\n\
    \x03XPY\x10\xf2\x08\x12\x08\n\x03XQN\x10\xf3\x08\x12\x08\n\x03XRA\x10\
    \xf4\x08\x12\x08\n\x03XRB\x10\xf5\x08\x12\x08\n\x03XRC\x10\xf6\x08\x12\
    \x08\n\x03XRE\x10\xf7\x08\x12\x08\n\x03XRL\x10\xf8\x08\x12\x08\n\x03XRY\
    \x10\xf9\x08\x12\n\n\x05XSPEC\x10\xfa\x08\x12\x08\n\x03XST\x10\xfb\x08\
    \x12\t\n\x04XSTC\x10\xfc\x08\x12\x08\n\x03XTC\x10\xfd\x08\x12\x08\n\x03X\
    TD\x10\xfe\x08\x12\x08\n\x03XTO\x10\xff\x08\x12\x08\n\x03XTZ\x10\x80\t\
    \x12\x08\n\x03XUC\x10\x81\t\x12\x08\n\x03XVC\x10\x82\t\x12\x08\n\x03XVE\
    \x10\x83\t\x12\x08\n\x03XVG\x10\x84\t\x12\x08\n\x03XVP\x10\x85\t\x12\x08\
    \n\x03XWC\x10\x86\t\x12\x08\n\x03XZC\x10\x87\t\x12\x08\n\x03YAC\x10\x88\
    \t\x12\t\n\x04YASH\x10\x89\t\x12\x08\n\x03YES\x10\x8a\t\x12\x08\n\x03YOC\
    \x10\x8b\t\x12\n\n\x05YOYOW\x10\x8c\t\x12\x08\n\x03ZBC\x10\x8d\t\x12\x08\
    \n\x03ZCC\x10\x8e\t\x12\x08\n\x03ZCL\x10\x8f\t\x12\t\n\x04ZEIT\x10\x90\t\
    \x12\x08\n\x03ZEN\x10\x91\t\x12\x0c\n\x07ZENGOLD\x10\x92\t\x12\t\n\x04ZE\
    NI\x10\x93\t\x12\x08\n\x03ZER\x10\x94\t\x12\x08\n\x03ZET\x10\x95\t\x12\
    \x08\n\x03ZMC\x10\x96\t\x12\x08\n\x03ZNE\x10\x97\t\x12\x08\n\x03ZNY\x10\
    \x98\t\x12\x08\n\x03ZOI\x10\x99\t\x12\x08\n\x03ZRC\x10\x9a\t\x12\x08\n\
    \x03ZRX\x10\x9b\t\x12\x08\n\x03ZSC\x10\x9c\t\x12\x08\n\x03ZSE\x10\x9d\t\
    \x12\x08\n\x03ZUR\x10\x9e\t\x12\x08\n\x03ZYD\x10\x9f\tb\x06proto3\
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
