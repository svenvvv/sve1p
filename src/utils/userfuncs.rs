use phf::phf_map;

pub enum UserFuncType {
    String,
    Integer,
}

pub struct UserFuncDefinition {
    pub name: &'static str,
    pub argument_types: &'static [UserFuncType],
}

impl UserFuncDefinition {
    const fn new(
        name: &'static str,
        argument_types: &'static [UserFuncType],
    ) -> UserFuncDefinition {
        UserFuncDefinition {
            name,
            argument_types,
        }
    }
}

pub static BUILTIN_USERFUNCS: phf::Map<u16, UserFuncDefinition> = phf_map! {
    0x75u16 => UserFuncDefinition::new(
        "AddCar",
        &[UserFuncType::String]
    ),
    0xd9 => UserFuncDefinition::new(
        "AddCarItem",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0xda => UserFuncDefinition::new(
        "AddCarItemById",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x29 => UserFuncDefinition::new(
        "AddGotoOrder",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0xaa => UserFuncDefinition::new(
        "AddGotoPointOrder",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xe4 => UserFuncDefinition::new(
        "AddInventoryItem",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0xe5 => UserFuncDefinition::new(
        "AddInventoryItemById",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x1f => UserFuncDefinition::new(
        "AddItemInCode",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x5c => UserFuncDefinition::new(
        "AddItemToSafe",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x59 => UserFuncDefinition::new(
        "AddNonDeletableAction",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0x3d => UserFuncDefinition::new(
        "AddSectorItem",
        &[UserFuncType::String, UserFuncType::String, UserFuncType::String]
    ),
    0x13f => UserFuncDefinition::new(
        "AddSlashes",
        &[UserFuncType::String]
    ),
    0x7e => UserFuncDefinition::new(
        "AddTurnToAction",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0x6c => UserFuncDefinition::new(
        "AddUseAction",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x7d => UserFuncDefinition::new(
        "AddWaitAction",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xd6 => UserFuncDefinition::new(
        "AppendNode",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xd5 => UserFuncDefinition::new(
        "AppendNodeToConfig",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::String]
    ),
    0x2a => UserFuncDefinition::new(
        "AttackPatrol",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x97 => UserFuncDefinition::new(
        "Briefing_AllowCar",
        &[UserFuncType::Integer]
    ),
    0x95 => UserFuncDefinition::new(
        "Briefing_AllowHire",
        &[UserFuncType::Integer]
    ),
    0x98 => UserFuncDefinition::new(
        "Briefing_AllowInfo",
        &[UserFuncType::Integer]
    ),
    0x96 => UserFuncDefinition::new(
        "Briefing_AllowInventory",
        &[UserFuncType::Integer]
    ),
    0xb0 => UserFuncDefinition::new(
        "Briefing_SetMaxTeamSize",
        &[UserFuncType::Integer]
    ),
    0x99 => UserFuncDefinition::new(
        "Briefing_SetMissionDescription",
        &[UserFuncType::String]
    ),
    0xa6 => UserFuncDefinition::new(
        "Briefing_SetMissionInfo",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0xad => UserFuncDefinition::new(
        "Briefing_SetMissionInfoFace",
        &[UserFuncType::String]
    ),
    0xac => UserFuncDefinition::new(
        "Briefing_SetMissionInfoName",
        &[UserFuncType::String]
    ),
    0x9d => UserFuncDefinition::new(
        "Briefing_SetMissionOwnerFace",
        &[UserFuncType::String]
    ),
    0x9c => UserFuncDefinition::new(
        "Briefing_SetMissionOwnerName",
        &[UserFuncType::String]
    ),
    0x9a => UserFuncDefinition::new(
        "Briefing_SetMissionPrimaryGoal",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0x9b => UserFuncDefinition::new(
        "Briefing_SetMissionSecondaryGoal",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0x9e => UserFuncDefinition::new(
        "Briefing_SetSector",
        &[UserFuncType::String]
    ),
    0x5b => UserFuncDefinition::new(
        "CallShop",
        &[UserFuncType::String]
    ),
    0xa2 => UserFuncDefinition::new(
        "CallShopAndStore",
        &[UserFuncType::String]
    ),
    0xf0 => UserFuncDefinition::new(
        "CallSpecShop",
        &[UserFuncType::String]
    ),
    0x48 => UserFuncDefinition::new(
        "CanHear",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x41 => UserFuncDefinition::new(
        "CanSee",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x2b => UserFuncDefinition::new(
        "CancelAttackOrder",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x57 => UserFuncDefinition::new(
        "ChangeArmy",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x56 => UserFuncDefinition::new(
        "ChangeCombatant",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0xb9 => UserFuncDefinition::new(
        "ChangeItemOwner",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x55 => UserFuncDefinition::new(
        "ChangeModel",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x58 => UserFuncDefinition::new(
        "ChangeRank",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x11f => UserFuncDefinition::new(
        "CheckBayonet",
        &[UserFuncType::Integer]
    ),
    300 => UserFuncDefinition::new(
        "CheckItemUnrecognized",
        &[UserFuncType::String]
    ),
    0x7f => UserFuncDefinition::new(
        "ClearActions",
        &[UserFuncType::Integer]
    ),
    0x38 => UserFuncDefinition::new(
        "ClearDisplayImage",
        &[]
    ),
    0x137 => UserFuncDefinition::new(
        "ClearGroupInventory",
        &[UserFuncType::Integer]
    ),
    0x11 => UserFuncDefinition::new(
        "ClearLocalAttitudes",
        &[]
    ),
    0x128 => UserFuncDefinition::new(
        "ClearMercsContractTimeout",
        &[]
    ),
    0x12d => UserFuncDefinition::new(
        "ClearShop",
        &[UserFuncType::String]
    ),
    0x7c => UserFuncDefinition::new(
        "ClearWounds",
        &[UserFuncType::Integer]
    ),
    0xca => UserFuncDefinition::new(
        "CloseConfig",
        &[UserFuncType::Integer]
    ),
    0xcb => UserFuncDefinition::new(
        "CloseNode",
        &[UserFuncType::Integer]
    ),
    0xcc => UserFuncDefinition::new(
        "CloseNodeSet",
        &[UserFuncType::Integer]
    ),
    0x27 => UserFuncDefinition::new(
        "CreateGlobalMonster",
        &[UserFuncType::String]
    ),
    0x28 => UserFuncDefinition::new(
        "CreateNewPatrol",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0xa9 => UserFuncDefinition::new(
        "CreateNewPatrolOnPoint",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xd2 => UserFuncDefinition::new(
        "CreateNode",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x2e => UserFuncDefinition::new(
        "CreatePlayerPatrol",
        &[UserFuncType::Integer]
    ),
    0x126 => UserFuncDefinition::new(
        "CreateShopItemsAndMove",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x84 => UserFuncDefinition::new(
        "DeHighlightPatrol",
        &[UserFuncType::Integer]
    ),
    0x134 => UserFuncDefinition::new(
        "DecreaseItemCount",
        &[UserFuncType::Integer]
    ),
    0x20 => UserFuncDefinition::new(
        "DelItemInCode",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0xdd => UserFuncDefinition::new(
        "DeleteCarItem",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0xde => UserFuncDefinition::new(
        "DeleteCarItemById",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x66 => UserFuncDefinition::new(
        "DeleteFromSectorByININame",
        &[UserFuncType::String]
    ),
    0x117 => UserFuncDefinition::new(
        "DeleteGroundItems",
        &[]
    ),
    0xe0 => UserFuncDefinition::new(
        "DeleteInventoryItem",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0xe1 => UserFuncDefinition::new(
        "DeleteInventoryItemById",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x131 => UserFuncDefinition::new(
        "DeleteItemByID",
        &[UserFuncType::Integer]
    ),
    0x2f => UserFuncDefinition::new(
        "DeletePatrol",
        &[UserFuncType::Integer]
    ),
    0x25 => UserFuncDefinition::new(
        "DestinationPlace",
        &[UserFuncType::String]
    ),
    0xa5 => UserFuncDefinition::new(
        "DestructObject",
        &[UserFuncType::String]
    ),
    0x3b => UserFuncDefinition::new(
        "DisplayPhrase",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0xe => UserFuncDefinition::new(
        "DisplaySystemMessage",
        &[UserFuncType::String]
    ),
    0x19 => UserFuncDefinition::new(
        "DoLoadSectorFromCode",
        &[UserFuncType::String]
    ),
    0x11a => UserFuncDefinition::new(
        "DoLua",
        &[UserFuncType::String]
    ),
    0x13a => UserFuncDefinition::new(
        "DoLuaWithResult",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0x36 => UserFuncDefinition::new(
        "DoUnloadCurrentSector",
        &[]
    ),
    0x3e => UserFuncDefinition::new(
        "DoUnloadSectorFromCode",
        &[UserFuncType::String]
    ),
    0x132 => UserFuncDefinition::new(
        "DropArmedGrenade",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    99 => UserFuncDefinition::new(
        "DumpQuestState",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0x14 => UserFuncDefinition::new(
        "DumpUF",
        &[UserFuncType::String]
    ),
    0xbb => UserFuncDefinition::new(
        "EnableSaveOnBattle",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x50 => UserFuncDefinition::new(
        "EndGame",
        &[]
    ),
    0x46 => UserFuncDefinition::new(
        "EnumerateNearbyPatrols",
        &[]
    ),
    0x32 => UserFuncDefinition::new(
        "EnumeratePlayerTeamCallback",
        &[UserFuncType::String]
    ),
    0x1c => UserFuncDefinition::new(
        "ExitFromSector",
        &[UserFuncType::Integer]
    ),
    0xf4 => UserFuncDefinition::new(
        "FindInventoryItem",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0x120 => UserFuncDefinition::new(
        "FindNextWeaponItem",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xd => UserFuncDefinition::new(
        "FinishQuestTalk",
        &[]
    ),
    0x2c => UserFuncDefinition::new(
        "FollowPatrol",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x40 => UserFuncDefinition::new(
        "ForceKill",
        &[UserFuncType::String]
    ),
    0x4b => UserFuncDefinition::new(
        "ForceKillByID",
        &[UserFuncType::Integer]
    ),
    0x8f => UserFuncDefinition::new(
        "GetAttachmentCondition",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x8e => UserFuncDefinition::new(
        "GetAttachmentName",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x6a => UserFuncDefinition::new(
        "GetAttribute",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xe8 => UserFuncDefinition::new(
        "GetAttributePerm",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x10e => UserFuncDefinition::new(
        "GetBit",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xf3 => UserFuncDefinition::new(
        "GetBoxAmmoCount",
        &[UserFuncType::Integer]
    ),
    0xd8 => UserFuncDefinition::new(
        "GetCarItemCount",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0x81 => UserFuncDefinition::new(
        "GetCarItemsTotalPrice",
        &[UserFuncType::String]
    ),
    0xf6 => UserFuncDefinition::new(
        "GetClipPlugType",
        &[UserFuncType::String]
    ),
    0x8a => UserFuncDefinition::new(
        "GetConditionOfItemInHands",
        &[UserFuncType::Integer]
    ),
    0xbe => UserFuncDefinition::new(
        "GetConfig",
        &[UserFuncType::String]
    ),
    0xc6 => UserFuncDefinition::new(
        "GetConfigInt",
        &[UserFuncType::Integer, UserFuncType::String, UserFuncType::Integer]
    ),
    0xbf => UserFuncDefinition::new(
        "GetConfigNodeSet",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    199 => UserFuncDefinition::new(
        "GetConfigString",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0xb8 => UserFuncDefinition::new(
        "GetCurrentMerc",
        &[]
    ),
    0x22 => UserFuncDefinition::new(
        "GetCurrentPlace",
        &[UserFuncType::String]
    ),
    0x24 => UserFuncDefinition::new(
        "GetCurrentSectorName",
        &[]
    ),
    0x118 => UserFuncDefinition::new(
        "GetCurrentShopName",
        &[]
    ),
    0xeb => UserFuncDefinition::new(
        "GetDiscount",
        &[UserFuncType::String]
    ),
    0x87 => UserFuncDefinition::new(
        "GetDisplayItemName",
        &[UserFuncType::String]
    ),
    0x123 => UserFuncDefinition::new(
        "GetDisplayItemNameByIndex",
        &[UserFuncType::Integer]
    ),
    0x47 => UserFuncDefinition::new(
        "GetDistanceFromItem",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0x49 => UserFuncDefinition::new(
        "GetDistanceFromMonster",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x127 => UserFuncDefinition::new(
        "GetGameTime",
        &[]
    ),
    0x116 => UserFuncDefinition::new(
        "GetGroundItemsPrice",
        &[UserFuncType::String]
    ),
    0xfe => UserFuncDefinition::new(
        "GetHLASkill",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xf1 => UserFuncDefinition::new(
        "GetHandsItemId",
        &[UserFuncType::Integer]
    ),
    0xec => UserFuncDefinition::new(
        "GetHandsItemResource",
        &[UserFuncType::Integer]
    ),
    0x68 => UserFuncDefinition::new(
        "GetHealth",
        &[UserFuncType::Integer]
    ),
    0xdf => UserFuncDefinition::new(
        "GetInventoryItemCount",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0x72 => UserFuncDefinition::new(
        "GetItemFamily",
        &[UserFuncType::String]
    ),
    0x12e => UserFuncDefinition::new(
        "GetItemFromSafe",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x122 => UserFuncDefinition::new(
        "GetItemININameByIndex",
        &[UserFuncType::Integer]
    ),
    0xf8 => UserFuncDefinition::new(
        "GetItemINIPrice",
        &[UserFuncType::String]
    ),
    0x86 => UserFuncDefinition::new(
        "GetItemInHands",
        &[UserFuncType::Integer]
    ),
    0x71 => UserFuncDefinition::new(
        "GetItemLevel",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0xfb => UserFuncDefinition::new(
        "GetItemLevelEx",
        &[UserFuncType::String]
    ),
    0x139 => UserFuncDefinition::new(
        "GetItemOtherType",
        &[UserFuncType::String]
    ),
    0x73 => UserFuncDefinition::new(
        "GetItemPrice",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0xed => UserFuncDefinition::new(
        "GetItemResourceById",
        &[UserFuncType::Integer]
    ),
    0x108 => UserFuncDefinition::new(
        "GetItemSubcount",
        &[UserFuncType::Integer]
    ),
    0xfa => UserFuncDefinition::new(
        "GetItemWeight",
        &[UserFuncType::Integer]
    ),
    0x34 => UserFuncDefinition::new(
        "GetMinutesTo",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xb3 => UserFuncDefinition::new(
        "GetMonsterWearName",
        &[UserFuncType::String]
    ),
    10 => UserFuncDefinition::new(
        "GetNameByININame",
        &[UserFuncType::String]
    ),
    0x67 => UserFuncDefinition::new(
        "GetNationality",
        &[UserFuncType::Integer]
    ),
    0xc4 => UserFuncDefinition::new(
        "GetNodeAttributeInt",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0xc5 => UserFuncDefinition::new(
        "GetNodeAttributeString",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    200 => UserFuncDefinition::new(
        "GetNodeInt",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0xd0 => UserFuncDefinition::new(
        "GetNodeSetAttributeInt",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::String]
    ),
    0xd1 => UserFuncDefinition::new(
        "GetNodeSetAttributeString",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::String]
    ),
    0xc1 => UserFuncDefinition::new(
        "GetNodeSetItem",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xc0 => UserFuncDefinition::new(
        "GetNodeSetSize",
        &[UserFuncType::Integer]
    ),
    0xce => UserFuncDefinition::new(
        "GetNodeSetValueInt",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xcf => UserFuncDefinition::new(
        "GetNodeSetValueString",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xc9 => UserFuncDefinition::new(
        "GetNodeString",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0xc2 => UserFuncDefinition::new(
        "GetNodeValueInt",
        &[UserFuncType::Integer]
    ),
    0xc3 => UserFuncDefinition::new(
        "GetNodeValueString",
        &[UserFuncType::Integer]
    ),
    0x8d => UserFuncDefinition::new(
        "GetNumberOfAttachments",
        &[UserFuncType::Integer]
    ),
    0x85 => UserFuncDefinition::new(
        "GetNumberOfItemInShop",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0xd7 => UserFuncDefinition::new(
        "GetPlayerInventory",
        &[UserFuncType::Integer]
    ),
    0xe6 => UserFuncDefinition::new(
        "GetPlayerInventoryByINI",
        &[UserFuncType::String]
    ),
    8 => UserFuncDefinition::new(
        "GetPlayerMoney",
        &[]
    ),
    0x80 => UserFuncDefinition::new(
        "GetPlayerTalkerID",
        &[]
    ),
    0x8b => UserFuncDefinition::new(
        "GetResourceConditionOfItemInHands",
        &[UserFuncType::Integer]
    ),
    0x5f => UserFuncDefinition::new(
        "GetSexByID",
        &[UserFuncType::Integer]
    ),
    0x119 => UserFuncDefinition::new(
        "GetShopDisplayName",
        &[UserFuncType::String]
    ),
    0x70 => UserFuncDefinition::new(
        "GetShopItem",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0x114 => UserFuncDefinition::new(
        "GetShopMoney",
        &[UserFuncType::String]
    ),
    0xf2 => UserFuncDefinition::new(
        "GetStackSize",
        &[UserFuncType::Integer]
    ),
    0x60 => UserFuncDefinition::new(
        "GetTalkerID",
        &[]
    ),
    0x6f => UserFuncDefinition::new(
        "GetTotalNumberOfItemsInShop",
        &[UserFuncType::String]
    ),
    0x39 => UserFuncDefinition::new(
        "GetTravelTime",
        &[UserFuncType::String, UserFuncType::String, UserFuncType::Integer]
    ),
    0x3a => UserFuncDefinition::new(
        "GetTravelTimeTo",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0x125 => UserFuncDefinition::new(
        "GetWeaponByWeaponIndex",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x124 => UserFuncDefinition::new(
        "GetWeaponClass",
        &[UserFuncType::String]
    ),
    0x121 => UserFuncDefinition::new(
        "GetWeaponCount",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x10d => UserFuncDefinition::new(
        "GetWoundsBitmask",
        &[UserFuncType::Integer]
    ),
    0x10f => UserFuncDefinition::new(
        "GetWoundsCount",
        &[UserFuncType::Integer]
    ),
    0x89 => UserFuncDefinition::new(
        "GoToMonster",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x1d => UserFuncDefinition::new(
        "GoToPlace",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0x3c => UserFuncDefinition::new(
        "HardPauseGame",
        &[]
    ),
    0x26 => UserFuncDefinition::new(
        "HasActiveActions",
        &[UserFuncType::String]
    ),
    0x76 => UserFuncDefinition::new(
        "HasCar",
        &[]
    ),
    0x21 => UserFuncDefinition::new(
        "HasItemInCode",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x74 => UserFuncDefinition::new(
        "HasLocalEnemy",
        &[UserFuncType::Integer]
    ),
    0x45 => UserFuncDefinition::new(
        "HideBattle",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x83 => UserFuncDefinition::new(
        "HighlightPatrol",
        &[UserFuncType::Integer]
    ),
    0xe7 => UserFuncDefinition::new(
        "IsFileExists",
        &[UserFuncType::String]
    ),
    0xaf => UserFuncDefinition::new(
        "IsMonsterInPlace",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x62 => UserFuncDefinition::new(
        "IsSectorNearby",
        &[UserFuncType::Integer]
    ),
    0x11c => UserFuncDefinition::new(
        "ItemGetAddon",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x78 => UserFuncDefinition::new(
        "LeaveCar",
        &[]
    ),
    0xa7 => UserFuncDefinition::new(
        "ListenForMonsterEvents",
        &[UserFuncType::Integer]
    ),
    0xb => UserFuncDefinition::new(
        "LoadGlobalMonster",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0xf => UserFuncDefinition::new(
        "LoadLocalMonster",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x136 => UserFuncDefinition::new(
        "LoadLocalMonsterInventory",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x13b => UserFuncDefinition::new(
        "LuaResultInt",
        &[UserFuncType::Integer]
    ),
    0x13c => UserFuncDefinition::new(
        "LuaResultString",
        &[UserFuncType::Integer]
    ),
    0x77 => UserFuncDefinition::new(
        "MakeExplosion",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x106 => UserFuncDefinition::new(
        "MonsterClearActions",
        &[UserFuncType::Integer]
    ),
    0x109 => UserFuncDefinition::new(
        "MonsterDropItem",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x103 => UserFuncDefinition::new(
        "MonsterGetPose",
        &[UserFuncType::Integer]
    ),
    0x11d => UserFuncDefinition::new(
        "MonsterGetShocked",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x104 => UserFuncDefinition::new(
        "MonsterGetWalkType",
        &[UserFuncType::Integer]
    ),
    0x10a => UserFuncDefinition::new(
        "MonsterRemoveItem",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x107 => UserFuncDefinition::new(
        "MonsterSetLightState",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x102 => UserFuncDefinition::new(
        "MonsterSetPose",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x105 => UserFuncDefinition::new(
        "MonsterSetPoseImmediate",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xb4 => UserFuncDefinition::new(
        "MoveCarInventoryToStore",
        &[]
    ),
    0xef => UserFuncDefinition::new(
        "MoveCarItemsToShop",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0x115 => UserFuncDefinition::new(
        "MoveGroundItemsToShop",
        &[UserFuncType::String]
    ),
    0xae => UserFuncDefinition::new(
        "MoveGroundToStore",
        &[]
    ),
    0x100 => UserFuncDefinition::new(
        "MoveInventoryToShop",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0xa3 => UserFuncDefinition::new(
        "MoveInventoryToStore",
        &[UserFuncType::String]
    ),
    0xee => UserFuncDefinition::new(
        "MoveItemToShop",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0x111 => UserFuncDefinition::new(
        "MoveShopContent",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x11b => UserFuncDefinition::new(
        "MoveShopToCar",
        &[UserFuncType::String]
    ),
    0xa8 => UserFuncDefinition::new(
        "NotListenForMonsterEvents",
        &[UserFuncType::Integer]
    ),
    0x23 => UserFuncDefinition::new(
        "PlayerInSector",
        &[]
    ),
    0x65 => UserFuncDefinition::new(
        "PlayerKnowsSector",
        &[UserFuncType::String]
    ),
    0x31 => UserFuncDefinition::new(
        "PrintAssert",
        &[UserFuncType::String]
    ),
    0x112 => UserFuncDefinition::new(
        "PrintToConsole",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0x15 => UserFuncDefinition::new(
        "PrintToJournalExact",
        &[UserFuncType::Integer, UserFuncType::String, UserFuncType::String]
    ),
    0xf7 => UserFuncDefinition::new(
        "PrintToLog",
        &[UserFuncType::String]
    ),
    0x13 => UserFuncDefinition::new(
        "PrintUF_",
        &[UserFuncType::String]
    ),
    0x1a => UserFuncDefinition::new(
        "QuietLoadingProgress",
        &[UserFuncType::Integer]
    ),
    0x30 => UserFuncDefinition::new(
        "RND",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xf9 => UserFuncDefinition::new(
        "RailRoadAutoSave",
        &[UserFuncType::String]
    ),
    0xfd => UserFuncDefinition::new(
        "ReadFloat",
        &[UserFuncType::Integer]
    ),
    0xfc => UserFuncDefinition::new(
        "ReadInt",
        &[UserFuncType::Integer]
    ),
    6 => UserFuncDefinition::new(
        "ReadString",
        &[UserFuncType::Integer]
    ),
    0x12 => UserFuncDefinition::new(
        "RebuildSides",
        &[]
    ),
    299 => UserFuncDefinition::new(
        "RecognizeItem",
        &[UserFuncType::String]
    ),
    0x16 => UserFuncDefinition::new(
        "RegisterSector",
        &[UserFuncType::String]
    ),
    0x5a => UserFuncDefinition::new(
        "RegisterShop",
        &[UserFuncType::String]
    ),
    0x133 => UserFuncDefinition::new(
        "RemoveAllMines",
        &[]
    ),
    0xdb => UserFuncDefinition::new(
        "RemoveCarItem",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0xdc => UserFuncDefinition::new(
        "RemoveCarItemById",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xe2 => UserFuncDefinition::new(
        "RemoveInventoryItem",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0xe3 => UserFuncDefinition::new(
        "RemoveInventoryItemById",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x5d => UserFuncDefinition::new(
        "RemoveItemFromSafe",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x12f => UserFuncDefinition::new(
        "RemoveItemFromSafeByID",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0x130 => UserFuncDefinition::new(
        "RemoveItemFromShop",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x138 => UserFuncDefinition::new(
        "RequestPlaceCreation",
        &[UserFuncType::String]
    ),
    0x93 => UserFuncDefinition::new(
        "ResetStats",
        &[]
    ),
    0x101 => UserFuncDefinition::new(
        "RestoreMaxStamina",
        &[UserFuncType::Integer]
    ),
    0x6d => UserFuncDefinition::new(
        "SQRT",
        &[UserFuncType::Integer]
    ),
    0xcd => UserFuncDefinition::new(
        "SaveConfig",
        &[UserFuncType::Integer]
    ),
    0x135 => UserFuncDefinition::new(
        "SaveLocalMonsterInventory",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x82 => UserFuncDefinition::new(
        "SellCarItems",
        &[UserFuncType::String]
    ),
    0x90 => UserFuncDefinition::new(
        "SetAttachmentCondition",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x6b => UserFuncDefinition::new(
        "SetAttribute",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xe9 => UserFuncDefinition::new(
        "SetAttributePerm",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xa4 => UserFuncDefinition::new(
        "SetAvailableCar",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0x92 => UserFuncDefinition::new(
        "SetAvailableMerc",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x8c => UserFuncDefinition::new(
        "SetConditionOfItemInHands",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    100 => UserFuncDefinition::new(
        "SetCurrentGameLevel",
        &[UserFuncType::Integer]
    ),
    0x5e => UserFuncDefinition::new(
        "SetDiscount",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0xea => UserFuncDefinition::new(
        "SetDiscountEx",
        &[UserFuncType::String, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x37 => UserFuncDefinition::new(
        "SetDisplayImage",
        &[UserFuncType::String]
    ),
    0xb2 => UserFuncDefinition::new(
        "SetEnergy",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xff => UserFuncDefinition::new(
        "SetHLASkill",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x69 => UserFuncDefinition::new(
        "SetHealth",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xbd => UserFuncDefinition::new(
        "SetItemCondition",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x11e => UserFuncDefinition::new(
        "SetItemMoneyCount",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x10b => UserFuncDefinition::new(
        "SetItemSubcount",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xb7 => UserFuncDefinition::new(
        "SetItemUseString",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x10 => UserFuncDefinition::new(
        "SetLocalRelations",
        &[UserFuncType::Integer, UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x4e => UserFuncDefinition::new(
        "SetLockInSector",
        &[UserFuncType::String, UserFuncType::String, UserFuncType::Integer]
    ),
    0x35 => UserFuncDefinition::new(
        "SetMayExitFromSector",
        &[UserFuncType::Integer]
    ),
    0x9f => UserFuncDefinition::new(
        "SetMayOpenGlobalMap",
        &[UserFuncType::Integer]
    ),
    0xa1 => UserFuncDefinition::new(
        "SetMayUseQuestObj",
        &[UserFuncType::Integer]
    ),
    0x129 => UserFuncDefinition::new(
        "SetMercContractTimeout",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x12a => UserFuncDefinition::new(
        "SetMercWeaponAdoptation",
        &[UserFuncType::Integer, UserFuncType::String, UserFuncType::Integer]
    ),
    0x4f => UserFuncDefinition::new(
        "SetMine",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0xd4 => UserFuncDefinition::new(
        "SetNodeAttribute",
        &[UserFuncType::Integer, UserFuncType::String, UserFuncType::String]
    ),
    0xd3 => UserFuncDefinition::new(
        "SetNodeValue",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0x6e => UserFuncDefinition::new(
        "SetNumberOfItemInShop",
        &[UserFuncType::String, UserFuncType::String, UserFuncType::Integer]
    ),
    0xf5 => UserFuncDefinition::new(
        "SetPatrolSpeed",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    9 => UserFuncDefinition::new(
        "SetPlayerMoney",
        &[UserFuncType::Integer]
    ),
    0x7a => UserFuncDefinition::new(
        "SetProfessionalship",
        &[UserFuncType::Integer]
    ),
    0xa0 => UserFuncDefinition::new(
        "SetQuestObjCursorHint",
        &[UserFuncType::String]
    ),
    0x7b => UserFuncDefinition::new(
        "SetReputation",
        &[UserFuncType::Integer]
    ),
    0x79 => UserFuncDefinition::new(
        "SetReputationInSector",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0x18 => UserFuncDefinition::new(
        "SetSectorOwnerInCode",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0x113 => UserFuncDefinition::new(
        "SetShopMoney",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0xab => UserFuncDefinition::new(
        "SetSquadVisRange",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xb1 => UserFuncDefinition::new(
        "SetStatsStatus",
        &[UserFuncType::Integer]
    ),
    0x51 => UserFuncDefinition::new(
        "SetTalkingWithInCode",
        &[UserFuncType::String]
    ),
    0x43 => UserFuncDefinition::new(
        "SetTimer",
        &[UserFuncType::Integer, UserFuncType::String]
    ),
    0x1e => UserFuncDefinition::new(
        "SetToPlace",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x44 => UserFuncDefinition::new(
        "ShowBattle",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0xbc => UserFuncDefinition::new(
        "ShowDialog",
        &[UserFuncType::String, UserFuncType::Integer]
    ),
    0x54 => UserFuncDefinition::new(
        "ShowEnd",
        &[UserFuncType::String]
    ),
    0xba => UserFuncDefinition::new(
        "ShowMap",
        &[UserFuncType::Integer]
    ),
    0x61 => UserFuncDefinition::new(
        "ShowMechanicDialog",
        &[UserFuncType::String]
    ),
    0x52 => UserFuncDefinition::new(
        "ShowMovie",
        &[UserFuncType::String]
    ),
    0x42 => UserFuncDefinition::new(
        "ShowNPC",
        &[UserFuncType::String]
    ),
    0x4d => UserFuncDefinition::new(
        "ShowPlaceInSector",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0x4c => UserFuncDefinition::new(
        "ShowSector",
        &[UserFuncType::String]
    ),
    0x53 => UserFuncDefinition::new(
        "ShowSex",
        &[]
    ),
    0x94 => UserFuncDefinition::new(
        "ShowStatsScreen",
        &[]
    ),
    5 => UserFuncDefinition::new(
        "SkipAnyIdent",
        &[UserFuncType::Integer]
    ),
    4 => UserFuncDefinition::new(
        "SkipIdentsUntil",
        &[UserFuncType::Integer, UserFuncType::String, UserFuncType::Integer]
    ),
    0x33 => UserFuncDefinition::new(
        "SkipTime",
        &[UserFuncType::Integer, UserFuncType::Integer]
    ),
    0x10c => UserFuncDefinition::new(
        "StackableItemsEnabled",
        &[]
    ),
    0x91 => UserFuncDefinition::new(
        "StartBriefing",
        &[]
    ),
    1 => UserFuncDefinition::new(
        "StartParsing",
        &[UserFuncType::String]
    ),
    0xc => UserFuncDefinition::new(
        "StartQuestTalk",
        &[UserFuncType::String]
    ),
    2 => UserFuncDefinition::new(
        "StopParsing",
        &[UserFuncType::Integer]
    ),
    0x1b => UserFuncDefinition::new(
        "StopTheGame",
        &[]
    ),
    0xb5 => UserFuncDefinition::new(
        "TempFunc181",
        &[UserFuncType::String, UserFuncType::String]
    ),
    0xb6 => UserFuncDefinition::new(
        "TempFunc182",
        &[UserFuncType::String, UserFuncType::String]
    ),
    7 => UserFuncDefinition::new(
        "TestEOF",
        &[UserFuncType::Integer]
    ),
    3 => UserFuncDefinition::new(
        "TestIdent",
        &[UserFuncType::Integer, UserFuncType::String, UserFuncType::Integer]
    ),
    0x3f => UserFuncDefinition::new(
        "ThrowFireMessages",
        &[UserFuncType::Integer]
    ),
    0x88 => UserFuncDefinition::new(
        "TryEnterSector",
        &[UserFuncType::String]
    ),
    0x4a => UserFuncDefinition::new(
        "UnloadGlobalMonster",
        &[UserFuncType::String]
    ),
    0x17 => UserFuncDefinition::new(
        "UnregisterSector",
        &[UserFuncType::String]
    ),
    0x2d => UserFuncDefinition::new(
        "UpdatePatrolVisibility",
        &[UserFuncType::Integer]
    ),
    0x110 => UserFuncDefinition::new(
        "WriteToLog",
        &[UserFuncType::String]
    ),
};
