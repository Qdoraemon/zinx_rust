// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardKv {
    /// 奖励类型id
    #[prost(int64, tag="1")]
    pub id: i64,
    /// 奖励内容
    #[prost(string, tag="2")]
    pub val: ::prost::alloc::string::String,
}
/// 数值属性键值列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttrKv {
    /// 属性code
    #[prost(enumeration="AttrCode", tag="1")]
    pub code: i32,
    /// 属性value
    #[prost(int64, tag="2")]
    pub val: i64,
}
/// 装扮数值属性键值列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttireKv {
    #[prost(enumeration="AttireCode", tag="1")]
    pub code: i32,
    #[prost(int64, tag="2")]
    pub val: i64,
}
/// 位置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    /// x轴
    #[prost(int64, tag="1")]
    pub x: i64,
    /// y轴
    #[prost(int64, tag="2")]
    pub y: i64,
    /// z轴
    #[prost(int64, tag="3")]
    pub z: i64,
}
/// Y轴旋转角度
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rotation {
    /// 角度
    #[prost(int64, tag="1")]
    pub y: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scale {
    /// x轴
    #[prost(int64, tag="1")]
    pub x: i64,
    /// y轴
    #[prost(int64, tag="2")]
    pub y: i64,
    /// z轴
    #[prost(int64, tag="3")]
    pub z: i64,
}
/// 语言信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Voice {
    /// 音色
    #[prost(int64, tag="1")]
    pub timbre: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionInfo {
    /// 任务类型
    #[prost(enumeration="MissionType", tag="1")]
    pub r#type: i32,
    /// 任务id
    #[prost(int64, tag="2")]
    pub id: i64,
    /// 任务状态 true = 完成， false = 未完成
    #[prost(bool, tag="3")]
    pub state: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trans {
    /// 位置
    #[prost(message, optional, tag="1")]
    pub pos: ::core::option::Option<Position>,
    /// 角度
    #[prost(message, optional, tag="2")]
    pub rot: ::core::option::Option<Rotation>,
    /// 大小
    #[prost(message, optional, tag="3")]
    pub scale: ::core::option::Option<Scale>,
    /// 是否瞬移
    #[prost(bool, tag="4")]
    pub is_teleport: bool,
}
/// 角色数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleData {
    /// 名字
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// 是否角色
    #[prost(bool, tag="2")]
    pub exist: bool,
    /// 属性信息
    #[prost(message, repeated, tag="3")]
    pub attrs: ::prost::alloc::vec::Vec<AttrKv>,
    /// 装扮数据
    #[prost(message, repeated, tag="4")]
    pub attires: ::prost::alloc::vec::Vec<AttireKv>,
    /// 变换信息
    #[prost(message, optional, tag="5")]
    pub trans: ::core::option::Option<Trans>,
    /// 头像
    #[prost(int64, tag="6")]
    pub avatar: i64,
}
/// 道具信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropInfo {
    #[prost(enumeration="PropType", tag="1")]
    pub r#type: i32,
    /// 物品id
    #[prost(int64, tag="2")]
    pub id: i64,
    /// 物品数量
    #[prost(int64, tag="3")]
    pub count: i64,
    /// 使用类型
    #[prost(enumeration="UseType", tag="4")]
    pub use_type: i32,
    /// 价格
    #[prost(int64, tag="5")]
    pub price: i64,
}
/// 背包数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bag {
    #[prost(message, repeated, tag="1")]
    pub info: ::prost::alloc::vec::Vec<PropInfo>,
}
/// 商品信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommodityInfo {
    /// 货币种类
    #[prost(enumeration="MoneyType", tag="1")]
    pub money_type: i32,
    /// 道具信息
    #[prost(message, optional, tag="2")]
    pub prop: ::core::option::Option<PropInfo>,
}
/// 商店数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shop {
    #[prost(message, repeated, tag="1")]
    pub info: ::prost::alloc::vec::Vec<CommodityInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sell {
    #[prost(enumeration="SellType", tag="1")]
    pub r#type: i32,
    #[prost(message, repeated, tag="2")]
    pub info: ::prost::alloc::vec::Vec<CommodityInfo>,
}
/// 猜谜
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Riddles {
    #[prost(string, tag="1")]
    pub question: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub options: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="3")]
    pub answer: ::prost::alloc::string::String,
}
/// 装扮数值属性键值列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserScoreMessage {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub sex: i64,
    #[prost(int64, tag="3")]
    pub score: i64,
}
/// 结果属性码表
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResCode {
    /// 成功
    Succ = 0,
    /// 失败
    Fail = 1,
}
impl ResCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResCode::Succ => "succ",
            ResCode::Fail => "fail",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "succ" => Some(Self::Succ),
            "fail" => Some(Self::Fail),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LoginType {
    /// steamId登录
    SteamId = 0,
}
impl LoginType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LoginType::SteamId => "steamId",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "steamId" => Some(Self::SteamId),
            _ => None,
        }
    }
}
/// 数值类属性码表(key-code)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AttrCode {
    /// 未知
    None = 0,
    /// 角色id
    RoleId = 1,
    /// 房间id
    RoomId = 2,
    /// 性别
    Sex = 3,
    /// 能量
    Ery = 4,
    /// 经验
    Exp = 5,
    /// 等级
    Lv = 6,
    /// 状态 0=正常 ，1 = 睡眠
    State = 7,
    /// 能量上限
    EryLimit = 8,
}
impl AttrCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AttrCode::None => "none",
            AttrCode::RoleId => "roleId",
            AttrCode::RoomId => "roomId",
            AttrCode::Sex => "sex",
            AttrCode::Ery => "ery",
            AttrCode::Exp => "exp",
            AttrCode::Lv => "lv",
            AttrCode::State => "state",
            AttrCode::EryLimit => "eryLimit",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "none" => Some(Self::None),
            "roleId" => Some(Self::RoleId),
            "roomId" => Some(Self::RoomId),
            "sex" => Some(Self::Sex),
            "ery" => Some(Self::Ery),
            "exp" => Some(Self::Exp),
            "lv" => Some(Self::Lv),
            "state" => Some(Self::State),
            "eryLimit" => Some(Self::EryLimit),
            _ => None,
        }
    }
}
/// 装扮值类型属性码表
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AttireCode {
    /// 头发
    Hair = 0,
    /// 眼睛
    Eye = 1,
    /// 面型
    Face = 2,
    /// 上装
    Cloth = 3,
    /// 下装
    Pant = 4,
    /// 鞋子
    Shoes = 5,
}
impl AttireCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AttireCode::Hair => "hair",
            AttireCode::Eye => "eye",
            AttireCode::Face => "face",
            AttireCode::Cloth => "cloth",
            AttireCode::Pant => "pant",
            AttireCode::Shoes => "shoes",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "hair" => Some(Self::Hair),
            "eye" => Some(Self::Eye),
            "face" => Some(Self::Face),
            "cloth" => Some(Self::Cloth),
            "pant" => Some(Self::Pant),
            "shoes" => Some(Self::Shoes),
            _ => None,
        }
    }
}
/// 任务类型，暂时只有每日任务
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MissionType {
    /// 每日任务
    D2d = 0,
    /// 主线任务
    Main = 1,
    /// 支线任务
    Branch = 2,
    /// 活动任务
    Activity = 3,
}
impl MissionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MissionType::D2d => "d2d",
            MissionType::Main => "main",
            MissionType::Branch => "branch",
            MissionType::Activity => "activity",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "d2d" => Some(Self::D2d),
            "main" => Some(Self::Main),
            "branch" => Some(Self::Branch),
            "activity" => Some(Self::Activity),
            _ => None,
        }
    }
}
/// 道具类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PropType {
    /// 食物
    Food = 0,
    /// 食材
    Ingredients = 1,
    /// 工具
    Tool = 2,
    /// 日常用品
    Daily = 3,
    /// 宠物用品
    Pet = 4,
    /// 种植物
    Plant = 5,
    /// 装饰物
    Decoration = 6,
    /// 收藏品
    Collection = 7,
    /// 任务道具
    Mission = 8,
    /// 商店道具（仅商店出售）
    Shop = 9,
}
impl PropType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PropType::Food => "food",
            PropType::Ingredients => "ingredients",
            PropType::Tool => "tool",
            PropType::Daily => "daily",
            PropType::Pet => "pet",
            PropType::Plant => "plant",
            PropType::Decoration => "decoration",
            PropType::Collection => "collection",
            PropType::Mission => "mission",
            PropType::Shop => "shop",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "food" => Some(Self::Food),
            "ingredients" => Some(Self::Ingredients),
            "tool" => Some(Self::Tool),
            "daily" => Some(Self::Daily),
            "pet" => Some(Self::Pet),
            "plant" => Some(Self::Plant),
            "decoration" => Some(Self::Decoration),
            "collection" => Some(Self::Collection),
            "mission" => Some(Self::Mission),
            "shop" => Some(Self::Shop),
            _ => None,
        }
    }
}
/// 商品类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SellType {
    /// 每日道具
    DailyProp = 0,
    /// 限时道具
    LimitProp = 1,
    /// 道具
    NormalProp = 2,
    /// 金币兑换
    GoldConvert = 3,
    /// VIP/月卡/免广等商品道具
    VipProp = 4,
}
impl SellType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SellType::DailyProp => "dailyProp",
            SellType::LimitProp => "limitProp",
            SellType::NormalProp => "normalProp",
            SellType::GoldConvert => "goldConvert",
            SellType::VipProp => "vipProp",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "dailyProp" => Some(Self::DailyProp),
            "limitProp" => Some(Self::LimitProp),
            "normalProp" => Some(Self::NormalProp),
            "goldConvert" => Some(Self::GoldConvert),
            "vipProp" => Some(Self::VipProp),
            _ => None,
        }
    }
}
/// 货币类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MoneyType {
    /// 金币
    MonGold = 0,
    /// 点券
    MonTicket = 1,
    /// 人民币
    MonRmb = 2,
    /// 港元
    MonHkd = 3,
    /// 新台币
    MonNtd = 4,
    /// 美元
    MonUsd = 5,
}
impl MoneyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MoneyType::MonGold => "mon_gold",
            MoneyType::MonTicket => "mon_ticket",
            MoneyType::MonRmb => "mon_RMB",
            MoneyType::MonHkd => "mon_HKD",
            MoneyType::MonNtd => "mon_NTD",
            MoneyType::MonUsd => "mon_USD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "mon_gold" => Some(Self::MonGold),
            "mon_ticket" => Some(Self::MonTicket),
            "mon_RMB" => Some(Self::MonRmb),
            "mon_HKD" => Some(Self::MonHkd),
            "mon_NTD" => Some(Self::MonNtd),
            "mon_USD" => Some(Self::MonUsd),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UseType {
    /// 普通道具，会存进背包
    UseNormal = 0,
    /// 直接使用能量点
    UseEner = 1,
    /// 直接使用金币
    UseGold = 2,
    /// 直接使用月卡
    UseVip = 3,
}
impl UseType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UseType::UseNormal => "use_normal",
            UseType::UseEner => "use_ener",
            UseType::UseGold => "use_gold",
            UseType::UseVip => "use_vip",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "use_normal" => Some(Self::UseNormal),
            "use_ener" => Some(Self::UseEner),
            "use_gold" => Some(Self::UseGold),
            "use_vip" => Some(Self::UseVip),
            _ => None,
        }
    }
}
/// 游戏类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GameType {
    /// 打地鼠
    WhacAMole = 0,
    /// 猜谜
    Riddle = 1,
}
impl GameType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GameType::WhacAMole => "WhacAMole",
            GameType::Riddle => "Riddle",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "WhacAMole" => Some(Self::WhacAMole),
            "Riddle" => Some(Self::Riddle),
            _ => None,
        }
    }
}
/// 排行榜类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GameModel {
    /// 最高
    Highest = 0,
    /// 历史
    History = 1,
}
impl GameModel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GameModel::Highest => "Highest",
            GameModel::History => "History",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Highest" => Some(Self::Highest),
            "History" => Some(Self::History),
            _ => None,
        }
    }
}
/// 心跳
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GHeart {
}
/// 测试数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CTest {
    /// 整型
    #[prost(int64, tag="1")]
    pub r#in: i64,
    /// 布尔
    #[prost(bool, tag="2")]
    pub boo: bool,
    /// 字符串
    #[prost(string, tag="3")]
    pub str: ::prost::alloc::string::String,
    /// 字节
    #[prost(bytes="vec", tag="4")]
    pub byte: ::prost::alloc::vec::Vec<u8>,
    /// 整型数组
    #[prost(int64, repeated, tag="5")]
    pub arr_int: ::prost::alloc::vec::Vec<i64>,
    /// 布尔数组
    #[prost(bool, repeated, tag="6")]
    pub arr_bool: ::prost::alloc::vec::Vec<bool>,
    /// 字符串数组
    #[prost(string, repeated, tag="7")]
    pub arr_string: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 字节数组
    #[prost(bytes="vec", repeated, tag="8")]
    pub arr_byte: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 测试数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct STest {
    /// 整型
    #[prost(int64, tag="1")]
    pub r#in: i64,
    /// 布尔
    #[prost(bool, tag="2")]
    pub boo: bool,
    /// 字符串
    #[prost(string, tag="3")]
    pub str: ::prost::alloc::string::String,
    /// 字节
    #[prost(bytes="vec", tag="4")]
    pub byte: ::prost::alloc::vec::Vec<u8>,
    /// 整型数组
    #[prost(int64, repeated, tag="5")]
    pub arr_int: ::prost::alloc::vec::Vec<i64>,
    /// 布尔数组
    #[prost(bool, repeated, tag="6")]
    pub arr_bool: ::prost::alloc::vec::Vec<bool>,
    /// 字符串数组
    #[prost(string, repeated, tag="7")]
    pub arr_string: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 字节数组
    #[prost(bytes="vec", repeated, tag="8")]
    pub arr_byte: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 退出
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CExit {
}
/// 请求手机验证码
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CPhoneCode {
    /// 手机号码
    #[prost(string, tag="1")]
    pub num: ::prost::alloc::string::String,
}
/// 下发手机验证码
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SPhoneCode {
    /// 结果
    #[prost(enumeration="ResCode", tag="1")]
    pub res: i32,
}
/// 请求注册
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CRegister {
    #[prost(string, tag="1")]
    pub num: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pw: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub code: ::prost::alloc::string::String,
}
/// 下发注册结果
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SRegister {
    /// 结果
    #[prost(enumeration="ResCode", tag="1")]
    pub res: i32,
    /// 错误信息 : 比如 账号已存在
    #[prost(string, tag="2")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CLogin {
    /// 登录类型
    #[prost(enumeration="LoginType", tag="1")]
    pub r#type: i32,
    /// 微信登录授权后返回的token type = LoginType.wechat 时使用
    #[prost(string, tag="2")]
    pub steam_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SLogin {
    /// 登录类型
    #[prost(enumeration="LoginType", tag="1")]
    pub r#type: i32,
    /// 登录结果
    #[prost(enumeration="ResCode", tag="2")]
    pub res: i32,
    /// 角色信息
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<RoleData>,
    /// token 信息
    #[prost(string, tag="4")]
    pub token: ::prost::alloc::string::String,
}
/// 请求断线重连
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CReconn {
    /// 登录类型（断线重连LoginType.reconn）
    #[prost(enumeration="LoginType", tag="1")]
    pub r#type: i32,
    /// 手机号码
    #[prost(string, tag="2")]
    pub num: ::prost::alloc::string::String,
    /// token 信息
    #[prost(string, tag="3")]
    pub token: ::prost::alloc::string::String,
}
/// 下发断线重连
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SReconn {
    /// 登录类型（断线重连LoginType.reconn）
    #[prost(enumeration="LoginType", tag="1")]
    pub r#type: i32,
    /// 登录结果
    #[prost(enumeration="ResCode", tag="2")]
    pub res: i32,
    /// 角色信息
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<RoleData>,
    /// token 信息
    #[prost(string, tag="4")]
    pub token: ::prost::alloc::string::String,
}
/// 创建角色
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCreateRole {
    /// 我的名字是什么？
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// 年龄
    #[prost(int64, tag="2")]
    pub age: i64,
    /// 1 男 0 女
    #[prost(int64, tag="3")]
    pub sex: i64,
    /// 个性
    #[prost(string, tag="4")]
    pub nature: ::prost::alloc::string::String,
    /// 装扮信息
    #[prost(message, repeated, tag="5")]
    pub attires: ::prost::alloc::vec::Vec<AttireKv>,
    /// 头像
    #[prost(int64, tag="6")]
    pub avatar: i64,
    /// 你的名字是什么？
    #[prost(string, tag="7")]
    pub you_name: ::prost::alloc::string::String,
    /// 我用什么语气和你对话？
    #[prost(string, tag="8")]
    pub tone: ::prost::alloc::string::String,
}
/// 下发创建角色
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SCreateRole {
    /// 结果
    #[prost(enumeration="ResCode", tag="1")]
    pub res: i32,
    /// 角色数据
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<RoleData>,
    /// 错误信息 比如：角色名已存在
    #[prost(string, tag="2")]
    pub error: ::prost::alloc::string::String,
}
/// 请求检查是否有重复命名
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCheckReName {
    /// 名字
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// 下发是否有重复命名结果
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SCheckReName {
    /// 是否重名
    #[prost(bool, tag="1")]
    pub res: bool,
}
/// 更新单条属性
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SSyncAttr {
    /// 角色Id
    #[prost(int64, tag="1")]
    pub role_id: i64,
    /// 更新属性
    #[prost(message, optional, tag="2")]
    pub attr: ::core::option::Option<AttrKv>,
}
/// 更新多条属性
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SSyncAttrs {
    /// 角色Id
    #[prost(int64, tag="1")]
    pub role_id: i64,
    /// 更新属性
    #[prost(message, repeated, tag="2")]
    pub attrs: ::prost::alloc::vec::Vec<AttrKv>,
}
/// 同步装扮
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSyncAttire {
    /// 装扮
    #[prost(message, repeated, tag="1")]
    pub arrires: ::prost::alloc::vec::Vec<AttireKv>,
}
/// 下发同步装扮
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SSyncAttire {
    /// 装扮
    #[prost(int64, tag="1")]
    pub role_id: i64,
    /// 装扮
    #[prost(message, repeated, tag="2")]
    pub arrires: ::prost::alloc::vec::Vec<AttireKv>,
}
/// 请求角色数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CRoleData {
    /// 角色id
    #[prost(int64, tag="1")]
    pub role_id: i64,
}
/// 下发角色数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SRoleData {
    /// 角色数据
    #[prost(message, optional, tag="1")]
    pub data: ::core::option::Option<RoleData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CTransform {
    #[prost(message, optional, tag="1")]
    pub trans: ::core::option::Option<Trans>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct STransform {
    #[prost(int64, tag="1")]
    pub role_id: i64,
    #[prost(message, optional, tag="2")]
    pub trans: ::core::option::Option<Trans>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSyncAnim {
    #[prost(string, tag="1")]
    pub anim: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub trans: ::core::option::Option<Trans>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SSyncAnim {
    #[prost(int64, tag="1")]
    pub user_id: i64,
    #[prost(string, tag="2")]
    pub anim: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub trans: ::core::option::Option<Trans>,
}
/// 请求进入房间
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CJoinRoom {
    /// 房间id
    #[prost(int64, tag="1")]
    pub room_id: i64,
}
/// 下发进入房间
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SJoinRoom {
    /// 角色数据
    #[prost(message, optional, tag="1")]
    pub data: ::core::option::Option<RoleData>,
}
/// 请求同步房间
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSyncRoom {
    #[prost(int64, tag="1")]
    pub room_id: i64,
}
/// 下发同步房间
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SSyncRoom {
    #[prost(message, repeated, tag="1")]
    pub datas: ::prost::alloc::vec::Vec<RoleData>,
}
/// 请求离开房间
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CLeaveRoom {
    #[prost(message, optional, tag="1")]
    pub trans: ::core::option::Option<Trans>,
}
/// 发送离开房间
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SLeaveRoom {
    /// 离开的角色id列表
    #[prost(int64, repeated, tag="1")]
    pub role_ids: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CTalk2Ai {
    /// 文本
    #[prost(string, tag="1")]
    pub text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct STalk2Ai {
    /// 文本
    #[prost(string, tag="1")]
    pub text: ::prost::alloc::string::String,
    /// 错误信息
    #[prost(string, tag="2")]
    pub error: ::prost::alloc::string::String,
}
/// 结束AI对话标志
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CEndTalk2Ai {
}
/// 结束AI对话标志
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SEndTalk2Ai {
}
/// 上传对话的文字内容
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSendAudio {
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub text: ::prost::alloc::string::String,
}
/// 返回对话的音频文件流
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SSendAudio {
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub audio: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub code: i64,
    #[prost(string, tag="4")]
    pub rec_text: ::prost::alloc::string::String,
}
///   发送Audio文字结束
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CEndSendAudio {
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub voice_type: ::prost::alloc::string::String,
}
///   下发结束Audio流 
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SEndSendAudio {
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
}
/// 获取全部任务状态
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMission {
    /// 请求任务类型数据
    #[prost(enumeration="MissionType", repeated, tag="1")]
    pub types: ::prost::alloc::vec::Vec<i32>,
}
/// 下发全部任务状态
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SMission {
    /// 下发请求的类型任务状态
    #[prost(message, repeated, tag="1")]
    pub missions: ::prost::alloc::vec::Vec<MissionInfo>,
}
/// 同步任务状态
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSyncMission {
    /// 任务类型
    #[prost(enumeration="MissionType", tag="1")]
    pub r#type: i32,
    /// 任务id
    #[prost(int64, tag="2")]
    pub id: i64,
    /// 任务状态 true = 完成 ，false = 未完成
    #[prost(bool, tag="3")]
    pub state: bool,
}
/// 任务状态
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SSyncMission {
    /// 任务类型
    #[prost(enumeration="MissionType", tag="1")]
    pub r#type: i32,
    /// 任务id
    #[prost(int64, tag="2")]
    pub id: i64,
    /// 任务状态 true = 完成 ，false = 未完成
    #[prost(bool, tag="3")]
    pub state: bool,
    /// 奖励
    #[prost(message, repeated, tag="4")]
    pub rewards: ::prost::alloc::vec::Vec<RewardKv>,
}
/// 进入睡眠模式
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSleep {
    /// 用户设置的睡眠时间(小时整)
    #[prost(int64, tag="1")]
    pub hour: i64,
}
/// 校准剩余时间
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SSleep {
    /// 当 收到 C_Sleep 协议的时候 返回 C_Sleep.time 进行校准
    #[prost(int64, tag="1")]
    pub hour: i64,
}
/// 请求剩余睡眠时间
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CRemainTime2Sleep {
    /// 角色ID
    #[prost(int64, tag="1")]
    pub role_id: i64,
}
/// 下发剩余睡眠时间
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SRemainTime2Sleep {
    /// 获取剩余秒数
    #[prost(int64, tag="1")]
    pub seconds: i64,
    /// 总共时间 
    #[prost(int64, tag="2")]
    pub total_time: i64,
}
/// 请求背包数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CBagData {
    /// 道具类型
    #[prost(enumeration="PropType", tag="1")]
    pub r#type: i32,
}
/// 下发背包数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SBagData {
    /// 返回背包数据
    #[prost(message, optional, tag="1")]
    pub data: ::core::option::Option<Bag>,
}
/// 同步背包道具
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSyncBagProp {
    /// 道具类型
    #[prost(enumeration="PropType", tag="1")]
    pub r#type: i32,
    /// 道具id
    #[prost(int64, tag="2")]
    pub id: i64,
    /// 道具数量，正数为添加道具，负数为消耗道具
    #[prost(int64, tag="3")]
    pub count: i64,
}
/// 下发背包道具
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SSyncBagProp {
    /// 道具类型
    #[prost(enumeration="PropType", tag="1")]
    pub r#type: i32,
    /// 道具id
    #[prost(int64, tag="2")]
    pub id: i64,
    /// 道具数量
    #[prost(int64, tag="3")]
    pub count: i64,
}
/// 请求商店数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CShopData {
    /// 发送商店标签类型
    #[prost(enumeration="SellType", tag="1")]
    pub r#type: i32,
}
/// 下发商店数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SShopData {
    /// 返回商店数据
    #[prost(message, optional, tag="1")]
    pub data: ::core::option::Option<Shop>,
}
/// 同步商店商品
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CBuyProp {
    /// 商品类型
    #[prost(enumeration="SellType", tag="1")]
    pub sell_type: i32,
    /// 道具类型
    #[prost(enumeration="PropType", tag="2")]
    pub prop_type: i32,
    /// 道具id
    #[prost(int64, tag="3")]
    pub prop_id: i64,
    /// 购买该道具的数量
    #[prost(int64, tag="4")]
    pub count: i64,
}
/// 下发商店商品
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SBuyProp {
    #[prost(message, optional, tag="1")]
    pub data: ::core::option::Option<Sell>,
}
/// 发送积分
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSendScore {
    #[prost(enumeration="GameType", tag="1")]
    pub types: i32,
    #[prost(int64, tag="2")]
    pub score: i64,
}
/// 下发积分
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SSendScore {
    #[prost(int64, tag="1")]
    pub code: i64,
    #[prost(string, tag="2")]
    pub msg: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub highest_score: i64,
}
/// 请求获取排行榜列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CScoreList {
    #[prost(enumeration="GameType", tag="1")]
    pub types: i32,
    #[prost(enumeration="GameModel", tag="2")]
    pub model: i32,
}
/// 下发排行榜列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SScoreList {
    #[prost(int64, tag="1")]
    pub code: i64,
    #[prost(string, tag="2")]
    pub msg: ::prost::alloc::string::String,
    /// 用户信息
    #[prost(message, repeated, tag="3")]
    pub user_score_message: ::prost::alloc::vec::Vec<UserScoreMessage>,
}
/// 请求猜谜
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CRiddle {
}
/// 下发猜谜
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SRiddle {
    #[prost(int64, tag="1")]
    pub code: i64,
    #[prost(string, tag="2")]
    pub msg: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub riddle: ::core::option::Option<Riddles>,
}
/// 请求首次请求AI
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CFirstAitAlK {
}
/// 请求唤醒
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSleepUp {
}
/// 下发唤醒
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SSleepUp {
    #[prost(int64, tag="1")]
    pub code: i64,
    #[prost(string, tag="2")]
    pub msg: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
