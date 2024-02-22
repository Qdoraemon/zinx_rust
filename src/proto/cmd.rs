// @generated
// 基础后台口令（口号、协议号、协议id）：
// G表示双端共用，
// C表示前端向后台请求，
// S表示后端反馈给前端,
// *SS表示后台服务器之间的口令（不需要生成给前端）

// 注意：因导出规则约定，枚举值注释说明要求全写在值的后面（不要单独起一行）

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Cmd {
    /// 未知
    None = 0,
    /// 心跳 n秒 调用一次 (前端要比后端短些)
    GHeart = 1,
    /// 退出
    CExit = 2,
    /// 发送测试协议
    CTest = 3,
    /// 下发测试协议
    STest = 4,
    /// 请求实名认证
    CIdCard = 104,
    /// 下发实名认证
    SIdCard = 105,
    /// 请求登录
    CLogin = 106,
    /// 下发登录
    SLogin = 107,
    /// 请求重新链接
    CReconn = 108,
    /// 下发重新链接
    SReconn = 109,
    /// 请求创建角色
    CCreateRole = 200,
    /// 下发创建角色
    SCreateRole = 201,
    /// 检查是否有重名
    CCheckReName = 202,
    /// 下发是否有重名
    SCheckReName = 203,
    /// 请求加入房间
    CJoinRoom = 300,
    /// 下发加入房间
    SJoinRoom = 301,
    /// 请求同步房间 (请求房间内玩家可显示玩家)
    CSyncRoom = 302,
    /// 下发同步房间 (下发房间内全部玩家，注意：只下发九宫格内的玩家)
    SSyncRoom = 303,
    /// 请求离开房间
    CLeaveRoom = 304,
    /// 下发离开房间
    SLeaveRoom = 305,
    /// 请求任务
    CMission = 500,
    /// 下发任务
    SMission = 501,
    /// 请求同步任务
    CSyncMission = 502,
    /// 下发同步任务
    SSyncMission = 503,
    /// 发送睡眠模式
    CSleep = 600,
    /// 下发睡眠模式
    SSleep = 601,
    /// 请求睡眠剩余时间
    CRemainTime2Sleep = 602,
    /// 下发睡眠剩余时间
    SRemainTime2Sleep = 603,
    /// 请求角色信息
    CRoleData = 1000,
    /// 下发角色信息
    SRoleData = 1001,
    /// 同步单挑属性
    SSyncAttr = 1002,
    /// 同步多条属性
    SSyncAttrs = 1003,
    /// 发送同步装扮
    CSyncAttire = 1004,
    /// 下发同步装扮
    SSyncAttire = 1005,
    /// 发送变换信息
    CTransform = 1006,
    /// 下发变换信息
    STransform = 1007,
    /// 发送同步动作
    CSyncAnim = 1008,
    /// 下发同步动作
    SSyncAnim = 1009,
    /// 请求背包数据
    CBagData = 1100,
    /// 下发背包数据
    SBagData = 1101,
    /// 同步背包道具
    CSyncBagProp = 1102,
    /// 下发同步背包道具
    SSyncBagProp = 1103,
    /// 请求商店数据
    CShopData = 1200,
    /// 下发商店数据
    SShopData = 1201,
    /// 同步商店商品
    CBuyProp = 1202,
    /// 下发同步商店商品
    SBuyProp = 1203,
    /// 发送与AI对话
    CTalk2Ai = 2000,
    /// 下发与AI对面
    STalk2Ai = 2001,
    /// 发送结束AI对话
    CEndTalk2Ai = 2002,
    /// 下发结束AI对面
    SEndTalk2Ai = 2003,
    /// 发送Audio文字
    CSendAudio = 2004,
    /// 下发Audio流
    SSendAudio = 2005,
    ///   发送Audio文字结束
    CEndSendAudio = 2006,
    ///   下发结束Audio流
    SEndSendAudio = 2007,
    /// 发送分数
    CSendScore = 2008,
    /// 下发获取分数结果
    SSendScore = 2009,
    /// 请求分数列表
    CScoreList = 2010,
    /// 下发分数列表
    SScoreList = 2011,
    /// 请求谜语游戏
    CRiddle = 2012,
    /// 下发谜语游戏
    SRiddle = 2013,
    /// 首次请求AI对话
    CFirstAitAlK = 2014,
    /// 唤醒睡眠状态
    CSleepUp = 2015,
    /// 下发睡眠状态
    SSleepUp = 2016,
}
impl Cmd {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Cmd::None => "NONE",
            Cmd::GHeart => "G_Heart",
            Cmd::CExit => "C_Exit",
            Cmd::CTest => "C_Test",
            Cmd::STest => "S_Test",
            Cmd::CIdCard => "C_IDCard",
            Cmd::SIdCard => "S_IDCard",
            Cmd::CLogin => "C_Login",
            Cmd::SLogin => "S_Login",
            Cmd::CReconn => "C_Reconn",
            Cmd::SReconn => "S_Reconn",
            Cmd::CCreateRole => "C_CreateRole",
            Cmd::SCreateRole => "S_CreateRole",
            Cmd::CCheckReName => "C_CheckReName",
            Cmd::SCheckReName => "S_CheckReName",
            Cmd::CJoinRoom => "C_JoinRoom",
            Cmd::SJoinRoom => "S_JoinRoom",
            Cmd::CSyncRoom => "C_SyncRoom",
            Cmd::SSyncRoom => "S_SyncRoom",
            Cmd::CLeaveRoom => "C_LeaveRoom",
            Cmd::SLeaveRoom => "S_LeaveRoom",
            Cmd::CMission => "C_Mission",
            Cmd::SMission => "S_Mission",
            Cmd::CSyncMission => "C_SyncMission",
            Cmd::SSyncMission => "S_SyncMission",
            Cmd::CSleep => "C_Sleep",
            Cmd::SSleep => "S_Sleep",
            Cmd::CRemainTime2Sleep => "C_RemainTime2Sleep",
            Cmd::SRemainTime2Sleep => "S_RemainTime2Sleep",
            Cmd::CRoleData => "C_RoleData",
            Cmd::SRoleData => "S_RoleData",
            Cmd::SSyncAttr => "S_SyncAttr",
            Cmd::SSyncAttrs => "S_SyncAttrs",
            Cmd::CSyncAttire => "C_SyncAttire",
            Cmd::SSyncAttire => "S_SyncAttire",
            Cmd::CTransform => "C_Transform",
            Cmd::STransform => "S_Transform",
            Cmd::CSyncAnim => "C_SyncAnim",
            Cmd::SSyncAnim => "S_SyncAnim",
            Cmd::CBagData => "C_BagData",
            Cmd::SBagData => "S_BagData",
            Cmd::CSyncBagProp => "C_SyncBagProp",
            Cmd::SSyncBagProp => "S_SyncBagProp",
            Cmd::CShopData => "C_ShopData",
            Cmd::SShopData => "S_ShopData",
            Cmd::CBuyProp => "C_BuyProp",
            Cmd::SBuyProp => "S_BuyProp",
            Cmd::CTalk2Ai => "C_Talk2AI",
            Cmd::STalk2Ai => "S_Talk2AI",
            Cmd::CEndTalk2Ai => "C_EndTalk2AI",
            Cmd::SEndTalk2Ai => "S_EndTalk2AI",
            Cmd::CSendAudio => "C_SendAudio",
            Cmd::SSendAudio => "S_SendAudio",
            Cmd::CEndSendAudio => "C_EndSendAudio",
            Cmd::SEndSendAudio => "S_EndSendAudio",
            Cmd::CSendScore => "C_SendScore",
            Cmd::SSendScore => "S_SendScore",
            Cmd::CScoreList => "C_ScoreList",
            Cmd::SScoreList => "S_ScoreList",
            Cmd::CRiddle => "C_Riddle",
            Cmd::SRiddle => "S_Riddle",
            Cmd::CFirstAitAlK => "C_FirstAITAlK",
            Cmd::CSleepUp => "C_SleepUp",
            Cmd::SSleepUp => "S_SleepUp",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "G_Heart" => Some(Self::GHeart),
            "C_Exit" => Some(Self::CExit),
            "C_Test" => Some(Self::CTest),
            "S_Test" => Some(Self::STest),
            "C_IDCard" => Some(Self::CIdCard),
            "S_IDCard" => Some(Self::SIdCard),
            "C_Login" => Some(Self::CLogin),
            "S_Login" => Some(Self::SLogin),
            "C_Reconn" => Some(Self::CReconn),
            "S_Reconn" => Some(Self::SReconn),
            "C_CreateRole" => Some(Self::CCreateRole),
            "S_CreateRole" => Some(Self::SCreateRole),
            "C_CheckReName" => Some(Self::CCheckReName),
            "S_CheckReName" => Some(Self::SCheckReName),
            "C_JoinRoom" => Some(Self::CJoinRoom),
            "S_JoinRoom" => Some(Self::SJoinRoom),
            "C_SyncRoom" => Some(Self::CSyncRoom),
            "S_SyncRoom" => Some(Self::SSyncRoom),
            "C_LeaveRoom" => Some(Self::CLeaveRoom),
            "S_LeaveRoom" => Some(Self::SLeaveRoom),
            "C_Mission" => Some(Self::CMission),
            "S_Mission" => Some(Self::SMission),
            "C_SyncMission" => Some(Self::CSyncMission),
            "S_SyncMission" => Some(Self::SSyncMission),
            "C_Sleep" => Some(Self::CSleep),
            "S_Sleep" => Some(Self::SSleep),
            "C_RemainTime2Sleep" => Some(Self::CRemainTime2Sleep),
            "S_RemainTime2Sleep" => Some(Self::SRemainTime2Sleep),
            "C_RoleData" => Some(Self::CRoleData),
            "S_RoleData" => Some(Self::SRoleData),
            "S_SyncAttr" => Some(Self::SSyncAttr),
            "S_SyncAttrs" => Some(Self::SSyncAttrs),
            "C_SyncAttire" => Some(Self::CSyncAttire),
            "S_SyncAttire" => Some(Self::SSyncAttire),
            "C_Transform" => Some(Self::CTransform),
            "S_Transform" => Some(Self::STransform),
            "C_SyncAnim" => Some(Self::CSyncAnim),
            "S_SyncAnim" => Some(Self::SSyncAnim),
            "C_BagData" => Some(Self::CBagData),
            "S_BagData" => Some(Self::SBagData),
            "C_SyncBagProp" => Some(Self::CSyncBagProp),
            "S_SyncBagProp" => Some(Self::SSyncBagProp),
            "C_ShopData" => Some(Self::CShopData),
            "S_ShopData" => Some(Self::SShopData),
            "C_BuyProp" => Some(Self::CBuyProp),
            "S_BuyProp" => Some(Self::SBuyProp),
            "C_Talk2AI" => Some(Self::CTalk2Ai),
            "S_Talk2AI" => Some(Self::STalk2Ai),
            "C_EndTalk2AI" => Some(Self::CEndTalk2Ai),
            "S_EndTalk2AI" => Some(Self::SEndTalk2Ai),
            "C_SendAudio" => Some(Self::CSendAudio),
            "S_SendAudio" => Some(Self::SSendAudio),
            "C_EndSendAudio" => Some(Self::CEndSendAudio),
            "S_EndSendAudio" => Some(Self::SEndSendAudio),
            "C_SendScore" => Some(Self::CSendScore),
            "S_SendScore" => Some(Self::SSendScore),
            "C_ScoreList" => Some(Self::CScoreList),
            "S_ScoreList" => Some(Self::SScoreList),
            "C_Riddle" => Some(Self::CRiddle),
            "S_Riddle" => Some(Self::SRiddle),
            "C_FirstAITAlK" => Some(Self::CFirstAitAlK),
            "C_SleepUp" => Some(Self::CSleepUp),
            "S_SleepUp" => Some(Self::SSleepUp),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
