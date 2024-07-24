struct ACTIONBUTTON_PARAM_ST {
#[allow(unused)]
	regionType: u8,
#[allow(unused)]
	category: u8,
#[allow(unused)]
	padding1: [u8;2],
#[allow(unused)]
	dummyPoly1: i32,
#[allow(unused)]
	dummyPoly2: i32,
#[allow(unused)]
	radius: f32,
#[allow(unused)]
	angle: i32,
#[allow(unused)]
	depth: f32,
#[allow(unused)]
	width: f32,
#[allow(unused)]
	height: f32,
#[allow(unused)]
	baseHeightOffset: f32,
#[allow(unused)]
	angleCheckType: u8,
#[allow(unused)]
	padding2: [u8;3],
#[allow(unused)]
	allowAngle: i32,
#[allow(unused)]
	spotDummyPoly: i32,
#[allow(unused)]
	textBoxType: u8,
#[allow(unused)]
	padding3: [u8;2],
#[allow(unused)]
	padding5: [u8;1],
#[allow(unused)]
	isInvalidForRide: u8,
#[allow(unused)]
	isGrayoutForRide: u8,
#[allow(unused)]
	isInvalidForCrouching: u8,
#[allow(unused)]
	isGrayoutForCrouching: u8,
#[allow(unused)]
	padding4: [u8;1],
#[allow(unused)]
	textId: i32,
#[allow(unused)]
	invalidFlag: i32,
#[allow(unused)]
	grayoutFlag: i32,
#[allow(unused)]
	overrideActionButtonIdForRide: i32,
#[allow(unused)]
	execInvalidTime: f32,
#[allow(unused)]
	padding6: [u8;28],
}

struct AI_ANIM_TBL_PARAM {
#[allow(unused)]
	atk0_EzStateId: i16,
#[allow(unused)]
	atk1_EzStateId: i16,
#[allow(unused)]
	atk2_EzStateId: i16,
#[allow(unused)]
	atk3_EzStateId: i16,
#[allow(unused)]
	atk4_EzStateId: i16,
#[allow(unused)]
	atk5_EzStateId: i16,
#[allow(unused)]
	atk6_EzStateId: i16,
#[allow(unused)]
	atk7_EzStateId: i16,
#[allow(unused)]
	atk8_EzStateId: i16,
#[allow(unused)]
	atk9_EzStateId: i16,
#[allow(unused)]
	atk10_EzStateId: i16,
#[allow(unused)]
	atk11_EzStateId: i16,
#[allow(unused)]
	atk12_EzStateId: i16,
#[allow(unused)]
	atk13_EzStateId: i16,
#[allow(unused)]
	atk14_EzStateId: i16,
#[allow(unused)]
	atk15_EzStateId: i16,
#[allow(unused)]
	atk16_EzStateId: i16,
#[allow(unused)]
	atk17_EzStateId: i16,
#[allow(unused)]
	atk18_EzStateId: i16,
#[allow(unused)]
	atk19_EzStateId: i16,
#[allow(unused)]
	atk20_EzStateId: i16,
#[allow(unused)]
	atk21_EzStateId: i16,
#[allow(unused)]
	atk22_EzStateId: i16,
#[allow(unused)]
	atk23_EzStateId: i16,
#[allow(unused)]
	atk24_EzStateId: i16,
#[allow(unused)]
	atk25_EzStateId: i16,
#[allow(unused)]
	atk26_EzStateId: i16,
#[allow(unused)]
	atk27_EzStateId: i16,
#[allow(unused)]
	atk28_EzStateId: i16,
#[allow(unused)]
	atk29_EzStateId: i16,
#[allow(unused)]
	atk0_MinDist: i16,
#[allow(unused)]
	atk1_MinDist: i16,
#[allow(unused)]
	atk2_MinDist: i16,
#[allow(unused)]
	atk3_MinDist: i16,
#[allow(unused)]
	atk4_MinDist: i16,
#[allow(unused)]
	atk5_MinDist: i16,
#[allow(unused)]
	atk6_MinDist: i16,
#[allow(unused)]
	atk7_MinDist: i16,
#[allow(unused)]
	atk8_MinDist: i16,
#[allow(unused)]
	atk9_MinDist: i16,
#[allow(unused)]
	atk10_MinDist: i16,
#[allow(unused)]
	atk11_MinDist: i16,
#[allow(unused)]
	atk12_MinDist: i16,
#[allow(unused)]
	atk13_MinDist: i16,
#[allow(unused)]
	atk14_MinDist: i16,
#[allow(unused)]
	atk15_MinDist: i16,
#[allow(unused)]
	atk16_MinDist: i16,
#[allow(unused)]
	atk17_MinDist: i16,
#[allow(unused)]
	atk18_MinDist: i16,
#[allow(unused)]
	atk19_MinDist: i16,
#[allow(unused)]
	atk20_MinDist: i16,
#[allow(unused)]
	atk21_MinDist: i16,
#[allow(unused)]
	atk22_MinDist: i16,
#[allow(unused)]
	atk23_MinDist: i16,
#[allow(unused)]
	atk24_MinDist: i16,
#[allow(unused)]
	atk25_MinDist: i16,
#[allow(unused)]
	atk26_MinDist: i16,
#[allow(unused)]
	atk27_MinDist: i16,
#[allow(unused)]
	atk28_MinDist: i16,
#[allow(unused)]
	atk29_MinDist: i16,
#[allow(unused)]
	atk0_MaxDist: i16,
#[allow(unused)]
	atk1_MaxDist: i16,
#[allow(unused)]
	atk2_MaxDist: i16,
#[allow(unused)]
	atk3_MaxDist: i16,
#[allow(unused)]
	atk4_MaxDist: i16,
#[allow(unused)]
	atk5_MaxDist: i16,
#[allow(unused)]
	atk6_MaxDist: i16,
#[allow(unused)]
	atk7_MaxDist: i16,
#[allow(unused)]
	atk8_MaxDist: i16,
#[allow(unused)]
	atk9_MaxDist: i16,
#[allow(unused)]
	atk10_MaxDist: i16,
#[allow(unused)]
	atk11_MaxDist: i16,
#[allow(unused)]
	atk12_MaxDist: i16,
#[allow(unused)]
	atk13_MaxDist: i16,
#[allow(unused)]
	atk14_MaxDist: i16,
#[allow(unused)]
	atk15_MaxDist: i16,
#[allow(unused)]
	atk16_MaxDist: i16,
#[allow(unused)]
	atk17_MaxDist: i16,
#[allow(unused)]
	atk18_MaxDist: i16,
#[allow(unused)]
	atk19_MaxDist: i16,
#[allow(unused)]
	atk20_MaxDist: i16,
#[allow(unused)]
	atk21_MaxDist: i16,
#[allow(unused)]
	atk22_MaxDist: i16,
#[allow(unused)]
	atk23_MaxDist: i16,
#[allow(unused)]
	atk24_MaxDist: i16,
#[allow(unused)]
	atk25_MaxDist: i16,
#[allow(unused)]
	atk26_MaxDist: i16,
#[allow(unused)]
	atk27_MaxDist: i16,
#[allow(unused)]
	atk28_MaxDist: i16,
#[allow(unused)]
	atk29_MaxDist: i16,
#[allow(unused)]
	atk0_AtkDistType: u8,
#[allow(unused)]
	atk1_AtkDistType: u8,
#[allow(unused)]
	atk2_AtkDistType: u8,
#[allow(unused)]
	atk3_AtkDistType: u8,
#[allow(unused)]
	atk4_AtkDistType: u8,
#[allow(unused)]
	atk5_AtkDistType: u8,
#[allow(unused)]
	atk6_AtkDistType: u8,
#[allow(unused)]
	atk7_AtkDistType: u8,
#[allow(unused)]
	atk8_AtkDistType: u8,
#[allow(unused)]
	atk9_AtkDistType: u8,
#[allow(unused)]
	atk10_AtkDistType: u8,
#[allow(unused)]
	atk11_AtkDistType: u8,
#[allow(unused)]
	atk12_AtkDistType: u8,
#[allow(unused)]
	atk13_AtkDistType: u8,
#[allow(unused)]
	atk14_AtkDistType: u8,
#[allow(unused)]
	atk15_AtkDistType: u8,
#[allow(unused)]
	atk16_AtkDistType: u8,
#[allow(unused)]
	atk17_AtkDistType: u8,
#[allow(unused)]
	atk18_AtkDistType: u8,
#[allow(unused)]
	atk19_AtkDistType: u8,
#[allow(unused)]
	atk20_AtkDistType: u8,
#[allow(unused)]
	atk21_AtkDistType: u8,
#[allow(unused)]
	atk22_AtkDistType: u8,
#[allow(unused)]
	atk23_AtkDistType: u8,
#[allow(unused)]
	atk24_AtkDistType: u8,
#[allow(unused)]
	atk25_AtkDistType: u8,
#[allow(unused)]
	atk26_AtkDistType: u8,
#[allow(unused)]
	atk27_AtkDistType: u8,
#[allow(unused)]
	atk28_AtkDistType: u8,
#[allow(unused)]
	atk29_AtkDistType: u8,
#[allow(unused)]
	pad0: [u8;13],
}

struct AI_ATTACK_PARAM_ST {
#[allow(unused)]
	attackTableId: i32,
#[allow(unused)]
	attackId: i32,
#[allow(unused)]
	successDistance: f32,
#[allow(unused)]
	turnTimeBeforeAttack: f32,
#[allow(unused)]
	frontAngleRange: i16,
#[allow(unused)]
	upAngleThreshold: i16,
#[allow(unused)]
	downAngleThershold: i16,
#[allow(unused)]
	isFirstAttack: u8,
#[allow(unused)]
	doesSelectOnOutRange: u8,
#[allow(unused)]
	minOptimalDistance: f32,
#[allow(unused)]
	maxOptimalDistance: f32,
#[allow(unused)]
	baseDirectionForOptimalAngle1: i16,
#[allow(unused)]
	optimalAttackAngleRange1: i16,
#[allow(unused)]
	baseDirectionForOptimalAngle2: i16,
#[allow(unused)]
	optimalAttackAngleRange2: i16,
#[allow(unused)]
	intervalForExec: f32,
#[allow(unused)]
	selectionTendency: f32,
#[allow(unused)]
	shortRangeTendency: f32,
#[allow(unused)]
	middleRangeTendency: f32,
#[allow(unused)]
	farRangeTendency: f32,
#[allow(unused)]
	outRangeTendency: f32,
#[allow(unused)]
	deriveAttackId1: i32,
#[allow(unused)]
	deriveAttackId2: i32,
#[allow(unused)]
	deriveAttackId3: i32,
#[allow(unused)]
	deriveAttackId4: i32,
#[allow(unused)]
	deriveAttackId5: i32,
#[allow(unused)]
	deriveAttackId6: i32,
#[allow(unused)]
	deriveAttackId7: i32,
#[allow(unused)]
	deriveAttackId8: i32,
#[allow(unused)]
	deriveAttackId9: i32,
#[allow(unused)]
	deriveAttackId10: i32,
#[allow(unused)]
	deriveAttackId11: i32,
#[allow(unused)]
	deriveAttackId12: i32,
#[allow(unused)]
	deriveAttackId13: i32,
#[allow(unused)]
	deriveAttackId14: i32,
#[allow(unused)]
	deriveAttackId15: i32,
#[allow(unused)]
	deriveAttackId16: i32,
#[allow(unused)]
	goalLifeMin: f32,
#[allow(unused)]
	goalLifeMax: f32,
#[allow(unused)]
	doesSelectOnInnerRange: u8,
#[allow(unused)]
	enableAttackOnBattleStart: u8,
#[allow(unused)]
	doesSelectOnTargetDown: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	minArriveDistance: f32,
#[allow(unused)]
	maxArriveDistance: f32,
#[allow(unused)]
	comboExecDistance: f32,
#[allow(unused)]
	comboExecRange: f32,
}

struct AI_ODDS_PARAM {
#[allow(unused)]
	act0: u8,
#[allow(unused)]
	act1: u8,
#[allow(unused)]
	act2: u8,
#[allow(unused)]
	act3: u8,
#[allow(unused)]
	act4: u8,
#[allow(unused)]
	act5: u8,
#[allow(unused)]
	act6: u8,
#[allow(unused)]
	act7: u8,
#[allow(unused)]
	act8: u8,
#[allow(unused)]
	act9: u8,
#[allow(unused)]
	act10: u8,
#[allow(unused)]
	act11: u8,
#[allow(unused)]
	act12: u8,
#[allow(unused)]
	act13: u8,
#[allow(unused)]
	act14: u8,
#[allow(unused)]
	act15: u8,
#[allow(unused)]
	act16: u8,
#[allow(unused)]
	act17: u8,
#[allow(unused)]
	act18: u8,
#[allow(unused)]
	act19: u8,
#[allow(unused)]
	act20: u8,
#[allow(unused)]
	act21: u8,
#[allow(unused)]
	act22: u8,
#[allow(unused)]
	act23: u8,
#[allow(unused)]
	act24: u8,
#[allow(unused)]
	act25: u8,
#[allow(unused)]
	act26: u8,
#[allow(unused)]
	act27: u8,
#[allow(unused)]
	act28: u8,
#[allow(unused)]
	act29: u8,
#[allow(unused)]
	act30: u8,
#[allow(unused)]
	act31: u8,
#[allow(unused)]
	act32: u8,
#[allow(unused)]
	act33: u8,
#[allow(unused)]
	act34: u8,
#[allow(unused)]
	act35: u8,
#[allow(unused)]
	act36: u8,
#[allow(unused)]
	act37: u8,
#[allow(unused)]
	act38: u8,
#[allow(unused)]
	act39: u8,
#[allow(unused)]
	act40: u8,
#[allow(unused)]
	act41: u8,
#[allow(unused)]
	act42: u8,
#[allow(unused)]
	act43: u8,
#[allow(unused)]
	act44: u8,
#[allow(unused)]
	act45: u8,
#[allow(unused)]
	act46: u8,
#[allow(unused)]
	act47: u8,
#[allow(unused)]
	act48: u8,
#[allow(unused)]
	act49: u8,
#[allow(unused)]
	act50: u8,
#[allow(unused)]
	act51: u8,
#[allow(unused)]
	act52: u8,
#[allow(unused)]
	act53: u8,
#[allow(unused)]
	act54: u8,
#[allow(unused)]
	act55: u8,
#[allow(unused)]
	act56: u8,
#[allow(unused)]
	act57: u8,
#[allow(unused)]
	act58: u8,
#[allow(unused)]
	act59: u8,
#[allow(unused)]
	act60: u8,
#[allow(unused)]
	act61: u8,
#[allow(unused)]
	act62: u8,
#[allow(unused)]
	act63: u8,
#[allow(unused)]
	act64: u8,
#[allow(unused)]
	act65: u8,
#[allow(unused)]
	act66: u8,
#[allow(unused)]
	act67: u8,
#[allow(unused)]
	act68: u8,
#[allow(unused)]
	act69: u8,
#[allow(unused)]
	act70: u8,
#[allow(unused)]
	act71: u8,
#[allow(unused)]
	act72: u8,
#[allow(unused)]
	act73: u8,
#[allow(unused)]
	act74: u8,
#[allow(unused)]
	act75: u8,
#[allow(unused)]
	act76: u8,
#[allow(unused)]
	act77: u8,
#[allow(unused)]
	act78: u8,
#[allow(unused)]
	act79: u8,
#[allow(unused)]
	act80: u8,
#[allow(unused)]
	act81: u8,
#[allow(unused)]
	act82: u8,
#[allow(unused)]
	act83: u8,
#[allow(unused)]
	act84: u8,
#[allow(unused)]
	act85: u8,
#[allow(unused)]
	act86: u8,
#[allow(unused)]
	act87: u8,
#[allow(unused)]
	act88: u8,
#[allow(unused)]
	act89: u8,
#[allow(unused)]
	act90: u8,
#[allow(unused)]
	act91: u8,
#[allow(unused)]
	act92: u8,
#[allow(unused)]
	act93: u8,
#[allow(unused)]
	act94: u8,
#[allow(unused)]
	act95: u8,
#[allow(unused)]
	act96: u8,
#[allow(unused)]
	act97: u8,
#[allow(unused)]
	act98: u8,
#[allow(unused)]
	act99: u8,
#[allow(unused)]
	pad0: [u8;12],
}

struct AI_SOUND_PARAM_ST {
#[allow(unused)]
	radius: f32,
#[allow(unused)]
	lifeFrame: f32,
#[allow(unused)]
	bSpEffectEnable: u8,
#[allow(unused)]
	type: u8,
#[allow(unused)]
	opposeTarget: u8,
#[allow(unused)]
	friendlyTarget: u8,
#[allow(unused)]
	selfTarget: u8,
#[allow(unused)]
	disableOnTargetPCompany: u8,
#[allow(unused)]
	rank: u8,
#[allow(unused)]
	forgetTime: f32,
#[allow(unused)]
	priority: i32,
#[allow(unused)]
	soundBehaviorId: i32,
#[allow(unused)]
	aiSoundLevel: u8,
#[allow(unused)]
	replaningState: u8,
#[allow(unused)]
	pad1: [u8;6],
}

struct AI_STANDARD_INFO_BANK {
#[allow(unused)]
	RadarRange: i16,
#[allow(unused)]
	RadarAngleX: u8,
#[allow(unused)]
	RadarAngleY: u8,
#[allow(unused)]
	TerritorySize: i16,
#[allow(unused)]
	ThreatBeforeAttackRate: u8,
#[allow(unused)]
	ForceThreatOnFirstLocked: u8,
#[allow(unused)]
	reserve0: [u8;24],
#[allow(unused)]
	Attack1_Distance: i16,
#[allow(unused)]
	Attack1_Margin: i16,
#[allow(unused)]
	Attack1_Rate: u8,
#[allow(unused)]
	Attack1_ActionID: u8,
#[allow(unused)]
	Attack1_DelayMin: u8,
#[allow(unused)]
	Attack1_DelayMax: u8,
#[allow(unused)]
	Attack1_ConeAngle: u8,
#[allow(unused)]
	reserve10: [u8;7],
#[allow(unused)]
	Attack2_Distance: i16,
#[allow(unused)]
	Attack2_Margin: i16,
#[allow(unused)]
	Attack2_Rate: u8,
#[allow(unused)]
	Attack2_ActionID: u8,
#[allow(unused)]
	Attack2_DelayMin: u8,
#[allow(unused)]
	Attack2_DelayMax: u8,
#[allow(unused)]
	Attack2_ConeAngle: u8,
#[allow(unused)]
	reserve11: [u8;7],
#[allow(unused)]
	Attack3_Distance: i16,
#[allow(unused)]
	Attack3_Margin: i16,
#[allow(unused)]
	Attack3_Rate: u8,
#[allow(unused)]
	Attack3_ActionID: u8,
#[allow(unused)]
	Attack3_DelayMin: u8,
#[allow(unused)]
	Attack3_DelayMax: u8,
#[allow(unused)]
	Attack3_ConeAngle: u8,
#[allow(unused)]
	reserve12: [u8;7],
#[allow(unused)]
	Attack4_Distance: i16,
#[allow(unused)]
	Attack4_Margin: i16,
#[allow(unused)]
	Attack4_Rate: u8,
#[allow(unused)]
	Attack4_ActionID: u8,
#[allow(unused)]
	Attack4_DelayMin: u8,
#[allow(unused)]
	Attack4_DelayMax: u8,
#[allow(unused)]
	Attack4_ConeAngle: u8,
#[allow(unused)]
	reserve13: [u8;7],
#[allow(unused)]
	reserve_last: [u8;32],
}

struct ASSET_GEOMETORY_PARAM_ST {
#[allow(unused)]
	soundBankId: i32,
#[allow(unused)]
	soundBreakSEId: i32,
#[allow(unused)]
	refDrawParamId: i32,
#[allow(unused)]
	hitCreateType: i8,
#[allow(unused)]
	behaviorType: u8,
#[allow(unused)]
	collisionType: u8,
#[allow(unused)]
	rainBlockingType: u8,
#[allow(unused)]
	hp: i16,
#[allow(unused)]
	defense: i16,
#[allow(unused)]
	breakStopTime: f32,
#[allow(unused)]
	breakSfxId: i32,
#[allow(unused)]
	breakSfxCpId: i32,
#[allow(unused)]
	breakLandingSfxId: i32,
#[allow(unused)]
	breakBulletBehaviorId: i32,
#[allow(unused)]
	breakBulletCpId: i32,
#[allow(unused)]
	FragmentInvisibleWaitTime: f32,
#[allow(unused)]
	FragmentInvisibleTime: f32,
#[allow(unused)]
	BreakAiSoundID: i32,
#[allow(unused)]
	breakItemLotType: i8,
#[allow(unused)]
	animBreakIdMax: u8,
#[allow(unused)]
	breakBulletAttributeDamageType: i8,
#[allow(unused)]
	isBreakByPlayerCollide: u8,
#[allow(unused)]
	isBreakByEnemyCollide: u8,
#[allow(unused)]
	isBreak_ByChrRide: u8,
#[allow(unused)]
	isDisableBreakForFirstAppear: u8,
#[allow(unused)]
	isAnimBreak: u8,
#[allow(unused)]
	isDamageCover: u8,
#[allow(unused)]
	isAttackBacklash: u8,
#[allow(unused)]
	Reserve_2: [u8;1],
#[allow(unused)]
	isLadder: u8,
#[allow(unused)]
	isMoveObj: u8,
#[allow(unused)]
	isSkydomeFlag: u8,
#[allow(unused)]
	isAnimPauseOnRemoPlay: u8,
#[allow(unused)]
	isBurn: u8,
#[allow(unused)]
	isEnableRepick: u8,
#[allow(unused)]
	isBreakOnPickUp: u8,
#[allow(unused)]
	isBreakByHugeenemyCollide: u8,
#[allow(unused)]
	navimeshFlag: u8,
#[allow(unused)]
	burnBulletInterval: i16,
#[allow(unused)]
	clothUpdateDist: f32,
#[allow(unused)]
	lifeTime_forRuntimeCreate: f32,
#[allow(unused)]
	contactSeId: i32,
#[allow(unused)]
	repickAnimIdOffset: i32,
#[allow(unused)]
	windEffectRate_0: f32,
#[allow(unused)]
	windEffectRate_1: f32,
#[allow(unused)]
	windEffectType_0: u8,
#[allow(unused)]
	windEffectType_1: u8,
#[allow(unused)]
	overrideMaterialId: i16,
#[allow(unused)]
	autoCreateOffsetHeight: f32,
#[allow(unused)]
	burnTime: f32,
#[allow(unused)]
	burnBraekRate: f32,
#[allow(unused)]
	burnSfxId: i32,
#[allow(unused)]
	burnSfxId_1: i32,
#[allow(unused)]
	burnSfxId_2: i32,
#[allow(unused)]
	burnSfxId_3: i32,
#[allow(unused)]
	burnSfxDelayTimeMin: f32,
#[allow(unused)]
	burnSfxDelayTimeMin_1: f32,
#[allow(unused)]
	burnSfxDelayTimeMin_2: f32,
#[allow(unused)]
	burnSfxDelayTimeMin_3: f32,
#[allow(unused)]
	burnSfxDelayTimeMax: f32,
#[allow(unused)]
	burnSfxDelayTimeMax_1: f32,
#[allow(unused)]
	burnSfxDelayTimeMax_2: f32,
#[allow(unused)]
	burnSfxDelayTimeMax_3: f32,
#[allow(unused)]
	burnBulletBehaviorId: i32,
#[allow(unused)]
	burnBulletBehaviorId_1: i32,
#[allow(unused)]
	burnBulletBehaviorId_2: i32,
#[allow(unused)]
	burnBulletBehaviorId_3: i32,
#[allow(unused)]
	burnBulletDelayTime: f32,
#[allow(unused)]
	paintDecalTargetTextureSize: i16,
#[allow(unused)]
	navimeshFlag_after: u8,
#[allow(unused)]
	camNearBehaviorType: i8,
#[allow(unused)]
	breakItemLotParamId: i32,
#[allow(unused)]
	pickUpActionButtonParamId: i32,
#[allow(unused)]
	pickUpItemLotParamId: i32,
#[allow(unused)]
	autoDrawGroupBackFaceCheck: u8,
#[allow(unused)]
	autoDrawGroupDepthWrite: u8,
#[allow(unused)]
	autoDrawGroupShadowTest: u8,
#[allow(unused)]
	debug_isHeightCheckEnable: u8,
#[allow(unused)]
	hitCarverCancelAreaFlag: u8,
#[allow(unused)]
	assetNavimeshNoCombine: u8,
#[allow(unused)]
	navimeshFlagApply: u8,
#[allow(unused)]
	navimeshFlagApply_after: u8,
#[allow(unused)]
	autoDrawGroupPassPixelNum: f32,
#[allow(unused)]
	pickUpReplacementEventFlag: i32,
#[allow(unused)]
	pickUpReplacementAnimIdOffset: i32,
#[allow(unused)]
	pickUpReplacementActionButtonParamId: i32,
#[allow(unused)]
	pickUpReplacementItemLotParamId: i32,
#[allow(unused)]
	slidingBulletHitType: u8,
#[allow(unused)]
	isBushesForDamage: u8,
#[allow(unused)]
	penetrationBulletType: u8,
#[allow(unused)]
	unkR3: u8,
#[allow(unused)]
	unkR4: f32,
#[allow(unused)]
	soundBreakSECpId: i32,
#[allow(unused)]
	debug_HeightCheckCapacityMin: f32,
#[allow(unused)]
	debug_HeightCheckCapacityMax: f32,
#[allow(unused)]
	repickActionButtonParamId: i32,
#[allow(unused)]
	repickItemLotParamId: i32,
#[allow(unused)]
	repickReplacementAnimIdOffset: i32,
#[allow(unused)]
	repickReplacementActionButtonParamId: i32,
#[allow(unused)]
	repickReplacementItemLotParamId: i32,
#[allow(unused)]
	noGenerateCarver: u8,
#[allow(unused)]
	noHitHugeAfterBreak: u8,
#[allow(unused)]
	isEnabledBreakSync: u8,
#[allow(unused)]
	isHiddenOnRepick: u8,
#[allow(unused)]
	isCreateMultiPlayOnly: u8,
#[allow(unused)]
	isDisableBulletHitSfx: u8,
#[allow(unused)]
	isEnableSignPreBreak: u8,
#[allow(unused)]
	isEnableSignPostBreak: u8,
#[allow(unused)]
	unkR1: u8,
#[allow(unused)]
	generateMultiForbiddenRegion: u8,
#[allow(unused)]
	residentSeId0: i32,
#[allow(unused)]
	residentSeId1: i32,
#[allow(unused)]
	residentSeId2: i32,
#[allow(unused)]
	residentSeId3: i32,
#[allow(unused)]
	residentSeDmypolyId0: i16,
#[allow(unused)]
	residentSeDmypolyId1: i16,
#[allow(unused)]
	residentSeDmypolyId2: i16,
#[allow(unused)]
	residentSeDmypolyId3: i16,
#[allow(unused)]
	excludeActivateRatio_Xboxone_Grid: u8,
#[allow(unused)]
	excludeActivateRatio_Xboxone_Legacy: u8,
#[allow(unused)]
	excludeActivateRatio_PS4_Grid: u8,
#[allow(unused)]
	excludeActivateRatio_PS4_Legacy: u8,
#[allow(unused)]
	Reserve_0: [u8;32],
}

struct ASSET_MATERIAL_SFX_PARAM_ST {
#[allow(unused)]
	sfxId_00: i32,
#[allow(unused)]
	sfxId_01: i32,
#[allow(unused)]
	sfxId_02: i32,
#[allow(unused)]
	sfxId_03: i32,
#[allow(unused)]
	sfxId_04: i32,
#[allow(unused)]
	sfxId_05: i32,
#[allow(unused)]
	sfxId_06: i32,
#[allow(unused)]
	sfxId_07: i32,
#[allow(unused)]
	sfxId_08: i32,
#[allow(unused)]
	sfxId_09: i32,
#[allow(unused)]
	sfxId_10: i32,
#[allow(unused)]
	sfxId_11: i32,
#[allow(unused)]
	sfxId_12: i32,
#[allow(unused)]
	sfxId_13: i32,
#[allow(unused)]
	sfxId_14: i32,
#[allow(unused)]
	sfxId_15: i32,
#[allow(unused)]
	sfxId_16: i32,
#[allow(unused)]
	sfxId_17: i32,
#[allow(unused)]
	sfxId_18: i32,
#[allow(unused)]
	sfxId_19: i32,
#[allow(unused)]
	sfxId_20: i32,
#[allow(unused)]
	sfxId_21: i32,
#[allow(unused)]
	sfxId_22: i32,
#[allow(unused)]
	sfxId_23: i32,
#[allow(unused)]
	sfxId_24: i32,
#[allow(unused)]
	sfxId_25: i32,
#[allow(unused)]
	sfxId_26: i32,
#[allow(unused)]
	sfxId_27: i32,
#[allow(unused)]
	sfxId_28: i32,
#[allow(unused)]
	sfxId_29: i32,
#[allow(unused)]
	sfxId_30: i32,
#[allow(unused)]
	sfxId_31: i32,
}

struct ASSET_MODEL_SFX_PARAM_ST {
#[allow(unused)]
	sfxId_0: i32,
#[allow(unused)]
	dmypolyId_0: i32,
#[allow(unused)]
	reserve_0: [u8;8],
#[allow(unused)]
	sfxId_1: i32,
#[allow(unused)]
	dmypolyId_1: i32,
#[allow(unused)]
	reserve_1: [u8;8],
#[allow(unused)]
	sfxId_2: i32,
#[allow(unused)]
	dmypolyId_2: i32,
#[allow(unused)]
	reserve_2: [u8;8],
#[allow(unused)]
	sfxId_3: i32,
#[allow(unused)]
	dmypolyId_3: i32,
#[allow(unused)]
	reserve_3: [u8;8],
#[allow(unused)]
	sfxId_4: i32,
#[allow(unused)]
	dmypolyId_4: i32,
#[allow(unused)]
	reserve_4: [u8;8],
#[allow(unused)]
	sfxId_5: i32,
#[allow(unused)]
	dmypolyId_5: i32,
#[allow(unused)]
	reserve_5: [u8;8],
#[allow(unused)]
	sfxId_6: i32,
#[allow(unused)]
	dmypolyId_6: i32,
#[allow(unused)]
	reserve_6: [u8;8],
#[allow(unused)]
	sfxId_7: i32,
#[allow(unused)]
	dmypolyId_7: i32,
#[allow(unused)]
	isDisableIV: u8,
#[allow(unused)]
	reserve_7: [u8;7],
}

struct ATK_PARAM_ST {
#[allow(unused)]
	hit0_Radius: f32,
#[allow(unused)]
	hit1_Radius: f32,
#[allow(unused)]
	hit2_Radius: f32,
#[allow(unused)]
	hit3_Radius: f32,
#[allow(unused)]
	knockbackDist: f32,
#[allow(unused)]
	hitStopTime: f32,
#[allow(unused)]
	spEffectId0: i32,
#[allow(unused)]
	spEffectId1: i32,
#[allow(unused)]
	spEffectId2: i32,
#[allow(unused)]
	spEffectId3: i32,
#[allow(unused)]
	spEffectId4: i32,
#[allow(unused)]
	hit0_DmyPoly1: i16,
#[allow(unused)]
	hit1_DmyPoly1: i16,
#[allow(unused)]
	hit2_DmyPoly1: i16,
#[allow(unused)]
	hit3_DmyPoly1: i16,
#[allow(unused)]
	hit0_DmyPoly2: i16,
#[allow(unused)]
	hit1_DmyPoly2: i16,
#[allow(unused)]
	hit2_DmyPoly2: i16,
#[allow(unused)]
	hit3_DmyPoly2: i16,
#[allow(unused)]
	blowingCorrection: i16,
#[allow(unused)]
	atkPhysCorrection: i16,
#[allow(unused)]
	atkMagCorrection: i16,
#[allow(unused)]
	atkFireCorrection: i16,
#[allow(unused)]
	atkThunCorrection: i16,
#[allow(unused)]
	atkStamCorrection: i16,
#[allow(unused)]
	guardAtkRateCorrection: i16,
#[allow(unused)]
	guardBreakCorrection: i16,
#[allow(unused)]
	atkThrowEscapeCorrection: i16,
#[allow(unused)]
	subCategory1: u8,
#[allow(unused)]
	subCategory2: u8,
#[allow(unused)]
	atkPhys: i16,
#[allow(unused)]
	atkMag: i16,
#[allow(unused)]
	atkFire: i16,
#[allow(unused)]
	atkThun: i16,
#[allow(unused)]
	atkStam: i16,
#[allow(unused)]
	guardAtkRate: i16,
#[allow(unused)]
	guardBreakRate: i16,
#[allow(unused)]
	pad6: [u8;1],
#[allow(unused)]
	isEnableCalcDamageForBushesObj: u8,
#[allow(unused)]
	atkThrowEscape: i16,
#[allow(unused)]
	atkObj: i16,
#[allow(unused)]
	guardStaminaCutRate: i16,
#[allow(unused)]
	guardRate: i16,
#[allow(unused)]
	throwTypeId: i16,
#[allow(unused)]
	hit0_hitType: u8,
#[allow(unused)]
	hit1_hitType: u8,
#[allow(unused)]
	hit2_hitType: u8,
#[allow(unused)]
	hit3_hitType: u8,
#[allow(unused)]
	hti0_Priority: u8,
#[allow(unused)]
	hti1_Priority: u8,
#[allow(unused)]
	hti2_Priority: u8,
#[allow(unused)]
	hti3_Priority: u8,
#[allow(unused)]
	dmgLevel: u8,
#[allow(unused)]
	mapHitType: u8,
#[allow(unused)]
	guardCutCancelRate: i8,
#[allow(unused)]
	atkAttribute: u8,
#[allow(unused)]
	spAttribute: u8,
#[allow(unused)]
	atkType: u8,
#[allow(unused)]
	atkMaterial: u8,
#[allow(unused)]
	guardRangeType: u8,
#[allow(unused)]
	defSeMaterial1: i16,
#[allow(unused)]
	hitSourceType: u8,
#[allow(unused)]
	throwFlag: u8,
#[allow(unused)]
	disableGuard: u8,
#[allow(unused)]
	disableStaminaAttack: u8,
#[allow(unused)]
	disableHitSpEffect: u8,
#[allow(unused)]
	IgnoreNotifyMissSwingForAI: u8,
#[allow(unused)]
	repeatHitSfx: u8,
#[allow(unused)]
	isArrowAtk: u8,
#[allow(unused)]
	isGhostAtk: u8,
#[allow(unused)]
	isDisableNoDamage: u8,
#[allow(unused)]
	atkPow_forSfx: i8,
#[allow(unused)]
	atkDir_forSfx: i8,
#[allow(unused)]
	opposeTarget: u8,
#[allow(unused)]
	friendlyTarget: u8,
#[allow(unused)]
	selfTarget: u8,
#[allow(unused)]
	isCheckDoorPenetration: u8,
#[allow(unused)]
	isVsRideAtk: u8,
#[allow(unused)]
	isAddBaseAtk: u8,
#[allow(unused)]
	excludeThreatLvNotify: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	atkBehaviorId: u8,
#[allow(unused)]
	atkPow_forSe: i8,
#[allow(unused)]
	atkSuperArmor: f32,
#[allow(unused)]
	decalId1: i32,
#[allow(unused)]
	decalId2: i32,
#[allow(unused)]
	AppearAiSoundId: i32,
#[allow(unused)]
	HitAiSoundId: i32,
#[allow(unused)]
	HitRumbleId: i32,
#[allow(unused)]
	HitRumbleIdByNormal: i32,
#[allow(unused)]
	HitRumbleIdByMiddle: i32,
#[allow(unused)]
	HitRumbleIdByRoot: i32,
#[allow(unused)]
	traceSfxId0: i32,
#[allow(unused)]
	traceDmyIdHead0: i32,
#[allow(unused)]
	traceDmyIdTail0: i32,
#[allow(unused)]
	traceSfxId1: i32,
#[allow(unused)]
	traceDmyIdHead1: i32,
#[allow(unused)]
	traceDmyIdTail1: i32,
#[allow(unused)]
	traceSfxId2: i32,
#[allow(unused)]
	traceDmyIdHead2: i32,
#[allow(unused)]
	traceDmyIdTail2: i32,
#[allow(unused)]
	traceSfxId3: i32,
#[allow(unused)]
	traceDmyIdHead3: i32,
#[allow(unused)]
	traceDmyIdTail3: i32,
#[allow(unused)]
	traceSfxId4: i32,
#[allow(unused)]
	traceDmyIdHead4: i32,
#[allow(unused)]
	traceDmyIdTail4: i32,
#[allow(unused)]
	traceSfxId5: i32,
#[allow(unused)]
	traceDmyIdHead5: i32,
#[allow(unused)]
	traceDmyIdTail5: i32,
#[allow(unused)]
	traceSfxId6: i32,
#[allow(unused)]
	traceDmyIdHead6: i32,
#[allow(unused)]
	traceDmyIdTail6: i32,
#[allow(unused)]
	traceSfxId7: i32,
#[allow(unused)]
	traceDmyIdHead7: i32,
#[allow(unused)]
	traceDmyIdTail7: i32,
#[allow(unused)]
	hit4_Radius: f32,
#[allow(unused)]
	hit5_Radius: f32,
#[allow(unused)]
	hit6_Radius: f32,
#[allow(unused)]
	hit7_Radius: f32,
#[allow(unused)]
	hit8_Radius: f32,
#[allow(unused)]
	hit9_Radius: f32,
#[allow(unused)]
	hit10_Radius: f32,
#[allow(unused)]
	hit11_Radius: f32,
#[allow(unused)]
	hit12_Radius: f32,
#[allow(unused)]
	hit13_Radius: f32,
#[allow(unused)]
	hit14_Radius: f32,
#[allow(unused)]
	hit15_Radius: f32,
#[allow(unused)]
	hit4_DmyPoly1: i16,
#[allow(unused)]
	hit5_DmyPoly1: i16,
#[allow(unused)]
	hit6_DmyPoly1: i16,
#[allow(unused)]
	hit7_DmyPoly1: i16,
#[allow(unused)]
	hit8_DmyPoly1: i16,
#[allow(unused)]
	hit9_DmyPoly1: i16,
#[allow(unused)]
	hit10_DmyPoly1: i16,
#[allow(unused)]
	hit11_DmyPoly1: i16,
#[allow(unused)]
	hit12_DmyPoly1: i16,
#[allow(unused)]
	hit13_DmyPoly1: i16,
#[allow(unused)]
	hit14_DmyPoly1: i16,
#[allow(unused)]
	hit15_DmyPoly1: i16,
#[allow(unused)]
	hit4_DmyPoly2: i16,
#[allow(unused)]
	hit5_DmyPoly2: i16,
#[allow(unused)]
	hit6_DmyPoly2: i16,
#[allow(unused)]
	hit7_DmyPoly2: i16,
#[allow(unused)]
	hit8_DmyPoly2: i16,
#[allow(unused)]
	hit9_DmyPoly2: i16,
#[allow(unused)]
	hit10_DmyPoly2: i16,
#[allow(unused)]
	hit11_DmyPoly2: i16,
#[allow(unused)]
	hit12_DmyPoly2: i16,
#[allow(unused)]
	hit13_DmyPoly2: i16,
#[allow(unused)]
	hit14_DmyPoly2: i16,
#[allow(unused)]
	hit15_DmyPoly2: i16,
#[allow(unused)]
	hit4_hitType: u8,
#[allow(unused)]
	hit5_hitType: u8,
#[allow(unused)]
	hit6_hitType: u8,
#[allow(unused)]
	hit7_hitType: u8,
#[allow(unused)]
	hit8_hitType: u8,
#[allow(unused)]
	hit9_hitType: u8,
#[allow(unused)]
	hit10_hitType: u8,
#[allow(unused)]
	hit11_hitType: u8,
#[allow(unused)]
	hit12_hitType: u8,
#[allow(unused)]
	hit13_hitType: u8,
#[allow(unused)]
	hit14_hitType: u8,
#[allow(unused)]
	hit15_hitType: u8,
#[allow(unused)]
	hti4_Priority: u8,
#[allow(unused)]
	hti5_Priority: u8,
#[allow(unused)]
	hti6_Priority: u8,
#[allow(unused)]
	hti7_Priority: u8,
#[allow(unused)]
	hti8_Priority: u8,
#[allow(unused)]
	hti9_Priority: u8,
#[allow(unused)]
	hti10_Priority: u8,
#[allow(unused)]
	hti11_Priority: u8,
#[allow(unused)]
	hti12_Priority: u8,
#[allow(unused)]
	hti13_Priority: u8,
#[allow(unused)]
	hti14_Priority: u8,
#[allow(unused)]
	hti15_Priority: u8,
#[allow(unused)]
	defSfxMaterial1: i16,
#[allow(unused)]
	defSeMaterial2: i16,
#[allow(unused)]
	defSfxMaterial2: i16,
#[allow(unused)]
	atkDarkCorrection: i16,
#[allow(unused)]
	atkDark: i16,
#[allow(unused)]
	pad5: [u8;1],
#[allow(unused)]
	isDisableParry: u8,
#[allow(unused)]
	isDisableBothHandsAtkBonus: u8,
#[allow(unused)]
	isInvalidatedByNoDamageInAir: u8,
#[allow(unused)]
	pad2: [u8;1],
#[allow(unused)]
	dmgLevel_vsPlayer: i8,
#[allow(unused)]
	statusAilmentAtkPowerCorrectRate: i16,
#[allow(unused)]
	spEffectAtkPowerCorrectRate_byPoint: i16,
#[allow(unused)]
	spEffectAtkPowerCorrectRate_byRate: i16,
#[allow(unused)]
	spEffectAtkPowerCorrectRate_byDmg: i16,
#[allow(unused)]
	atkBehaviorId_2: u8,
#[allow(unused)]
	throwDamageAttribute: u8,
#[allow(unused)]
	statusAilmentAtkPowerCorrectRate_byPoint: i16,
#[allow(unused)]
	overwriteAttackElementCorrectId: i32,
#[allow(unused)]
	decalBaseId1: i16,
#[allow(unused)]
	decalBaseId2: i16,
#[allow(unused)]
	wepRegainHpScale: i16,
#[allow(unused)]
	atkRegainHp: i16,
#[allow(unused)]
	regainableTimeScale: f32,
#[allow(unused)]
	regainableHpRateScale: f32,
#[allow(unused)]
	regainableSlotId: i8,
#[allow(unused)]
	spAttributeVariationValue: u8,
#[allow(unused)]
	parryForwardOffset: i16,
#[allow(unused)]
	atkSuperArmorCorrection: f32,
#[allow(unused)]
	defSfxMaterialVariationValue: u8,
#[allow(unused)]
	pad4: [u8;3],
#[allow(unused)]
	finalDamageRateId: i32,
#[allow(unused)]
	pad7: [u8;12],
}

struct ATTACK_ELEMENT_CORRECT_PARAM_ST {
#[allow(unused)]
	isStrengthCorrect_byPhysics: u8,
#[allow(unused)]
	isDexterityCorrect_byPhysics: u8,
#[allow(unused)]
	isMagicCorrect_byPhysics: u8,
#[allow(unused)]
	isFaithCorrect_byPhysics: u8,
#[allow(unused)]
	isLuckCorrect_byPhysics: u8,
#[allow(unused)]
	isStrengthCorrect_byMagic: u8,
#[allow(unused)]
	isDexterityCorrect_byMagic: u8,
#[allow(unused)]
	isMagicCorrect_byMagic: u8,
#[allow(unused)]
	isFaithCorrect_byMagic: u8,
#[allow(unused)]
	isLuckCorrect_byMagic: u8,
#[allow(unused)]
	isStrengthCorrect_byFire: u8,
#[allow(unused)]
	isDexterityCorrect_byFire: u8,
#[allow(unused)]
	isMagicCorrect_byFire: u8,
#[allow(unused)]
	isFaithCorrect_byFire: u8,
#[allow(unused)]
	isLuckCorrect_byFire: u8,
#[allow(unused)]
	isStrengthCorrect_byThunder: u8,
#[allow(unused)]
	isDexterityCorrect_byThunder: u8,
#[allow(unused)]
	isMagicCorrect_byThunder: u8,
#[allow(unused)]
	isFaithCorrect_byThunder: u8,
#[allow(unused)]
	isLuckCorrect_byThunder: u8,
#[allow(unused)]
	isStrengthCorrect_byDark: u8,
#[allow(unused)]
	isDexterityCorrect_byDark: u8,
#[allow(unused)]
	isMagicCorrect_byDark: u8,
#[allow(unused)]
	isFaithCorrect_byDark: u8,
#[allow(unused)]
	isLuckCorrect_byDark: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	overwriteStrengthCorrectRate_byPhysics: i16,
#[allow(unused)]
	overwriteDexterityCorrectRate_byPhysics: i16,
#[allow(unused)]
	overwriteMagicCorrectRate_byPhysics: i16,
#[allow(unused)]
	overwriteFaithCorrectRate_byPhysics: i16,
#[allow(unused)]
	overwriteLuckCorrectRate_byPhysics: i16,
#[allow(unused)]
	overwriteStrengthCorrectRate_byMagic: i16,
#[allow(unused)]
	overwriteDexterityCorrectRate_byMagic: i16,
#[allow(unused)]
	overwriteMagicCorrectRate_byMagic: i16,
#[allow(unused)]
	overwriteFaithCorrectRate_byMagic: i16,
#[allow(unused)]
	overwriteLuckCorrectRate_byMagic: i16,
#[allow(unused)]
	overwriteStrengthCorrectRate_byFire: i16,
#[allow(unused)]
	overwriteDexterityCorrectRate_byFire: i16,
#[allow(unused)]
	overwriteMagicCorrectRate_byFire: i16,
#[allow(unused)]
	overwriteFaithCorrectRate_byFire: i16,
#[allow(unused)]
	overwriteLuckCorrectRate_byFire: i16,
#[allow(unused)]
	overwriteStrengthCorrectRate_byThunder: i16,
#[allow(unused)]
	overwriteDexterityCorrectRate_byThunder: i16,
#[allow(unused)]
	overwriteMagicCorrectRate_byThunder: i16,
#[allow(unused)]
	overwriteFaithCorrectRate_byThunder: i16,
#[allow(unused)]
	overwriteLuckCorrectRate_byThunder: i16,
#[allow(unused)]
	overwriteStrengthCorrectRate_byDark: i16,
#[allow(unused)]
	overwriteDexterityCorrectRate_byDark: i16,
#[allow(unused)]
	overwriteMagicCorrectRate_byDark: i16,
#[allow(unused)]
	overwriteFaithCorrectRate_byDark: i16,
#[allow(unused)]
	overwriteLuckCorrectRate_byDark: i16,
#[allow(unused)]
	InfluenceStrengthCorrectRate_byPhysics: i16,
#[allow(unused)]
	InfluenceDexterityCorrectRate_byPhysics: i16,
#[allow(unused)]
	InfluenceMagicCorrectRate_byPhysics: i16,
#[allow(unused)]
	InfluenceFaithCorrectRate_byPhysics: i16,
#[allow(unused)]
	InfluenceLuckCorrectRate_byPhysics: i16,
#[allow(unused)]
	InfluenceStrengthCorrectRate_byMagic: i16,
#[allow(unused)]
	InfluenceDexterityCorrectRate_byMagic: i16,
#[allow(unused)]
	InfluenceMagicCorrectRate_byMagic: i16,
#[allow(unused)]
	InfluenceFaithCorrectRate_byMagic: i16,
#[allow(unused)]
	InfluenceLuckCorrectRate_byMagic: i16,
#[allow(unused)]
	InfluenceStrengthCorrectRate_byFire: i16,
#[allow(unused)]
	InfluenceDexterityCorrectRate_byFire: i16,
#[allow(unused)]
	InfluenceMagicCorrectRate_byFire: i16,
#[allow(unused)]
	InfluenceFaithCorrectRate_byFire: i16,
#[allow(unused)]
	InfluenceLuckCorrectRate_byFire: i16,
#[allow(unused)]
	InfluenceStrengthCorrectRate_byThunder: i16,
#[allow(unused)]
	InfluenceDexterityCorrectRate_byThunder: i16,
#[allow(unused)]
	InfluenceMagicCorrectRate_byThunder: i16,
#[allow(unused)]
	InfluenceFaithCorrectRate_byThunder: i16,
#[allow(unused)]
	InfluenceLuckCorrectRate_byThunder: i16,
#[allow(unused)]
	InfluenceStrengthCorrectRate_byDark: i16,
#[allow(unused)]
	InfluenceDexterityCorrectRate_byDark: i16,
#[allow(unused)]
	InfluenceMagicCorrectRate_byDark: i16,
#[allow(unused)]
	InfluenceFaithCorrectRate_byDark: i16,
#[allow(unused)]
	InfluenceLuckCorrectRate_byDark: i16,
#[allow(unused)]
	pad2: [u8;24],
}

struct AUTO_CREATE_ENV_SOUND_PARAM_ST {
#[allow(unused)]
	RangeMin: f32,
#[allow(unused)]
	RangeMax: f32,
#[allow(unused)]
	LifeTimeMin: f32,
#[allow(unused)]
	LifeTimeMax: f32,
#[allow(unused)]
	DeleteDist: f32,
#[allow(unused)]
	NearDist: f32,
#[allow(unused)]
	LimiteRotateMin: f32,
#[allow(unused)]
	LimiteRotateMax: f32,
}

struct BASECHR_SELECT_MENU_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	chrInitParam: i32,
#[allow(unused)]
	originChrInitParam: i32,
#[allow(unused)]
	imageId: i32,
#[allow(unused)]
	textId: i32,
#[allow(unused)]
	reserve: [u8;12],
}

struct BEHAVIOR_PARAM_ST {
#[allow(unused)]
	variationId: i32,
#[allow(unused)]
	behaviorJudgeId: i32,
#[allow(unused)]
	ezStateBehaviorType_old: u8,
#[allow(unused)]
	refType: u8,
#[allow(unused)]
	pad2: [u8;2],
#[allow(unused)]
	refId: i32,
#[allow(unused)]
	consumeSA: f32,
#[allow(unused)]
	stamina: i32,
#[allow(unused)]
	consumeDurability: i32,
#[allow(unused)]
	category: u8,
#[allow(unused)]
	heroPoint: u8,
#[allow(unused)]
	pad1: [u8;2],
}

struct BONFIRE_WARP_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	eventflagId: i32,
#[allow(unused)]
	bonfireEntityId: i32,
#[allow(unused)]
	pad4: [u8;2],
#[allow(unused)]
	bonfireSubCategorySortId: i16,
#[allow(unused)]
	forbiddenIconId: i16,
#[allow(unused)]
	dispMinZoomStep: u8,
#[allow(unused)]
	selectMinZoomStep: u8,
#[allow(unused)]
	bonfireSubCategoryId: i32,
#[allow(unused)]
	clearedEventFlagId: i32,
#[allow(unused)]
	iconId: i16,
#[allow(unused)]
	dispMask00: u8,
#[allow(unused)]
	dispMask01: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	pad2: [u8;1],
#[allow(unused)]
	areaNo: u8,
#[allow(unused)]
	gridXNo: u8,
#[allow(unused)]
	gridZNo: u8,
#[allow(unused)]
	pad3: [u8;1],
#[allow(unused)]
	posX: f32,
#[allow(unused)]
	posY: f32,
#[allow(unused)]
	posZ: f32,
#[allow(unused)]
	textId1: i32,
#[allow(unused)]
	textEnableFlagId1: i32,
#[allow(unused)]
	textDisableFlagId1: i32,
#[allow(unused)]
	textId2: i32,
#[allow(unused)]
	textEnableFlagId2: i32,
#[allow(unused)]
	textDisableFlagId2: i32,
#[allow(unused)]
	textId3: i32,
#[allow(unused)]
	textEnableFlagId3: i32,
#[allow(unused)]
	textDisableFlagId3: i32,
#[allow(unused)]
	textId4: i32,
#[allow(unused)]
	textEnableFlagId4: i32,
#[allow(unused)]
	textDisableFlagId4: i32,
#[allow(unused)]
	textId5: i32,
#[allow(unused)]
	textEnableFlagId5: i32,
#[allow(unused)]
	textDisableFlagId5: i32,
#[allow(unused)]
	textId6: i32,
#[allow(unused)]
	textEnableFlagId6: i32,
#[allow(unused)]
	textDisableFlagId6: i32,
#[allow(unused)]
	textId7: i32,
#[allow(unused)]
	textEnableFlagId7: i32,
#[allow(unused)]
	textDisableFlagId7: i32,
#[allow(unused)]
	textId8: i32,
#[allow(unused)]
	textEnableFlagId8: i32,
#[allow(unused)]
	textDisableFlagId8: i32,
#[allow(unused)]
	textType1: u8,
#[allow(unused)]
	textType2: u8,
#[allow(unused)]
	textType3: u8,
#[allow(unused)]
	textType4: u8,
#[allow(unused)]
	textType5: u8,
#[allow(unused)]
	textType6: u8,
#[allow(unused)]
	textType7: u8,
#[allow(unused)]
	textType8: u8,
#[allow(unused)]
	noIgnitionSfxDmypolyId_0: i32,
#[allow(unused)]
	noIgnitionSfxId_0: i32,
#[allow(unused)]
	noIgnitionSfxDmypolyId_1: i32,
#[allow(unused)]
	noIgnitionSfxId_1: i32,
#[allow(unused)]
	unkA8: i32,
#[allow(unused)]
	unkAC: i32,
#[allow(unused)]
	unkB0: i32,
#[allow(unused)]
	unkB4: i32,
#[allow(unused)]
	unkB8: i32,
#[allow(unused)]
	unkBC: i32,
#[allow(unused)]
	unkC0: i32,
#[allow(unused)]
	unkC4: i32,
#[allow(unused)]
	unkC8: i32,
#[allow(unused)]
	unkCC: i32,
#[allow(unused)]
	unkD0: i32,
#[allow(unused)]
	unkD4: i32,
#[allow(unused)]
	unkD8: i32,
#[allow(unused)]
	unkDC: i32,
#[allow(unused)]
	unkE0: i32,
#[allow(unused)]
	unkE4: i32,
#[allow(unused)]
	unkE8: i32,
}

struct BONFIRE_WARP_SUB_CATEGORY_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	textId: i32,
#[allow(unused)]
	tabId: i16,
#[allow(unused)]
	sortId: i16,
#[allow(unused)]
	pad: [u8;4],
}

struct BONFIRE_WARP_TAB_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	textId: i32,
#[allow(unused)]
	sortId: i32,
#[allow(unused)]
	iconId: i16,
#[allow(unused)]
	pad: [u8;2],
}

struct BUDDY_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	triggerSpEffectId: i32,
#[allow(unused)]
	npcParamId: i32,
#[allow(unused)]
	npcThinkParamId: i32,
#[allow(unused)]
	npcParamId_ridden: i32,
#[allow(unused)]
	npcThinkParamId_ridden: i32,
#[allow(unused)]
	x_offset: f32,
#[allow(unused)]
	z_offset: f32,
#[allow(unused)]
	y_angle: f32,
#[allow(unused)]
	appearOnAroundSekihi: u8,
#[allow(unused)]
	disablePCTargetShare: u8,
#[allow(unused)]
	pcFollowType: u8,
#[allow(unused)]
	Reserve: [u8;1],
#[allow(unused)]
	dopingSpEffect_lv0: i32,
#[allow(unused)]
	dopingSpEffect_lv1: i32,
#[allow(unused)]
	dopingSpEffect_lv2: i32,
#[allow(unused)]
	dopingSpEffect_lv3: i32,
#[allow(unused)]
	dopingSpEffect_lv4: i32,
#[allow(unused)]
	dopingSpEffect_lv5: i32,
#[allow(unused)]
	dopingSpEffect_lv6: i32,
#[allow(unused)]
	dopingSpEffect_lv7: i32,
#[allow(unused)]
	dopingSpEffect_lv8: i32,
#[allow(unused)]
	dopingSpEffect_lv9: i32,
#[allow(unused)]
	dopingSpEffect_lv10: i32,
#[allow(unused)]
	npcPlayerInitParamId: i32,
#[allow(unused)]
	generateAnimId: i32,
#[allow(unused)]
	Reserve2: [u8;4],
#[allow(unused)]
	Unk1: i32,
#[allow(unused)]
	Unk2: i32,
#[allow(unused)]
	Unk3: i32,
#[allow(unused)]
	Unk4: i32,
#[allow(unused)]
	Unk5: i32,
#[allow(unused)]
	Unk6: i32,
#[allow(unused)]
	Unk7: i32,
#[allow(unused)]
	Unk8: i32,
#[allow(unused)]
	Unk9: i32,
#[allow(unused)]
	Unk10: i32,
#[allow(unused)]
	Unk11: i32,
#[allow(unused)]
	Unk12: i32,
#[allow(unused)]
	Unk13: i32,
#[allow(unused)]
	Unk14: i32,
#[allow(unused)]
	Unk15: i32,
#[allow(unused)]
	Unk16: i32,
#[allow(unused)]
	Unk17: i32,
}

struct BUDDY_STONE_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	talkChrEntityId: i32,
#[allow(unused)]
	eliminateTargetEntityId: i32,
#[allow(unused)]
	summonedEventFlagId: i32,
#[allow(unused)]
	isSpecial: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	pad2: [u8;3],
#[allow(unused)]
	buddyId: i32,
#[allow(unused)]
	dopingSpEffectId: i32,
#[allow(unused)]
	activateRange: i16,
#[allow(unused)]
	overwriteReturnRange: i16,
#[allow(unused)]
	overwriteActivateRegionEntityId: i32,
#[allow(unused)]
	warnRegionEntityId: i32,
#[allow(unused)]
	pad3: [u8;24],
}

struct BUDGET_PARAM_ST {
#[allow(unused)]
	vram_all: f32,
#[allow(unused)]
	vram_mapobj_tex: f32,
#[allow(unused)]
	vram_mapobj_mdl: f32,
#[allow(unused)]
	vram_map: f32,
#[allow(unused)]
	vram_chr: f32,
#[allow(unused)]
	vram_parts: f32,
#[allow(unused)]
	vram_sfx: f32,
#[allow(unused)]
	vram_chr_tex: f32,
#[allow(unused)]
	vram_chr_mdl: f32,
#[allow(unused)]
	vram_parts_tex: f32,
#[allow(unused)]
	vram_parts_mdl: f32,
#[allow(unused)]
	vram_sfx_tex: f32,
#[allow(unused)]
	vram_sfx_mdl: f32,
#[allow(unused)]
	vram_gi: f32,
#[allow(unused)]
	vram_menu_tex: f32,
#[allow(unused)]
	vram_decal_rt: f32,
#[allow(unused)]
	vram_decal: f32,
#[allow(unused)]
	reserve_0: [u8;4],
#[allow(unused)]
	vram_other_tex: f32,
#[allow(unused)]
	vram_other_mdl: f32,
#[allow(unused)]
	havok_anim: f32,
#[allow(unused)]
	havok_ins: f32,
#[allow(unused)]
	havok_hit: f32,
#[allow(unused)]
	vram_other: f32,
#[allow(unused)]
	vram_detail_all: f32,
#[allow(unused)]
	vram_chr_and_parts: f32,
#[allow(unused)]
	havok_navimesh: f32,
#[allow(unused)]
	reserve_1: [u8;24],
}

struct BULLET_CREATE_LIMIT_PARAM_ST {
#[allow(unused)]
	limitNum_byGroup: u8,
#[allow(unused)]
	isLimitEachOwner: u8,
#[allow(unused)]
	pad2: [u8;1],
#[allow(unused)]
	pad: [u8;30],
}

struct BULLET_PARAM_ST {
#[allow(unused)]
	atkId_Bullet: i32,
#[allow(unused)]
	sfxId_Bullet: i32,
#[allow(unused)]
	sfxId_Hit: i32,
#[allow(unused)]
	sfxId_Flick: i32,
#[allow(unused)]
	life: f32,
#[allow(unused)]
	dist: f32,
#[allow(unused)]
	shootInterval: f32,
#[allow(unused)]
	gravityInRange: f32,
#[allow(unused)]
	gravityOutRange: f32,
#[allow(unused)]
	hormingStopRange: f32,
#[allow(unused)]
	initVellocity: f32,
#[allow(unused)]
	accelInRange: f32,
#[allow(unused)]
	accelOutRange: f32,
#[allow(unused)]
	maxVellocity: f32,
#[allow(unused)]
	minVellocity: f32,
#[allow(unused)]
	accelTime: f32,
#[allow(unused)]
	homingBeginDist: f32,
#[allow(unused)]
	hitRadius: f32,
#[allow(unused)]
	hitRadiusMax: f32,
#[allow(unused)]
	spreadTime: f32,
#[allow(unused)]
	expDelay: f32,
#[allow(unused)]
	hormingOffsetRange: f32,
#[allow(unused)]
	dmgHitRecordLifeTime: f32,
#[allow(unused)]
	externalForce: f32,
#[allow(unused)]
	spEffectIDForShooter: i32,
#[allow(unused)]
	autoSearchNPCThinkID: i32,
#[allow(unused)]
	HitBulletID: i32,
#[allow(unused)]
	spEffectId0: i32,
#[allow(unused)]
	spEffectId1: i32,
#[allow(unused)]
	spEffectId2: i32,
#[allow(unused)]
	spEffectId3: i32,
#[allow(unused)]
	spEffectId4: i32,
#[allow(unused)]
	numShoot: i16,
#[allow(unused)]
	homingAngle: i16,
#[allow(unused)]
	shootAngle: i16,
#[allow(unused)]
	shootAngleInterval: i16,
#[allow(unused)]
	shootAngleXInterval: i16,
#[allow(unused)]
	damageDamp: i8,
#[allow(unused)]
	spelDamageDamp: i8,
#[allow(unused)]
	fireDamageDamp: i8,
#[allow(unused)]
	thunderDamageDamp: i8,
#[allow(unused)]
	staminaDamp: i8,
#[allow(unused)]
	knockbackDamp: i8,
#[allow(unused)]
	shootAngleXZ: i8,
#[allow(unused)]
	lockShootLimitAng: u8,
#[allow(unused)]
	pad2: [u8;1],
#[allow(unused)]
	prevVelocityDirRate: u8,
#[allow(unused)]
	atkAttribute: u8,
#[allow(unused)]
	spAttribute: u8,
#[allow(unused)]
	Material_AttackType: u8,
#[allow(unused)]
	Material_AttackMaterial: u8,
#[allow(unused)]
	isPenetrateChr: u8,
#[allow(unused)]
	isPenetrateObj: u8,
#[allow(unused)]
	pad: [u8;1],
#[allow(unused)]
	launchConditionType: u8,
#[allow(unused)]
	FollowType: u8,
#[allow(unused)]
	EmittePosType: u8,
#[allow(unused)]
	isAttackSFX: u8,
#[allow(unused)]
	isEndlessHit: u8,
#[allow(unused)]
	isPenetrateMap: u8,
#[allow(unused)]
	isHitBothTeam: u8,
#[allow(unused)]
	isUseSharedHitList: u8,
#[allow(unused)]
	isUseMultiDmyPolyIfPlace: u8,
#[allow(unused)]
	isHitOtherBulletForceEraseA: u8,
#[allow(unused)]
	isHitOtherBulletForceEraseB: u8,
#[allow(unused)]
	isHitForceMagic: u8,
#[allow(unused)]
	isIgnoreSfxIfHitWater: u8,
#[allow(unused)]
	isIgnoreMoveStateIfHitWater: u8,
#[allow(unused)]
	isHitDarkForceMagic: u8,
#[allow(unused)]
	dmgCalcSide: u8,
#[allow(unused)]
	isEnableAutoHoming: u8,
#[allow(unused)]
	isSyncBulletCulcDumypolyPos: u8,
#[allow(unused)]
	isOwnerOverrideInitAngle: u8,
#[allow(unused)]
	isInheritSfxToChild: u8,
#[allow(unused)]
	darkDamageDamp: i8,
#[allow(unused)]
	bulletSfxDeleteType_byHit: i8,
#[allow(unused)]
	bulletSfxDeleteType_byLifeDead: i8,
#[allow(unused)]
	targetYOffsetRange: f32,
#[allow(unused)]
	shootAngleYMaxRandom: f32,
#[allow(unused)]
	shootAngleXMaxRandom: f32,
#[allow(unused)]
	intervalCreateBulletId: i32,
#[allow(unused)]
	intervalCreateTimeMin: f32,
#[allow(unused)]
	intervalCreateTimeMax: f32,
#[allow(unused)]
	predictionShootObserveTime: f32,
#[allow(unused)]
	intervalCreateWaitTime: f32,
#[allow(unused)]
	sfxPostureType: u8,
#[allow(unused)]
	createLimitGroupId: u8,
#[allow(unused)]
	pad5: [u8;1],
#[allow(unused)]
	isInheritSpeedToChild: u8,
#[allow(unused)]
	isDisableHitSfx_byChrAndObj: u8,
#[allow(unused)]
	isCheckWall_byCenterRay: u8,
#[allow(unused)]
	isHitFlare: u8,
#[allow(unused)]
	isUseBulletWallFilter: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	isNonDependenceMagicForFunnleNum: u8,
#[allow(unused)]
	isAiInterruptShootNoDamageBullet: u8,
#[allow(unused)]
	randomCreateRadius: f32,
#[allow(unused)]
	followOffset_BaseHeight: f32,
#[allow(unused)]
	assetNo_Hit: i32,
#[allow(unused)]
	lifeRandomRange: f32,
#[allow(unused)]
	homingAngleX: i16,
#[allow(unused)]
	ballisticCalcType: u8,
#[allow(unused)]
	attachEffectType: u8,
#[allow(unused)]
	seId_Bullet1: i32,
#[allow(unused)]
	seId_Bullet2: i32,
#[allow(unused)]
	seId_Hit: i32,
#[allow(unused)]
	seId_Flick: i32,
#[allow(unused)]
	howitzerShootAngleXMin: i16,
#[allow(unused)]
	howitzerShootAngleXMax: i16,
#[allow(unused)]
	howitzerInitMinVelocity: f32,
#[allow(unused)]
	howitzerInitMaxVelocity: f32,
#[allow(unused)]
	sfxId_ForceErase: i32,
#[allow(unused)]
	bulletSfxDeleteType_byForceErase: i8,
#[allow(unused)]
	pad3: [u8;1],
#[allow(unused)]
	followDmypoly_forSfxPose: i16,
#[allow(unused)]
	followOffset_Radius: f32,
#[allow(unused)]
	spBulletDistUpRate: f32,
#[allow(unused)]
	nolockTargetDist: f32,
#[allow(unused)]
	pad4: [u8;8],
}

struct CACL_CORRECT_GRAPH_ST {
#[allow(unused)]
	stageMaxVal0: f32,
#[allow(unused)]
	stageMaxVal1: f32,
#[allow(unused)]
	stageMaxVal2: f32,
#[allow(unused)]
	stageMaxVal3: f32,
#[allow(unused)]
	stageMaxVal4: f32,
#[allow(unused)]
	stageMaxGrowVal0: f32,
#[allow(unused)]
	stageMaxGrowVal1: f32,
#[allow(unused)]
	stageMaxGrowVal2: f32,
#[allow(unused)]
	stageMaxGrowVal3: f32,
#[allow(unused)]
	stageMaxGrowVal4: f32,
#[allow(unused)]
	adjPt_maxGrowVal0: f32,
#[allow(unused)]
	adjPt_maxGrowVal1: f32,
#[allow(unused)]
	adjPt_maxGrowVal2: f32,
#[allow(unused)]
	adjPt_maxGrowVal3: f32,
#[allow(unused)]
	adjPt_maxGrowVal4: f32,
#[allow(unused)]
	init_inclination_soul: f32,
#[allow(unused)]
	adjustment_value: f32,
#[allow(unused)]
	boundry_inclination_soul: f32,
#[allow(unused)]
	boundry_value: f32,
#[allow(unused)]
	pad: [u8;4],
}

struct CAMERA_FADE_PARAM_ST {
#[allow(unused)]
	NearMinDist: f32,
#[allow(unused)]
	NearMaxDist: f32,
#[allow(unused)]
	FarMinDist: f32,
#[allow(unused)]
	FarMaxDist: f32,
#[allow(unused)]
	MiddleAlpha: f32,
#[allow(unused)]
	dummy: [u8;12],
}

struct CEREMONY_PARAM_ST {
#[allow(unused)]
	eventLayerId: i32,
#[allow(unused)]
	mapStudioLayerId: i32,
#[allow(unused)]
	multiPlayAreaOffset: i32,
#[allow(unused)]
	overrideMapPlaceNameId: i32,
#[allow(unused)]
	overrideSaveMapNameId: i32,
#[allow(unused)]
	pad2: [u8;16],
}

struct CHARACTER_INIT_PARAM {
#[allow(unused)]
	baseRec_mp: f32,
#[allow(unused)]
	baseRec_sp: f32,
#[allow(unused)]
	red_Falldam: f32,
#[allow(unused)]
	soul: i32,
#[allow(unused)]
	equip_Wep_Right: i32,
#[allow(unused)]
	equip_Subwep_Right: i32,
#[allow(unused)]
	equip_Wep_Left: i32,
#[allow(unused)]
	equip_Subwep_Left: i32,
#[allow(unused)]
	equip_Helm: i32,
#[allow(unused)]
	equip_Armer: i32,
#[allow(unused)]
	equip_Gaunt: i32,
#[allow(unused)]
	equip_Leg: i32,
#[allow(unused)]
	equip_Arrow: i32,
#[allow(unused)]
	equip_Bolt: i32,
#[allow(unused)]
	equip_SubArrow: i32,
#[allow(unused)]
	equip_SubBolt: i32,
#[allow(unused)]
	equip_Accessory01: i32,
#[allow(unused)]
	equip_Accessory02: i32,
#[allow(unused)]
	equip_Accessory03: i32,
#[allow(unused)]
	equip_Accessory04: i32,
#[allow(unused)]
	pad8: [u8;4],
#[allow(unused)]
	elixir_material00: i32,
#[allow(unused)]
	elixir_material01: i32,
#[allow(unused)]
	elixir_material02: i32,
#[allow(unused)]
	equip_Spell_01: i32,
#[allow(unused)]
	equip_Spell_02: i32,
#[allow(unused)]
	equip_Spell_03: i32,
#[allow(unused)]
	equip_Spell_04: i32,
#[allow(unused)]
	equip_Spell_05: i32,
#[allow(unused)]
	equip_Spell_06: i32,
#[allow(unused)]
	equip_Spell_07: i32,
#[allow(unused)]
	item_01: i32,
#[allow(unused)]
	item_02: i32,
#[allow(unused)]
	item_03: i32,
#[allow(unused)]
	item_04: i32,
#[allow(unused)]
	item_05: i32,
#[allow(unused)]
	item_06: i32,
#[allow(unused)]
	item_07: i32,
#[allow(unused)]
	item_08: i32,
#[allow(unused)]
	item_09: i32,
#[allow(unused)]
	item_10: i32,
#[allow(unused)]
	npcPlayerFaceGenId: i32,
#[allow(unused)]
	npcPlayerThinkId: i32,
#[allow(unused)]
	baseHp: i16,
#[allow(unused)]
	baseMp: i16,
#[allow(unused)]
	baseSp: i16,
#[allow(unused)]
	arrowNum: i16,
#[allow(unused)]
	boltNum: i16,
#[allow(unused)]
	subArrowNum: i16,
#[allow(unused)]
	subBoltNum: i16,
#[allow(unused)]
	pad4: [u8;6],
#[allow(unused)]
	soulLv: i16,
#[allow(unused)]
	baseVit: u8,
#[allow(unused)]
	baseWil: u8,
#[allow(unused)]
	baseEnd: u8,
#[allow(unused)]
	baseStr: u8,
#[allow(unused)]
	baseDex: u8,
#[allow(unused)]
	baseMag: u8,
#[allow(unused)]
	baseFai: u8,
#[allow(unused)]
	baseLuc: u8,
#[allow(unused)]
	baseHeroPoint: u8,
#[allow(unused)]
	baseDurability: u8,
#[allow(unused)]
	itemNum_01: u8,
#[allow(unused)]
	itemNum_02: u8,
#[allow(unused)]
	itemNum_03: u8,
#[allow(unused)]
	itemNum_04: u8,
#[allow(unused)]
	itemNum_05: u8,
#[allow(unused)]
	itemNum_06: u8,
#[allow(unused)]
	itemNum_07: u8,
#[allow(unused)]
	itemNum_08: u8,
#[allow(unused)]
	itemNum_09: u8,
#[allow(unused)]
	itemNum_10: u8,
#[allow(unused)]
	pad5: [u8;5],
#[allow(unused)]
	gestureId0: i8,
#[allow(unused)]
	gestureId1: i8,
#[allow(unused)]
	gestureId2: i8,
#[allow(unused)]
	gestureId3: i8,
#[allow(unused)]
	gestureId4: i8,
#[allow(unused)]
	gestureId5: i8,
#[allow(unused)]
	gestureId6: i8,
#[allow(unused)]
	npcPlayerType: u8,
#[allow(unused)]
	npcPlayerDrawType: i8,
#[allow(unused)]
	npcPlayerSex: u8,
#[allow(unused)]
	vowType: u8,
#[allow(unused)]
	isSyncTarget: u8,
#[allow(unused)]
	pad: [u8;1],
#[allow(unused)]
	pad6: [u8;2],
#[allow(unused)]
	wepParamType_Right1: u8,
#[allow(unused)]
	wepParamType_Right2: u8,
#[allow(unused)]
	wepParamType_Right3: u8,
#[allow(unused)]
	wepParamType_Left1: u8,
#[allow(unused)]
	wepParamType_Left2: u8,
#[allow(unused)]
	wepParamType_Left3: u8,
#[allow(unused)]
	pad2: [u8;26],
#[allow(unused)]
	equip_Subwep_Right3: i32,
#[allow(unused)]
	equip_Subwep_Left3: i32,
#[allow(unused)]
	pad3: [u8;4],
#[allow(unused)]
	secondaryItem_01: i32,
#[allow(unused)]
	secondaryItem_02: i32,
#[allow(unused)]
	secondaryItem_03: i32,
#[allow(unused)]
	secondaryItem_04: i32,
#[allow(unused)]
	secondaryItem_05: i32,
#[allow(unused)]
	secondaryItem_06: i32,
#[allow(unused)]
	secondaryItemNum_01: u8,
#[allow(unused)]
	secondaryItemNum_02: u8,
#[allow(unused)]
	secondaryItemNum_03: u8,
#[allow(unused)]
	secondaryItemNum_04: u8,
#[allow(unused)]
	secondaryItemNum_05: u8,
#[allow(unused)]
	secondaryItemNum_06: u8,
#[allow(unused)]
	HpEstMax: i8,
#[allow(unused)]
	MpEstMax: i8,
#[allow(unused)]
	pad7: [u8;5],
#[allow(unused)]
	voiceType: u8,
#[allow(unused)]
	reserve: [u8;6],
}

struct CHARMAKEMENU_LISTITEM_PARAM_ST {
#[allow(unused)]
	value: i32,
#[allow(unused)]
	captionId: i32,
#[allow(unused)]
	iconId: u8,
#[allow(unused)]
	reserved: [u8;7],
}

struct CHARMAKEMENUTOP_PARAM_ST {
#[allow(unused)]
	commandType: i32,
#[allow(unused)]
	captionId: i32,
#[allow(unused)]
	faceParamId: i32,
#[allow(unused)]
	tableId: i32,
#[allow(unused)]
	viewCondition: i32,
#[allow(unused)]
	previewMode: i8,
#[allow(unused)]
	reserved2: [u8;3],
#[allow(unused)]
	tableId2: i32,
#[allow(unused)]
	refFaceParamId: i32,
#[allow(unused)]
	refTextId: i32,
#[allow(unused)]
	helpTextId: i32,
#[allow(unused)]
	unlockEventFlagId: i32,
#[allow(unused)]
	reserved: [u8;4],
}

struct CHR_ACTIVATE_CONDITION_PARAM_ST {
#[allow(unused)]
	weatherSunny: u8,
#[allow(unused)]
	weatherClearSky: u8,
#[allow(unused)]
	weatherWeakCloudy: u8,
#[allow(unused)]
	weatherCloudy: u8,
#[allow(unused)]
	weatherRain: u8,
#[allow(unused)]
	weatherHeavyRain: u8,
#[allow(unused)]
	weatherStorm: u8,
#[allow(unused)]
	weatherStormForBattle: u8,
#[allow(unused)]
	weatherSnow: u8,
#[allow(unused)]
	weatherHeavySnow: u8,
#[allow(unused)]
	weatherFog: u8,
#[allow(unused)]
	weatherHeavyFog: u8,
#[allow(unused)]
	weatherHeavyFogRain: u8,
#[allow(unused)]
	weatherSandStorm: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	timeStartHour: u8,
#[allow(unused)]
	timeStartMin: u8,
#[allow(unused)]
	timeEndHour: u8,
#[allow(unused)]
	timeEndMin: u8,
#[allow(unused)]
	pad2: [u8;2],
}

struct CHR_MODEL_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	modelMemoryType: u8,
#[allow(unused)]
	texMemoryType: u8,
#[allow(unused)]
	cameraDitherFadeId: i16,
#[allow(unused)]
	reportAnimMemSizeMb: f32,
#[allow(unused)]
	unk: i32,
}

struct CLEAR_COUNT_CORRECT_PARAM_ST {
#[allow(unused)]
	MaxHpRate: f32,
#[allow(unused)]
	MaxMpRate: f32,
#[allow(unused)]
	MaxStaminaRate: f32,
#[allow(unused)]
	PhysicsAttackRate: f32,
#[allow(unused)]
	SlashAttackRate: f32,
#[allow(unused)]
	BlowAttackRate: f32,
#[allow(unused)]
	ThrustAttackRate: f32,
#[allow(unused)]
	NeturalAttackRate: f32,
#[allow(unused)]
	MagicAttackRate: f32,
#[allow(unused)]
	FireAttackRate: f32,
#[allow(unused)]
	ThunderAttackRate: f32,
#[allow(unused)]
	DarkAttackRate: f32,
#[allow(unused)]
	PhysicsDefenseRate: f32,
#[allow(unused)]
	MagicDefenseRate: f32,
#[allow(unused)]
	FireDefenseRate: f32,
#[allow(unused)]
	ThunderDefenseRate: f32,
#[allow(unused)]
	DarkDefenseRate: f32,
#[allow(unused)]
	StaminaAttackRate: f32,
#[allow(unused)]
	SoulRate: f32,
#[allow(unused)]
	PoisionResistRate: f32,
#[allow(unused)]
	DiseaseResistRate: f32,
#[allow(unused)]
	BloodResistRate: f32,
#[allow(unused)]
	CurseResistRate: f32,
#[allow(unused)]
	FreezeResistRate: f32,
#[allow(unused)]
	BloodDamageRate: f32,
#[allow(unused)]
	SuperArmorDamageRate: f32,
#[allow(unused)]
	FreezeDamageRate: f32,
#[allow(unused)]
	SleepResistRate: f32,
#[allow(unused)]
	MadnessResistRate: f32,
#[allow(unused)]
	SleepDamageRate: f32,
#[allow(unused)]
	MadnessDamageRate: f32,
#[allow(unused)]
	pad1: [u8;4],
}

struct COMMON_SYSTEM_PARAM_ST {
#[allow(unused)]
	mapSaveMapNameIdOnGameStart: i32,
#[allow(unused)]
	reserve0: [u8;60],
}

struct COOL_TIME_PARAM_ST {
#[allow(unused)]
	limitationTime_0: f32,
#[allow(unused)]
	observeTime_0: f32,
#[allow(unused)]
	limitationTime_1: f32,
#[allow(unused)]
	observeTime_1: f32,
#[allow(unused)]
	limitationTime_2: f32,
#[allow(unused)]
	observeTime_2: f32,
#[allow(unused)]
	limitationTime_3: f32,
#[allow(unused)]
	observeTime_3: f32,
}

struct CUTSCENE_GPARAM_TIME_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParam_Debug: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	DstTimezone_Morning: u8,
#[allow(unused)]
	DstTimezone_Noon: u8,
#[allow(unused)]
	DstTimezone_AfterNoon: u8,
#[allow(unused)]
	DstTimezone_Evening: u8,
#[allow(unused)]
	DstTimezone_Night: u8,
#[allow(unused)]
	DstTimezone_DeepNightA: u8,
#[allow(unused)]
	DstTimezone_DeepNightB: u8,
#[allow(unused)]
	reserved: [u8;1],
#[allow(unused)]
	PostPlayIngameTime: f32,
}

struct CUTSCENE_GPARAM_WEATHER_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParam_Debug: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	DstWeather_Sunny: i16,
#[allow(unused)]
	DstWeather_ClearSky: i16,
#[allow(unused)]
	DstWeather_WeakCloudy: i16,
#[allow(unused)]
	DstWeather_Cloud: i16,
#[allow(unused)]
	DstWeather_Rain: i16,
#[allow(unused)]
	DstWeather_HeavyRain: i16,
#[allow(unused)]
	DstWeather_Storm: i16,
#[allow(unused)]
	DstWeather_StormForBattle: i16,
#[allow(unused)]
	DstWeather_Snow: i16,
#[allow(unused)]
	DstWeather_HeavySnow: i16,
#[allow(unused)]
	DstWeather_Fog: i16,
#[allow(unused)]
	DstWeather_HeavyFog: i16,
#[allow(unused)]
	DstWeather_SandStorm: i16,
#[allow(unused)]
	DstWeather_HeavyFogRain: i16,
#[allow(unused)]
	PostPlayIngameWeather: i16,
#[allow(unused)]
	IndoorOutdoorType: u8,
#[allow(unused)]
	TakeOverDstWeather_Sunny: u8,
#[allow(unused)]
	TakeOverDstWeather_ClearSky: u8,
#[allow(unused)]
	TakeOverDstWeather_WeakCloudy: u8,
#[allow(unused)]
	TakeOverDstWeather_Cloud: u8,
#[allow(unused)]
	TakeOverDstWeather_Rain: u8,
#[allow(unused)]
	TakeOverDstWeather_HeavyRain: u8,
#[allow(unused)]
	TakeOverDstWeather_Storm: u8,
#[allow(unused)]
	TakeOverDstWeather_StormForBattle: u8,
#[allow(unused)]
	TakeOverDstWeather_Snow: u8,
#[allow(unused)]
	TakeOverDstWeather_HeavySnow: u8,
#[allow(unused)]
	TakeOverDstWeather_Fog: u8,
#[allow(unused)]
	TakeOverDstWeather_HeavyFog: u8,
#[allow(unused)]
	TakeOverDstWeather_SandStorm: u8,
#[allow(unused)]
	TakeOverDstWeather_HeavyFogRain: u8,
#[allow(unused)]
	reserved: [u8;7],
#[allow(unused)]
	DstWeather_Snowstorm: i16,
#[allow(unused)]
	DstWeather_LightningStorm: i16,
#[allow(unused)]
	DstWeather_Reserved3: i16,
#[allow(unused)]
	DstWeather_Reserved4: i16,
#[allow(unused)]
	DstWeather_Reserved5: i16,
#[allow(unused)]
	DstWeather_Reserved6: i16,
#[allow(unused)]
	DstWeather_Reserved7: i16,
#[allow(unused)]
	DstWeather_Reserved8: i16,
#[allow(unused)]
	TakeOverDstWeather_Snowstorm: u8,
#[allow(unused)]
	TakeOverDstWeather_LightningStorm: u8,
#[allow(unused)]
	TakeOverDstWeather_Reserved3: u8,
#[allow(unused)]
	TakeOverDstWeather_Reserved4: u8,
#[allow(unused)]
	TakeOverDstWeather_Reserved5: u8,
#[allow(unused)]
	TakeOverDstWeather_Reserved6: u8,
#[allow(unused)]
	TakeOverDstWeather_Reserved7: u8,
#[allow(unused)]
	TakeOverDstWeather_Reserved8: u8,
#[allow(unused)]
	IsEnableApplyMapGdRegionIdForGparam: u8,
#[allow(unused)]
	reserved2: [u8;1],
#[allow(unused)]
	OverrideMapGdRegionId: i16,
#[allow(unused)]
	reserved1: [u8;12],
}

struct CUTSCENE_MAP_ID_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParam_Debug: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	PlayMapId: i32,
#[allow(unused)]
	RequireMapId0: i32,
#[allow(unused)]
	RequireMapId1: i32,
#[allow(unused)]
	RequireMapId2: i32,
#[allow(unused)]
	RefCamPosHitPartsID: i32,
#[allow(unused)]
	reserved_2: [u8;12],
#[allow(unused)]
	ClientDisableViewTimeForProgress: i16,
#[allow(unused)]
	reserved: [u8;2],
#[allow(unused)]
	HitParts_0: i32,
#[allow(unused)]
	HitParts_1: i32,
}

struct CUTSCENE_TEXTURE_LOAD_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParam_Debug: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	texName_00: [u8;16],
#[allow(unused)]
	texName_01: [u8;16],
#[allow(unused)]
	texName_02: [u8;16],
#[allow(unused)]
	texName_03: [u8;16],
#[allow(unused)]
	texName_04: [u8;16],
#[allow(unused)]
	texName_05: [u8;16],
#[allow(unused)]
	texName_06: [u8;16],
#[allow(unused)]
	texName_07: [u8;16],
#[allow(unused)]
	texName_08: [u8;16],
#[allow(unused)]
	texName_09: [u8;16],
#[allow(unused)]
	texName_10: [u8;16],
#[allow(unused)]
	texName_11: [u8;16],
#[allow(unused)]
	texName_12: [u8;16],
#[allow(unused)]
	texName_13: [u8;16],
#[allow(unused)]
	texName_14: [u8;16],
#[allow(unused)]
	texName_15: [u8;16],
}

struct CUTSCENE_TIMEZONE_CONVERT_PARAM_ST {
#[allow(unused)]
	SrcTimezoneStart: f32,
#[allow(unused)]
	DstCutscenTime: f32,
}

struct CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST {
#[allow(unused)]
	weatherOverrideGparamId: i32,
}

struct DECAL_PARAM_ST {
#[allow(unused)]
	textureId: i32,
#[allow(unused)]
	dmypolyId: i32,
#[allow(unused)]
	pitchAngle: f32,
#[allow(unused)]
	yawAngle: f32,
#[allow(unused)]
	nearDistance: f32,
#[allow(unused)]
	farDistance: f32,
#[allow(unused)]
	nearSize: f32,
#[allow(unused)]
	farSize: f32,
#[allow(unused)]
	maskSpeffectId: i32,
#[allow(unused)]
	pad_10: i32,
#[allow(unused)]
	replaceTextureId_byMaterial: i32,
#[allow(unused)]
	dmypolyCategory: i32,
#[allow(unused)]
	pad_05: i32,
#[allow(unused)]
	useDeferredDecal: i32,
#[allow(unused)]
	usePaintDecal: i32,
#[allow(unused)]
	bloodTypeEnable: i32,
#[allow(unused)]
	bUseNormal: i32,
#[allow(unused)]
	pad_08: i32,
#[allow(unused)]
	pad_09: i32,
#[allow(unused)]
	usePom: i32,
#[allow(unused)]
	useEmissive: i32,
#[allow(unused)]
	putVertical: i32,
#[allow(unused)]
	randomSizeMin: i16,
#[allow(unused)]
	randomSizeMax: i16,
#[allow(unused)]
	randomRollMin: f32,
#[allow(unused)]
	randomRollMax: f32,
#[allow(unused)]
	randomPitchMin: f32,
#[allow(unused)]
	randomPitchMax: f32,
#[allow(unused)]
	randomYawMin: f32,
#[allow(unused)]
	randomYawMax: f32,
#[allow(unused)]
	pomHightScale: f32,
#[allow(unused)]
	pomSampleMin: u8,
#[allow(unused)]
	pomSampleMax: u8,
#[allow(unused)]
	blendMode: i8,
#[allow(unused)]
	appearDirType: i8,
#[allow(unused)]
	emissiveValueBegin: f32,
#[allow(unused)]
	emissiveValueEnd: f32,
#[allow(unused)]
	emissiveTime: f32,
#[allow(unused)]
	bIntpEnable: u8,
#[allow(unused)]
	pad_01: [u8;3],
#[allow(unused)]
	intpIntervalDist: f32,
#[allow(unused)]
	beginIntpTextureId: i32,
#[allow(unused)]
	endIntpTextureId: i32,
#[allow(unused)]
	appearSfxId: i32,
#[allow(unused)]
	appearSfxOffsetPos: f32,
#[allow(unused)]
	maskTextureId: i32,
#[allow(unused)]
	diffuseTextureId: i32,
#[allow(unused)]
	reflecTextureId: i32,
#[allow(unused)]
	maskScale: f32,
#[allow(unused)]
	normalTextureId: i32,
#[allow(unused)]
	heightTextureId: i32,
#[allow(unused)]
	emissiveTextureId: i32,
#[allow(unused)]
	diffuseColorR: u8,
#[allow(unused)]
	diffuseColorG: u8,
#[allow(unused)]
	diffuseColorB: u8,
#[allow(unused)]
	pad_03: [u8;1],
#[allow(unused)]
	reflecColorR: u8,
#[allow(unused)]
	reflecColorG: u8,
#[allow(unused)]
	reflecColorB: u8,
#[allow(unused)]
	bLifeEnable: u8,
#[allow(unused)]
	siniScale: f32,
#[allow(unused)]
	lifeTimeSec: f32,
#[allow(unused)]
	fadeOutTimeSec: f32,
#[allow(unused)]
	priority: i16,
#[allow(unused)]
	bDistThinOutEnable: u8,
#[allow(unused)]
	bAlignedTexRandomVariationEnable: u8,
#[allow(unused)]
	distThinOutCheckDist: f32,
#[allow(unused)]
	distThinOutCheckAngleDeg: f32,
#[allow(unused)]
	distThinOutMaxNum: u8,
#[allow(unused)]
	distThinOutCheckNum: u8,
#[allow(unused)]
	delayAppearFrame: i16,
#[allow(unused)]
	randVaria_Diffuse: i32,
#[allow(unused)]
	randVaria_Mask: i32,
#[allow(unused)]
	randVaria_Reflec: i32,
#[allow(unused)]
	pad_12: i32,
#[allow(unused)]
	randVaria_Normal: i32,
#[allow(unused)]
	randVaria_Height: i32,
#[allow(unused)]
	randVaria_Emissive: i32,
#[allow(unused)]
	pad_11: i32,
#[allow(unused)]
	fadeInTimeSec: f32,
#[allow(unused)]
	thinOutOverlapMultiRadius: f32,
#[allow(unused)]
	thinOutNeighborAddRadius: f32,
#[allow(unused)]
	thinOutOverlapLimitNum: i32,
#[allow(unused)]
	thinOutNeighborLimitNum: i32,
#[allow(unused)]
	thinOutMode: i8,
#[allow(unused)]
	emissiveColorR: u8,
#[allow(unused)]
	emissiveColorG: u8,
#[allow(unused)]
	emissiveColorB: u8,
#[allow(unused)]
	maxDecalSfxCreatableSlopeAngleDeg: f32,
#[allow(unused)]
	pad_02: [u8;40],
}

struct DEFAULT_KEY_ASSIGN {
#[allow(unused)]
	priority0: u8,
#[allow(unused)]
	priority1: u8,
#[allow(unused)]
	priority2: u8,
#[allow(unused)]
	priority3: u8,
#[allow(unused)]
	priority4: u8,
#[allow(unused)]
	priority5: u8,
#[allow(unused)]
	priority6: u8,
#[allow(unused)]
	priority7: u8,
#[allow(unused)]
	priority8: u8,
#[allow(unused)]
	priority9: u8,
#[allow(unused)]
	priority10: u8,
#[allow(unused)]
	priority11: u8,
#[allow(unused)]
	priority12: u8,
#[allow(unused)]
	priority13: u8,
#[allow(unused)]
	priority14: u8,
#[allow(unused)]
	priority15: u8,
#[allow(unused)]
	priority16: u8,
#[allow(unused)]
	priority17: u8,
#[allow(unused)]
	priority18: u8,
#[allow(unused)]
	priority19: u8,
#[allow(unused)]
	priority20: u8,
#[allow(unused)]
	priority21: u8,
#[allow(unused)]
	priority22: u8,
#[allow(unused)]
	priority23: u8,
#[allow(unused)]
	priority24: u8,
#[allow(unused)]
	priority25: u8,
#[allow(unused)]
	priority26: u8,
#[allow(unused)]
	priority27: u8,
#[allow(unused)]
	priority28: u8,
#[allow(unused)]
	priority29: u8,
#[allow(unused)]
	priority30: u8,
#[allow(unused)]
	priority31: u8,
#[allow(unused)]
	dummy: [u8;12],
#[allow(unused)]
	phyisicalKey_0: i32,
#[allow(unused)]
	traitsType_0: u8,
#[allow(unused)]
	a2dOperator_0: u8,
#[allow(unused)]
	applyTarget_0: u8,
#[allow(unused)]
	isAnalog_0: u8,
#[allow(unused)]
	enableWin64_0: u8,
#[allow(unused)]
	enablePS4_0: u8,
#[allow(unused)]
	enableXboxOne_0: u8,
#[allow(unused)]
	time1_0: f32,
#[allow(unused)]
	time2_0: f32,
#[allow(unused)]
	a2dThreshold_0: f32,
#[allow(unused)]
	phyisicalKey_1: i32,
#[allow(unused)]
	traitsType_1: u8,
#[allow(unused)]
	a2dOperator_1: u8,
#[allow(unused)]
	applyTarget_1: u8,
#[allow(unused)]
	isAnalog_1: u8,
#[allow(unused)]
	enableWin64_1: u8,
#[allow(unused)]
	enablePS4_1: u8,
#[allow(unused)]
	enableXboxOne_1: u8,
#[allow(unused)]
	time1_1: f32,
#[allow(unused)]
	time2_1: f32,
#[allow(unused)]
	a2dThreshold_1: f32,
#[allow(unused)]
	phyisicalKey_2: i32,
#[allow(unused)]
	traitsType_2: u8,
#[allow(unused)]
	a2dOperator_2: u8,
#[allow(unused)]
	applyTarget_2: u8,
#[allow(unused)]
	isAnalog_2: u8,
#[allow(unused)]
	enableWin64_2: u8,
#[allow(unused)]
	enablePS4_2: u8,
#[allow(unused)]
	enableXboxOne_2: u8,
#[allow(unused)]
	time1_2: f32,
#[allow(unused)]
	time2_2: f32,
#[allow(unused)]
	a2dThreshold_2: f32,
#[allow(unused)]
	phyisicalKey_3: i32,
#[allow(unused)]
	traitsType_3: u8,
#[allow(unused)]
	a2dOperator_3: u8,
#[allow(unused)]
	applyTarget_3: u8,
#[allow(unused)]
	isAnalog_3: u8,
#[allow(unused)]
	enableWin64_3: u8,
#[allow(unused)]
	enablePS4_3: u8,
#[allow(unused)]
	enableXboxOne_3: u8,
#[allow(unused)]
	time1_3: f32,
#[allow(unused)]
	time2_3: f32,
#[allow(unused)]
	a2dThreshold_3: f32,
#[allow(unused)]
	phyisicalKey_4: i32,
#[allow(unused)]
	traitsType_4: u8,
#[allow(unused)]
	a2dOperator_4: u8,
#[allow(unused)]
	applyTarget_4: u8,
#[allow(unused)]
	isAnalog_4: u8,
#[allow(unused)]
	enableWin64_4: u8,
#[allow(unused)]
	enablePS4_4: u8,
#[allow(unused)]
	enableXboxOne_4: u8,
#[allow(unused)]
	time1_4: f32,
#[allow(unused)]
	time2_4: f32,
#[allow(unused)]
	a2dThreshold_4: f32,
#[allow(unused)]
	phyisicalKey_5: i32,
#[allow(unused)]
	traitsType_5: u8,
#[allow(unused)]
	a2dOperator_5: u8,
#[allow(unused)]
	applyTarget_5: u8,
#[allow(unused)]
	isAnalog_5: u8,
#[allow(unused)]
	enableWin64_5: u8,
#[allow(unused)]
	enablePS4_5: u8,
#[allow(unused)]
	enableXboxOne_5: u8,
#[allow(unused)]
	time1_5: f32,
#[allow(unused)]
	time2_5: f32,
#[allow(unused)]
	a2dThreshold_5: f32,
#[allow(unused)]
	phyisicalKey_6: i32,
#[allow(unused)]
	traitsType_6: u8,
#[allow(unused)]
	a2dOperator_6: u8,
#[allow(unused)]
	applyTarget_6: u8,
#[allow(unused)]
	isAnalog_6: u8,
#[allow(unused)]
	enableWin64_6: u8,
#[allow(unused)]
	enablePS4_6: u8,
#[allow(unused)]
	enableXboxOne_6: u8,
#[allow(unused)]
	time1_6: f32,
#[allow(unused)]
	time2_6: f32,
#[allow(unused)]
	a2dThreshold_6: f32,
#[allow(unused)]
	phyisicalKey_7: i32,
#[allow(unused)]
	traitsType_7: u8,
#[allow(unused)]
	a2dOperator_7: u8,
#[allow(unused)]
	applyTarget_7: u8,
#[allow(unused)]
	isAnalog_7: u8,
#[allow(unused)]
	enableWin64_7: u8,
#[allow(unused)]
	enablePS4_7: u8,
#[allow(unused)]
	enableXboxOne_7: u8,
#[allow(unused)]
	time1_7: f32,
#[allow(unused)]
	time2_7: f32,
#[allow(unused)]
	a2dThreshold_7: f32,
}

struct DIRECTION_CAMERA_PARAM_ST {
#[allow(unused)]
	isUseOption: u8,
#[allow(unused)]
	pad2: [u8;1],
#[allow(unused)]
	pad1: [u8;15],
}

struct ENEMY_COMMON_PARAM_ST {
#[allow(unused)]
	reserved0: [u8;8],
#[allow(unused)]
	soundTargetTryApproachTime: i32,
#[allow(unused)]
	searchTargetTryApproachTime: i32,
#[allow(unused)]
	memoryTargetTryApproachTime: i32,
#[allow(unused)]
	reserved5: [u8;40],
#[allow(unused)]
	activateChrByTime_PhantomId: i32,
#[allow(unused)]
	findUnfavorableFailedPointDist: f32,
#[allow(unused)]
	findUnfavorableFailedPointHeight: f32,
#[allow(unused)]
	reserved18: [u8;184],
}

struct ENEMY_STANDARD_INFO_BANK {
#[allow(unused)]
	EnemyBehaviorID: i32,
#[allow(unused)]
	HP: i16,
#[allow(unused)]
	AttackPower: i16,
#[allow(unused)]
	ChrType: i32,
#[allow(unused)]
	HitHeight: f32,
#[allow(unused)]
	HitRadius: f32,
#[allow(unused)]
	Weight: f32,
#[allow(unused)]
	DynamicFriction: f32,
#[allow(unused)]
	StaticFriction: f32,
#[allow(unused)]
	UpperDefState: i32,
#[allow(unused)]
	ActionDefState: i32,
#[allow(unused)]
	RotY_per_Second: f32,
#[allow(unused)]
	reserve0: [u8;20],
#[allow(unused)]
	RotY_per_Second_old: u8,
#[allow(unused)]
	EnableSideStep: u8,
#[allow(unused)]
	UseRagdollHit: u8,
#[allow(unused)]
	reserve_last: [u8;5],
#[allow(unused)]
	stamina: i16,
#[allow(unused)]
	staminaRecover: i16,
#[allow(unused)]
	staminaConsumption: i16,
#[allow(unused)]
	deffenct_Phys: i16,
#[allow(unused)]
	reserve_last2: [u8;48],
}

struct ENV_OBJ_LOT_PARAM_ST {
#[allow(unused)]
	AssetId_0: i32,
#[allow(unused)]
	AssetId_1: i32,
#[allow(unused)]
	AssetId_2: i32,
#[allow(unused)]
	AssetId_3: i32,
#[allow(unused)]
	AssetId_4: i32,
#[allow(unused)]
	AssetId_5: i32,
#[allow(unused)]
	AssetId_6: i32,
#[allow(unused)]
	AssetId_7: i32,
#[allow(unused)]
	CreateWeight_0: u8,
#[allow(unused)]
	CreateWeight_1: u8,
#[allow(unused)]
	CreateWeight_2: u8,
#[allow(unused)]
	CreateWeight_3: u8,
#[allow(unused)]
	CreateWeight_4: u8,
#[allow(unused)]
	CreateWeight_5: u8,
#[allow(unused)]
	CreateWeight_6: u8,
#[allow(unused)]
	CreateWeight_7: u8,
#[allow(unused)]
	Reserve_0: [u8;24],
}

struct EQUIP_MTRL_SET_PARAM_ST {
#[allow(unused)]
	materialId01: i32,
#[allow(unused)]
	materialId02: i32,
#[allow(unused)]
	materialId03: i32,
#[allow(unused)]
	materialId04: i32,
#[allow(unused)]
	materialId05: i32,
#[allow(unused)]
	materialId06: i32,
#[allow(unused)]
	pad_id: [u8;8],
#[allow(unused)]
	itemNum01: i8,
#[allow(unused)]
	itemNum02: i8,
#[allow(unused)]
	itemNum03: i8,
#[allow(unused)]
	itemNum04: i8,
#[allow(unused)]
	itemNum05: i8,
#[allow(unused)]
	itemNum06: i8,
#[allow(unused)]
	pad_num: [u8;2],
#[allow(unused)]
	materialCate01: u8,
#[allow(unused)]
	materialCate02: u8,
#[allow(unused)]
	materialCate03: u8,
#[allow(unused)]
	materialCate04: u8,
#[allow(unused)]
	materialCate05: u8,
#[allow(unused)]
	materialCate06: u8,
#[allow(unused)]
	pad_cate: [u8;2],
#[allow(unused)]
	isDisableDispNum01: u8,
#[allow(unused)]
	isDisableDispNum02: u8,
#[allow(unused)]
	isDisableDispNum03: u8,
#[allow(unused)]
	isDisableDispNum04: u8,
#[allow(unused)]
	isDisableDispNum05: u8,
#[allow(unused)]
	isDisableDispNum06: u8,
#[allow(unused)]
	pad: [u8;3],
}

struct EQUIP_PARAM_ACCESSORY_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	refId: i32,
#[allow(unused)]
	sfxVariationId: i32,
#[allow(unused)]
	weight: f32,
#[allow(unused)]
	behaviorId: i32,
#[allow(unused)]
	basicPrice: i32,
#[allow(unused)]
	sellValue: i32,
#[allow(unused)]
	sortId: i32,
#[allow(unused)]
	qwcId: i32,
#[allow(unused)]
	equipModelId: i16,
#[allow(unused)]
	iconId: i16,
#[allow(unused)]
	shopLv: i16,
#[allow(unused)]
	trophySGradeId: i16,
#[allow(unused)]
	trophySeqId: i16,
#[allow(unused)]
	equipModelCategory: u8,
#[allow(unused)]
	equipModelGender: u8,
#[allow(unused)]
	accessoryCategory: u8,
#[allow(unused)]
	refCategory: u8,
#[allow(unused)]
	spEffectCategory: u8,
#[allow(unused)]
	sortGroupId: u8,
#[allow(unused)]
	vagrantItemLotId: i32,
#[allow(unused)]
	vagrantBonusEneDropItemLotId: i32,
#[allow(unused)]
	vagrantItemEneDropItemLotId: i32,
#[allow(unused)]
	isDeposit: u8,
#[allow(unused)]
	isEquipOutBrake: u8,
#[allow(unused)]
	disableMultiDropShare: u8,
#[allow(unused)]
	isDiscard: u8,
#[allow(unused)]
	isDrop: u8,
#[allow(unused)]
	showLogCondType: u8,
#[allow(unused)]
	showDialogCondType: u8,
#[allow(unused)]
	rarity: u8,
#[allow(unused)]
	pad2: [u8;2],
#[allow(unused)]
	saleValue: i32,
#[allow(unused)]
	accessoryGroup: i16,
#[allow(unused)]
	pad3: [u8;1],
#[allow(unused)]
	compTrophySedId: i8,
#[allow(unused)]
	residentSpEffectId1: i32,
#[allow(unused)]
	residentSpEffectId2: i32,
#[allow(unused)]
	residentSpEffectId3: i32,
#[allow(unused)]
	residentSpEffectId4: i32,
#[allow(unused)]
	pad1: [u8;4],
}

struct EQUIP_PARAM_CUSTOM_WEAPON_ST {
#[allow(unused)]
	baseWepId: i32,
#[allow(unused)]
	gemId: i32,
#[allow(unused)]
	reinforceLv: u8,
#[allow(unused)]
	pad: [u8;7],
}

struct EQUIP_PARAM_GEM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	iconId: i16,
#[allow(unused)]
	rank: i8,
#[allow(unused)]
	sortGroupId: u8,
#[allow(unused)]
	spEffectId0: i32,
#[allow(unused)]
	spEffectId1: i32,
#[allow(unused)]
	spEffectId2: i32,
#[allow(unused)]
	itemGetTutorialFlagId: i32,
#[allow(unused)]
	swordArtsParamId: i32,
#[allow(unused)]
	mountValue: i32,
#[allow(unused)]
	sellValue: i32,
#[allow(unused)]
	saleValue: i32,
#[allow(unused)]
	sortId: i32,
#[allow(unused)]
	compTrophySedId: i16,
#[allow(unused)]
	trophySeqId: i16,
#[allow(unused)]
	configurableWepAttr00: u8,
#[allow(unused)]
	configurableWepAttr01: u8,
#[allow(unused)]
	configurableWepAttr02: u8,
#[allow(unused)]
	configurableWepAttr03: u8,
#[allow(unused)]
	configurableWepAttr04: u8,
#[allow(unused)]
	configurableWepAttr05: u8,
#[allow(unused)]
	configurableWepAttr06: u8,
#[allow(unused)]
	configurableWepAttr07: u8,
#[allow(unused)]
	configurableWepAttr08: u8,
#[allow(unused)]
	configurableWepAttr09: u8,
#[allow(unused)]
	configurableWepAttr10: u8,
#[allow(unused)]
	configurableWepAttr11: u8,
#[allow(unused)]
	configurableWepAttr12: u8,
#[allow(unused)]
	configurableWepAttr13: u8,
#[allow(unused)]
	configurableWepAttr14: u8,
#[allow(unused)]
	configurableWepAttr15: u8,
#[allow(unused)]
	rarity: u8,
#[allow(unused)]
	configurableWepAttr16: u8,
#[allow(unused)]
	configurableWepAttr17: u8,
#[allow(unused)]
	configurableWepAttr18: u8,
#[allow(unused)]
	configurableWepAttr19: u8,
#[allow(unused)]
	configurableWepAttr20: u8,
#[allow(unused)]
	configurableWepAttr21: u8,
#[allow(unused)]
	configurableWepAttr22: u8,
#[allow(unused)]
	configurableWepAttr23: u8,
#[allow(unused)]
	isDiscard: u8,
#[allow(unused)]
	isDrop: u8,
#[allow(unused)]
	isDeposit: u8,
#[allow(unused)]
	disableMultiDropShare: u8,
#[allow(unused)]
	showDialogCondType: u8,
#[allow(unused)]
	showLogCondType: u8,
#[allow(unused)]
	pad: [u8;1],
#[allow(unused)]
	defaultWepAttr: u8,
#[allow(unused)]
	pad2: [u8;2],
#[allow(unused)]
	canMountWep_Dagger: u8,
#[allow(unused)]
	canMountWep_SwordNormal: u8,
#[allow(unused)]
	canMountWep_SwordLarge: u8,
#[allow(unused)]
	canMountWep_SwordGigantic: u8,
#[allow(unused)]
	canMountWep_SaberNormal: u8,
#[allow(unused)]
	canMountWep_SaberLarge: u8,
#[allow(unused)]
	canMountWep_katana: u8,
#[allow(unused)]
	canMountWep_SwordDoubleEdge: u8,
#[allow(unused)]
	canMountWep_SwordPierce: u8,
#[allow(unused)]
	canMountWep_RapierHeavy: u8,
#[allow(unused)]
	canMountWep_AxeNormal: u8,
#[allow(unused)]
	canMountWep_AxeLarge: u8,
#[allow(unused)]
	canMountWep_HammerNormal: u8,
#[allow(unused)]
	canMountWep_HammerLarge: u8,
#[allow(unused)]
	canMountWep_Flail: u8,
#[allow(unused)]
	canMountWep_SpearNormal: u8,
#[allow(unused)]
	canMountWep_SpearLarge: u8,
#[allow(unused)]
	canMountWep_SpearHeavy: u8,
#[allow(unused)]
	canMountWep_SpearAxe: u8,
#[allow(unused)]
	canMountWep_Sickle: u8,
#[allow(unused)]
	canMountWep_Knuckle: u8,
#[allow(unused)]
	canMountWep_Claw: u8,
#[allow(unused)]
	canMountWep_Whip: u8,
#[allow(unused)]
	canMountWep_AxhammerLarge: u8,
#[allow(unused)]
	canMountWep_BowSmall: u8,
#[allow(unused)]
	canMountWep_BowNormal: u8,
#[allow(unused)]
	canMountWep_BowLarge: u8,
#[allow(unused)]
	canMountWep_ClossBow: u8,
#[allow(unused)]
	canMountWep_Ballista: u8,
#[allow(unused)]
	canMountWep_Staff: u8,
#[allow(unused)]
	canMountWep_Sorcery: u8,
#[allow(unused)]
	canMountWep_Talisman: u8,
#[allow(unused)]
	canMountWep_ShieldSmall: u8,
#[allow(unused)]
	canMountWep_ShieldNormal: u8,
#[allow(unused)]
	canMountWep_ShieldLarge: u8,
#[allow(unused)]
	canMountWep_Torch: u8,
#[allow(unused)]
	reserved_canMountWep: [u8;1],
#[allow(unused)]
	reserved2_canMountWep: [u8;3],
#[allow(unused)]
	spEffectMsgId0: i32,
#[allow(unused)]
	spEffectMsgId1: i32,
#[allow(unused)]
	spEffectId_forAtk0: i32,
#[allow(unused)]
	spEffectId_forAtk1: i32,
#[allow(unused)]
	spEffectId_forAtk2: i32,
#[allow(unused)]
	mountWepTextId: i32,
#[allow(unused)]
	pad6: [u8;8],
}

struct EQUIP_PARAM_GOODS_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	refId_default: i32,
#[allow(unused)]
	sfxVariationId: i32,
#[allow(unused)]
	weight: f32,
#[allow(unused)]
	basicPrice: i32,
#[allow(unused)]
	sellValue: i32,
#[allow(unused)]
	behaviorId: i32,
#[allow(unused)]
	replaceItemId: i32,
#[allow(unused)]
	sortId: i32,
#[allow(unused)]
	appearanceReplaceItemId: i32,
#[allow(unused)]
	yesNoDialogMessageId: i32,
#[allow(unused)]
	useEnableSpEffectType: i16,
#[allow(unused)]
	potGroupId: i8,
#[allow(unused)]
	pad: [u8;1],
#[allow(unused)]
	iconId: i16,
#[allow(unused)]
	modelId: i16,
#[allow(unused)]
	shopLv: i16,
#[allow(unused)]
	compTrophySedId: i16,
#[allow(unused)]
	trophySeqId: i16,
#[allow(unused)]
	maxNum: i16,
#[allow(unused)]
	consumeHeroPoint: u8,
#[allow(unused)]
	overDexterity: u8,
#[allow(unused)]
	goodsType: u8,
#[allow(unused)]
	refCategory: u8,
#[allow(unused)]
	spEffectCategory: u8,
#[allow(unused)]
	pad3: [u8;1],
#[allow(unused)]
	goodsUseAnim: u8,
#[allow(unused)]
	opmeMenuType: u8,
#[allow(unused)]
	useLimitCategory: u8,
#[allow(unused)]
	replaceCategory: u8,
#[allow(unused)]
	reserve4: [u8;2],
#[allow(unused)]
	enable_live: u8,
#[allow(unused)]
	enable_gray: u8,
#[allow(unused)]
	enable_white: u8,
#[allow(unused)]
	enable_black: u8,
#[allow(unused)]
	enable_multi: u8,
#[allow(unused)]
	disable_offline: u8,
#[allow(unused)]
	isEquip: u8,
#[allow(unused)]
	isConsume: u8,
#[allow(unused)]
	isAutoEquip: u8,
#[allow(unused)]
	isEstablishment: u8,
#[allow(unused)]
	isOnlyOne: u8,
#[allow(unused)]
	isDiscard: u8,
#[allow(unused)]
	isDeposit: u8,
#[allow(unused)]
	isDisableHand: u8,
#[allow(unused)]
	isRemoveItem_forGameClear: u8,
#[allow(unused)]
	isSuppleItem: u8,
#[allow(unused)]
	isFullSuppleItem: u8,
#[allow(unused)]
	isEnhance: u8,
#[allow(unused)]
	isFixItem: u8,
#[allow(unused)]
	disableMultiDropShare: u8,
#[allow(unused)]
	disableUseAtColiseum: u8,
#[allow(unused)]
	disableUseAtOutOfColiseum: u8,
#[allow(unused)]
	isEnableFastUseItem: u8,
#[allow(unused)]
	isApplySpecialEffect: u8,
#[allow(unused)]
	syncNumVaryId: u8,
#[allow(unused)]
	refId_1: i32,
#[allow(unused)]
	refVirtualWepId: i32,
#[allow(unused)]
	vagrantItemLotId: i32,
#[allow(unused)]
	vagrantBonusEneDropItemLotId: i32,
#[allow(unused)]
	vagrantItemEneDropItemLotId: i32,
#[allow(unused)]
	castSfxId: i32,
#[allow(unused)]
	fireSfxId: i32,
#[allow(unused)]
	effectSfxId: i32,
#[allow(unused)]
	enable_ActiveBigRune: u8,
#[allow(unused)]
	isBonfireWarpItem: u8,
#[allow(unused)]
	enable_Ladder: u8,
#[allow(unused)]
	isUseMultiPlayPreparation: u8,
#[allow(unused)]
	canMultiUse: u8,
#[allow(unused)]
	isShieldEnchant: u8,
#[allow(unused)]
	isWarpProhibited: u8,
#[allow(unused)]
	isUseMultiPenaltyOnly: u8,
#[allow(unused)]
	suppleType: u8,
#[allow(unused)]
	autoReplenishType: u8,
#[allow(unused)]
	isDrop: u8,
#[allow(unused)]
	showLogCondType: u8,
#[allow(unused)]
	isSummonHorse: u8,
#[allow(unused)]
	showDialogCondType: u8,
#[allow(unused)]
	isSleepCollectionItem: u8,
#[allow(unused)]
	enableRiding: u8,
#[allow(unused)]
	disableRiding: u8,
#[allow(unused)]
	maxRepositoryNum: i16,
#[allow(unused)]
	sortGroupId: u8,
#[allow(unused)]
	isUseNoAttackRegion: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	saleValue: i32,
#[allow(unused)]
	rarity: u8,
#[allow(unused)]
	useLimitSummonBuddy: u8,
#[allow(unused)]
	useLimitSpEffectType: i16,
#[allow(unused)]
	aiUseJudgeId: i32,
#[allow(unused)]
	consumeMP: i16,
#[allow(unused)]
	consumeHP: i16,
#[allow(unused)]
	reinforceGoodsId: i32,
#[allow(unused)]
	reinforceMaterialId: i32,
#[allow(unused)]
	reinforcePrice: i32,
#[allow(unused)]
	useLevel_vowType0: i8,
#[allow(unused)]
	useLevel_vowType1: i8,
#[allow(unused)]
	useLevel_vowType2: i8,
#[allow(unused)]
	useLevel_vowType3: i8,
#[allow(unused)]
	useLevel_vowType4: i8,
#[allow(unused)]
	useLevel_vowType5: i8,
#[allow(unused)]
	useLevel_vowType6: i8,
#[allow(unused)]
	useLevel_vowType7: i8,
#[allow(unused)]
	useLevel_vowType8: i8,
#[allow(unused)]
	useLevel_vowType9: i8,
#[allow(unused)]
	useLevel_vowType10: i8,
#[allow(unused)]
	useLevel_vowType11: i8,
#[allow(unused)]
	useLevel_vowType12: i8,
#[allow(unused)]
	useLevel_vowType13: i8,
#[allow(unused)]
	useLevel_vowType14: i8,
#[allow(unused)]
	useLevel_vowType15: i8,
#[allow(unused)]
	useLevel: i16,
#[allow(unused)]
	reserve5: [u8;2],
#[allow(unused)]
	itemGetTutorialFlagId: i32,
#[allow(unused)]
	reserve3: [u8;8],
}

struct EQUIP_PARAM_PROTECTOR_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	sortId: i32,
#[allow(unused)]
	wanderingEquipId: i32,
#[allow(unused)]
	resistSleep: i16,
#[allow(unused)]
	resistMadness: i16,
#[allow(unused)]
	saDurability: f32,
#[allow(unused)]
	toughnessCorrectRate: f32,
#[allow(unused)]
	fixPrice: i32,
#[allow(unused)]
	basicPrice: i32,
#[allow(unused)]
	sellValue: i32,
#[allow(unused)]
	weight: f32,
#[allow(unused)]
	residentSpEffectId: i32,
#[allow(unused)]
	residentSpEffectId2: i32,
#[allow(unused)]
	residentSpEffectId3: i32,
#[allow(unused)]
	materialSetId: i32,
#[allow(unused)]
	partsDamageRate: f32,
#[allow(unused)]
	corectSARecover: f32,
#[allow(unused)]
	originEquipPro: i32,
#[allow(unused)]
	originEquipPro1: i32,
#[allow(unused)]
	originEquipPro2: i32,
#[allow(unused)]
	originEquipPro3: i32,
#[allow(unused)]
	originEquipPro4: i32,
#[allow(unused)]
	originEquipPro5: i32,
#[allow(unused)]
	originEquipPro6: i32,
#[allow(unused)]
	originEquipPro7: i32,
#[allow(unused)]
	originEquipPro8: i32,
#[allow(unused)]
	originEquipPro9: i32,
#[allow(unused)]
	originEquipPro10: i32,
#[allow(unused)]
	originEquipPro11: i32,
#[allow(unused)]
	originEquipPro12: i32,
#[allow(unused)]
	originEquipPro13: i32,
#[allow(unused)]
	originEquipPro14: i32,
#[allow(unused)]
	originEquipPro15: i32,
#[allow(unused)]
	faceScaleM_ScaleX: f32,
#[allow(unused)]
	faceScaleM_ScaleZ: f32,
#[allow(unused)]
	faceScaleM_MaxX: f32,
#[allow(unused)]
	faceScaleM_MaxZ: f32,
#[allow(unused)]
	faceScaleF_ScaleX: f32,
#[allow(unused)]
	faceScaleF_ScaleZ: f32,
#[allow(unused)]
	faceScaleF_MaxX: f32,
#[allow(unused)]
	faceScaleF_MaxZ: f32,
#[allow(unused)]
	qwcId: i32,
#[allow(unused)]
	equipModelId: i16,
#[allow(unused)]
	iconIdM: i16,
#[allow(unused)]
	iconIdF: i16,
#[allow(unused)]
	knockBack: i16,
#[allow(unused)]
	knockbackBounceRate: i16,
#[allow(unused)]
	durability: i16,
#[allow(unused)]
	durabilityMax: i16,
#[allow(unused)]
	pad03: [u8;2],
#[allow(unused)]
	defFlickPower: i16,
#[allow(unused)]
	defensePhysics: i16,
#[allow(unused)]
	defenseMagic: i16,
#[allow(unused)]
	defenseFire: i16,
#[allow(unused)]
	defenseThunder: i16,
#[allow(unused)]
	defenseSlash: i16,
#[allow(unused)]
	defenseBlow: i16,
#[allow(unused)]
	defenseThrust: i16,
#[allow(unused)]
	resistPoison: i16,
#[allow(unused)]
	resistDisease: i16,
#[allow(unused)]
	resistBlood: i16,
#[allow(unused)]
	resistCurse: i16,
#[allow(unused)]
	reinforceTypeId: i16,
#[allow(unused)]
	trophySGradeId: i16,
#[allow(unused)]
	shopLv: i16,
#[allow(unused)]
	knockbackParamId: u8,
#[allow(unused)]
	flickDamageCutRate: u8,
#[allow(unused)]
	equipModelCategory: u8,
#[allow(unused)]
	equipModelGender: u8,
#[allow(unused)]
	protectorCategory: u8,
#[allow(unused)]
	rarity: u8,
#[allow(unused)]
	sortGroupId: u8,
#[allow(unused)]
	partsDmgType: u8,
#[allow(unused)]
	pad04: [u8;2],
#[allow(unused)]
	isDeposit: u8,
#[allow(unused)]
	headEquip: u8,
#[allow(unused)]
	bodyEquip: u8,
#[allow(unused)]
	armEquip: u8,
#[allow(unused)]
	legEquip: u8,
#[allow(unused)]
	useFaceScale: u8,
#[allow(unused)]
	isSkipWeakDamageAnim: u8,
#[allow(unused)]
	pad06: [u8;1],
#[allow(unused)]
	defenseMaterialVariationValue_Weak: u8,
#[allow(unused)]
	autoFootEffectDecalBaseId2: i16,
#[allow(unused)]
	autoFootEffectDecalBaseId3: i16,
#[allow(unused)]
	defenseMaterialVariationValue: u8,
#[allow(unused)]
	isDiscard: u8,
#[allow(unused)]
	isDrop: u8,
#[allow(unused)]
	disableMultiDropShare: u8,
#[allow(unused)]
	simpleModelForDlc: u8,
#[allow(unused)]
	showLogCondType: u8,
#[allow(unused)]
	showDialogCondType: u8,
#[allow(unused)]
	pad: [u8;1],
#[allow(unused)]
	neutralDamageCutRate: f32,
#[allow(unused)]
	slashDamageCutRate: f32,
#[allow(unused)]
	blowDamageCutRate: f32,
#[allow(unused)]
	thrustDamageCutRate: f32,
#[allow(unused)]
	magicDamageCutRate: f32,
#[allow(unused)]
	fireDamageCutRate: f32,
#[allow(unused)]
	thunderDamageCutRate: f32,
#[allow(unused)]
	defenseMaterialSfx1: i16,
#[allow(unused)]
	defenseMaterialSfx_Weak1: i16,
#[allow(unused)]
	defenseMaterial1: i16,
#[allow(unused)]
	defenseMaterial_Weak1: i16,
#[allow(unused)]
	defenseMaterialSfx2: i16,
#[allow(unused)]
	defenseMaterialSfx_Weak2: i16,
#[allow(unused)]
	footMaterialSe: i16,
#[allow(unused)]
	defenseMaterial_Weak2: i16,
#[allow(unused)]
	autoFootEffectDecalBaseId1: i32,
#[allow(unused)]
	toughnessDamageCutRate: f32,
#[allow(unused)]
	toughnessRecoverCorrection: f32,
#[allow(unused)]
	darkDamageCutRate: f32,
#[allow(unused)]
	defenseDark: i16,
#[allow(unused)]
	invisibleFlag48: [u8;1],
#[allow(unused)]
	invisibleFlag49: [u8;1],
#[allow(unused)]
	invisibleFlag50: [u8;1],
#[allow(unused)]
	invisibleFlag51: [u8;1],
#[allow(unused)]
	invisibleFlag52: [u8;1],
#[allow(unused)]
	invisibleFlag53: [u8;1],
#[allow(unused)]
	invisibleFlag54: [u8;1],
#[allow(unused)]
	invisibleFlag55: [u8;1],
#[allow(unused)]
	invisibleFlag56: [u8;1],
#[allow(unused)]
	invisibleFlag57: [u8;1],
#[allow(unused)]
	invisibleFlag58: [u8;1],
#[allow(unused)]
	invisibleFlag59: [u8;1],
#[allow(unused)]
	invisibleFlag60: [u8;1],
#[allow(unused)]
	invisibleFlag61: [u8;1],
#[allow(unused)]
	invisibleFlag62: [u8;1],
#[allow(unused)]
	invisibleFlag63: [u8;1],
#[allow(unused)]
	invisibleFlag64: [u8;1],
#[allow(unused)]
	invisibleFlag65: [u8;1],
#[allow(unused)]
	invisibleFlag66: [u8;1],
#[allow(unused)]
	invisibleFlag67: [u8;1],
#[allow(unused)]
	invisibleFlag68: [u8;1],
#[allow(unused)]
	invisibleFlag69: [u8;1],
#[allow(unused)]
	invisibleFlag70: [u8;1],
#[allow(unused)]
	invisibleFlag71: [u8;1],
#[allow(unused)]
	invisibleFlag72: [u8;1],
#[allow(unused)]
	invisibleFlag73: [u8;1],
#[allow(unused)]
	invisibleFlag74: [u8;1],
#[allow(unused)]
	invisibleFlag75: [u8;1],
#[allow(unused)]
	invisibleFlag76: [u8;1],
#[allow(unused)]
	invisibleFlag77: [u8;1],
#[allow(unused)]
	invisibleFlag78: [u8;1],
#[allow(unused)]
	invisibleFlag79: [u8;1],
#[allow(unused)]
	invisibleFlag80: [u8;1],
#[allow(unused)]
	padbit: [u8;1],
#[allow(unused)]
	postureControlId: u8,
#[allow(unused)]
	pad2: [u8;4],
#[allow(unused)]
	saleValue: i32,
#[allow(unused)]
	resistFreeze: i16,
#[allow(unused)]
	invisibleFlag_SexVer00: u8,
#[allow(unused)]
	invisibleFlag_SexVer01: u8,
#[allow(unused)]
	invisibleFlag_SexVer02: u8,
#[allow(unused)]
	invisibleFlag_SexVer03: u8,
#[allow(unused)]
	invisibleFlag_SexVer04: u8,
#[allow(unused)]
	invisibleFlag_SexVer05: u8,
#[allow(unused)]
	invisibleFlag_SexVer06: u8,
#[allow(unused)]
	invisibleFlag_SexVer07: u8,
#[allow(unused)]
	invisibleFlag_SexVer08: u8,
#[allow(unused)]
	invisibleFlag_SexVer09: u8,
#[allow(unused)]
	invisibleFlag_SexVer10: u8,
#[allow(unused)]
	invisibleFlag_SexVer11: u8,
#[allow(unused)]
	invisibleFlag_SexVer12: u8,
#[allow(unused)]
	invisibleFlag_SexVer13: u8,
#[allow(unused)]
	invisibleFlag_SexVer14: u8,
#[allow(unused)]
	invisibleFlag_SexVer15: u8,
#[allow(unused)]
	invisibleFlag_SexVer16: u8,
#[allow(unused)]
	invisibleFlag_SexVer17: u8,
#[allow(unused)]
	invisibleFlag_SexVer18: u8,
#[allow(unused)]
	invisibleFlag_SexVer19: u8,
#[allow(unused)]
	invisibleFlag_SexVer20: u8,
#[allow(unused)]
	invisibleFlag_SexVer21: u8,
#[allow(unused)]
	invisibleFlag_SexVer22: u8,
#[allow(unused)]
	invisibleFlag_SexVer23: u8,
#[allow(unused)]
	invisibleFlag_SexVer24: u8,
#[allow(unused)]
	invisibleFlag_SexVer25: u8,
#[allow(unused)]
	invisibleFlag_SexVer26: u8,
#[allow(unused)]
	invisibleFlag_SexVer27: u8,
#[allow(unused)]
	invisibleFlag_SexVer28: u8,
#[allow(unused)]
	invisibleFlag_SexVer29: u8,
#[allow(unused)]
	invisibleFlag_SexVer30: u8,
#[allow(unused)]
	invisibleFlag_SexVer31: u8,
#[allow(unused)]
	invisibleFlag_SexVer32: u8,
#[allow(unused)]
	invisibleFlag_SexVer33: u8,
#[allow(unused)]
	invisibleFlag_SexVer34: u8,
#[allow(unused)]
	invisibleFlag_SexVer35: u8,
#[allow(unused)]
	invisibleFlag_SexVer36: u8,
#[allow(unused)]
	invisibleFlag_SexVer37: u8,
#[allow(unused)]
	invisibleFlag_SexVer38: u8,
#[allow(unused)]
	invisibleFlag_SexVer39: u8,
#[allow(unused)]
	invisibleFlag_SexVer40: u8,
#[allow(unused)]
	invisibleFlag_SexVer41: u8,
#[allow(unused)]
	invisibleFlag_SexVer42: u8,
#[allow(unused)]
	invisibleFlag_SexVer43: u8,
#[allow(unused)]
	invisibleFlag_SexVer44: u8,
#[allow(unused)]
	invisibleFlag_SexVer45: u8,
#[allow(unused)]
	invisibleFlag_SexVer46: u8,
#[allow(unused)]
	invisibleFlag_SexVer47: u8,
#[allow(unused)]
	invisibleFlag_SexVer48: u8,
#[allow(unused)]
	invisibleFlag_SexVer49: u8,
#[allow(unused)]
	invisibleFlag_SexVer50: u8,
#[allow(unused)]
	invisibleFlag_SexVer51: u8,
#[allow(unused)]
	invisibleFlag_SexVer52: u8,
#[allow(unused)]
	invisibleFlag_SexVer53: u8,
#[allow(unused)]
	invisibleFlag_SexVer54: u8,
#[allow(unused)]
	invisibleFlag_SexVer55: u8,
#[allow(unused)]
	invisibleFlag_SexVer56: u8,
#[allow(unused)]
	invisibleFlag_SexVer57: u8,
#[allow(unused)]
	invisibleFlag_SexVer58: u8,
#[allow(unused)]
	invisibleFlag_SexVer59: u8,
#[allow(unused)]
	invisibleFlag_SexVer60: u8,
#[allow(unused)]
	invisibleFlag_SexVer61: u8,
#[allow(unused)]
	invisibleFlag_SexVer62: u8,
#[allow(unused)]
	invisibleFlag_SexVer63: u8,
#[allow(unused)]
	invisibleFlag_SexVer64: u8,
#[allow(unused)]
	invisibleFlag_SexVer65: u8,
#[allow(unused)]
	invisibleFlag_SexVer66: u8,
#[allow(unused)]
	invisibleFlag_SexVer67: u8,
#[allow(unused)]
	invisibleFlag_SexVer68: u8,
#[allow(unused)]
	invisibleFlag_SexVer69: u8,
#[allow(unused)]
	invisibleFlag_SexVer70: u8,
#[allow(unused)]
	invisibleFlag_SexVer71: u8,
#[allow(unused)]
	invisibleFlag_SexVer72: u8,
#[allow(unused)]
	invisibleFlag_SexVer73: u8,
#[allow(unused)]
	invisibleFlag_SexVer74: u8,
#[allow(unused)]
	invisibleFlag_SexVer75: u8,
#[allow(unused)]
	invisibleFlag_SexVer76: u8,
#[allow(unused)]
	invisibleFlag_SexVer77: u8,
#[allow(unused)]
	invisibleFlag_SexVer78: u8,
#[allow(unused)]
	invisibleFlag_SexVer79: u8,
#[allow(unused)]
	invisibleFlag_SexVer80: u8,
#[allow(unused)]
	invisibleFlag_SexVer81: u8,
#[allow(unused)]
	invisibleFlag_SexVer82: u8,
#[allow(unused)]
	invisibleFlag_SexVer83: u8,
#[allow(unused)]
	invisibleFlag_SexVer84: u8,
#[allow(unused)]
	invisibleFlag_SexVer85: u8,
#[allow(unused)]
	invisibleFlag_SexVer86: u8,
#[allow(unused)]
	invisibleFlag_SexVer87: u8,
#[allow(unused)]
	invisibleFlag_SexVer88: u8,
#[allow(unused)]
	invisibleFlag_SexVer89: u8,
#[allow(unused)]
	invisibleFlag_SexVer90: u8,
#[allow(unused)]
	invisibleFlag_SexVer91: u8,
#[allow(unused)]
	invisibleFlag_SexVer92: u8,
#[allow(unused)]
	invisibleFlag_SexVer93: u8,
#[allow(unused)]
	invisibleFlag_SexVer94: u8,
#[allow(unused)]
	invisibleFlag_SexVer95: u8,
#[allow(unused)]
	pad404: [u8;14],
}

struct EQUIP_PARAM_WEAPON_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	behaviorVariationId: i32,
#[allow(unused)]
	sortId: i32,
#[allow(unused)]
	wanderingEquipId: i32,
#[allow(unused)]
	weight: f32,
#[allow(unused)]
	weaponWeightRate: f32,
#[allow(unused)]
	fixPrice: i32,
#[allow(unused)]
	reinforcePrice: i32,
#[allow(unused)]
	sellValue: i32,
#[allow(unused)]
	correctStrength: f32,
#[allow(unused)]
	correctAgility: f32,
#[allow(unused)]
	correctMagic: f32,
#[allow(unused)]
	correctFaith: f32,
#[allow(unused)]
	physGuardCutRate: f32,
#[allow(unused)]
	magGuardCutRate: f32,
#[allow(unused)]
	fireGuardCutRate: f32,
#[allow(unused)]
	thunGuardCutRate: f32,
#[allow(unused)]
	spEffectBehaviorId0: i32,
#[allow(unused)]
	spEffectBehaviorId1: i32,
#[allow(unused)]
	spEffectBehaviorId2: i32,
#[allow(unused)]
	residentSpEffectId: i32,
#[allow(unused)]
	residentSpEffectId1: i32,
#[allow(unused)]
	residentSpEffectId2: i32,
#[allow(unused)]
	materialSetId: i32,
#[allow(unused)]
	originEquipWep: i32,
#[allow(unused)]
	originEquipWep1: i32,
#[allow(unused)]
	originEquipWep2: i32,
#[allow(unused)]
	originEquipWep3: i32,
#[allow(unused)]
	originEquipWep4: i32,
#[allow(unused)]
	originEquipWep5: i32,
#[allow(unused)]
	originEquipWep6: i32,
#[allow(unused)]
	originEquipWep7: i32,
#[allow(unused)]
	originEquipWep8: i32,
#[allow(unused)]
	originEquipWep9: i32,
#[allow(unused)]
	originEquipWep10: i32,
#[allow(unused)]
	originEquipWep11: i32,
#[allow(unused)]
	originEquipWep12: i32,
#[allow(unused)]
	originEquipWep13: i32,
#[allow(unused)]
	originEquipWep14: i32,
#[allow(unused)]
	originEquipWep15: i32,
#[allow(unused)]
	weakA_DamageRate: f32,
#[allow(unused)]
	weakB_DamageRate: f32,
#[allow(unused)]
	weakC_DamageRate: f32,
#[allow(unused)]
	weakD_DamageRate: f32,
#[allow(unused)]
	sleepGuardResist_MaxCorrect: f32,
#[allow(unused)]
	madnessGuardResist_MaxCorrect: f32,
#[allow(unused)]
	saWeaponDamage: f32,
#[allow(unused)]
	equipModelId: i16,
#[allow(unused)]
	iconId: i16,
#[allow(unused)]
	durability: i16,
#[allow(unused)]
	durabilityMax: i16,
#[allow(unused)]
	attackThrowEscape: i16,
#[allow(unused)]
	parryDamageLife: i16,
#[allow(unused)]
	attackBasePhysics: i16,
#[allow(unused)]
	attackBaseMagic: i16,
#[allow(unused)]
	attackBaseFire: i16,
#[allow(unused)]
	attackBaseThunder: i16,
#[allow(unused)]
	attackBaseStamina: i16,
#[allow(unused)]
	guardAngle: i16,
#[allow(unused)]
	saDurability: f32,
#[allow(unused)]
	staminaGuardDef: i16,
#[allow(unused)]
	reinforceTypeId: i16,
#[allow(unused)]
	trophySGradeId: i16,
#[allow(unused)]
	trophySeqId: i16,
#[allow(unused)]
	throwAtkRate: i16,
#[allow(unused)]
	bowDistRate: i16,
#[allow(unused)]
	equipModelCategory: u8,
#[allow(unused)]
	equipModelGender: u8,
#[allow(unused)]
	weaponCategory: u8,
#[allow(unused)]
	wepmotionCategory: u8,
#[allow(unused)]
	guardmotionCategory: u8,
#[allow(unused)]
	atkMaterial: u8,
#[allow(unused)]
	defSeMaterial1: i16,
#[allow(unused)]
	correctType_Physics: u8,
#[allow(unused)]
	spAttribute: u8,
#[allow(unused)]
	spAtkcategory: i16,
#[allow(unused)]
	wepmotionOneHandId: u8,
#[allow(unused)]
	wepmotionBothHandId: u8,
#[allow(unused)]
	properStrength: u8,
#[allow(unused)]
	properAgility: u8,
#[allow(unused)]
	properMagic: u8,
#[allow(unused)]
	properFaith: u8,
#[allow(unused)]
	overStrength: u8,
#[allow(unused)]
	attackBaseParry: u8,
#[allow(unused)]
	defenseBaseParry: u8,
#[allow(unused)]
	guardBaseRepel: u8,
#[allow(unused)]
	attackBaseRepel: u8,
#[allow(unused)]
	guardCutCancelRate: i8,
#[allow(unused)]
	guardLevel: i8,
#[allow(unused)]
	slashGuardCutRate: i8,
#[allow(unused)]
	blowGuardCutRate: i8,
#[allow(unused)]
	thrustGuardCutRate: i8,
#[allow(unused)]
	poisonGuardResist: i8,
#[allow(unused)]
	diseaseGuardResist: i8,
#[allow(unused)]
	bloodGuardResist: i8,
#[allow(unused)]
	curseGuardResist: i8,
#[allow(unused)]
	atkAttribute: u8,
#[allow(unused)]
	rightHandEquipable: u8,
#[allow(unused)]
	leftHandEquipable: u8,
#[allow(unused)]
	bothHandEquipable: u8,
#[allow(unused)]
	arrowSlotEquipable: u8,
#[allow(unused)]
	boltSlotEquipable: u8,
#[allow(unused)]
	enableGuard: u8,
#[allow(unused)]
	enableParry: u8,
#[allow(unused)]
	enableMagic: u8,
#[allow(unused)]
	enableSorcery: u8,
#[allow(unused)]
	enableMiracle: u8,
#[allow(unused)]
	enableVowMagic: u8,
#[allow(unused)]
	isNormalAttackType: u8,
#[allow(unused)]
	isBlowAttackType: u8,
#[allow(unused)]
	isSlashAttackType: u8,
#[allow(unused)]
	isThrustAttackType: u8,
#[allow(unused)]
	isEnhance: u8,
#[allow(unused)]
	isHeroPointCorrect: u8,
#[allow(unused)]
	isCustom: u8,
#[allow(unused)]
	disableBaseChangeReset: u8,
#[allow(unused)]
	disableRepair: u8,
#[allow(unused)]
	isDarkHand: u8,
#[allow(unused)]
	simpleModelForDlc: u8,
#[allow(unused)]
	lanternWep: u8,
#[allow(unused)]
	isVersusGhostWep: u8,
#[allow(unused)]
	baseChangeCategory: u8,
#[allow(unused)]
	isDragonSlayer: u8,
#[allow(unused)]
	isDeposit: u8,
#[allow(unused)]
	disableMultiDropShare: u8,
#[allow(unused)]
	isDiscard: u8,
#[allow(unused)]
	isDrop: u8,
#[allow(unused)]
	showLogCondType: u8,
#[allow(unused)]
	enableThrow: u8,
#[allow(unused)]
	showDialogCondType: u8,
#[allow(unused)]
	disableGemAttr: u8,
#[allow(unused)]
	defSfxMaterial1: i16,
#[allow(unused)]
	wepCollidableType0: u8,
#[allow(unused)]
	wepCollidableType1: u8,
#[allow(unused)]
	postureControlId_Right: u8,
#[allow(unused)]
	postureControlId_Left: u8,
#[allow(unused)]
	traceSfxId0: i32,
#[allow(unused)]
	traceDmyIdHead0: i32,
#[allow(unused)]
	traceDmyIdTail0: i32,
#[allow(unused)]
	traceSfxId1: i32,
#[allow(unused)]
	traceDmyIdHead1: i32,
#[allow(unused)]
	traceDmyIdTail1: i32,
#[allow(unused)]
	traceSfxId2: i32,
#[allow(unused)]
	traceDmyIdHead2: i32,
#[allow(unused)]
	traceDmyIdTail2: i32,
#[allow(unused)]
	traceSfxId3: i32,
#[allow(unused)]
	traceDmyIdHead3: i32,
#[allow(unused)]
	traceDmyIdTail3: i32,
#[allow(unused)]
	traceSfxId4: i32,
#[allow(unused)]
	traceDmyIdHead4: i32,
#[allow(unused)]
	traceDmyIdTail4: i32,
#[allow(unused)]
	traceSfxId5: i32,
#[allow(unused)]
	traceDmyIdHead5: i32,
#[allow(unused)]
	traceDmyIdTail5: i32,
#[allow(unused)]
	traceSfxId6: i32,
#[allow(unused)]
	traceDmyIdHead6: i32,
#[allow(unused)]
	traceDmyIdTail6: i32,
#[allow(unused)]
	traceSfxId7: i32,
#[allow(unused)]
	traceDmyIdHead7: i32,
#[allow(unused)]
	traceDmyIdTail7: i32,
#[allow(unused)]
	defSfxMaterial2: i16,
#[allow(unused)]
	defSeMaterial2: i16,
#[allow(unused)]
	absorpParamId: i32,
#[allow(unused)]
	toughnessCorrectRate: f32,
#[allow(unused)]
	isValidTough_ProtSADmg: u8,
#[allow(unused)]
	isDualBlade: u8,
#[allow(unused)]
	isAutoEquip: u8,
#[allow(unused)]
	isEnableEmergencyStep: u8,
#[allow(unused)]
	invisibleOnRemo: u8,
#[allow(unused)]
	unk1: [u8;1],
#[allow(unused)]
	correctType_Magic: u8,
#[allow(unused)]
	correctType_Fire: u8,
#[allow(unused)]
	correctType_Thunder: u8,
#[allow(unused)]
	weakE_DamageRate: f32,
#[allow(unused)]
	weakF_DamageRate: f32,
#[allow(unused)]
	darkGuardCutRate: f32,
#[allow(unused)]
	attackBaseDark: i16,
#[allow(unused)]
	correctType_Dark: u8,
#[allow(unused)]
	correctType_Poison: u8,
#[allow(unused)]
	sortGroupId: u8,
#[allow(unused)]
	atkAttribute2: u8,
#[allow(unused)]
	sleepGuardResist: i8,
#[allow(unused)]
	madnessGuardResist: i8,
#[allow(unused)]
	correctType_Blood: u8,
#[allow(unused)]
	properLuck: u8,
#[allow(unused)]
	freezeGuardResist: i8,
#[allow(unused)]
	autoReplenishType: u8,
#[allow(unused)]
	swordArtsParamId: i32,
#[allow(unused)]
	correctLuck: f32,
#[allow(unused)]
	arrowBoltEquipId: i32,
#[allow(unused)]
	DerivationLevelType: u8,
#[allow(unused)]
	enchantSfxSize: u8,
#[allow(unused)]
	wepType: i16,
#[allow(unused)]
	physGuardCutRate_MaxCorrect: f32,
#[allow(unused)]
	magGuardCutRate_MaxCorrect: f32,
#[allow(unused)]
	fireGuardCutRate_MaxCorrect: f32,
#[allow(unused)]
	thunGuardCutRate_MaxCorrect: f32,
#[allow(unused)]
	darkGuardCutRate_MaxCorrect: f32,
#[allow(unused)]
	poisonGuardResist_MaxCorrect: f32,
#[allow(unused)]
	diseaseGuardResist_MaxCorrect: f32,
#[allow(unused)]
	bloodGuardResist_MaxCorrect: f32,
#[allow(unused)]
	curseGuardResist_MaxCorrect: f32,
#[allow(unused)]
	freezeGuardResist_MaxCorrect: f32,
#[allow(unused)]
	staminaGuardDef_MaxCorrect: f32,
#[allow(unused)]
	residentSfxId_1: i32,
#[allow(unused)]
	residentSfxId_2: i32,
#[allow(unused)]
	residentSfxId_3: i32,
#[allow(unused)]
	residentSfxId_4: i32,
#[allow(unused)]
	residentSfx_DmyId_1: i32,
#[allow(unused)]
	residentSfx_DmyId_2: i32,
#[allow(unused)]
	residentSfx_DmyId_3: i32,
#[allow(unused)]
	residentSfx_DmyId_4: i32,
#[allow(unused)]
	staminaConsumptionRate: f32,
#[allow(unused)]
	vsPlayerDmgCorrectRate_Physics: f32,
#[allow(unused)]
	vsPlayerDmgCorrectRate_Magic: f32,
#[allow(unused)]
	vsPlayerDmgCorrectRate_Fire: f32,
#[allow(unused)]
	vsPlayerDmgCorrectRate_Thunder: f32,
#[allow(unused)]
	vsPlayerDmgCorrectRate_Dark: f32,
#[allow(unused)]
	vsPlayerDmgCorrectRate_Poison: f32,
#[allow(unused)]
	vsPlayerDmgCorrectRate_Blood: f32,
#[allow(unused)]
	vsPlayerDmgCorrectRate_Freeze: f32,
#[allow(unused)]
	attainmentWepStatusStr: i32,
#[allow(unused)]
	attainmentWepStatusDex: i32,
#[allow(unused)]
	attainmentWepStatusMag: i32,
#[allow(unused)]
	attainmentWepStatusFai: i32,
#[allow(unused)]
	attainmentWepStatusLuc: i32,
#[allow(unused)]
	attackElementCorrectId: i32,
#[allow(unused)]
	saleValue: i32,
#[allow(unused)]
	reinforceShopCategory: u8,
#[allow(unused)]
	maxArrowQuantity: u8,
#[allow(unused)]
	residentSfx_1_IsVisibleForHang: u8,
#[allow(unused)]
	residentSfx_2_IsVisibleForHang: u8,
#[allow(unused)]
	residentSfx_3_IsVisibleForHang: u8,
#[allow(unused)]
	residentSfx_4_IsVisibleForHang: u8,
#[allow(unused)]
	isSoulParamIdChange_model0: u8,
#[allow(unused)]
	isSoulParamIdChange_model1: u8,
#[allow(unused)]
	isSoulParamIdChange_model2: u8,
#[allow(unused)]
	isSoulParamIdChange_model3: u8,
#[allow(unused)]
	wepSeIdOffset: i8,
#[allow(unused)]
	baseChangePrice: i32,
#[allow(unused)]
	levelSyncCorrectId: i16,
#[allow(unused)]
	correctType_Sleep: u8,
#[allow(unused)]
	correctType_Madness: u8,
#[allow(unused)]
	rarity: u8,
#[allow(unused)]
	gemMountType: u8,
#[allow(unused)]
	wepRegainHp: i16,
#[allow(unused)]
	spEffectMsgId0: i32,
#[allow(unused)]
	spEffectMsgId1: i32,
#[allow(unused)]
	spEffectMsgId2: i32,
#[allow(unused)]
	originEquipWep16: i32,
#[allow(unused)]
	originEquipWep17: i32,
#[allow(unused)]
	originEquipWep18: i32,
#[allow(unused)]
	originEquipWep19: i32,
#[allow(unused)]
	originEquipWep20: i32,
#[allow(unused)]
	originEquipWep21: i32,
#[allow(unused)]
	originEquipWep22: i32,
#[allow(unused)]
	originEquipWep23: i32,
#[allow(unused)]
	originEquipWep24: i32,
#[allow(unused)]
	originEquipWep25: i32,
#[allow(unused)]
	vsPlayerDmgCorrectRate_Sleep: f32,
#[allow(unused)]
	vsPlayerDmgCorrectRate_Madness: f32,
#[allow(unused)]
	saGuardCutRate: f32,
#[allow(unused)]
	defMaterialVariationValue: u8,
#[allow(unused)]
	spAttributeVariationValue: u8,
#[allow(unused)]
	stealthAtkRate: i16,
#[allow(unused)]
	vsPlayerDmgCorrectRate_Disease: f32,
#[allow(unused)]
	vsPlayerDmgCorrectRate_Curse: f32,
#[allow(unused)]
	pad: [u8;8],
}

struct ESTUS_FLASK_RECOVERY_PARAM_ST {
#[allow(unused)]
	host: u8,
#[allow(unused)]
	invadeOrb_None: u8,
#[allow(unused)]
	invadeOrb_Umbasa: u8,
#[allow(unused)]
	invadeOrb_Berserker: u8,
#[allow(unused)]
	invadeOrb_Sinners: u8,
#[allow(unused)]
	invadeSign_None: u8,
#[allow(unused)]
	invadeSign_Umbasa: u8,
#[allow(unused)]
	invadeSign_Berserker: u8,
#[allow(unused)]
	invadeSign_Sinners: u8,
#[allow(unused)]
	invadeRing_Sinners: u8,
#[allow(unused)]
	invadeRing_Rosalia: u8,
#[allow(unused)]
	invadeRing_Forest: u8,
#[allow(unused)]
	coopSign_None: u8,
#[allow(unused)]
	coopSign_Umbasa: u8,
#[allow(unused)]
	coopSign_Berserker: u8,
#[allow(unused)]
	coopSign_Sinners: u8,
#[allow(unused)]
	coopRing_RedHunter: u8,
#[allow(unused)]
	invadeRing_Anor: u8,
#[allow(unused)]
	paramReplaceRate: i16,
#[allow(unused)]
	paramReplaceId: i32,
#[allow(unused)]
	pad: [u8;8],
}

struct EVENT_FLAG_USAGE_PARAM_ST {
#[allow(unused)]
	usageType: u8,
#[allow(unused)]
	playlogCategory: u8,
#[allow(unused)]
	padding1: [u8;2],
#[allow(unused)]
	flagNum: i32,
#[allow(unused)]
	padding2: [u8;24],
}

struct FACE_PARAM_ST {
#[allow(unused)]
	face_partsId: u8,
#[allow(unused)]
	skin_color_R: u8,
#[allow(unused)]
	skin_color_G: u8,
#[allow(unused)]
	skin_color_B: u8,
#[allow(unused)]
	skin_gloss: u8,
#[allow(unused)]
	skin_pores: u8,
#[allow(unused)]
	face_beard: u8,
#[allow(unused)]
	face_aroundEye: u8,
#[allow(unused)]
	face_aroundEyeColor_R: u8,
#[allow(unused)]
	face_aroundEyeColor_G: u8,
#[allow(unused)]
	face_aroundEyeColor_B: u8,
#[allow(unused)]
	face_cheek: u8,
#[allow(unused)]
	face_cheekColor_R: u8,
#[allow(unused)]
	face_cheekColor_G: u8,
#[allow(unused)]
	face_cheekColor_B: u8,
#[allow(unused)]
	face_eyeLine: u8,
#[allow(unused)]
	face_eyeLineColor_R: u8,
#[allow(unused)]
	face_eyeLineColor_G: u8,
#[allow(unused)]
	face_eyeLineColor_B: u8,
#[allow(unused)]
	face_eyeShadowDown: u8,
#[allow(unused)]
	face_eyeShadowDownColor_R: u8,
#[allow(unused)]
	face_eyeShadowDownColor_G: u8,
#[allow(unused)]
	face_eyeShadowDownColor_B: u8,
#[allow(unused)]
	face_eyeShadowUp: u8,
#[allow(unused)]
	face_eyeShadowUpColor_R: u8,
#[allow(unused)]
	face_eyeShadowUpColor_G: u8,
#[allow(unused)]
	face_eyeShadowUpColor_B: u8,
#[allow(unused)]
	face_lip: u8,
#[allow(unused)]
	face_lipColor_R: u8,
#[allow(unused)]
	face_lipColor_G: u8,
#[allow(unused)]
	face_lipColor_B: u8,
#[allow(unused)]
	body_hair: u8,
#[allow(unused)]
	body_hairColor_R: u8,
#[allow(unused)]
	body_hairColor_G: u8,
#[allow(unused)]
	body_hairColor_B: u8,
#[allow(unused)]
	eye_partsId: u8,
#[allow(unused)]
	eyeR_irisColor_R: u8,
#[allow(unused)]
	eyeR_irisColor_G: u8,
#[allow(unused)]
	eyeR_irisColor_B: u8,
#[allow(unused)]
	eyeR_irisScale: u8,
#[allow(unused)]
	eyeR_cataract: u8,
#[allow(unused)]
	eyeR_cataractColor_R: u8,
#[allow(unused)]
	eyeR_cataractColor_G: u8,
#[allow(unused)]
	eyeR_cataractColor_B: u8,
#[allow(unused)]
	eyeR_scleraColor_R: u8,
#[allow(unused)]
	eyeR_scleraColor_G: u8,
#[allow(unused)]
	eyeR_scleraColor_B: u8,
#[allow(unused)]
	eyeR_irisDistance: u8,
#[allow(unused)]
	eyeL_irisColor_R: u8,
#[allow(unused)]
	eyeL_irisColor_G: u8,
#[allow(unused)]
	eyeL_irisColor_B: u8,
#[allow(unused)]
	eyeL_irisScale: u8,
#[allow(unused)]
	eyeL_cataract: u8,
#[allow(unused)]
	eyeL_cataractColor_R: u8,
#[allow(unused)]
	eyeL_cataractColor_G: u8,
#[allow(unused)]
	eyeL_cataractColor_B: u8,
#[allow(unused)]
	eyeL_scleraColor_R: u8,
#[allow(unused)]
	eyeL_scleraColor_G: u8,
#[allow(unused)]
	eyeL_scleraColor_B: u8,
#[allow(unused)]
	eyeL_irisDistance: u8,
#[allow(unused)]
	hair_partsId: u8,
#[allow(unused)]
	hair_color_R: u8,
#[allow(unused)]
	hair_color_G: u8,
#[allow(unused)]
	hair_color_B: u8,
#[allow(unused)]
	hair_shininess: u8,
#[allow(unused)]
	hair_rootBlack: u8,
#[allow(unused)]
	hair_whiteDensity: u8,
#[allow(unused)]
	beard_partsId: u8,
#[allow(unused)]
	beard_color_R: u8,
#[allow(unused)]
	beard_color_G: u8,
#[allow(unused)]
	beard_color_B: u8,
#[allow(unused)]
	beard_shininess: u8,
#[allow(unused)]
	beard_rootBlack: u8,
#[allow(unused)]
	beard_whiteDensity: u8,
#[allow(unused)]
	eyebrow_partsId: u8,
#[allow(unused)]
	eyebrow_color_R: u8,
#[allow(unused)]
	eyebrow_color_G: u8,
#[allow(unused)]
	eyebrow_color_B: u8,
#[allow(unused)]
	eyebrow_shininess: u8,
#[allow(unused)]
	eyebrow_rootBlack: u8,
#[allow(unused)]
	eyebrow_whiteDensity: u8,
#[allow(unused)]
	eyelash_partsId: u8,
#[allow(unused)]
	eyelash_color_R: u8,
#[allow(unused)]
	eyelash_color_G: u8,
#[allow(unused)]
	eyelash_color_B: u8,
#[allow(unused)]
	accessories_partsId: u8,
#[allow(unused)]
	accessories_color_R: u8,
#[allow(unused)]
	accessories_color_G: u8,
#[allow(unused)]
	accessories_color_B: u8,
#[allow(unused)]
	decal_partsId: u8,
#[allow(unused)]
	decal_posX: u8,
#[allow(unused)]
	decal_posY: u8,
#[allow(unused)]
	decal_angle: u8,
#[allow(unused)]
	decal_scale: u8,
#[allow(unused)]
	decal_color_R: u8,
#[allow(unused)]
	decal_color_G: u8,
#[allow(unused)]
	decal_color_B: u8,
#[allow(unused)]
	decal_gloss: u8,
#[allow(unused)]
	decal_mirror: u8,
#[allow(unused)]
	chrBodyScaleHead: u8,
#[allow(unused)]
	chrBodyScaleBreast: u8,
#[allow(unused)]
	chrBodyScaleAbdomen: u8,
#[allow(unused)]
	chrBodyScaleRArm: u8,
#[allow(unused)]
	chrBodyScaleRLeg: u8,
#[allow(unused)]
	chrBodyScaleLArm: u8,
#[allow(unused)]
	chrBodyScaleLLeg: u8,
#[allow(unused)]
	burn_scar: u8,
#[allow(unused)]
	override_eye_partsId: u8,
#[allow(unused)]
	override_eye_irisColor: u8,
#[allow(unused)]
	override_eye_cataract: u8,
#[allow(unused)]
	override_eye_cataractColor: u8,
#[allow(unused)]
	override_eye_scleraColor: u8,
#[allow(unused)]
	override_burn_scar: u8,
#[allow(unused)]
	pad2: [u8;1],
#[allow(unused)]
	pad: [u8;5],
#[allow(unused)]
	age: u8,
#[allow(unused)]
	gender: u8,
#[allow(unused)]
	caricatureGeometry: u8,
#[allow(unused)]
	caricatureTexture: u8,
#[allow(unused)]
	faceGeoData00: u8,
#[allow(unused)]
	faceGeoData01: u8,
#[allow(unused)]
	faceGeoData02: u8,
#[allow(unused)]
	faceGeoData03: u8,
#[allow(unused)]
	faceGeoData04: u8,
#[allow(unused)]
	faceGeoData05: u8,
#[allow(unused)]
	faceGeoData06: u8,
#[allow(unused)]
	faceGeoData07: u8,
#[allow(unused)]
	faceGeoData08: u8,
#[allow(unused)]
	faceGeoData09: u8,
#[allow(unused)]
	faceGeoData10: u8,
#[allow(unused)]
	faceGeoData11: u8,
#[allow(unused)]
	faceGeoData12: u8,
#[allow(unused)]
	faceGeoData13: u8,
#[allow(unused)]
	faceGeoData14: u8,
#[allow(unused)]
	faceGeoData15: u8,
#[allow(unused)]
	faceGeoData16: u8,
#[allow(unused)]
	faceGeoData17: u8,
#[allow(unused)]
	faceGeoData18: u8,
#[allow(unused)]
	faceGeoData19: u8,
#[allow(unused)]
	faceGeoData20: u8,
#[allow(unused)]
	faceGeoData21: u8,
#[allow(unused)]
	faceGeoData22: u8,
#[allow(unused)]
	faceGeoData23: u8,
#[allow(unused)]
	faceGeoData24: u8,
#[allow(unused)]
	faceGeoData25: u8,
#[allow(unused)]
	faceGeoData26: u8,
#[allow(unused)]
	faceGeoData27: u8,
#[allow(unused)]
	faceGeoData28: u8,
#[allow(unused)]
	faceGeoData29: u8,
#[allow(unused)]
	faceGeoData30: u8,
#[allow(unused)]
	faceGeoData31: u8,
#[allow(unused)]
	faceGeoData32: u8,
#[allow(unused)]
	faceGeoData33: u8,
#[allow(unused)]
	faceGeoData34: u8,
#[allow(unused)]
	faceGeoData35: u8,
#[allow(unused)]
	faceGeoData36: u8,
#[allow(unused)]
	faceGeoData37: u8,
#[allow(unused)]
	faceGeoData38: u8,
#[allow(unused)]
	faceGeoData39: u8,
#[allow(unused)]
	faceGeoData40: u8,
#[allow(unused)]
	faceGeoData41: u8,
#[allow(unused)]
	faceGeoData42: u8,
#[allow(unused)]
	faceGeoData43: u8,
#[allow(unused)]
	faceGeoData44: u8,
#[allow(unused)]
	faceGeoData45: u8,
#[allow(unused)]
	faceGeoData46: u8,
#[allow(unused)]
	faceGeoData47: u8,
#[allow(unused)]
	faceGeoData48: u8,
#[allow(unused)]
	faceGeoData49: u8,
#[allow(unused)]
	faceGeoData50: u8,
#[allow(unused)]
	faceGeoData51: u8,
#[allow(unused)]
	faceGeoData52: u8,
#[allow(unused)]
	faceGeoData53: u8,
#[allow(unused)]
	faceGeoData54: u8,
#[allow(unused)]
	faceGeoData55: u8,
#[allow(unused)]
	faceGeoData56: u8,
#[allow(unused)]
	faceGeoData57: u8,
#[allow(unused)]
	faceGeoData58: u8,
#[allow(unused)]
	faceGeoData59: u8,
#[allow(unused)]
	faceGeoData60: u8,
#[allow(unused)]
	faceTexData00: u8,
#[allow(unused)]
	faceTexData01: u8,
#[allow(unused)]
	faceTexData02: u8,
#[allow(unused)]
	faceTexData03: u8,
#[allow(unused)]
	faceTexData04: u8,
#[allow(unused)]
	faceTexData05: u8,
#[allow(unused)]
	faceTexData06: u8,
#[allow(unused)]
	faceTexData07: u8,
#[allow(unused)]
	faceTexData08: u8,
#[allow(unused)]
	faceTexData09: u8,
#[allow(unused)]
	faceTexData10: u8,
#[allow(unused)]
	faceTexData11: u8,
#[allow(unused)]
	faceTexData12: u8,
#[allow(unused)]
	faceTexData13: u8,
#[allow(unused)]
	faceTexData14: u8,
#[allow(unused)]
	faceTexData15: u8,
#[allow(unused)]
	faceTexData16: u8,
#[allow(unused)]
	faceTexData17: u8,
#[allow(unused)]
	faceTexData18: u8,
#[allow(unused)]
	faceTexData19: u8,
#[allow(unused)]
	faceTexData20: u8,
#[allow(unused)]
	faceTexData21: u8,
#[allow(unused)]
	faceTexData22: u8,
#[allow(unused)]
	faceTexData23: u8,
#[allow(unused)]
	faceTexData24: u8,
#[allow(unused)]
	faceTexData25: u8,
#[allow(unused)]
	faceTexData26: u8,
#[allow(unused)]
	faceTexData27: u8,
#[allow(unused)]
	faceTexData28: u8,
#[allow(unused)]
	faceTexData29: u8,
#[allow(unused)]
	faceTexData30: u8,
#[allow(unused)]
	faceTexData31: u8,
#[allow(unused)]
	faceTexData32: u8,
#[allow(unused)]
	faceTexData33: u8,
#[allow(unused)]
	faceTexData34: u8,
#[allow(unused)]
	faceTexData35: u8,
#[allow(unused)]
	faceGeoAsymData00: u8,
#[allow(unused)]
	faceGeoAsymData01: u8,
#[allow(unused)]
	faceGeoAsymData02: u8,
#[allow(unused)]
	faceGeoAsymData03: u8,
#[allow(unused)]
	faceGeoAsymData04: u8,
#[allow(unused)]
	faceGeoAsymData05: u8,
#[allow(unused)]
	faceGeoAsymData06: u8,
#[allow(unused)]
	faceGeoAsymData07: u8,
#[allow(unused)]
	faceGeoAsymData08: u8,
#[allow(unused)]
	faceGeoAsymData09: u8,
#[allow(unused)]
	faceGeoAsymData10: u8,
#[allow(unused)]
	faceGeoAsymData11: u8,
#[allow(unused)]
	faceGeoAsymData12: u8,
#[allow(unused)]
	faceGeoAsymData13: u8,
#[allow(unused)]
	faceGeoAsymData14: u8,
#[allow(unused)]
	faceGeoAsymData15: u8,
#[allow(unused)]
	faceGeoAsymData16: u8,
#[allow(unused)]
	faceGeoAsymData17: u8,
#[allow(unused)]
	faceGeoAsymData18: u8,
#[allow(unused)]
	faceGeoAsymData19: u8,
#[allow(unused)]
	faceGeoAsymData20: u8,
#[allow(unused)]
	faceGeoAsymData21: u8,
#[allow(unused)]
	faceGeoAsymData22: u8,
#[allow(unused)]
	faceGeoAsymData23: u8,
#[allow(unused)]
	faceGeoAsymData24: u8,
#[allow(unused)]
	faceGeoAsymData25: u8,
}

struct FACE_RANGE_PARAM_ST {
#[allow(unused)]
	face_partsId: f32,
#[allow(unused)]
	skin_color_R: f32,
#[allow(unused)]
	skin_color_G: f32,
#[allow(unused)]
	skin_color_B: f32,
#[allow(unused)]
	skin_gloss: f32,
#[allow(unused)]
	skin_pores: f32,
#[allow(unused)]
	face_beard: f32,
#[allow(unused)]
	face_aroundEye: f32,
#[allow(unused)]
	face_aroundEyeColor_R: f32,
#[allow(unused)]
	face_aroundEyeColor_G: f32,
#[allow(unused)]
	face_aroundEyeColor_B: f32,
#[allow(unused)]
	face_cheek: f32,
#[allow(unused)]
	face_cheekColor_R: f32,
#[allow(unused)]
	face_cheekColor_G: f32,
#[allow(unused)]
	face_cheekColor_B: f32,
#[allow(unused)]
	face_eyeLine: f32,
#[allow(unused)]
	face_eyeLineColor_R: f32,
#[allow(unused)]
	face_eyeLineColor_G: f32,
#[allow(unused)]
	face_eyeLineColor_B: f32,
#[allow(unused)]
	face_eyeShadowDown: f32,
#[allow(unused)]
	face_eyeShadowDownColor_R: f32,
#[allow(unused)]
	face_eyeShadowDownColor_G: f32,
#[allow(unused)]
	face_eyeShadowDownColor_B: f32,
#[allow(unused)]
	face_eyeShadowUp: f32,
#[allow(unused)]
	face_eyeShadowUpColor_R: f32,
#[allow(unused)]
	face_eyeShadowUpColor_G: f32,
#[allow(unused)]
	face_eyeShadowUpColor_B: f32,
#[allow(unused)]
	face_lip: f32,
#[allow(unused)]
	face_lipColor_R: f32,
#[allow(unused)]
	face_lipColor_G: f32,
#[allow(unused)]
	face_lipColor_B: f32,
#[allow(unused)]
	body_hair: f32,
#[allow(unused)]
	body_hairColor_R: f32,
#[allow(unused)]
	body_hairColor_G: f32,
#[allow(unused)]
	body_hairColor_B: f32,
#[allow(unused)]
	eye_partsId: f32,
#[allow(unused)]
	eyeR_irisColor_R: f32,
#[allow(unused)]
	eyeR_irisColor_G: f32,
#[allow(unused)]
	eyeR_irisColor_B: f32,
#[allow(unused)]
	eyeR_irisScale: f32,
#[allow(unused)]
	eyeR_cataract: f32,
#[allow(unused)]
	eyeR_cataractColor_R: f32,
#[allow(unused)]
	eyeR_cataractColor_G: f32,
#[allow(unused)]
	eyeR_cataractColor_B: f32,
#[allow(unused)]
	eyeR_scleraColor_R: f32,
#[allow(unused)]
	eyeR_scleraColor_G: f32,
#[allow(unused)]
	eyeR_scleraColor_B: f32,
#[allow(unused)]
	eyeR_irisDistance: f32,
#[allow(unused)]
	eyeL_irisColor_R: f32,
#[allow(unused)]
	eyeL_irisColor_G: f32,
#[allow(unused)]
	eyeL_irisColor_B: f32,
#[allow(unused)]
	eyeL_irisScale: f32,
#[allow(unused)]
	eyeL_cataract: f32,
#[allow(unused)]
	eyeL_cataractColor_R: f32,
#[allow(unused)]
	eyeL_cataractColor_G: f32,
#[allow(unused)]
	eyeL_cataractColor_B: f32,
#[allow(unused)]
	eyeL_scleraColor_R: f32,
#[allow(unused)]
	eyeL_scleraColor_G: f32,
#[allow(unused)]
	eyeL_scleraColor_B: f32,
#[allow(unused)]
	eyeL_irisDistance: f32,
#[allow(unused)]
	hair_partsId: f32,
#[allow(unused)]
	hair_color_R: f32,
#[allow(unused)]
	hair_color_G: f32,
#[allow(unused)]
	hair_color_B: f32,
#[allow(unused)]
	hair_shininess: f32,
#[allow(unused)]
	hair_rootBlack: f32,
#[allow(unused)]
	hair_whiteDensity: f32,
#[allow(unused)]
	beard_partsId: f32,
#[allow(unused)]
	beard_color_R: f32,
#[allow(unused)]
	beard_color_G: f32,
#[allow(unused)]
	beard_color_B: f32,
#[allow(unused)]
	beard_shininess: f32,
#[allow(unused)]
	beard_rootBlack: f32,
#[allow(unused)]
	beard_whiteDensity: f32,
#[allow(unused)]
	eyebrow_partsId: f32,
#[allow(unused)]
	eyebrow_color_R: f32,
#[allow(unused)]
	eyebrow_color_G: f32,
#[allow(unused)]
	eyebrow_color_B: f32,
#[allow(unused)]
	eyebrow_shininess: f32,
#[allow(unused)]
	eyebrow_rootBlack: f32,
#[allow(unused)]
	eyebrow_whiteDensity: f32,
#[allow(unused)]
	eyelash_partsId: f32,
#[allow(unused)]
	eyelash_color_R: f32,
#[allow(unused)]
	eyelash_color_G: f32,
#[allow(unused)]
	eyelash_color_B: f32,
#[allow(unused)]
	accessories_partsId: f32,
#[allow(unused)]
	accessories_color_R: f32,
#[allow(unused)]
	accessories_color_G: f32,
#[allow(unused)]
	accessories_color_B: f32,
#[allow(unused)]
	decal_partsId: f32,
#[allow(unused)]
	decal_posX: f32,
#[allow(unused)]
	decal_posY: f32,
#[allow(unused)]
	decal_angle: f32,
#[allow(unused)]
	decal_scale: f32,
#[allow(unused)]
	decal_color_R: f32,
#[allow(unused)]
	decal_color_G: f32,
#[allow(unused)]
	decal_color_B: f32,
#[allow(unused)]
	decal_gloss: f32,
#[allow(unused)]
	decal_mirror: f32,
#[allow(unused)]
	chrBodyScaleHead: f32,
#[allow(unused)]
	chrBodyScaleBreast: f32,
#[allow(unused)]
	chrBodyScaleAbdomen: f32,
#[allow(unused)]
	chrBodyScaleArm: f32,
#[allow(unused)]
	chrBodyScaleLeg: f32,
#[allow(unused)]
	age: f32,
#[allow(unused)]
	gender: f32,
#[allow(unused)]
	caricatureGeometry: f32,
#[allow(unused)]
	caricatureTexture: f32,
#[allow(unused)]
	faceGeoData00: f32,
#[allow(unused)]
	faceGeoData01: f32,
#[allow(unused)]
	faceGeoData02: f32,
#[allow(unused)]
	faceGeoData03: f32,
#[allow(unused)]
	faceGeoData04: f32,
#[allow(unused)]
	faceGeoData05: f32,
#[allow(unused)]
	faceGeoData06: f32,
#[allow(unused)]
	faceGeoData07: f32,
#[allow(unused)]
	faceGeoData08: f32,
#[allow(unused)]
	faceGeoData09: f32,
#[allow(unused)]
	faceGeoData10: f32,
#[allow(unused)]
	faceGeoData11: f32,
#[allow(unused)]
	faceGeoData12: f32,
#[allow(unused)]
	faceGeoData13: f32,
#[allow(unused)]
	faceGeoData14: f32,
#[allow(unused)]
	faceGeoData15: f32,
#[allow(unused)]
	faceGeoData16: f32,
#[allow(unused)]
	faceGeoData17: f32,
#[allow(unused)]
	faceGeoData18: f32,
#[allow(unused)]
	faceGeoData19: f32,
#[allow(unused)]
	faceGeoData20: f32,
#[allow(unused)]
	faceGeoData21: f32,
#[allow(unused)]
	faceGeoData22: f32,
#[allow(unused)]
	faceGeoData23: f32,
#[allow(unused)]
	faceGeoData24: f32,
#[allow(unused)]
	faceGeoData25: f32,
#[allow(unused)]
	faceGeoData26: f32,
#[allow(unused)]
	faceGeoData27: f32,
#[allow(unused)]
	faceGeoData28: f32,
#[allow(unused)]
	faceGeoData29: f32,
#[allow(unused)]
	faceGeoData30: f32,
#[allow(unused)]
	faceGeoData31: f32,
#[allow(unused)]
	faceGeoData32: f32,
#[allow(unused)]
	faceGeoData33: f32,
#[allow(unused)]
	faceGeoData34: f32,
#[allow(unused)]
	faceGeoData35: f32,
#[allow(unused)]
	faceGeoData36: f32,
#[allow(unused)]
	faceGeoData37: f32,
#[allow(unused)]
	faceGeoData38: f32,
#[allow(unused)]
	faceGeoData39: f32,
#[allow(unused)]
	faceGeoData40: f32,
#[allow(unused)]
	faceGeoData41: f32,
#[allow(unused)]
	faceGeoData42: f32,
#[allow(unused)]
	faceGeoData43: f32,
#[allow(unused)]
	faceGeoData44: f32,
#[allow(unused)]
	faceGeoData45: f32,
#[allow(unused)]
	faceGeoData46: f32,
#[allow(unused)]
	faceGeoData47: f32,
#[allow(unused)]
	faceGeoData48: f32,
#[allow(unused)]
	faceGeoData49: f32,
#[allow(unused)]
	faceGeoData50: f32,
#[allow(unused)]
	faceGeoData51: f32,
#[allow(unused)]
	faceGeoData52: f32,
#[allow(unused)]
	faceGeoData53: f32,
#[allow(unused)]
	faceGeoData54: f32,
#[allow(unused)]
	faceGeoData55: f32,
#[allow(unused)]
	faceGeoData56: f32,
#[allow(unused)]
	faceGeoData57: f32,
#[allow(unused)]
	faceGeoData58: f32,
#[allow(unused)]
	faceGeoData59: f32,
#[allow(unused)]
	faceGeoData60: f32,
#[allow(unused)]
	faceTexData00: f32,
#[allow(unused)]
	faceTexData01: f32,
#[allow(unused)]
	faceTexData02: f32,
#[allow(unused)]
	faceTexData03: f32,
#[allow(unused)]
	faceTexData04: f32,
#[allow(unused)]
	faceTexData05: f32,
#[allow(unused)]
	faceTexData06: f32,
#[allow(unused)]
	faceTexData07: f32,
#[allow(unused)]
	faceTexData08: f32,
#[allow(unused)]
	faceTexData09: f32,
#[allow(unused)]
	faceTexData10: f32,
#[allow(unused)]
	faceTexData11: f32,
#[allow(unused)]
	faceTexData12: f32,
#[allow(unused)]
	faceTexData13: f32,
#[allow(unused)]
	faceTexData14: f32,
#[allow(unused)]
	faceTexData15: f32,
#[allow(unused)]
	faceTexData16: f32,
#[allow(unused)]
	faceTexData17: f32,
#[allow(unused)]
	faceTexData18: f32,
#[allow(unused)]
	faceTexData19: f32,
#[allow(unused)]
	faceTexData20: f32,
#[allow(unused)]
	faceTexData21: f32,
#[allow(unused)]
	faceTexData22: f32,
#[allow(unused)]
	faceTexData23: f32,
#[allow(unused)]
	faceTexData24: f32,
#[allow(unused)]
	faceTexData25: f32,
#[allow(unused)]
	faceTexData26: f32,
#[allow(unused)]
	faceTexData27: f32,
#[allow(unused)]
	faceTexData28: f32,
#[allow(unused)]
	faceTexData29: f32,
#[allow(unused)]
	faceTexData30: f32,
#[allow(unused)]
	faceTexData31: f32,
#[allow(unused)]
	faceTexData32: f32,
#[allow(unused)]
	faceTexData33: f32,
#[allow(unused)]
	faceTexData34: f32,
#[allow(unused)]
	faceTexData35: f32,
#[allow(unused)]
	burn_scar: f32,
}

struct FE_TEXT_EFFECT_PARAM_ST {
#[allow(unused)]
	resId: i16,
#[allow(unused)]
	pad1: [u8;2],
#[allow(unused)]
	textId: i32,
#[allow(unused)]
	seId: i32,
#[allow(unused)]
	canMixMapName: u8,
#[allow(unused)]
	pad3: [u8;1],
#[allow(unused)]
	pad2: [u8;19],
}

struct FINAL_DAMAGE_RATE_PARAM_ST {
#[allow(unused)]
	physRate: f32,
#[allow(unused)]
	magRate: f32,
#[allow(unused)]
	fireRate: f32,
#[allow(unused)]
	thunRate: f32,
#[allow(unused)]
	darkRate: f32,
#[allow(unused)]
	staminaRate: f32,
#[allow(unused)]
	saRate: f32,
}

struct FOOT_SFX_PARAM_ST {
#[allow(unused)]
	sfxId_00: i32,
#[allow(unused)]
	sfxId_01: i32,
#[allow(unused)]
	sfxId_02: i32,
#[allow(unused)]
	sfxId_03: i32,
#[allow(unused)]
	sfxId_04: i32,
#[allow(unused)]
	sfxId_05: i32,
#[allow(unused)]
	sfxId_06: i32,
#[allow(unused)]
	sfxId_07: i32,
#[allow(unused)]
	sfxId_08: i32,
#[allow(unused)]
	sfxId_09: i32,
#[allow(unused)]
	sfxId_10: i32,
#[allow(unused)]
	sfxId_11: i32,
#[allow(unused)]
	sfxId_12: i32,
#[allow(unused)]
	sfxId_13: i32,
#[allow(unused)]
	sfxId_14: i32,
#[allow(unused)]
	sfxId_15: i32,
#[allow(unused)]
	sfxId_16: i32,
#[allow(unused)]
	sfxId_17: i32,
#[allow(unused)]
	sfxId_18: i32,
#[allow(unused)]
	sfxId_19: i32,
#[allow(unused)]
	sfxId_20: i32,
#[allow(unused)]
	sfxId_21: i32,
#[allow(unused)]
	sfxId_22: i32,
#[allow(unused)]
	sfxId_23: i32,
#[allow(unused)]
	sfxId_24: i32,
#[allow(unused)]
	sfxId_25: i32,
#[allow(unused)]
	sfxId_26: i32,
#[allow(unused)]
	sfxId_27: i32,
#[allow(unused)]
	sfxId_28: i32,
#[allow(unused)]
	sfxId_29: i32,
#[allow(unused)]
	sfxId_30: i32,
#[allow(unused)]
	sfxId_31: i32,
#[allow(unused)]
	sfxId_32: i32,
#[allow(unused)]
	sfxId_33: i32,
#[allow(unused)]
	sfxId_34: i32,
#[allow(unused)]
	sfxId_35: i32,
#[allow(unused)]
	sfxId_36: i32,
#[allow(unused)]
	sfxId_37: i32,
#[allow(unused)]
	sfxId_38: i32,
#[allow(unused)]
	sfxId_39: i32,
#[allow(unused)]
	sfxId_40: i32,
#[allow(unused)]
	sfxId_41: i32,
#[allow(unused)]
	sfxId_42: i32,
#[allow(unused)]
	sfxId_43: i32,
#[allow(unused)]
	sfxId_44: i32,
#[allow(unused)]
	sfxId_45: i32,
#[allow(unused)]
	sfxId_46: i32,
#[allow(unused)]
	sfxId_47: i32,
#[allow(unused)]
	sfxId_48: i32,
#[allow(unused)]
	sfxId_49: i32,
#[allow(unused)]
	sfxId_50: i32,
#[allow(unused)]
	sfxId_51: i32,
#[allow(unused)]
	sfxId_52: i32,
#[allow(unused)]
	sfxId_53: i32,
#[allow(unused)]
	sfxId_54: i32,
#[allow(unused)]
	sfxId_55: i32,
#[allow(unused)]
	sfxId_56: i32,
#[allow(unused)]
	sfxId_57: i32,
#[allow(unused)]
	sfxId_58: i32,
#[allow(unused)]
	sfxId_59: i32,
#[allow(unused)]
	sfxId_60: i32,
#[allow(unused)]
	sfxId_61: i32,
#[allow(unused)]
	sfxId_62: i32,
#[allow(unused)]
	sfxId_63: i32,
#[allow(unused)]
	sfxId_64: i32,
#[allow(unused)]
	sfxId_65: i32,
#[allow(unused)]
	sfxId_66: i32,
#[allow(unused)]
	sfxId_67: i32,
#[allow(unused)]
	sfxId_68: i32,
#[allow(unused)]
	sfxId_69: i32,
#[allow(unused)]
	sfxId_70: i32,
#[allow(unused)]
	sfxId_71: i32,
#[allow(unused)]
	sfxId_72: i32,
#[allow(unused)]
	sfxId_73: i32,
#[allow(unused)]
	sfxId_74: i32,
#[allow(unused)]
	sfxId_75: i32,
#[allow(unused)]
	sfxId_76: i32,
#[allow(unused)]
	sfxId_77: i32,
#[allow(unused)]
	sfxId_78: i32,
#[allow(unused)]
	sfxId_79: i32,
#[allow(unused)]
	sfxId_80: i32,
#[allow(unused)]
	sfxId_81: i32,
#[allow(unused)]
	sfxId_82: i32,
#[allow(unused)]
	sfxId_83: i32,
#[allow(unused)]
	sfxId_84: i32,
#[allow(unused)]
	sfxId_85: i32,
#[allow(unused)]
	sfxId_86: i32,
#[allow(unused)]
	sfxId_87: i32,
#[allow(unused)]
	sfxId_88: i32,
#[allow(unused)]
	sfxId_89: i32,
#[allow(unused)]
	sfxId_90: i32,
#[allow(unused)]
	sfxId_91: i32,
#[allow(unused)]
	sfxId_92: i32,
#[allow(unused)]
	sfxId_93: i32,
#[allow(unused)]
	sfxId_94: i32,
#[allow(unused)]
	sfxId_95: i32,
#[allow(unused)]
	sfxId_96: i32,
#[allow(unused)]
	sfxId_97: i32,
#[allow(unused)]
	sfxId_98: i32,
#[allow(unused)]
	sfxId_99: i32,
#[allow(unused)]
	sfxId_100: i32,
#[allow(unused)]
	sfxId_101: i32,
#[allow(unused)]
	sfxId_102: i32,
#[allow(unused)]
	sfxId_103: i32,
#[allow(unused)]
	sfxId_104: i32,
#[allow(unused)]
	sfxId_105: i32,
#[allow(unused)]
	sfxId_106: i32,
#[allow(unused)]
	sfxId_107: i32,
#[allow(unused)]
	sfxId_108: i32,
#[allow(unused)]
	sfxId_109: i32,
#[allow(unused)]
	sfxId_110: i32,
#[allow(unused)]
	sfxId_111: i32,
#[allow(unused)]
	sfxId_112: i32,
#[allow(unused)]
	sfxId_113: i32,
#[allow(unused)]
	sfxId_114: i32,
#[allow(unused)]
	sfxId_115: i32,
#[allow(unused)]
	sfxId_116: i32,
#[allow(unused)]
	sfxId_117: i32,
#[allow(unused)]
	sfxId_118: i32,
#[allow(unused)]
	sfxId_119: i32,
#[allow(unused)]
	sfxId_120: i32,
#[allow(unused)]
	sfxId_121: i32,
#[allow(unused)]
	sfxId_122: i32,
#[allow(unused)]
	sfxId_123: i32,
#[allow(unused)]
	sfxId_124: i32,
#[allow(unused)]
	sfxId_125: i32,
#[allow(unused)]
	sfxId_126: i32,
#[allow(unused)]
	sfxId_127: i32,
#[allow(unused)]
	sfxId_128: i32,
#[allow(unused)]
	sfxId_129: i32,
#[allow(unused)]
	sfxId_130: i32,
#[allow(unused)]
	sfxId_131: i32,
#[allow(unused)]
	sfxId_132: i32,
#[allow(unused)]
	sfxId_133: i32,
#[allow(unused)]
	sfxId_134: i32,
#[allow(unused)]
	sfxId_135: i32,
#[allow(unused)]
	sfxId_136: i32,
#[allow(unused)]
	sfxId_137: i32,
#[allow(unused)]
	sfxId_138: i32,
#[allow(unused)]
	sfxId_139: i32,
#[allow(unused)]
	sfxId_140: i32,
#[allow(unused)]
	sfxId_141: i32,
#[allow(unused)]
	sfxId_142: i32,
#[allow(unused)]
	sfxId_143: i32,
#[allow(unused)]
	sfxId_144: i32,
#[allow(unused)]
	sfxId_145: i32,
#[allow(unused)]
	sfxId_146: i32,
#[allow(unused)]
	sfxId_147: i32,
#[allow(unused)]
	sfxId_148: i32,
#[allow(unused)]
	sfxId_149: i32,
#[allow(unused)]
	sfxId_150: i32,
#[allow(unused)]
	sfxId_151: i32,
#[allow(unused)]
	sfxId_152: i32,
#[allow(unused)]
	sfxId_153: i32,
#[allow(unused)]
	sfxId_154: i32,
#[allow(unused)]
	sfxId_155: i32,
#[allow(unused)]
	sfxId_156: i32,
#[allow(unused)]
	sfxId_157: i32,
#[allow(unused)]
	sfxId_158: i32,
#[allow(unused)]
	sfxId_159: i32,
#[allow(unused)]
	sfxId_160: i32,
#[allow(unused)]
	sfxId_161: i32,
#[allow(unused)]
	sfxId_162: i32,
#[allow(unused)]
	sfxId_163: i32,
#[allow(unused)]
	sfxId_164: i32,
#[allow(unused)]
	sfxId_165: i32,
#[allow(unused)]
	sfxId_166: i32,
#[allow(unused)]
	sfxId_167: i32,
#[allow(unused)]
	sfxId_168: i32,
#[allow(unused)]
	sfxId_169: i32,
#[allow(unused)]
	sfxId_170: i32,
#[allow(unused)]
	sfxId_171: i32,
#[allow(unused)]
	sfxId_172: i32,
#[allow(unused)]
	sfxId_173: i32,
#[allow(unused)]
	sfxId_174: i32,
#[allow(unused)]
	sfxId_175: i32,
#[allow(unused)]
	sfxId_176: i32,
#[allow(unused)]
	sfxId_177: i32,
#[allow(unused)]
	sfxId_178: i32,
#[allow(unused)]
	sfxId_179: i32,
#[allow(unused)]
	sfxId_180: i32,
#[allow(unused)]
	sfxId_181: i32,
#[allow(unused)]
	sfxId_182: i32,
#[allow(unused)]
	sfxId_183: i32,
#[allow(unused)]
	sfxId_184: i32,
#[allow(unused)]
	sfxId_185: i32,
#[allow(unused)]
	sfxId_186: i32,
#[allow(unused)]
	sfxId_187: i32,
#[allow(unused)]
	sfxId_188: i32,
#[allow(unused)]
	sfxId_189: i32,
#[allow(unused)]
	sfxId_190: i32,
#[allow(unused)]
	sfxId_191: i32,
#[allow(unused)]
	sfxId_192: i32,
#[allow(unused)]
	sfxId_193: i32,
#[allow(unused)]
	sfxId_194: i32,
#[allow(unused)]
	sfxId_195: i32,
#[allow(unused)]
	sfxId_196: i32,
#[allow(unused)]
	sfxId_197: i32,
#[allow(unused)]
	sfxId_198: i32,
#[allow(unused)]
	sfxId_199: i32,
}

struct GAME_AREA_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	bonusSoul_single: i32,
#[allow(unused)]
	bonusSoul_multi: i32,
#[allow(unused)]
	humanityPointCountFlagIdTop: i32,
#[allow(unused)]
	humanityDropPoint1: i16,
#[allow(unused)]
	humanityDropPoint2: i16,
#[allow(unused)]
	humanityDropPoint3: i16,
#[allow(unused)]
	humanityDropPoint4: i16,
#[allow(unused)]
	humanityDropPoint5: i16,
#[allow(unused)]
	humanityDropPoint6: i16,
#[allow(unused)]
	humanityDropPoint7: i16,
#[allow(unused)]
	humanityDropPoint8: i16,
#[allow(unused)]
	humanityDropPoint9: i16,
#[allow(unused)]
	humanityDropPoint10: i16,
#[allow(unused)]
	soloBreakInPoint_Min: i32,
#[allow(unused)]
	soloBreakInPoint_Max: i32,
#[allow(unused)]
	defeatBossFlagId_forSignAimList: i32,
#[allow(unused)]
	displayAimFlagId: i32,
#[allow(unused)]
	foundBossFlagId: i32,
#[allow(unused)]
	foundBossTextId: i32,
#[allow(unused)]
	notFindBossTextId: i32,
#[allow(unused)]
	bossChallengeFlagId: i32,
#[allow(unused)]
	defeatBossFlagId: i32,
#[allow(unused)]
	bossPosX: f32,
#[allow(unused)]
	bossPosY: f32,
#[allow(unused)]
	bossPosZ: f32,
#[allow(unused)]
	bossMapAreaNo: u8,
#[allow(unused)]
	bossMapBlockNo: u8,
#[allow(unused)]
	bossMapMapNo: u8,
#[allow(unused)]
	reserve: [u8;9],
}

struct GAME_INFO_PARAM {
#[allow(unused)]
	titleMsgId: i32,
#[allow(unused)]
	contentMsgId: i32,
#[allow(unused)]
	value: i32,
#[allow(unused)]
	sortId: i32,
#[allow(unused)]
	eventId: i32,
#[allow(unused)]
	Pad: [u8;12],
}

struct GAME_SYSTEM_COMMON_PARAM_ST {
#[allow(unused)]
	baseToughnessRecoverTime: f32,
#[allow(unused)]
	chrEventTrun_byLeft90: i32,
#[allow(unused)]
	chrEventTrun_byRight90: i32,
#[allow(unused)]
	chrEventTrun_byLeft180: i32,
#[allow(unused)]
	chrEventTrun_byRight180: i32,
#[allow(unused)]
	chrEventTrun_90TurnStartAngle: i16,
#[allow(unused)]
	chrEventTrun_180TurnStartAngle: i16,
#[allow(unused)]
	stealthAtkDamageRate: f32,
#[allow(unused)]
	flickDamageCutRateSuccessGurad: f32,
#[allow(unused)]
	npcTalkAnimBeginDiffAngle: f32,
#[allow(unused)]
	npcTalkAnimEndDiffAngle: f32,
#[allow(unused)]
	sleepCollectorItemActionButtonParamId: i32,
#[allow(unused)]
	allowUseBuddyItem_sfxInterval: f32,
#[allow(unused)]
	allowUseBuddyItem_sfxDmyPolyId: i32,
#[allow(unused)]
	allowUseBuddyItem_sfxDmyPolyId_horse: i32,
#[allow(unused)]
	allowUseBuddyItem_sfxId: i32,
#[allow(unused)]
	onBuddySummon_inActivateRange_sfxInterval: f32,
#[allow(unused)]
	onBuddySummon_inActivateRange_sfxDmyPolyId: i32,
#[allow(unused)]
	onBuddySummon_inActivateRange_sfxDmyPolyId_horse: i32,
#[allow(unused)]
	onBuddySummon_inActivateRange_sfxId: i32,
#[allow(unused)]
	onBuddySummon_inActivateRange_spEffectId_pc: i32,
#[allow(unused)]
	onBuddySummon_inWarnRange_spEffectId_pc: i32,
#[allow(unused)]
	onBuddySummon_atBuddyUnsummon_spEffectId_pc: i32,
#[allow(unused)]
	onBuddySummon_inWarnRange_spEffectId_buddy: i32,
#[allow(unused)]
	morningIngameHour: u8,
#[allow(unused)]
	morningIngameMinute: u8,
#[allow(unused)]
	morningIngameSecond: u8,
#[allow(unused)]
	noonIngameHour: u8,
#[allow(unused)]
	noonIngameMinute: u8,
#[allow(unused)]
	noonIngameSecond: u8,
#[allow(unused)]
	nightIngameHour: u8,
#[allow(unused)]
	nightIngameMinute: u8,
#[allow(unused)]
	nightIngameSecond: u8,
#[allow(unused)]
	aiSightRateStart_Morning_Hour: u8,
#[allow(unused)]
	aiSightRateStart_Morning_Minute: u8,
#[allow(unused)]
	aiSightRateStart_Noon_Hour: u8,
#[allow(unused)]
	aiSightRateStart_Noon_Minute: u8,
#[allow(unused)]
	aiSightRateStart_Evening_Hour: u8,
#[allow(unused)]
	aiSightRateStart_Evening_Minute: u8,
#[allow(unused)]
	aiSightRateStart_Night_Hour: u8,
#[allow(unused)]
	aiSightRateStart_Night_Minute: u8,
#[allow(unused)]
	aiSightRateStart_Midnight_Hour: u8,
#[allow(unused)]
	aiSightRateStart_Midnight_Minute: u8,
#[allow(unused)]
	saLargeDamageHitSfx_Threshold: u8,
#[allow(unused)]
	saLargeDamageHitSfx_SfxId: i32,
#[allow(unused)]
	signCreatableDistFromSafePos: f32,
#[allow(unused)]
	guestResummonDist: f32,
#[allow(unused)]
	guestLeavingMessageDistMax: f32,
#[allow(unused)]
	guestLeavingMessageDistMin: f32,
#[allow(unused)]
	guestLeaveSessionDist: f32,
#[allow(unused)]
	retryPointAreaRadius: f32,
#[allow(unused)]
	sleepCollectorSpEffectId: i32,
#[allow(unused)]
	recoverBelowMaxHpCompletionNoticeSpEffectId: i32,
#[allow(unused)]
	estusFlaskRecovery_AbsorptionProductionSfxId_byHp: i32,
#[allow(unused)]
	estusFlaskRecovery_AbsorptionProductionSfxId_byMp: i32,
#[allow(unused)]
	respawnSpecialEffectActiveCheckerSpEffectId: i32,
#[allow(unused)]
	onBuddySummon_inActivateRange_spEffectId_buddy: i32,
#[allow(unused)]
	estusFlaskRecovery_AddEstusTime: f32,
#[allow(unused)]
	defeatMultiModeEnemyOfSoulCorrectRate_byHost: f32,
#[allow(unused)]
	defeatMultiModeEnemyOfSoulCorrectRate_byTeamGhost: f32,
#[allow(unused)]
	defeatMultiModeBossOfSoulCorrectRate_byHost: f32,
#[allow(unused)]
	defeatMultiModeBossOfSoulCorrectRate_byTeamGhost: f32,
#[allow(unused)]
	enemyHpGaugeScreenOffset_byUp: i16,
#[allow(unused)]
	playRegionCollectDist: i16,
#[allow(unused)]
	enemyDetectionSpEffect_ShootBulletDummypolyId: i16,
#[allow(unused)]
	bigRuneGreaterDemonBreakInGoodsNum: i16,
#[allow(unused)]
	bigRuneGreaterDemonBreakInGoodsId: i32,
#[allow(unused)]
	rideJumpRegionDefaultSfxId: i32,
#[allow(unused)]
	saAttackRate_forVsRideAtk: f32,
#[allow(unused)]
	enemySpEffectIdAfterSleepCollectorItemLot: i32,
#[allow(unused)]
	afterEndingMapUid: i32,
#[allow(unused)]
	afterEndingReturnPointEntityId: i32,
#[allow(unused)]
	enemyDetectionSpEffect_BulletId_byCoopRing_RedHunter: i32,
#[allow(unused)]
	enemyDetectionSpEffect_BulletId_byInvadeOrb_None: i32,
#[allow(unused)]
	tutorialFlagOnAccessDistView: i32,
#[allow(unused)]
	tutorialFlagOnAccessRetryPoint: i32,
#[allow(unused)]
	tutorialFlagOnGetGroupReward: i32,
#[allow(unused)]
	tutorialFlagOnEnterRideJumpRegion: i32,
#[allow(unused)]
	tutorialCheckRideJumpRegionExpandRange: f32,
#[allow(unused)]
	retryPointActivatedPcAnimId: i32,
#[allow(unused)]
	retryPointActivatedDialogDelayTime: f32,
#[allow(unused)]
	retryPointActivatedDialogTextId: i32,
#[allow(unused)]
	signPuddleOpenPcAnimId: i32,
#[allow(unused)]
	signPuddleOpenDialogDelayTime: f32,
#[allow(unused)]
	activityOfDeadSpEffect_BulletId: i32,
#[allow(unused)]
	activityOfDeadSpEffect_ShootBulletDummypolyId: i32,
#[allow(unused)]
	activityOfDeadSpEffect_DeadFadeOutTime: f32,
#[allow(unused)]
	ignorNetStateSyncTime_ForThrow: f32,
#[allow(unused)]
	netPenaltyPointLanDisconnect: i16,
#[allow(unused)]
	netPenaltyPointProfileSignout: i16,
#[allow(unused)]
	netPenaltyPointReboot: i16,
#[allow(unused)]
	netPnaltyPointSuspend: i16,
#[allow(unused)]
	netPenaltyForgiveItemLimitTime: f32,
#[allow(unused)]
	netPenaltyPointThreshold: i16,
#[allow(unused)]
	uncontrolledMoveThresholdTime: i16,
#[allow(unused)]
	enemyDetectionSpEffect_BulletId_byNpcEnemy: i32,
#[allow(unused)]
	activityOfDeadTargetSearchSpEffect_OnHitSpEffect: i32,
#[allow(unused)]
	activityOfDeadTargetSearchSpEffect_MaxLength: f32,
#[allow(unused)]
	sightRangeLowerPromiseRate: f32,
#[allow(unused)]
	saLargeDamageHitSfx_MinDamage: i16,
#[allow(unused)]
	saLargeDamageHitSfx_ForceDamage: i16,
#[allow(unused)]
	soloBreakInMaxPoint: i32,
#[allow(unused)]
	npcTalkTimeOutThreshold: f32,
#[allow(unused)]
	sendPlayLogIntervalTime: f32,
#[allow(unused)]
	item370_MaxSfxNum: u8,
#[allow(unused)]
	chrActivateDist_forLeavePC: u8,
#[allow(unused)]
	summonDataCoopMatchingLevelUpperAbs: i16,
#[allow(unused)]
	summonDataCoopMatchingLevelUpperRel: i16,
#[allow(unused)]
	summonDataCoopMatchingWepLevelMul: i16,
#[allow(unused)]
	pickUpBerserkerSignSpEffectBulletId: i32,
#[allow(unused)]
	succeedBerserkerSelfKillingEffectId: i32,
#[allow(unused)]
	machingLevelWhiteSignUpperRel: u8,
#[allow(unused)]
	machingLevelWhiteSignUpperAbs: u8,
#[allow(unused)]
	machingLevelRedSignUpperRel: u8,
#[allow(unused)]
	machingLevelRedSignUpperAbs: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_0: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_1: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_2: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_3: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_4: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_5: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_6: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_7: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_8: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_9: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_10: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_0: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_1: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_2: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_3: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_4: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_5: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_6: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_7: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_8: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_9: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_10: u8,
#[allow(unused)]
	autoInvadePoint_generateDist: u8,
#[allow(unused)]
	autoInvadePoint_cancelDist: u8,
#[allow(unused)]
	sendGlobalEventLogIntervalTime: f32,
#[allow(unused)]
	addSoloBreakInPoint_White: i16,
#[allow(unused)]
	addSoloBreakInPoint_Black: i16,
#[allow(unused)]
	addSoloBreakInPoint_ForceJoin: i16,
#[allow(unused)]
	addSoloBreakInPoint_VisitorGuardian: i16,
#[allow(unused)]
	addSoloBreakInPoint_VisitorRedHunter: i16,
#[allow(unused)]
	invincibleTimer_forNetPC_initSync: u8,
#[allow(unused)]
	invincibleTimer_forNetPC: u8,
#[allow(unused)]
	redHunter_HostBossAreaGetSoulRate: f32,
#[allow(unused)]
	ghostFootprintDecalParamId: i32,
#[allow(unused)]
	leaveAroundHostWarningTime: f32,
#[allow(unused)]
	hostModeCostItemId: i32,
#[allow(unused)]
	aIJump_DecelerateParam: f32,
#[allow(unused)]
	buddyDisappearDelaySec: f32,
#[allow(unused)]
	aIJump_AnimYMoveCorrectRate_onJumpOff: f32,
#[allow(unused)]
	stealthSystemSightRate_NotInStealthRigid_NotSightHide_StealthMode: f32,
#[allow(unused)]
	stealthSystemSightRate_NotInStealthRigid_SightHide_NotStealthMode: f32,
#[allow(unused)]
	stealthSystemSightRate_NotInStealthRigid_SightHide_StealthMode: f32,
#[allow(unused)]
	stealthSystemSightRate_InStealthRigid_NotSightHide_NotStealthMode: f32,
#[allow(unused)]
	stealthSystemSightRate_InStealthRigid_NotSightHide_StealthMode: f32,
#[allow(unused)]
	stealthSystemSightRate_InStealthRigid_SightHide_NotStealthMode: f32,
#[allow(unused)]
	stealthSystemSightRate_InStealthRigid_SightHide_StealthMode: f32,
#[allow(unused)]
	msbEventGeomTreasureInfo_actionButtonParamId_corpse: i32,
#[allow(unused)]
	msbEventGeomTreasureInfo_itemGetAnimId_corpse: i32,
#[allow(unused)]
	msbEventGeomTreasureInfo_actionButtonParamId_box: i32,
#[allow(unused)]
	msbEventGeomTreasureInfo_itemGetAnimId_box: i32,
#[allow(unused)]
	msbEventGeomTreasureInfo_actionButtonParamId_shine: i32,
#[allow(unused)]
	msbEventGeomTreasureInfo_itemGetAnimId_shine: i32,
#[allow(unused)]
	signPuddleAssetId: i32,
#[allow(unused)]
	signPuddleAppearDmypolyId0: i32,
#[allow(unused)]
	signPuddleAppearDmypolyId1: i32,
#[allow(unused)]
	signPuddleAppearDmypolyId2: i32,
#[allow(unused)]
	signPuddleAppearDmypolyId3: i32,
#[allow(unused)]
	fallDamageRate_forRidePC: f32,
#[allow(unused)]
	fallDamageRate_forRideNPC: f32,
#[allow(unused)]
	OldMonkOfYellow_CreateSignSpEffectId: i32,
#[allow(unused)]
	StragglerActivateDist: f32,
#[allow(unused)]
	SpEffectId_EnableUseItem_StragglerActivate: i32,
#[allow(unused)]
	SpEffectId_StragglerWakeUp: i32,
#[allow(unused)]
	SpEffectId_StragglerTarget: i32,
#[allow(unused)]
	SpEffectId_StragglerOppose: i32,
#[allow(unused)]
	buddyWarp_TriggerTimeRayBlocked: f32,
#[allow(unused)]
	buddyWarp_TriggerDistToPlayer: f32,
#[allow(unused)]
	buddyWarp_ThresholdTimePathStacked: f32,
#[allow(unused)]
	buddyWarp_ThresholdRangePathStacked: f32,
#[allow(unused)]
	aiSightRate_morning: f32,
#[allow(unused)]
	aiSightRate_noonA: f32,
#[allow(unused)]
	buddyPassThroughTriggerTime: f32,
#[allow(unused)]
	aiSightRate_evening: f32,
#[allow(unused)]
	aiSightRate_night: f32,
#[allow(unused)]
	aiSightRate_midnightA: f32,
#[allow(unused)]
	reserve4_2: [u8;4],
#[allow(unused)]
	aiSightRate_sunloss_light: f32,
#[allow(unused)]
	aiSightRate_sunloss_dark: f32,
#[allow(unused)]
	aiSightRate_sunloss_veryDark: f32,
#[allow(unused)]
	stealthSystemSightAngleReduceRate_NotInStealthRigid_NotSightHide_StealthMode: f32,
#[allow(unused)]
	stealthSystemSightAngleReduceRate_NotInStealthRigid_SightHide_NotStealthMode: f32,
#[allow(unused)]
	stealthSystemSightAngleReduceRate_NotInStealthRigid_SightHide_StealthMode: f32,
#[allow(unused)]
	stealthSystemSightAngleReduceRate_InStealthRigid_NotSightHide_NotStealthMode: f32,
#[allow(unused)]
	stealthSystemSightAngleReduceRate_InStealthRigid_NotSightHide_StealthMode: f32,
#[allow(unused)]
	stealthSystemSightAngleReduceRate_InStealthRigid_SightHide_NotStealthMode: f32,
#[allow(unused)]
	stealthSystemSightAngleReduceRate_InStealthRigid_SightHide_StealthMode: f32,
#[allow(unused)]
	weatherLotConditionStart_Morning_Hour: u8,
#[allow(unused)]
	weatherLotConditionStart_Morning_Minute: u8,
#[allow(unused)]
	weatherLotConditionStart_Day_Hour: u8,
#[allow(unused)]
	weatherLotConditionStart_Day_Minute: u8,
#[allow(unused)]
	weatherLotConditionStart_Evening_Hour: u8,
#[allow(unused)]
	weatherLotConditionStart_Evening_Minute: u8,
#[allow(unused)]
	weatherLotConditionStart_Night_Hour: u8,
#[allow(unused)]
	weatherLotConditionStart_Night_Minute: u8,
#[allow(unused)]
	weatherLotConditionStart_DayBreak_Hour: u8,
#[allow(unused)]
	weatherLotConditionStart_DayBreak_Minute: u8,
#[allow(unused)]
	weatherLotCondition_reserved: [u8;2],
#[allow(unused)]
	pclightScaleChangeStart_Hour: u8,
#[allow(unused)]
	pclightScaleChangeStart_Minute: u8,
#[allow(unused)]
	pclightScaleChangeEnd_Hour: u8,
#[allow(unused)]
	pclightScaleChangeEnd_Minute: u8,
#[allow(unused)]
	pclightScaleByTimezone: f32,
#[allow(unused)]
	bigRuneGreaterDemon_SummonBuddySpecialEffectId_Buddy: i32,
#[allow(unused)]
	bigRuneGreaterDemon_SummonBuddySpecialEffectId_Pc: i32,
#[allow(unused)]
	homeBonfireParamId: i32,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_11: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_12: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_13: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_14: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_15: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_16: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_17: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_18: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_19: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_20: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_21: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_22: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_23: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_24: u8,
#[allow(unused)]
	machingWeaponLevelUpperWhiteSign_25: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_11: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_12: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_13: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_14: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_15: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_16: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_17: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_18: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_19: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_20: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_21: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_22: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_23: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_24: u8,
#[allow(unused)]
	machingWeaponLevelUpperRedSign_25: u8,
#[allow(unused)]
	menuTimezoneStart_Morning_Hour: u8,
#[allow(unused)]
	menuTimezoneStart_Morning_Minute: u8,
#[allow(unused)]
	menuTimezoneStart_Day1_Hour: u8,
#[allow(unused)]
	menuTimezoneStart_Day1_Minute: u8,
#[allow(unused)]
	menuTimezoneStart_Day2_Hour: u8,
#[allow(unused)]
	menuTimezoneStart_Day2_Minute: u8,
#[allow(unused)]
	menuTimezoneStart_Evening_Hour: u8,
#[allow(unused)]
	menuTimezoneStart_Evening_Minute: u8,
#[allow(unused)]
	menuTimezoneStart_Night_Hour: u8,
#[allow(unused)]
	menuTimezoneStart_Night_Minute: u8,
#[allow(unused)]
	menuTimezoneStart_Midnight_Hour: u8,
#[allow(unused)]
	menuTimezoneStart_Midnight_Minute: u8,
#[allow(unused)]
	remotePlayerThreatLvNotify_ThreatLv: i16,
#[allow(unused)]
	remotePlayerThreatLvNotify_NotifyDist: f32,
#[allow(unused)]
	remotePlayerThreatLvNotify_EndNotifyDist: f32,
#[allow(unused)]
	worldMapPointDiscoveryExpandRange: f32,
#[allow(unused)]
	worldMapPointReentryExpandRange: f32,
#[allow(unused)]
	remotePlayerThreatLvNotify_NotifyTime: i16,
#[allow(unused)]
	breakIn_A_rebreakInGoodsNum: i16,
#[allow(unused)]
	breakIn_A_rebreakInGoodsId: i32,
#[allow(unused)]
	rideJumpoff_SfxId: i32,
#[allow(unused)]
	rideJumpoff_SfxHeightOffset: f32,
#[allow(unused)]
	rideJumpoff_SpEffectId: i32,
#[allow(unused)]
	rideJumpoff_SpEffectIdPc: i32,
#[allow(unused)]
	unlockExchangeMenuEventFlagId: i32,
#[allow(unused)]
	unlockMessageMenuEventFlagId: i32,
#[allow(unused)]
	breakInOnce_A_rebreakInGoodsNum: i16,
#[allow(unused)]
	breakIn_B_rebreakInGoodsNum: i16,
#[allow(unused)]
	breakInOnce_A_rebreakInGoodsId: i32,
#[allow(unused)]
	breakIn_B_rebreakInGoodsId: i32,
#[allow(unused)]
	actionButtonInputCancelTime: f32,
#[allow(unused)]
	blockClearBonusDelayTime: f32,
#[allow(unused)]
	bonfireCheckEnemyRange: f32,
#[allow(unused)]
	reserved_124: [u8;48],
#[allow(unused)]
	reserved_124: [u8;32],
#[allow(unused)]
	unkR00: f32,
#[allow(unused)]
	unkR04: f32,
#[allow(unused)]
	unkR08: f32,
#[allow(unused)]
	unkR12: f32,
#[allow(unused)]
	unkR16: f32,
#[allow(unused)]
	unkR20: f32,
#[allow(unused)]
	unkR24: f32,
#[allow(unused)]
	unkR28: f32,
#[allow(unused)]
	unkR32: f32,
#[allow(unused)]
	unkR36: f32,
#[allow(unused)]
	unkR40: f32,
#[allow(unused)]
	unkR44: f32,
#[allow(unused)]
	unkR48: f32,
#[allow(unused)]
	unkR52: f32,
#[allow(unused)]
	reserved_124_2: [u8;40],
}

struct CS_AA_QUALITY_DETAIL {
#[allow(unused)]
	enabled: u8,
#[allow(unused)]
	forceFXAA2: u8,
#[allow(unused)]
	dmy: [u8;2],
}

struct CS_DECAL_QUALITY_DETAIL {
#[allow(unused)]
	enabled: u8,
#[allow(unused)]
	dmy: [u8;3],
}

struct CS_DOF_QUALITY_DETAIL {
#[allow(unused)]
	enabled: u8,
#[allow(unused)]
	dmy: [u8;3],
#[allow(unused)]
	forceHiResoBlur: i32,
#[allow(unused)]
	maxBlurLevel: i32,
}

struct CS_EFFECT_QUALITY_DETAIL {
#[allow(unused)]
	softParticleEnabled: u8,
#[allow(unused)]
	glowEnabled: u8,
#[allow(unused)]
	distortionEnable: u8,
#[allow(unused)]
	cs_upScaleEnabledType: u8,
#[allow(unused)]
	fNumOnceEmitsScale: f32,
#[allow(unused)]
	fEmitSpanScale: f32,
#[allow(unused)]
	fLodDistance1Scale: f32,
#[allow(unused)]
	fLodDistance2Scale: f32,
#[allow(unused)]
	fLodDistance3Scale: f32,
#[allow(unused)]
	fLodDistance4Scale: f32,
#[allow(unused)]
	fScaleRenderDistanceScale: f32,
#[allow(unused)]
	dmy: [u8;4],
}

struct CS_LIGHTING_QUALITY_DETAIL {
#[allow(unused)]
	localLightDistFactor: f32,
#[allow(unused)]
	localLightShadowEnabled: u8,
#[allow(unused)]
	forwardPassLightingEnabled: u8,
#[allow(unused)]
	localLightShadowSpecLevelMax: u8,
#[allow(unused)]
	dmy: [u8;1],
}

struct CS_MOTION_BLUR_QUALITY_DETAIL {
#[allow(unused)]
	enabled: u8,
#[allow(unused)]
	ombEnabled: u8,
#[allow(unused)]
	forceScaleVelocityBuffer: u8,
#[allow(unused)]
	cheapFilterMode: u8,
#[allow(unused)]
	sampleCountBias: i32,
#[allow(unused)]
	recurrenceCountBias: i32,
#[allow(unused)]
	blurMaxLengthScale: f32,
}

struct CS_RAYTRACING_QUALITY_DETAIL {
#[allow(unused)]
	enableRaytraceAO: u8,
#[allow(unused)]
	enableRaytraceShadows: u8,
#[allow(unused)]
	Unk0x02: u8,
#[allow(unused)]
	Unk0x03: u8,
#[allow(unused)]
	UnkFloat0x04: f32,
#[allow(unused)]
	Unk0x08: i32,
#[allow(unused)]
	UnkFloat0x0C: f32,
#[allow(unused)]
	Unk0x10: i32,
#[allow(unused)]
	UnkFloat0x14: f32,
#[allow(unused)]
	UnkFloat0x18: f32,
}

struct CS_REFLECTION_QUALITY_DETAIL {
#[allow(unused)]
	enabled: u8,
#[allow(unused)]
	localLightEnabled: u8,
#[allow(unused)]
	localLightForceEnabled: u8,
#[allow(unused)]
	dmy: [u8;1],
#[allow(unused)]
	resolutionDivider: i32,
#[allow(unused)]
	ssrEnabled: u8,
#[allow(unused)]
	ssrGaussianBlurEnabled: u8,
#[allow(unused)]
	dmy2: [u8;2],
#[allow(unused)]
	ssrDepthRejectThresholdScale: f32,
#[allow(unused)]
	ssrRayTraceStepScale: f32,
#[allow(unused)]
	ssrFadeToViewerBias: f32,
#[allow(unused)]
	ssrFresnelRejectBias: f32,
}

struct CS_SHADER_QUALITY_DETAIL {
#[allow(unused)]
	sssEnabled: u8,
#[allow(unused)]
	tessellationEnabled: u8,
#[allow(unused)]
	highPrecisionNormalEnabled: u8,
#[allow(unused)]
	dmy: [u8;1],
}

struct CS_SHADOW_QUALITY_DETAIL {
#[allow(unused)]
	enabled: u8,
#[allow(unused)]
	maxFilterLevel: u8,
#[allow(unused)]
	dmy: [u8;2],
#[allow(unused)]
	textureSizeScaler: i32,
#[allow(unused)]
	textureSizeDivider: i32,
#[allow(unused)]
	textureMinSize: i32,
#[allow(unused)]
	textureMaxSize: i32,
#[allow(unused)]
	blurCountBias: i32,
}

struct CS_SSAO_QUALITY_DETAIL {
#[allow(unused)]
	enabled: u8,
#[allow(unused)]
	cs_reprojEnabledType: u8,
#[allow(unused)]
	cs_upScaleEnabledType: u8,
#[allow(unused)]
	cs_useNormalEnabledType: u8,
#[allow(unused)]
	dmy: [u8;1],
}

struct CS_TEXTURE_FILTER_QUALITY_DETAIL {
#[allow(unused)]
	filter: u8,
#[allow(unused)]
	dmy: [u8;3],
#[allow(unused)]
	maxAnisoLevel: i32,
}

struct CS_VOLUMETRIC_EFFECT_QUALITY_DETAIL {
#[allow(unused)]
	fogEnabled: u8,
#[allow(unused)]
	fogShadowEnabled: u8,
#[allow(unused)]
	dmy: [u8;2],
#[allow(unused)]
	fogShadowSampleCountBias: i32,
#[allow(unused)]
	fogLocalLightDistScale: f32,
#[allow(unused)]
	fogVolueSizeScaler: i32,
#[allow(unused)]
	fogVolueSizeDivider: i32,
#[allow(unused)]
	fogVolumeDepthScaler: i32,
#[allow(unused)]
	fogVolumeDepthDivider: i32,
#[allow(unused)]
	fogVolumeEnabled: u8,
#[allow(unused)]
	fogVolumeUpScaleType: u8,
#[allow(unused)]
	fogVolumeEdgeCorrectionLevel: u8,
#[allow(unused)]
	fogVolumeRayMarcingSampleCountOffset: i8,
#[allow(unused)]
	fogVolumeShadowEnabled: u8,
#[allow(unused)]
	fogVolumeForceShadowing: u8,
#[allow(unused)]
	fogVolumeResolution: u8,
#[allow(unused)]
	pad2: [u8;1],
}

struct CS_WATER_QUALITY_DETAIL {
#[allow(unused)]
	interactionEnabled: u8,
#[allow(unused)]
	dmy: [u8;3],
}

struct GESTURE_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	itemId: i32,
#[allow(unused)]
	msgAnimId: i32,
#[allow(unused)]
	cannotUseRiding: u8,
#[allow(unused)]
	pad2: [u8;1],
#[allow(unused)]
	pad1: [u8;3],
}

struct GPARAM_GRID_REGION_INFO_PARAM_ST {
#[allow(unused)]
	GparamGridRegionId: i32,
#[allow(unused)]
	Reserve: [u8;28],
}

struct GPARAM_REF_SETTINGS_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	RefTargetMapId: i32,
#[allow(unused)]
	Reserve: [u8;24],
}

struct GRAPHICS_COMMON_PARAM_ST {
#[allow(unused)]
	hitBulletDecalOffset_HitIns: f32,
#[allow(unused)]
	reserved02: [u8;8],
#[allow(unused)]
	charaWetDecalFadeRange: f32,
#[allow(unused)]
	reserved04: [u8;240],
}

struct CS_GRAPHICS_CONFIG_PARAM_ST {
#[allow(unused)]
	m_textureFilterQuality: u8,
#[allow(unused)]
	m_aaQuality: u8,
#[allow(unused)]
	m_ssaoQuality: u8,
#[allow(unused)]
	m_dofQuality: u8,
#[allow(unused)]
	m_motionBlurQuality: u8,
#[allow(unused)]
	m_shadowQuality: u8,
#[allow(unused)]
	m_lightingQuality: u8,
#[allow(unused)]
	m_effectQuality: u8,
#[allow(unused)]
	m_decalQuality: u8,
#[allow(unused)]
	m_reflectionQuality: u8,
#[allow(unused)]
	m_waterQuality: u8,
#[allow(unused)]
	m_shaderQuality: u8,
#[allow(unused)]
	m_volumetricEffectQuality: u8,
#[allow(unused)]
	m_dummy: [u8;3],
}

struct GRASS_LOD_RANGE_PARAM_ST {
#[allow(unused)]
	LOD0_range: f32,
#[allow(unused)]
	LOD0_play: f32,
#[allow(unused)]
	LOD1_range: f32,
#[allow(unused)]
	LOD1_play: f32,
#[allow(unused)]
	LOD2_range: f32,
#[allow(unused)]
	LOD2_play: f32,
}

struct GRASS_MAP_SETTINGS_PARAM_ST {
#[allow(unused)]
	grassType0: i32,
#[allow(unused)]
	grassType1: i32,
#[allow(unused)]
	grassType2: i32,
}

struct GRASS_TYPE_PARAM_ST {
#[allow(unused)]
	lodRange: i16,
#[allow(unused)]
	lod0ClusterType: u8,
#[allow(unused)]
	lod1ClusterType: u8,
#[allow(unused)]
	lod2ClusterType: u8,
#[allow(unused)]
	pad0: [u8;2],
#[allow(unused)]
	distributionType: u8,
#[allow(unused)]
	baseDensity: f32,
#[allow(unused)]
	model0Name: [u8;16],
#[allow(unused)]
	flatTextureName: [u8;32],
#[allow(unused)]
	billboardTextureName: [u8;32],
#[allow(unused)]
	normalInfluence: u8,
#[allow(unused)]
	inclinationMax: u8,
#[allow(unused)]
	inclinationJitter: u8,
#[allow(unused)]
	scaleBaseMin: u8,
#[allow(unused)]
	scaleBaseMax: u8,
#[allow(unused)]
	scaleHeightMin: u8,
#[allow(unused)]
	scaleHeightMax: u8,
#[allow(unused)]
	colorShade1_r: u8,
#[allow(unused)]
	colorShade1_g: u8,
#[allow(unused)]
	colorShade1_b: u8,
#[allow(unused)]
	colorShade2_r: u8,
#[allow(unused)]
	colorShade2_g: u8,
#[allow(unused)]
	colorShade2_b: u8,
#[allow(unused)]
	flatSplitType: u8,
#[allow(unused)]
	flatBladeCount: u8,
#[allow(unused)]
	flatSlant: i8,
#[allow(unused)]
	flatRadius: f32,
#[allow(unused)]
	castShadow: u8,
#[allow(unused)]
	windAmplitude: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	windCycle: u8,
#[allow(unused)]
	orientationAngle: f32,
#[allow(unused)]
	orientationRange: f32,
#[allow(unused)]
	spacing: f32,
#[allow(unused)]
	dithering: u8,
#[allow(unused)]
	pad: [u8;3],
#[allow(unused)]
	simpleModelName: [u8;16],
#[allow(unused)]
	model1Name: [u8;16],
}

struct HIT_EFFECT_SE_PARAM_ST {
#[allow(unused)]
	Iron_Slash_S: i32,
#[allow(unused)]
	Iron_Slash_L: i32,
#[allow(unused)]
	Iron_Slash_LL: i32,
#[allow(unused)]
	Iron_Thrust_S: i32,
#[allow(unused)]
	Iron_Thrust_L: i32,
#[allow(unused)]
	Iron_Thrust_LL: i32,
#[allow(unused)]
	Iron_Blow_S: i32,
#[allow(unused)]
	Iron_Blow_L: i32,
#[allow(unused)]
	Iron_Blow_LL: i32,
#[allow(unused)]
	Fire_Slash_S: i32,
#[allow(unused)]
	Fire_Slash_L: i32,
#[allow(unused)]
	Fire_Slash_LL: i32,
#[allow(unused)]
	Fire_Thrust_S: i32,
#[allow(unused)]
	Fire_Thrust_L: i32,
#[allow(unused)]
	Fire_Thrust_LL: i32,
#[allow(unused)]
	Fire_Blow_S: i32,
#[allow(unused)]
	Fire_Blow_L: i32,
#[allow(unused)]
	Fire_Blow_LL: i32,
#[allow(unused)]
	Wood_Slash_S: i32,
#[allow(unused)]
	Wood_Slash_L: i32,
#[allow(unused)]
	Wood_Slash_LL: i32,
#[allow(unused)]
	Wood_Thrust_S: i32,
#[allow(unused)]
	Wood_Thrust_L: i32,
#[allow(unused)]
	Wood_Thrust_LL: i32,
#[allow(unused)]
	Wood_Blow_S: i32,
#[allow(unused)]
	Wood_Blow_L: i32,
#[allow(unused)]
	Wood_Blow_LL: i32,
#[allow(unused)]
	Body_Slash_S: i32,
#[allow(unused)]
	Body_Slash_L: i32,
#[allow(unused)]
	Body_Slash_LL: i32,
#[allow(unused)]
	Body_Thrust_S: i32,
#[allow(unused)]
	Body_Thrust_L: i32,
#[allow(unused)]
	Body_Thrust_LL: i32,
#[allow(unused)]
	Body_Blow_S: i32,
#[allow(unused)]
	Body_Blow_L: i32,
#[allow(unused)]
	Body_Blow_LL: i32,
#[allow(unused)]
	Eclipse_Slash_S: i32,
#[allow(unused)]
	Eclipse_Slash_L: i32,
#[allow(unused)]
	Eclipse_Slash_LL: i32,
#[allow(unused)]
	Eclipse_Thrust_S: i32,
#[allow(unused)]
	Eclipse_Thrust_L: i32,
#[allow(unused)]
	Eclipse_Thrust_LL: i32,
#[allow(unused)]
	Eclipse_Blow_S: i32,
#[allow(unused)]
	Eclipse_Blow_L: i32,
#[allow(unused)]
	Eclipse_Blow_LL: i32,
#[allow(unused)]
	Energy_Slash_S: i32,
#[allow(unused)]
	Energy_Slash_L: i32,
#[allow(unused)]
	Energy_Slash_LL: i32,
#[allow(unused)]
	Energy_Thrust_S: i32,
#[allow(unused)]
	Energy_Thrust_L: i32,
#[allow(unused)]
	Energy_Thrust_LL: i32,
#[allow(unused)]
	Energy_Blow_S: i32,
#[allow(unused)]
	Energy_Blow_L: i32,
#[allow(unused)]
	Energy_Blow_LL: i32,
#[allow(unused)]
	None_Slash_S: i32,
#[allow(unused)]
	None_Slash_L: i32,
#[allow(unused)]
	None_Slash_LL: i32,
#[allow(unused)]
	None_Thrust_S: i32,
#[allow(unused)]
	None_Thrust_L: i32,
#[allow(unused)]
	None_Thrust_LL: i32,
#[allow(unused)]
	None_Blow_S: i32,
#[allow(unused)]
	None_Blow_L: i32,
#[allow(unused)]
	None_Blow_LL: i32,
#[allow(unused)]
	Dmy1_Slash_S: i32,
#[allow(unused)]
	Dmy1_Slash_L: i32,
#[allow(unused)]
	Dmy1_Slash_LL: i32,
#[allow(unused)]
	Dmy1_Thrust_S: i32,
#[allow(unused)]
	Dmy1_Thrust_L: i32,
#[allow(unused)]
	Dmy1_Thrust_LL: i32,
#[allow(unused)]
	Dmy1_Blow_S: i32,
#[allow(unused)]
	Dmy1_Blow_L: i32,
#[allow(unused)]
	Dmy1_Blow_LL: i32,
#[allow(unused)]
	Dmy2_Slash_S: i32,
#[allow(unused)]
	Dmy2_Slash_L: i32,
#[allow(unused)]
	Dmy2_Slash_LL: i32,
#[allow(unused)]
	Dmy2_Thrust_S: i32,
#[allow(unused)]
	Dmy2_Thrust_L: i32,
#[allow(unused)]
	Dmy2_Thrust_LL: i32,
#[allow(unused)]
	Dmy2_Blow_S: i32,
#[allow(unused)]
	Dmy2_Blow_L: i32,
#[allow(unused)]
	Dmy2_Blow_LL: i32,
#[allow(unused)]
	Dmy3_Slash_S: i32,
#[allow(unused)]
	Dmy3_Slash_L: i32,
#[allow(unused)]
	Dmy3_Slash_LL: i32,
#[allow(unused)]
	Dmy3_Thrust_S: i32,
#[allow(unused)]
	Dmy3_Thrust_L: i32,
#[allow(unused)]
	Dmy3_Thrust_LL: i32,
#[allow(unused)]
	Dmy3_Blow_S: i32,
#[allow(unused)]
	Dmy3_Blow_L: i32,
#[allow(unused)]
	Dmy3_Blow_LL: i32,
#[allow(unused)]
	Maggot_Slash_S: i32,
#[allow(unused)]
	Maggot_Slash_L: i32,
#[allow(unused)]
	Maggot_Slash_LL: i32,
#[allow(unused)]
	Maggot_Thrust_S: i32,
#[allow(unused)]
	Maggot_Thrust_L: i32,
#[allow(unused)]
	Maggot_Thrust_LL: i32,
#[allow(unused)]
	Maggot_Blow_S: i32,
#[allow(unused)]
	Maggot_Blow_L: i32,
#[allow(unused)]
	Maggot_Blow_LL: i32,
#[allow(unused)]
	Wax_Slash_S: i32,
#[allow(unused)]
	Wax_Slash_L: i32,
#[allow(unused)]
	Wax_Slash_LL: i32,
#[allow(unused)]
	Wax_Thrust_S: i32,
#[allow(unused)]
	Wax_Thrust_L: i32,
#[allow(unused)]
	Wax_Thrust_LL: i32,
#[allow(unused)]
	Wax_Blow_S: i32,
#[allow(unused)]
	Wax_Blow_L: i32,
#[allow(unused)]
	Wax_Blow_LL: i32,
#[allow(unused)]
	FireFlame_Slash_S: i32,
#[allow(unused)]
	FireFlame_Slash_L: i32,
#[allow(unused)]
	FireFlame_Slash_LL: i32,
#[allow(unused)]
	FireFlame_Thrust_S: i32,
#[allow(unused)]
	FireFlame_Thrust_L: i32,
#[allow(unused)]
	FireFlame_Thrust_LL: i32,
#[allow(unused)]
	FireFlame_Blow_S: i32,
#[allow(unused)]
	FireFlame_Blow_L: i32,
#[allow(unused)]
	FireFlame_Blow_LL: i32,
#[allow(unused)]
	EclipseGas_Slash_S: i32,
#[allow(unused)]
	EclipseGas_Slash_L: i32,
#[allow(unused)]
	EclipseGas_Slash_LL: i32,
#[allow(unused)]
	EclipseGas_Thrust_S: i32,
#[allow(unused)]
	EclipseGas_Thrust_L: i32,
#[allow(unused)]
	EclipseGas_Thrust_LL: i32,
#[allow(unused)]
	EclipseGas_Blow_S: i32,
#[allow(unused)]
	EclipseGas_Blow_L: i32,
#[allow(unused)]
	EclipseGas_Blow_LL: i32,
#[allow(unused)]
	EnergyStrong_Slash_S: i32,
#[allow(unused)]
	EnergyStrong_Slash_L: i32,
#[allow(unused)]
	EnergyStrong_Slash_LL: i32,
#[allow(unused)]
	EnergyStrong_Thrust_S: i32,
#[allow(unused)]
	EnergyStrong_Thrust_L: i32,
#[allow(unused)]
	EnergyStrong_Thrust_LL: i32,
#[allow(unused)]
	EnergyStrong_Blow_S: i32,
#[allow(unused)]
	EnergyStrong_Blow_L: i32,
#[allow(unused)]
	EnergyStrong_Blow_LL: i32,
#[allow(unused)]
	reserve: [u8;100],
}

struct HIT_EFFECT_SFX_CONCEPT_PARAM_ST {
#[allow(unused)]
	atkIron_1: i16,
#[allow(unused)]
	atkIron_2: i16,
#[allow(unused)]
	atkLeather_1: i16,
#[allow(unused)]
	atkLeather_2: i16,
#[allow(unused)]
	atkWood_1: i16,
#[allow(unused)]
	atkWood_2: i16,
#[allow(unused)]
	atkBody_1: i16,
#[allow(unused)]
	atkBody_2: i16,
#[allow(unused)]
	atkStone_1: i16,
#[allow(unused)]
	atkStone_2: i16,
#[allow(unused)]
	pad: [u8;4],
#[allow(unused)]
	atkNone_1: i16,
#[allow(unused)]
	atkNone_2: i16,
#[allow(unused)]
	reserve: [u8;52],
}

struct HIT_EFFECT_SFX_PARAM_ST {
#[allow(unused)]
	Slash_Normal: i32,
#[allow(unused)]
	Slash_S: i32,
#[allow(unused)]
	Slash_L: i32,
#[allow(unused)]
	Slash_Specific1: i32,
#[allow(unused)]
	Slash_Specific2: i32,
#[allow(unused)]
	Blow_Normal: i32,
#[allow(unused)]
	Blow_S: i32,
#[allow(unused)]
	Blow_L: i32,
#[allow(unused)]
	Blow_Specific1: i32,
#[allow(unused)]
	Blow_Specific2: i32,
#[allow(unused)]
	Thrust_Normal: i32,
#[allow(unused)]
	Thrust_S: i32,
#[allow(unused)]
	Thrust_L: i32,
#[allow(unused)]
	Thrust_Specific1: i32,
#[allow(unused)]
	Thrust_Specific2: i32,
#[allow(unused)]
	Neutral_Normal: i32,
#[allow(unused)]
	Neutral_S: i32,
#[allow(unused)]
	Neutral_L: i32,
#[allow(unused)]
	Neutral_Specific1: i32,
#[allow(unused)]
	Neutral_Specific2: i32,
}

struct HIT_MTRL_PARAM_ST {
#[allow(unused)]
	aiVolumeRate: f32,
#[allow(unused)]
	spEffectIdOnHit0: i32,
#[allow(unused)]
	spEffectIdOnHit1: i32,
#[allow(unused)]
	footEffectHeightType: u8,
#[allow(unused)]
	footEffectDirType: u8,
#[allow(unused)]
	floorHeightType: u8,
#[allow(unused)]
	disableFallDamage: u8,
#[allow(unused)]
	isHardnessForSoundReverb: u8,
#[allow(unused)]
	hardnessType: u8,
#[allow(unused)]
	pad2: [u8;6],
#[allow(unused)]
	spEffectIdOnHit0_ClearCount_2: i32,
#[allow(unused)]
	spEffectIdOnHit0_ClearCount_3: i32,
#[allow(unused)]
	spEffectIdOnHit0_ClearCount_4: i32,
#[allow(unused)]
	spEffectIdOnHit0_ClearCount_5: i32,
#[allow(unused)]
	spEffectIdOnHit0_ClearCount_6: i32,
#[allow(unused)]
	spEffectIdOnHit0_ClearCount_7: i32,
#[allow(unused)]
	spEffectIdOnHit0_ClearCount_8: i32,
#[allow(unused)]
	spEffectIdOnHit1_ClearCount_2: i32,
#[allow(unused)]
	spEffectIdOnHit1_ClearCount_3: i32,
#[allow(unused)]
	spEffectIdOnHit1_ClearCount_4: i32,
#[allow(unused)]
	spEffectIdOnHit1_ClearCount_5: i32,
#[allow(unused)]
	spEffectIdOnHit1_ClearCount_6: i32,
#[allow(unused)]
	spEffectIdOnHit1_ClearCount_7: i32,
#[allow(unused)]
	spEffectIdOnHit1_ClearCount_8: i32,
#[allow(unused)]
	replaceMateiralId_Rain: i16,
#[allow(unused)]
	pad4: [u8;2],
#[allow(unused)]
	spEffectId_forWet00: i32,
#[allow(unused)]
	spEffectId_forWet01: i32,
#[allow(unused)]
	spEffectId_forWet02: i32,
#[allow(unused)]
	spEffectId_forWet03: i32,
#[allow(unused)]
	spEffectId_forWet04: i32,
}

struct ITEMLOT_PARAM_ST {
#[allow(unused)]
	lotItemId01: i32,
#[allow(unused)]
	lotItemId02: i32,
#[allow(unused)]
	lotItemId03: i32,
#[allow(unused)]
	lotItemId04: i32,
#[allow(unused)]
	lotItemId05: i32,
#[allow(unused)]
	lotItemId06: i32,
#[allow(unused)]
	lotItemId07: i32,
#[allow(unused)]
	lotItemId08: i32,
#[allow(unused)]
	lotItemCategory01: i32,
#[allow(unused)]
	lotItemCategory02: i32,
#[allow(unused)]
	lotItemCategory03: i32,
#[allow(unused)]
	lotItemCategory04: i32,
#[allow(unused)]
	lotItemCategory05: i32,
#[allow(unused)]
	lotItemCategory06: i32,
#[allow(unused)]
	lotItemCategory07: i32,
#[allow(unused)]
	lotItemCategory08: i32,
#[allow(unused)]
	lotItemBasePoint01: i16,
#[allow(unused)]
	lotItemBasePoint02: i16,
#[allow(unused)]
	lotItemBasePoint03: i16,
#[allow(unused)]
	lotItemBasePoint04: i16,
#[allow(unused)]
	lotItemBasePoint05: i16,
#[allow(unused)]
	lotItemBasePoint06: i16,
#[allow(unused)]
	lotItemBasePoint07: i16,
#[allow(unused)]
	lotItemBasePoint08: i16,
#[allow(unused)]
	cumulateLotPoint01: i16,
#[allow(unused)]
	cumulateLotPoint02: i16,
#[allow(unused)]
	cumulateLotPoint03: i16,
#[allow(unused)]
	cumulateLotPoint04: i16,
#[allow(unused)]
	cumulateLotPoint05: i16,
#[allow(unused)]
	cumulateLotPoint06: i16,
#[allow(unused)]
	cumulateLotPoint07: i16,
#[allow(unused)]
	cumulateLotPoint08: i16,
#[allow(unused)]
	getItemFlagId01: i32,
#[allow(unused)]
	getItemFlagId02: i32,
#[allow(unused)]
	getItemFlagId03: i32,
#[allow(unused)]
	getItemFlagId04: i32,
#[allow(unused)]
	getItemFlagId05: i32,
#[allow(unused)]
	getItemFlagId06: i32,
#[allow(unused)]
	getItemFlagId07: i32,
#[allow(unused)]
	getItemFlagId08: i32,
#[allow(unused)]
	getItemFlagId: i32,
#[allow(unused)]
	cumulateNumFlagId: i32,
#[allow(unused)]
	cumulateNumMax: u8,
#[allow(unused)]
	lotItem_Rarity: i8,
#[allow(unused)]
	lotItemNum01: u8,
#[allow(unused)]
	lotItemNum02: u8,
#[allow(unused)]
	lotItemNum03: u8,
#[allow(unused)]
	lotItemNum04: u8,
#[allow(unused)]
	lotItemNum05: u8,
#[allow(unused)]
	lotItemNum06: u8,
#[allow(unused)]
	lotItemNum07: u8,
#[allow(unused)]
	lotItemNum08: u8,
#[allow(unused)]
	enableLuck01: i16,
#[allow(unused)]
	enableLuck02: i16,
#[allow(unused)]
	enableLuck03: i16,
#[allow(unused)]
	enableLuck04: i16,
#[allow(unused)]
	enableLuck05: i16,
#[allow(unused)]
	enableLuck06: i16,
#[allow(unused)]
	enableLuck07: i16,
#[allow(unused)]
	enableLuck08: i16,
#[allow(unused)]
	cumulateReset01: i16,
#[allow(unused)]
	cumulateReset02: i16,
#[allow(unused)]
	cumulateReset03: i16,
#[allow(unused)]
	cumulateReset04: i16,
#[allow(unused)]
	cumulateReset05: i16,
#[allow(unused)]
	cumulateReset06: i16,
#[allow(unused)]
	cumulateReset07: i16,
#[allow(unused)]
	cumulateReset08: i16,
#[allow(unused)]
	GameClearOffset: i8,
#[allow(unused)]
	canExecByFriendlyGhost: u8,
#[allow(unused)]
	canExecByHostileGhost: u8,
#[allow(unused)]
	PAD1: u8,
#[allow(unused)]
	PAD2: i16,
}

struct CS_KEY_ASSIGN_MENUITEM_PARAM {
#[allow(unused)]
	textID: i32,
#[allow(unused)]
	key: i32,
#[allow(unused)]
	enableUnassign: u8,
#[allow(unused)]
	enablePadConfig: u8,
#[allow(unused)]
	enableMouseConfig: u8,
#[allow(unused)]
	group: u8,
#[allow(unused)]
	mappingTextID: i32,
#[allow(unused)]
	viewPad: u8,
#[allow(unused)]
	viewKeyboardMouse: u8,
#[allow(unused)]
	padding: [u8;6],
}

struct KEY_ASSIGN_PARAM_ST {
#[allow(unused)]
	padKeyId: i32,
#[allow(unused)]
	keyboardModifyKey: i32,
#[allow(unused)]
	keyboardKeyId: i32,
#[allow(unused)]
	mouseModifyKey: i32,
#[allow(unused)]
	mouseKeyId: i32,
#[allow(unused)]
	reserved: [u8;12],
}

struct KNOCKBACK_PARAM_ST {
#[allow(unused)]
	damage_Min_ContTime: f32,
#[allow(unused)]
	damage_S_ContTime: f32,
#[allow(unused)]
	damage_M_ContTime: f32,
#[allow(unused)]
	damage_L_ContTime: f32,
#[allow(unused)]
	damage_BlowS_ContTime: f32,
#[allow(unused)]
	damage_BlowM_ContTime: f32,
#[allow(unused)]
	damage_Strike_ContTime: f32,
#[allow(unused)]
	damage_Uppercut_ContTime: f32,
#[allow(unused)]
	damage_Push_ContTime: f32,
#[allow(unused)]
	damage_Breath_ContTime: f32,
#[allow(unused)]
	damage_HeadShot_ContTime: f32,
#[allow(unused)]
	guard_S_ContTime: f32,
#[allow(unused)]
	guard_L_ContTime: f32,
#[allow(unused)]
	guard_LL_ContTime: f32,
#[allow(unused)]
	guardBrake_ContTime: f32,
#[allow(unused)]
	damage_Min_DecTime: f32,
#[allow(unused)]
	damage_S_DecTime: f32,
#[allow(unused)]
	damage_M_DecTime: f32,
#[allow(unused)]
	damage_L_DecTime: f32,
#[allow(unused)]
	damage_BlowS_DecTime: f32,
#[allow(unused)]
	damage_BlowM_DecTime: f32,
#[allow(unused)]
	damage_Strike_DecTime: f32,
#[allow(unused)]
	damage_Uppercut_DecTime: f32,
#[allow(unused)]
	damage_Push_DecTime: f32,
#[allow(unused)]
	damage_Breath_DecTime: f32,
#[allow(unused)]
	damage_HeadShot_DecTime: f32,
#[allow(unused)]
	guard_S_DecTime: f32,
#[allow(unused)]
	guard_L_DecTime: f32,
#[allow(unused)]
	guard_LL_DecTime: f32,
#[allow(unused)]
	guardBrake_DecTime: f32,
#[allow(unused)]
	pad: [u8;8],
}

struct KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	unlockFlagId: i32,
#[allow(unused)]
	invalidFlagId: i32,
#[allow(unused)]
	msgId: i32,
}

struct LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM {
#[allow(unused)]
	TargetMapId: i32,
#[allow(unused)]
	TargetEventId: i32,
#[allow(unused)]
	SrcAssetId: i32,
#[allow(unused)]
	SrcAssetPartsNo: i32,
#[allow(unused)]
	DstAssetId: i32,
#[allow(unused)]
	DstAssetPartsNo: i32,
#[allow(unused)]
	SrcAssetIdRangeMin: i32,
#[allow(unused)]
	SrcAssetIdRangeMax: i32,
#[allow(unused)]
	DstAssetIdRangeMin: i32,
#[allow(unused)]
	DstAssetIdRangeMax: i32,
#[allow(unused)]
	LimitedMapRegionId0: i8,
#[allow(unused)]
	LimitedMapRegionId1: i8,
#[allow(unused)]
	LimitedMapRegionId2: i8,
#[allow(unused)]
	LimitedMapRegionId3: i8,
#[allow(unused)]
	reserve: [u8;4],
#[allow(unused)]
	LimitedMapRegionAssetId: i32,
#[allow(unused)]
	LimitedMapRegioAssetPartsNo: i32,
#[allow(unused)]
	LimitedMapRegioAssetIdRangeMin: i32,
#[allow(unused)]
	LimitedMapRegioAssetIdRangeMax: i32,
}

struct LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST {
#[allow(unused)]
	Lv00: f32,
#[allow(unused)]
	Lv01: f32,
#[allow(unused)]
	Lv02: f32,
#[allow(unused)]
	Lv03: f32,
#[allow(unused)]
	Lv04: f32,
#[allow(unused)]
	Lv05: f32,
#[allow(unused)]
	Lv06: f32,
#[allow(unused)]
	Lv07: f32,
#[allow(unused)]
	Lv08: f32,
#[allow(unused)]
	Lv09: f32,
#[allow(unused)]
	Lv10: f32,
#[allow(unused)]
	Lv11: f32,
#[allow(unused)]
	Lv12: f32,
#[allow(unused)]
	Lv13: f32,
#[allow(unused)]
	Lv14: f32,
#[allow(unused)]
	Lv15: f32,
#[allow(unused)]
	Lv16: f32,
#[allow(unused)]
	Lv17: f32,
#[allow(unused)]
	Lv18: f32,
#[allow(unused)]
	Lv19: f32,
#[allow(unused)]
	Lv20: f32,
#[allow(unused)]
	reserve: [u8;44],
}

struct LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST {
#[allow(unused)]
	DrawDist_LvBegin: u8,
#[allow(unused)]
	DrawDist_LvEnd: u8,
#[allow(unused)]
	reserve0: [u8;2],
#[allow(unused)]
	DrawDist_ScaleBegin: f32,
#[allow(unused)]
	DrawDist_ScaleEnd: f32,
#[allow(unused)]
	ShadwDrawDist_LvBegin: u8,
#[allow(unused)]
	ShadwDrawDist_LvEnd: u8,
#[allow(unused)]
	reserve1: [u8;2],
#[allow(unused)]
	ShadwDrawDist_ScaleBegin: f32,
#[allow(unused)]
	ShadwDrawDist_ScaleEnd: f32,
#[allow(unused)]
	reserve2: [u8;24],
}

struct LOAD_BALANCER_PARAM_ST {
#[allow(unused)]
	lowerFpsThreshold: f32,
#[allow(unused)]
	upperFpsThreshold: f32,
#[allow(unused)]
	lowerFpsContinousCount: i32,
#[allow(unused)]
	upperFpsContinousCount: i32,
#[allow(unused)]
	downAfterChangeSleep: i32,
#[allow(unused)]
	upAfterChangeSleep: i32,
#[allow(unused)]
	postProcessLightShaft: u8,
#[allow(unused)]
	postProcessBloom: u8,
#[allow(unused)]
	postProcessGlow: u8,
#[allow(unused)]
	postProcessAA: u8,
#[allow(unused)]
	postProcessSSAO: u8,
#[allow(unused)]
	postProcessDOF: u8,
#[allow(unused)]
	postProcessMotionBlur: u8,
#[allow(unused)]
	postProcessMotionBlurIteration: u8,
#[allow(unused)]
	reserve0: [u8;1],
#[allow(unused)]
	shadowBlur: u8,
#[allow(unused)]
	sfxParticleHalf: u8,
#[allow(unused)]
	sfxReflection: u8,
#[allow(unused)]
	sfxWaterInteraction: u8,
#[allow(unused)]
	sfxGlow: u8,
#[allow(unused)]
	sfxDistortion: u8,
#[allow(unused)]
	sftSoftSprite: u8,
#[allow(unused)]
	sfxLightShaft: u8,
#[allow(unused)]
	sfxScaleRenderDistanceScale: u8,
#[allow(unused)]
	dynamicResolution: u8,
#[allow(unused)]
	shadowCascade0ResolutionHalf: u8,
#[allow(unused)]
	shadowCascade1ResolutionHalf: u8,
#[allow(unused)]
	chrWetDisablePlayer: u8,
#[allow(unused)]
	chrWetDisableRemotePlayer: u8,
#[allow(unused)]
	chrWetDisableEnemy: u8,
#[allow(unused)]
	dynamicResolutionPercentageMin: u8,
#[allow(unused)]
	dynamicResolutionPercentageMax: u8,
#[allow(unused)]
	reserve1: [u8;30],
}

struct LOCK_CAM_PARAM_ST {
#[allow(unused)]
	camDistTarget: f32,
#[allow(unused)]
	rotRangeMinX: f32,
#[allow(unused)]
	lockRotXShiftRatio: f32,
#[allow(unused)]
	chrOrgOffset_Y: f32,
#[allow(unused)]
	chrLockRangeMaxRadius: f32,
#[allow(unused)]
	camFovY: f32,
#[allow(unused)]
	chrLockRangeMaxRadius_forD: f32,
#[allow(unused)]
	chrLockRangeMaxRadius_forPD: f32,
#[allow(unused)]
	closeMaxHeight: f32,
#[allow(unused)]
	closeMinHeight: f32,
#[allow(unused)]
	closeAngRange: f32,
#[allow(unused)]
	closeMaxRadius: f32,
#[allow(unused)]
	closeMaxRadius_forD: f32,
#[allow(unused)]
	closeMaxRadius_forPD: f32,
#[allow(unused)]
	bulletMaxRadius: f32,
#[allow(unused)]
	bulletMaxRadius_forD: f32,
#[allow(unused)]
	bulletMaxRadius_forPD: f32,
#[allow(unused)]
	bulletAngRange: f32,
#[allow(unused)]
	lockTgtKeepTime: f32,
#[allow(unused)]
	chrTransChaseRateForNormal: f32,
#[allow(unused)]
	pad: [u8;48],
}

struct MAGIC_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	yesNoDialogMessageId: i32,
#[allow(unused)]
	limitCancelSpEffectId: i32,
#[allow(unused)]
	sortId: i16,
#[allow(unused)]
	requirementLuck: u8,
#[allow(unused)]
	aiNotifyType: u8,
#[allow(unused)]
	mp: i16,
#[allow(unused)]
	stamina: i16,
#[allow(unused)]
	iconId: i16,
#[allow(unused)]
	behaviorId: i16,
#[allow(unused)]
	mtrlItemId: i16,
#[allow(unused)]
	replaceMagicId: i16,
#[allow(unused)]
	maxQuantity: i16,
#[allow(unused)]
	refCategory1: u8,
#[allow(unused)]
	overDexterity: u8,
#[allow(unused)]
	refCategory2: u8,
#[allow(unused)]
	slotLength: u8,
#[allow(unused)]
	requirementIntellect: u8,
#[allow(unused)]
	requirementFaith: u8,
#[allow(unused)]
	analogDexterityMin: u8,
#[allow(unused)]
	analogDexterityMax: u8,
#[allow(unused)]
	ezStateBehaviorType: u8,
#[allow(unused)]
	refCategory3: u8,
#[allow(unused)]
	spEffectCategory: u8,
#[allow(unused)]
	refType: u8,
#[allow(unused)]
	opmeMenuType: u8,
#[allow(unused)]
	refCategory4: u8,
#[allow(unused)]
	hasSpEffectType: i16,
#[allow(unused)]
	replaceCategory: u8,
#[allow(unused)]
	useLimitCategory: u8,
#[allow(unused)]
	vowType0: u8,
#[allow(unused)]
	vowType1: u8,
#[allow(unused)]
	vowType2: u8,
#[allow(unused)]
	vowType3: u8,
#[allow(unused)]
	vowType4: u8,
#[allow(unused)]
	vowType5: u8,
#[allow(unused)]
	vowType6: u8,
#[allow(unused)]
	vowType7: u8,
#[allow(unused)]
	enable_multi: u8,
#[allow(unused)]
	enable_multi_only: u8,
#[allow(unused)]
	isEnchant: u8,
#[allow(unused)]
	isShieldEnchant: u8,
#[allow(unused)]
	enable_live: u8,
#[allow(unused)]
	enable_gray: u8,
#[allow(unused)]
	enable_white: u8,
#[allow(unused)]
	enable_black: u8,
#[allow(unused)]
	disableOffline: u8,
#[allow(unused)]
	castResonanceMagic: u8,
#[allow(unused)]
	isValidTough_ProtSADmg: u8,
#[allow(unused)]
	isWarpMagic: u8,
#[allow(unused)]
	enableRiding: u8,
#[allow(unused)]
	disableRiding: u8,
#[allow(unused)]
	isUseNoAttackRegion: u8,
#[allow(unused)]
	pad_1: [u8;1],
#[allow(unused)]
	vowType8: u8,
#[allow(unused)]
	vowType9: u8,
#[allow(unused)]
	vowType10: u8,
#[allow(unused)]
	vowType11: u8,
#[allow(unused)]
	vowType12: u8,
#[allow(unused)]
	vowType13: u8,
#[allow(unused)]
	vowType14: u8,
#[allow(unused)]
	vowType15: u8,
#[allow(unused)]
	castSfxId: i32,
#[allow(unused)]
	fireSfxId: i32,
#[allow(unused)]
	effectSfxId: i32,
#[allow(unused)]
	toughnessCorrectRate: f32,
#[allow(unused)]
	ReplacementStatusType: u8,
#[allow(unused)]
	ReplacementStatus1: i8,
#[allow(unused)]
	ReplacementStatus2: i8,
#[allow(unused)]
	ReplacementStatus3: i8,
#[allow(unused)]
	ReplacementStatus4: i8,
#[allow(unused)]
	refCategory5: u8,
#[allow(unused)]
	consumeSA: i16,
#[allow(unused)]
	ReplacementMagic1: i32,
#[allow(unused)]
	ReplacementMagic2: i32,
#[allow(unused)]
	ReplacementMagic3: i32,
#[allow(unused)]
	ReplacementMagic4: i32,
#[allow(unused)]
	mp_charge: i16,
#[allow(unused)]
	stamina_charge: i16,
#[allow(unused)]
	createLimitGroupId: u8,
#[allow(unused)]
	refCategory6: u8,
#[allow(unused)]
	subCategory1: u8,
#[allow(unused)]
	subCategory2: u8,
#[allow(unused)]
	refCategory7: u8,
#[allow(unused)]
	refCategory8: u8,
#[allow(unused)]
	refCategory9: u8,
#[allow(unused)]
	refCategory10: u8,
#[allow(unused)]
	refId1: i32,
#[allow(unused)]
	refId2: i32,
#[allow(unused)]
	refId3: i32,
#[allow(unused)]
	aiUseJudgeId: i32,
#[allow(unused)]
	refId4: i32,
#[allow(unused)]
	refId5: i32,
#[allow(unused)]
	refId6: i32,
#[allow(unused)]
	refId7: i32,
#[allow(unused)]
	refId8: i32,
#[allow(unused)]
	refId9: i32,
#[allow(unused)]
	refId10: i32,
#[allow(unused)]
	consumeType1: u8,
#[allow(unused)]
	consumeType2: u8,
#[allow(unused)]
	consumeType3: u8,
#[allow(unused)]
	consumeType4: u8,
#[allow(unused)]
	consumeType5: u8,
#[allow(unused)]
	consumeType6: u8,
#[allow(unused)]
	consumeType7: u8,
#[allow(unused)]
	consumeType8: u8,
#[allow(unused)]
	consumeType9: u8,
#[allow(unused)]
	consumeType10: u8,
#[allow(unused)]
	consumeLoopMP_forMenu: i16,
#[allow(unused)]
	pad: [u8;8],
}

struct MAP_DEFAULT_INFO_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	EnableFastTravelEventFlagId: i32,
#[allow(unused)]
	WeatherLotTimeOffsetIngameSeconds: i32,
#[allow(unused)]
	WeatherCreateAssetLimitId: i8,
#[allow(unused)]
	MapAiSightType: u8,
#[allow(unused)]
	SoundIndoorType: u8,
#[allow(unused)]
	ReverbDefaultType: i8,
#[allow(unused)]
	BgmPlaceInfo: i16,
#[allow(unused)]
	EnvPlaceInfo: i16,
#[allow(unused)]
	MapAdditionalSoundBankId: i32,
#[allow(unused)]
	MapHeightForSound: i16,
#[allow(unused)]
	IsEnableBlendTimezoneEnvmap: u8,
#[allow(unused)]
	OverrideGIResolution_XSS: i8,
#[allow(unused)]
	MapLoHiChangeBorderDist_XZ: f32,
#[allow(unused)]
	MapLoHiChangeBorderDist_Y: f32,
#[allow(unused)]
	MapLoHiChangePlayDist: f32,
#[allow(unused)]
	MapAutoDrawGroupBackFacePixelNum: i32,
#[allow(unused)]
	PlayerLigntScale: f32,
#[allow(unused)]
	IsEnableTimezonnePlayerLigntScale: u8,
#[allow(unused)]
	isDisableAutoCliffWind: u8,
#[allow(unused)]
	OpenChrActivateThreshold: i16,
#[allow(unused)]
	MapMimicryEstablishmentParamId: i32,
#[allow(unused)]
	OverrideGIResolution_XSX: i8,
#[allow(unused)]
	Reserve: [u8;7],
}

struct MAP_GD_REGION_DRAW_PARAM {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	overrideIVLocalLightScale: f32,
}

struct MAP_GD_REGION_ID_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	mapRegionId: i32,
#[allow(unused)]
	Reserve: [u8;24],
}

struct MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST {
#[allow(unused)]
	GridEnableCreateHeightMin: f32,
#[allow(unused)]
	GridEnableCreateHeightMax: f32,
#[allow(unused)]
	Reserve: [u8;24],
}

struct MAP_MIMICRY_ESTABLISHMENT_PARAM_ST {
#[allow(unused)]
	mimicryEstablishment0: f32,
#[allow(unused)]
	mimicryEstablishment1: f32,
#[allow(unused)]
	mimicryEstablishment2: f32,
#[allow(unused)]
	mimicryBeginSfxId0: i32,
#[allow(unused)]
	mimicrySfxId0: i32,
#[allow(unused)]
	mimicryEndSfxId0: i32,
#[allow(unused)]
	mimicryBeginSfxId1: i32,
#[allow(unused)]
	mimicrySfxId1: i32,
#[allow(unused)]
	mimicryEndSfxId1: i32,
#[allow(unused)]
	mimicryBeginSfxId2: i32,
#[allow(unused)]
	mimicrySfxId2: i32,
#[allow(unused)]
	mimicryEndSfxId2: i32,
#[allow(unused)]
	pad1: [u8;16],
}

struct MAP_NAME_TEX_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	srcR: u8,
#[allow(unused)]
	srcG: u8,
#[allow(unused)]
	srcB: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	mapNameId: i32,
#[allow(unused)]
	pad2: [u8;4],
}

struct MAP_PIECE_TEX_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	srcR: u8,
#[allow(unused)]
	srcG: u8,
#[allow(unused)]
	srcB: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	saveMapNameId: i32,
#[allow(unused)]
	multiPlayAreaId: i32,
}

struct MATERIAL_EX_PARAM_ST {
#[allow(unused)]
	paramName: [u8;32],
#[allow(unused)]
	materialId: i32,
#[allow(unused)]
	materialParamValue0: f32,
#[allow(unused)]
	materialParamValue1: f32,
#[allow(unused)]
	materialParamValue2: f32,
#[allow(unused)]
	materialParamValue3: f32,
#[allow(unused)]
	materialParamValue4: f32,
#[allow(unused)]
	pad: [u8;8],
}

struct MENU_COMMON_PARAM_ST {
#[allow(unused)]
	soloPlayDeath_ToFadeOutTime: f32,
#[allow(unused)]
	partyGhostDeath_ToFadeOutTime: f32,
#[allow(unused)]
	playerMaxHpLimit: i32,
#[allow(unused)]
	playerMaxMpLimit: i32,
#[allow(unused)]
	playerMaxSpLimit: i32,
#[allow(unused)]
	actionPanelChangeThreshold_Vel: f32,
#[allow(unused)]
	actionPanelChangeThreshold_PassTime: f32,
#[allow(unused)]
	kgIconVspace: i32,
#[allow(unused)]
	worldMapCursorSelectRadius: f32,
#[allow(unused)]
	reserved8: [u8;4],
#[allow(unused)]
	decalPosOffsetX: i32,
#[allow(unused)]
	decalPosOffsetY: i32,
#[allow(unused)]
	targetStateSearchDurationTime: f32,
#[allow(unused)]
	targetStateBattleDurationTime: f32,
#[allow(unused)]
	worldMapCursorSpeed: f32,
#[allow(unused)]
	worldMapCursorFirstDistance: f32,
#[allow(unused)]
	worldMapCursorFirstDelay: f32,
#[allow(unused)]
	worldMapCursorWaitTime: f32,
#[allow(unused)]
	worldMapCursorSnapRadius: f32,
#[allow(unused)]
	worldMapCursorSnapTime: f32,
#[allow(unused)]
	itemGetLogAliveTime: f32,
#[allow(unused)]
	playerMaxSaLimit: i32,
#[allow(unused)]
	worldMap_IsChangeableLayerEventFlagId: i32,
#[allow(unused)]
	worldMap_TravelMargin: f32,
#[allow(unused)]
	systemAnnounceScrollBufferTime: f32,
#[allow(unused)]
	systemAnnounceScrollSpeed: i32,
#[allow(unused)]
	systemAnnounceNoScrollWaitTime: f32,
#[allow(unused)]
	systemAnnounceScrollCount: u8,
#[allow(unused)]
	reserved17: [u8;3],
#[allow(unused)]
	compassMemoDispDistance: f32,
#[allow(unused)]
	compassBonfireDispDistance: f32,
#[allow(unused)]
	markerGoalThreshold: f32,
#[allow(unused)]
	svSliderStep: f32,
#[allow(unused)]
	preOpeningMovie_WaitSec: f32,
#[allow(unused)]
	kgIconScale: f32,
#[allow(unused)]
	kgIconScale_forTable: f32,
#[allow(unused)]
	kgIconVspace_forTable: i32,
#[allow(unused)]
	kgIconScale_forConfig: f32,
#[allow(unused)]
	kgIconVspace_forConfig: i32,
#[allow(unused)]
	worldMap_SearchRadius: f32,
#[allow(unused)]
	tutorialDisplayTime: f32,
#[allow(unused)]
	compassFriendHostInnerDistance: f32,
#[allow(unused)]
	compassEnemyHostInnerDistance: f32,
#[allow(unused)]
	compassFriendGuestInnerDistance: f32,
#[allow(unused)]
	cutsceneKeyGuideAliveTime: f32,
#[allow(unused)]
	autoHideHpThresholdRatio: f32,
#[allow(unused)]
	autoHideHpThresholdValue: i32,
#[allow(unused)]
	autoHideMpThresholdRatio: f32,
#[allow(unused)]
	autoHideMpThresholdValue: i32,
#[allow(unused)]
	autoHideSpThresholdRatio: f32,
#[allow(unused)]
	autoHideSpThresholdValue: i32,
#[allow(unused)]
	worldMapZoomAnimationTime: f32,
#[allow(unused)]
	worldMapIconScaleMin: f32,
#[allow(unused)]
	worldMap_TravelMargin_Point: f32,
#[allow(unused)]
	enemyTagSafeLeft: i16,
#[allow(unused)]
	enemyTagSafeRight: i16,
#[allow(unused)]
	enemyTagSafeTop: i16,
#[allow(unused)]
	enemyTagSafeBottom: i16,
#[allow(unused)]
	pcHorseHpRecoverDispThreshold: i32,
#[allow(unused)]
	reserved33: [u8;32],
}

struct MENU_OFFSCR_REND_PARAM_ST {
#[allow(unused)]
	camAtPosX: f32,
#[allow(unused)]
	camAtPosY: f32,
#[allow(unused)]
	camAtPosZ: f32,
#[allow(unused)]
	camDist: f32,
#[allow(unused)]
	camRotX: f32,
#[allow(unused)]
	camRotY: f32,
#[allow(unused)]
	camFov: f32,
#[allow(unused)]
	camDistMin: f32,
#[allow(unused)]
	camDistMax: f32,
#[allow(unused)]
	camRotXMin: f32,
#[allow(unused)]
	camRotXMax: f32,
#[allow(unused)]
	GparamID: i32,
#[allow(unused)]
	envTexId: i32,
#[allow(unused)]
	Grapm_ID_forPS4: i32,
#[allow(unused)]
	Grapm_ID_forXB1: i32,
#[allow(unused)]
	pad: [u8;4],
}

struct MENU_PARAM_COLOR_TABLE_ST {
#[allow(unused)]
	lerpMode: u8,
#[allow(unused)]
	pad1: [u8;3],
#[allow(unused)]
	h: i16,
#[allow(unused)]
	pad2: [u8;2],
#[allow(unused)]
	s1: f32,
#[allow(unused)]
	v1: f32,
#[allow(unused)]
	s2: f32,
#[allow(unused)]
	v2: f32,
#[allow(unused)]
	s3: f32,
#[allow(unused)]
	v3: f32,
}

struct MENUPROPERTY_LAYOUT {
#[allow(unused)]
	LayoutPath: [u8;16],
#[allow(unused)]
	PropertyID: i32,
#[allow(unused)]
	CaptionTextID: i32,
#[allow(unused)]
	HelpTextID: i32,
#[allow(unused)]
	reserved: [u8;4],
}

struct MENUPROPERTY_SPEC {
#[allow(unused)]
	CaptionTextID: i32,
#[allow(unused)]
	IconID: i32,
#[allow(unused)]
	RequiredPropertyID: i32,
#[allow(unused)]
	CompareType: i8,
#[allow(unused)]
	pad2: [u8;1],
#[allow(unused)]
	FormatType: i16,
#[allow(unused)]
	pad: [u8;16],
}

struct MENU_VALUE_TABLE_SPEC {
#[allow(unused)]
	value: i32,
#[allow(unused)]
	textId: i32,
#[allow(unused)]
	compareType: i8,
#[allow(unused)]
	padding: [u8;3],
}

struct MIMICRY_ESTABLISHMENT_TEX_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	srcR: u8,
#[allow(unused)]
	srcG: u8,
#[allow(unused)]
	srcB: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	mimicryEstablishmentParamId: i32,
#[allow(unused)]
	pad2: [u8;4],
}

struct MISSILE_PARAM_ST {
#[allow(unused)]
	FFXID: i32,
#[allow(unused)]
	LifeTime: i16,
#[allow(unused)]
	HitSphereRadius: i16,
#[allow(unused)]
	HitDamage: i16,
#[allow(unused)]
	reserve0: [u8;6],
#[allow(unused)]
	InitVelocity: f32,
#[allow(unused)]
	distance: f32,
#[allow(unused)]
	gravityInRange: f32,
#[allow(unused)]
	gravityOutRange: f32,
#[allow(unused)]
	mp: i32,
#[allow(unused)]
	accelInRange: f32,
#[allow(unused)]
	accelOutRange: f32,
#[allow(unused)]
	reserve1: [u8;20],
#[allow(unused)]
	HitMissileID: i16,
#[allow(unused)]
	DiedNaturaly: u8,
#[allow(unused)]
	ExplosionDie: u8,
#[allow(unused)]
	behaviorId: i32,
#[allow(unused)]
	reserve_last: [u8;56],
}

struct MODEL_SFX_PARAM_ST {
#[allow(unused)]
	sfxId_0: i32,
#[allow(unused)]
	dmypolyId_0: i32,
#[allow(unused)]
	reserve_0: [u8;8],
#[allow(unused)]
	sfxId_1: i32,
#[allow(unused)]
	dmypolyId_1: i32,
#[allow(unused)]
	reserve_1: [u8;8],
#[allow(unused)]
	sfxId_2: i32,
#[allow(unused)]
	dmypolyId_2: i32,
#[allow(unused)]
	reserve_2: [u8;8],
#[allow(unused)]
	sfxId_3: i32,
#[allow(unused)]
	dmypolyId_3: i32,
#[allow(unused)]
	reserve_3: [u8;8],
#[allow(unused)]
	sfxId_4: i32,
#[allow(unused)]
	dmypolyId_4: i32,
#[allow(unused)]
	reserve_4: [u8;8],
#[allow(unused)]
	sfxId_5: i32,
#[allow(unused)]
	dmypolyId_5: i32,
#[allow(unused)]
	reserve_5: [u8;8],
#[allow(unused)]
	sfxId_6: i32,
#[allow(unused)]
	dmypolyId_6: i32,
#[allow(unused)]
	reserve_6: [u8;8],
#[allow(unused)]
	sfxId_7: i32,
#[allow(unused)]
	dmypolyId_7: i32,
#[allow(unused)]
	reserve_7: [u8;8],
}

struct MOVE_PARAM_ST {
#[allow(unused)]
	stayId: i32,
#[allow(unused)]
	walkF: i32,
#[allow(unused)]
	walkB: i32,
#[allow(unused)]
	walkL: i32,
#[allow(unused)]
	walkR: i32,
#[allow(unused)]
	dashF: i32,
#[allow(unused)]
	dashB: i32,
#[allow(unused)]
	dashL: i32,
#[allow(unused)]
	dashR: i32,
#[allow(unused)]
	superDash: i32,
#[allow(unused)]
	escapeF: i32,
#[allow(unused)]
	escapeB: i32,
#[allow(unused)]
	escapeL: i32,
#[allow(unused)]
	escapeR: i32,
#[allow(unused)]
	turnL: i32,
#[allow(unused)]
	trunR: i32,
#[allow(unused)]
	largeTurnL: i32,
#[allow(unused)]
	largeTurnR: i32,
#[allow(unused)]
	stepMove: i32,
#[allow(unused)]
	flyStay: i32,
#[allow(unused)]
	flyWalkF: i32,
#[allow(unused)]
	flyWalkFL: i32,
#[allow(unused)]
	flyWalkFR: i32,
#[allow(unused)]
	flyWalkFL2: i32,
#[allow(unused)]
	flyWalkFR2: i32,
#[allow(unused)]
	flyDashF: i32,
#[allow(unused)]
	flyDashFL: i32,
#[allow(unused)]
	flyDashFR: i32,
#[allow(unused)]
	flyDashFL2: i32,
#[allow(unused)]
	flyDashFR2: i32,
#[allow(unused)]
	dashEscapeF: i32,
#[allow(unused)]
	dashEscapeB: i32,
#[allow(unused)]
	dashEscapeL: i32,
#[allow(unused)]
	dashEscapeR: i32,
#[allow(unused)]
	analogMoveParamId: i32,
#[allow(unused)]
	turnNoAnimAngle: u8,
#[allow(unused)]
	turn45Angle: u8,
#[allow(unused)]
	turn90Angle: u8,
#[allow(unused)]
	turnWaitNoAnimAngle: u8,
}

struct MULTI_ESTUS_FLASK_BONUS_PARAM_ST {
#[allow(unused)]
	host: u8,
#[allow(unused)]
	WhiteGhost_None: u8,
#[allow(unused)]
	WhiteGhost_Umbasa: u8,
#[allow(unused)]
	WhiteGhost_Berserker: u8,
#[allow(unused)]
	BlackGhost_None_Sign: u8,
#[allow(unused)]
	BlackGhost_Umbasa_Sign: u8,
#[allow(unused)]
	BlackGhost_Berserker_Sign: u8,
#[allow(unused)]
	BlackGhost_None_Invade: u8,
#[allow(unused)]
	BlackGhost_Umbasa_Invade: u8,
#[allow(unused)]
	BlackGhost_Berserker_Invade: u8,
#[allow(unused)]
	RedHunter1: u8,
#[allow(unused)]
	RedHunter2: u8,
#[allow(unused)]
	GuardianOfForest: u8,
#[allow(unused)]
	GuardianOfAnor: u8,
#[allow(unused)]
	BattleRoyal: u8,
#[allow(unused)]
	YellowMonk: u8,
#[allow(unused)]
	pad1: [u8;48],
}

struct MULTI_PLAY_CORRECTION_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	client1SpEffectId: i32,
#[allow(unused)]
	client2SpEffectId: i32,
#[allow(unused)]
	client3SpEffectId: i32,
#[allow(unused)]
	bOverrideSpEffect: u8,
#[allow(unused)]
	pad3: [u8;15],
}

struct MULTI_SOUL_BONUS_RATE_PARAM_ST {
#[allow(unused)]
	host: f32,
#[allow(unused)]
	WhiteGhost_None: f32,
#[allow(unused)]
	WhiteGhost_Umbasa: f32,
#[allow(unused)]
	WhiteGhost_Berserker: f32,
#[allow(unused)]
	BlackGhost_None_Sign: f32,
#[allow(unused)]
	BlackGhost_Umbasa_Sign: f32,
#[allow(unused)]
	BlackGhost_Berserker_Sign: f32,
#[allow(unused)]
	BlackGhost_None_Invade: f32,
#[allow(unused)]
	BlackGhost_Umbasa_Invade: f32,
#[allow(unused)]
	BlackGhost_Berserker_Invade: f32,
#[allow(unused)]
	RedHunter1: f32,
#[allow(unused)]
	RedHunter2: f32,
#[allow(unused)]
	GuardianOfForest: f32,
#[allow(unused)]
	GuardianOfAnor: f32,
#[allow(unused)]
	BattleRoyal: f32,
#[allow(unused)]
	YellowMonk: f32,
#[allow(unused)]
	pad1: [u8;64],
}

struct NETWORK_AREA_PARAM_ST {
#[allow(unused)]
	cellSizeX: f32,
#[allow(unused)]
	cellSizeY: f32,
#[allow(unused)]
	cellSizeZ: f32,
#[allow(unused)]
	cellOffsetX: f32,
#[allow(unused)]
	cellOffsetY: f32,
#[allow(unused)]
	cellOffsetZ: f32,
#[allow(unused)]
	enableBloodstain: u8,
#[allow(unused)]
	enableBloodMessage: u8,
#[allow(unused)]
	enableGhost: u8,
#[allow(unused)]
	enableMultiPlay: u8,
#[allow(unused)]
	enableRingSearch: u8,
#[allow(unused)]
	enableBreakInSearch: u8,
#[allow(unused)]
	dummy: [u8;3],
}

struct NETWORK_MSG_PARAM_ST {
#[allow(unused)]
	priority: i16,
#[allow(unused)]
	forcePlay: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	normalWhite: i32,
#[allow(unused)]
	umbasaWhite: i32,
#[allow(unused)]
	berserkerWhite: i32,
#[allow(unused)]
	sinnerHeroWhite: i32,
#[allow(unused)]
	normalBlack: i32,
#[allow(unused)]
	umbasaBlack: i32,
#[allow(unused)]
	berserkerBlack: i32,
#[allow(unused)]
	forceJoinBlack: i32,
#[allow(unused)]
	forceJoinUmbasaBlack: i32,
#[allow(unused)]
	forceJoinBerserkerBlack: i32,
#[allow(unused)]
	sinnerHunterVisitor: i32,
#[allow(unused)]
	redHunterVisitor: i32,
#[allow(unused)]
	guardianOfBossVisitor: i32,
#[allow(unused)]
	guardianOfForestMapVisitor: i32,
#[allow(unused)]
	guardianOfAnolisVisitor: i32,
#[allow(unused)]
	rosaliaBlack: i32,
#[allow(unused)]
	forceJoinRosaliaBlack: i32,
#[allow(unused)]
	redHunterVisitor2: i32,
#[allow(unused)]
	npc1: i32,
#[allow(unused)]
	npc2: i32,
#[allow(unused)]
	npc3: i32,
#[allow(unused)]
	npc4: i32,
#[allow(unused)]
	battleRoyal: i32,
#[allow(unused)]
	npc5: i32,
#[allow(unused)]
	npc6: i32,
#[allow(unused)]
	npc7: i32,
#[allow(unused)]
	npc8: i32,
#[allow(unused)]
	npc9: i32,
#[allow(unused)]
	npc10: i32,
#[allow(unused)]
	npc11: i32,
#[allow(unused)]
	npc12: i32,
#[allow(unused)]
	npc13: i32,
#[allow(unused)]
	npc14: i32,
#[allow(unused)]
	npc15: i32,
#[allow(unused)]
	npc16: i32,
#[allow(unused)]
	forceJoinBlack_B: i32,
#[allow(unused)]
	normalWhite_Npc: i32,
#[allow(unused)]
	forceJoinBlack_Npc: i32,
#[allow(unused)]
	forceJoinBlack_B_Npc: i32,
#[allow(unused)]
	forceJoinBlack_C_Npc: i32,
#[allow(unused)]
	pad2: [u8;28],
}

struct NETWORK_PARAM_ST {
#[allow(unused)]
	signVerticalOffset: f32,
#[allow(unused)]
	maxSignPosCorrectionRange: f32,
#[allow(unused)]
	summonTimeoutTime: f32,
#[allow(unused)]
	pad_0: [u8;4],
#[allow(unused)]
	signPuddleActiveMessageIntervalSec: f32,
#[allow(unused)]
	keyGuideHeight_0: f32,
#[allow(unused)]
	reloadSignIntervalTime1: f32,
#[allow(unused)]
	reloadSignIntervalTime2: f32,
#[allow(unused)]
	reloadSignTotalCount_0: i32,
#[allow(unused)]
	reloadSignCellCount_0: i32,
#[allow(unused)]
	updateSignIntervalTime: f32,
#[allow(unused)]
	basicExclusiveRange_0: f32,
#[allow(unused)]
	basicExclusiveHeight_0: f32,
#[allow(unused)]
	previewChrWaitingTime: f32,
#[allow(unused)]
	signVisibleRange_0: f32,
#[allow(unused)]
	cellGroupHorizontalRange_0: i32,
#[allow(unused)]
	cellGroupTopRange_0: i32,
#[allow(unused)]
	cellGroupBottomRange_0: i32,
#[allow(unused)]
	minWhitePhantomLimitTimeScale: f32,
#[allow(unused)]
	minSmallPhantomLimitTimeScale: f32,
#[allow(unused)]
	whiteKeywordLimitTimeScale: f32,
#[allow(unused)]
	smallKeywordLimitTimeScale: f32,
#[allow(unused)]
	blackKeywordLimitTimeScale: f32,
#[allow(unused)]
	dragonKeywordLimitTimeScale: f32,
#[allow(unused)]
	singGetMax: i32,
#[allow(unused)]
	signDownloadSpan: f32,
#[allow(unused)]
	signUpdateSpan: f32,
#[allow(unused)]
	signPad: [u8;4],
#[allow(unused)]
	maxBreakInTargetListCount: i32,
#[allow(unused)]
	breakInRequestIntervalTimeSec: f32,
#[allow(unused)]
	breakInRequestTimeOutSec: f32,
#[allow(unused)]
	pad_1: [u8;4],
#[allow(unused)]
	keyGuideRange: f32,
#[allow(unused)]
	keyGuideHeight_1: f32,
#[allow(unused)]
	reloadSignTotalCount_1: i32,
#[allow(unused)]
	reloadNewSignCellCount: i32,
#[allow(unused)]
	reloadRandomSignCellCount: i32,
#[allow(unused)]
	maxSignTotalCount_0: i32,
#[allow(unused)]
	maxSignCellCount_0: i32,
#[allow(unused)]
	basicExclusiveRange_1: f32,
#[allow(unused)]
	basicExclusiveHeight_1: f32,
#[allow(unused)]
	signVisibleRange_1: f32,
#[allow(unused)]
	maxWriteSignCount: i32,
#[allow(unused)]
	maxReadSignCount: i32,
#[allow(unused)]
	reloadSignIntervalTime_0: f32,
#[allow(unused)]
	cellGroupHorizontalRange_1: i32,
#[allow(unused)]
	cellGroupTopRange_1: i32,
#[allow(unused)]
	cellGroupBottomRange_1: i32,
#[allow(unused)]
	lifeTime_0: i32,
#[allow(unused)]
	downloadSpan_0: f32,
#[allow(unused)]
	downloadEvaluationSpan: f32,
#[allow(unused)]
	pad_2: [u8;4],
#[allow(unused)]
	deadingGhostStartPosThreshold: f32,
#[allow(unused)]
	keyGuideHeight_2: f32,
#[allow(unused)]
	keyGuideRangePlayer: f32,
#[allow(unused)]
	keyGuideHeightPlayer: f32,
#[allow(unused)]
	reloadSignTotalCount_2: i32,
#[allow(unused)]
	reloadSignCellCount_1: i32,
#[allow(unused)]
	maxSignTotalCount_1: i32,
#[allow(unused)]
	maxSignCellCount_1: i32,
#[allow(unused)]
	reloadSignIntervalTime_1: f32,
#[allow(unused)]
	signVisibleRange_2: f32,
#[allow(unused)]
	basicExclusiveRange_2: f32,
#[allow(unused)]
	basicExclusiveHeight_2: f32,
#[allow(unused)]
	cellGroupHorizontalRange_2: i32,
#[allow(unused)]
	cellGroupTopRange_2: i32,
#[allow(unused)]
	cellGroupBottomRange_2: i32,
#[allow(unused)]
	lifeTime_1: i32,
#[allow(unused)]
	recordDeadingGhostTotalTime: f32,
#[allow(unused)]
	recordDeadingGhostMinTime: f32,
#[allow(unused)]
	downloadSpan_1: f32,
#[allow(unused)]
	statueCreatableDistance: f32,
#[allow(unused)]
	reloadGhostTotalCount: i32,
#[allow(unused)]
	reloadGhostCellCount: i32,
#[allow(unused)]
	maxGhostTotalCount: i32,
#[allow(unused)]
	distanceOfBeginRecordVersus: f32,
#[allow(unused)]
	distanceOfEndRecordVersus: f32,
#[allow(unused)]
	updateWanderGhostIntervalTime: f32,
#[allow(unused)]
	updateVersusGhostIntervalTime: f32,
#[allow(unused)]
	recordWanderingGhostTime: f32,
#[allow(unused)]
	recordWanderingGhostMinTime: f32,
#[allow(unused)]
	updateBonfireGhostIntervalTime: f32,
#[allow(unused)]
	replayGhostRangeOnView: f32,
#[allow(unused)]
	replayGhostRangeOutView: f32,
#[allow(unused)]
	replayBonfireGhostTime: f32,
#[allow(unused)]
	minBonfireGhostValidRange: f32,
#[allow(unused)]
	maxBonfireGhostValidRange: f32,
#[allow(unused)]
	minReplayIntervalTime: f32,
#[allow(unused)]
	maxReplayIntervalTime: f32,
#[allow(unused)]
	reloadGhostIntervalTime: f32,
#[allow(unused)]
	cellGroupHorizontalRange_3: i32,
#[allow(unused)]
	cellGroupTopRange_3: i32,
#[allow(unused)]
	replayBonfirePhantomParamIdForCodename: i32,
#[allow(unused)]
	replayBonfireModeRange: f32,
#[allow(unused)]
	replayBonfirePhantomParamId: i32,
#[allow(unused)]
	ghostpad: [u8;4],
#[allow(unused)]
	reloadVisitListCoolTime: f32,
#[allow(unused)]
	maxCoopBlueSummonCount: i32,
#[allow(unused)]
	maxBellGuardSummonCount: i32,
#[allow(unused)]
	maxVisitListCount: i32,
#[allow(unused)]
	reloadSearch_CoopBlue_Min: f32,
#[allow(unused)]
	reloadSearch_CoopBlue_Max: f32,
#[allow(unused)]
	reloadSearch_BellGuard_Min: f32,
#[allow(unused)]
	reloadSearch_BellGuard_Max: f32,
#[allow(unused)]
	reloadSearch_RatKing_Min: f32,
#[allow(unused)]
	reloadSearch_RatKing_Max: f32,
#[allow(unused)]
	visitpad00: [u8;8],
#[allow(unused)]
	srttMaxLimit: f32,
#[allow(unused)]
	srttMeanLimit: f32,
#[allow(unused)]
	srttMeanDeviationLimit: f32,
#[allow(unused)]
	darkPhantomLimitBoostTime: f32,
#[allow(unused)]
	darkPhantomLimitBoostScale: f32,
#[allow(unused)]
	multiplayDisableLifeTime: f32,
#[allow(unused)]
	abyssMultiplayLimit: u8,
#[allow(unused)]
	phantomWarpMinimumTime: u8,
#[allow(unused)]
	phantomReturnDelayTime: u8,
#[allow(unused)]
	terminateTimeoutTime: u8,
#[allow(unused)]
	penaltyPointLanDisconnect: i16,
#[allow(unused)]
	penaltyPointSignout: i16,
#[allow(unused)]
	penaltyPointReboot: i16,
#[allow(unused)]
	penaltyPointBeginPenalize: i16,
#[allow(unused)]
	penaltyForgiveItemLimitTime: f32,
#[allow(unused)]
	allAreaSearchRate_CoopBlue: u8,
#[allow(unused)]
	allAreaSearchRate_VsBlue: u8,
#[allow(unused)]
	allAreaSearchRate_BellGuard: u8,
#[allow(unused)]
	bloodMessageEvalHealRate: u8,
#[allow(unused)]
	smallGoldSuccessHostRewardId: i32,
#[allow(unused)]
	doorInvalidPlayAreaExtents: f32,
#[allow(unused)]
	signDisplayMax: u8,
#[allow(unused)]
	bloodStainDisplayMax: u8,
#[allow(unused)]
	bloodMessageDisplayMax: u8,
#[allow(unused)]
	pad00: [u8;9],
#[allow(unused)]
	pad10: [u8;32],
#[allow(unused)]
	summonMessageInterval: f32,
#[allow(unused)]
	hostRegisterUpdateTime: f32,
#[allow(unused)]
	hostTimeOutTime: f32,
#[allow(unused)]
	guestUpdateTime: f32,
#[allow(unused)]
	guestPlayerNoTimeOutTime: f32,
#[allow(unused)]
	hostPlayerNoTimeOutTime: f32,
#[allow(unused)]
	requestSearchQuickMatchLimit: i32,
#[allow(unused)]
	AvatarMatchSearchMax: i32,
#[allow(unused)]
	BattleRoyalMatchSearchMin: i32,
#[allow(unused)]
	BattleRoyalMatchSearchMax: i32,
#[allow(unused)]
	pad11: [u8;8],
#[allow(unused)]
	VisitorListMax: i32,
#[allow(unused)]
	VisitorTimeOutTime: f32,
#[allow(unused)]
	DownloadSpan_2: f32,
#[allow(unused)]
	VisitorGuestRequestMessageIntervalSec: f32,
#[allow(unused)]
	wanderGhostIntervalLifeTime: f32,
#[allow(unused)]
	pad13: [u8;12],
#[allow(unused)]
	YellowMonkTimeOutTime: f32,
#[allow(unused)]
	YellowMonkDownloadSpan: f32,
#[allow(unused)]
	YellowMonkOverallFlowTimeOutTime: f32,
#[allow(unused)]
	pad14_0: [u8;4],
#[allow(unused)]
	pad14_1: [u8;8],
}

struct NPC_AI_ACTION_PARAM_ST {
#[allow(unused)]
	moveDir: u8,
#[allow(unused)]
	key1: u8,
#[allow(unused)]
	key2: u8,
#[allow(unused)]
	key3: u8,
#[allow(unused)]
	bMoveDirHold: u8,
#[allow(unused)]
	bKeyHold1: u8,
#[allow(unused)]
	bKeyHold2: u8,
#[allow(unused)]
	bKeyHold3: u8,
#[allow(unused)]
	gestureId: i32,
#[allow(unused)]
	bLifeEndSuccess: u8,
#[allow(unused)]
	pad1: [u8;3],
}

struct NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST {
#[allow(unused)]
	param000: i16,
#[allow(unused)]
	param001: i16,
#[allow(unused)]
	param002: i16,
#[allow(unused)]
	param003: i16,
#[allow(unused)]
	param004: i16,
#[allow(unused)]
	param005: i16,
#[allow(unused)]
	param006: i16,
#[allow(unused)]
	param007: i16,
#[allow(unused)]
	param008: i16,
#[allow(unused)]
	param009: i16,
#[allow(unused)]
	param010: i16,
#[allow(unused)]
	param011: i16,
#[allow(unused)]
	param012: i16,
#[allow(unused)]
	param013: i16,
#[allow(unused)]
	param014: i16,
#[allow(unused)]
	param015: i16,
#[allow(unused)]
	param016: i16,
#[allow(unused)]
	param017: i16,
#[allow(unused)]
	param018: i16,
#[allow(unused)]
	param019: i16,
#[allow(unused)]
	param020: i16,
#[allow(unused)]
	param021: i16,
#[allow(unused)]
	param022: i16,
#[allow(unused)]
	param023: i16,
#[allow(unused)]
	param024: i16,
#[allow(unused)]
	param025: i16,
#[allow(unused)]
	param026: i16,
#[allow(unused)]
	param027: i16,
#[allow(unused)]
	param028: i16,
#[allow(unused)]
	param029: i16,
#[allow(unused)]
	param030: i16,
#[allow(unused)]
	param031: i16,
#[allow(unused)]
	param032: i16,
#[allow(unused)]
	param033: i16,
#[allow(unused)]
	param034: i16,
#[allow(unused)]
	param035: i16,
#[allow(unused)]
	param036: i16,
#[allow(unused)]
	param037: i16,
#[allow(unused)]
	param038: i16,
#[allow(unused)]
	param039: i16,
#[allow(unused)]
	param040: i16,
#[allow(unused)]
	param041: i16,
#[allow(unused)]
	param042: i16,
#[allow(unused)]
	param043: i16,
#[allow(unused)]
	param044: i16,
#[allow(unused)]
	param045: i16,
#[allow(unused)]
	param046: i16,
#[allow(unused)]
	param047: i16,
#[allow(unused)]
	param048: i16,
#[allow(unused)]
	param049: i16,
#[allow(unused)]
	param050: i16,
#[allow(unused)]
	param051: i16,
#[allow(unused)]
	param052: i16,
#[allow(unused)]
	param053: i16,
#[allow(unused)]
	param054: i16,
#[allow(unused)]
	param055: i16,
#[allow(unused)]
	param056: i16,
#[allow(unused)]
	param057: i16,
#[allow(unused)]
	param058: i16,
#[allow(unused)]
	param059: i16,
#[allow(unused)]
	param060: i16,
#[allow(unused)]
	param061: i16,
#[allow(unused)]
	param062: i16,
#[allow(unused)]
	param063: i16,
#[allow(unused)]
	param064: i16,
#[allow(unused)]
	param065: i16,
#[allow(unused)]
	param066: i16,
#[allow(unused)]
	param067: i16,
#[allow(unused)]
	param068: i16,
#[allow(unused)]
	param069: i16,
#[allow(unused)]
	param070: i16,
#[allow(unused)]
	param071: i16,
#[allow(unused)]
	param072: i16,
#[allow(unused)]
	param073: i16,
#[allow(unused)]
	param074: i16,
#[allow(unused)]
	param075: i16,
#[allow(unused)]
	param076: i16,
#[allow(unused)]
	param077: i16,
#[allow(unused)]
	param078: i16,
#[allow(unused)]
	param079: i16,
#[allow(unused)]
	param080: i16,
#[allow(unused)]
	param081: i16,
#[allow(unused)]
	param082: i16,
#[allow(unused)]
	param083: i16,
#[allow(unused)]
	param084: i16,
#[allow(unused)]
	param085: i16,
#[allow(unused)]
	param086: i16,
#[allow(unused)]
	param087: i16,
#[allow(unused)]
	param088: i16,
#[allow(unused)]
	param089: i16,
#[allow(unused)]
	param090: i16,
#[allow(unused)]
	param091: i16,
#[allow(unused)]
	param092: i16,
#[allow(unused)]
	param093: i16,
#[allow(unused)]
	param094: i16,
#[allow(unused)]
	param095: i16,
#[allow(unused)]
	param096: i16,
#[allow(unused)]
	param097: i16,
#[allow(unused)]
	param098: i16,
#[allow(unused)]
	param099: i16,
#[allow(unused)]
	param100: i16,
#[allow(unused)]
	param101: i16,
#[allow(unused)]
	param102: i16,
#[allow(unused)]
	param103: i16,
#[allow(unused)]
	param104: i16,
#[allow(unused)]
	param105: i16,
#[allow(unused)]
	param106: i16,
#[allow(unused)]
	param107: i16,
#[allow(unused)]
	param108: i16,
#[allow(unused)]
	param109: i16,
#[allow(unused)]
	param110: i16,
#[allow(unused)]
	param111: i16,
#[allow(unused)]
	param112: i16,
#[allow(unused)]
	param113: i16,
#[allow(unused)]
	param114: i16,
#[allow(unused)]
	param115: i16,
#[allow(unused)]
	param116: i16,
#[allow(unused)]
	param117: i16,
#[allow(unused)]
	param118: i16,
#[allow(unused)]
	param119: i16,
#[allow(unused)]
	param120: i16,
#[allow(unused)]
	param121: i16,
#[allow(unused)]
	param122: i16,
#[allow(unused)]
	param123: i16,
#[allow(unused)]
	param124: i16,
#[allow(unused)]
	param125: i16,
#[allow(unused)]
	param126: i16,
#[allow(unused)]
	param127: i16,
#[allow(unused)]
	param128: i16,
#[allow(unused)]
	param129: i16,
#[allow(unused)]
	param130: i16,
#[allow(unused)]
	param131: i16,
#[allow(unused)]
	param132: i16,
#[allow(unused)]
	param133: i16,
#[allow(unused)]
	param134: i16,
#[allow(unused)]
	param135: i16,
#[allow(unused)]
	param136: i16,
#[allow(unused)]
	param137: i16,
#[allow(unused)]
	param138: i16,
#[allow(unused)]
	param139: i16,
#[allow(unused)]
	param140: i16,
#[allow(unused)]
	param141: i16,
#[allow(unused)]
	param142: i16,
#[allow(unused)]
	param143: i16,
#[allow(unused)]
	param144: i16,
#[allow(unused)]
	param145: i16,
#[allow(unused)]
	param146: i16,
#[allow(unused)]
	param147: i16,
#[allow(unused)]
	param148: i16,
#[allow(unused)]
	param149: i16,
#[allow(unused)]
	param150: i16,
#[allow(unused)]
	param151: i16,
#[allow(unused)]
	param152: i16,
#[allow(unused)]
	param153: i16,
#[allow(unused)]
	param154: i16,
#[allow(unused)]
	param155: i16,
#[allow(unused)]
	param156: i16,
#[allow(unused)]
	param157: i16,
#[allow(unused)]
	param158: i16,
#[allow(unused)]
	param159: i16,
#[allow(unused)]
	param160: i16,
#[allow(unused)]
	param161: i16,
#[allow(unused)]
	param162: i16,
#[allow(unused)]
	param163: i16,
#[allow(unused)]
	param164: i16,
#[allow(unused)]
	param165: i16,
#[allow(unused)]
	param166: i16,
#[allow(unused)]
	param167: i16,
#[allow(unused)]
	param168: i16,
#[allow(unused)]
	param169: i16,
#[allow(unused)]
	param170: i16,
#[allow(unused)]
	param171: i16,
#[allow(unused)]
	param172: i16,
#[allow(unused)]
	param173: i16,
#[allow(unused)]
	param174: i16,
#[allow(unused)]
	param175: i16,
#[allow(unused)]
	param176: i16,
#[allow(unused)]
	param177: i16,
#[allow(unused)]
	param178: i16,
#[allow(unused)]
	param179: i16,
#[allow(unused)]
	param180: i16,
#[allow(unused)]
	param181: i16,
#[allow(unused)]
	param182: i16,
#[allow(unused)]
	param183: i16,
#[allow(unused)]
	param184: i16,
#[allow(unused)]
	param185: i16,
#[allow(unused)]
	param186: i16,
#[allow(unused)]
	param187: i16,
#[allow(unused)]
	param188: i16,
#[allow(unused)]
	param189: i16,
#[allow(unused)]
	param190: i16,
#[allow(unused)]
	param191: i16,
#[allow(unused)]
	param192: i16,
#[allow(unused)]
	param193: i16,
#[allow(unused)]
	param194: i16,
#[allow(unused)]
	param195: i16,
#[allow(unused)]
	param196: i16,
#[allow(unused)]
	param197: i16,
#[allow(unused)]
	param198: i16,
#[allow(unused)]
	param199: i16,
}

struct NPC_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	behaviorVariationId: i32,
#[allow(unused)]
	resistCorrectId_poison: i32,
#[allow(unused)]
	nameId: i32,
#[allow(unused)]
	turnVellocity: f32,
#[allow(unused)]
	hitHeight: f32,
#[allow(unused)]
	hitRadius: f32,
#[allow(unused)]
	weight: i32,
#[allow(unused)]
	hitYOffset: f32,
#[allow(unused)]
	hp: i32,
#[allow(unused)]
	mp: i32,
#[allow(unused)]
	getSoul: i32,
#[allow(unused)]
	itemLotId_enemy: i32,
#[allow(unused)]
	itemLotId_map: i32,
#[allow(unused)]
	maxAnkleRollAngle: f32,
#[allow(unused)]
	chrHitGroupAndNavimesh: u8,
#[allow(unused)]
	faceIconId: u8,
#[allow(unused)]
	deactivateDist: i16,
#[allow(unused)]
	chrActivateConditionParamId: i32,
#[allow(unused)]
	footIkErrorHeightLimit: f32,
#[allow(unused)]
	humanityLotId: i32,
#[allow(unused)]
	spEffectID0: i32,
#[allow(unused)]
	spEffectID1: i32,
#[allow(unused)]
	spEffectID2: i32,
#[allow(unused)]
	spEffectID3: i32,
#[allow(unused)]
	spEffectID4: i32,
#[allow(unused)]
	spEffectID5: i32,
#[allow(unused)]
	spEffectID6: i32,
#[allow(unused)]
	spEffectID7: i32,
#[allow(unused)]
	GameClearSpEffectID: i32,
#[allow(unused)]
	physGuardCutRate: f32,
#[allow(unused)]
	magGuardCutRate: f32,
#[allow(unused)]
	fireGuardCutRate: f32,
#[allow(unused)]
	thunGuardCutRate: f32,
#[allow(unused)]
	animIdOffset: i32,
#[allow(unused)]
	lockGazePoint0: i16,
#[allow(unused)]
	lockGazePoint1: i16,
#[allow(unused)]
	lockGazePoint2: i16,
#[allow(unused)]
	lockGazePoint3: i16,
#[allow(unused)]
	lockGazePoint4: i16,
#[allow(unused)]
	lockGazePoint5: i16,
#[allow(unused)]
	networkWarpDist: f32,
#[allow(unused)]
	dbgBehaviorR1: i32,
#[allow(unused)]
	dbgBehaviorL1: i32,
#[allow(unused)]
	dbgBehaviorR2: i32,
#[allow(unused)]
	dbgBehaviorL2: i32,
#[allow(unused)]
	dbgBehaviorRL: i32,
#[allow(unused)]
	dbgBehaviorRR: i32,
#[allow(unused)]
	dbgBehaviorRD: i32,
#[allow(unused)]
	dbgBehaviorRU: i32,
#[allow(unused)]
	dbgBehaviorLL: i32,
#[allow(unused)]
	dbgBehaviorLR: i32,
#[allow(unused)]
	dbgBehaviorLD: i32,
#[allow(unused)]
	dbgBehaviorLU: i32,
#[allow(unused)]
	animIdOffset2: i32,
#[allow(unused)]
	partsDamageRate1: f32,
#[allow(unused)]
	partsDamageRate2: f32,
#[allow(unused)]
	partsDamageRate3: f32,
#[allow(unused)]
	partsDamageRate4: f32,
#[allow(unused)]
	partsDamageRate5: f32,
#[allow(unused)]
	partsDamageRate6: f32,
#[allow(unused)]
	partsDamageRate7: f32,
#[allow(unused)]
	partsDamageRate8: f32,
#[allow(unused)]
	weakPartsDamageRate: f32,
#[allow(unused)]
	superArmorRecoverCorrection: f32,
#[allow(unused)]
	superArmorBrakeKnockbackDist: f32,
#[allow(unused)]
	stamina: i16,
#[allow(unused)]
	staminaRecoverBaseVel: i16,
#[allow(unused)]
	def_phys: i16,
#[allow(unused)]
	def_slash: i16,
#[allow(unused)]
	def_blow: i16,
#[allow(unused)]
	def_thrust: i16,
#[allow(unused)]
	def_mag: i16,
#[allow(unused)]
	def_fire: i16,
#[allow(unused)]
	def_thunder: i16,
#[allow(unused)]
	defFlickPower: i16,
#[allow(unused)]
	resist_poison: i16,
#[allow(unused)]
	resist_desease: i16,
#[allow(unused)]
	resist_blood: i16,
#[allow(unused)]
	resist_curse: i16,
#[allow(unused)]
	ghostModelId: i16,
#[allow(unused)]
	normalChangeResouceId: i16,
#[allow(unused)]
	guardAngle: i16,
#[allow(unused)]
	slashGuardCutRate: i16,
#[allow(unused)]
	blowGuardCutRate: i16,
#[allow(unused)]
	thrustGuardCutRate: i16,
#[allow(unused)]
	lockGazePoint6: i16,
#[allow(unused)]
	normalChangeTexChrId: i16,
#[allow(unused)]
	dropType: i16,
#[allow(unused)]
	knockbackRate: u8,
#[allow(unused)]
	knockbackParamId: u8,
#[allow(unused)]
	fallDamageDump: u8,
#[allow(unused)]
	staminaGuardDef: u8,
#[allow(unused)]
	resist_sleep: i16,
#[allow(unused)]
	resist_madness: i16,
#[allow(unused)]
	sleepGuardResist: i8,
#[allow(unused)]
	madnessGuardResist: i8,
#[allow(unused)]
	lockGazePoint7: i16,
#[allow(unused)]
	mpRecoverBaseVel: u8,
#[allow(unused)]
	flickDamageCutRate: u8,
#[allow(unused)]
	defaultLodParamId: i8,
#[allow(unused)]
	drawType: i8,
#[allow(unused)]
	npcType: u8,
#[allow(unused)]
	teamType: u8,
#[allow(unused)]
	moveType: u8,
#[allow(unused)]
	lockDist: u8,
#[allow(unused)]
	materialSe_Weak1: i16,
#[allow(unused)]
	materialSfx_Weak1: i16,
#[allow(unused)]
	partsDamageType: u8,
#[allow(unused)]
	vowType: u8,
#[allow(unused)]
	guardLevel: i8,
#[allow(unused)]
	burnSfxType: u8,
#[allow(unused)]
	poisonGuardResist: i8,
#[allow(unused)]
	diseaseGuardResist: i8,
#[allow(unused)]
	bloodGuardResist: i8,
#[allow(unused)]
	curseGuardResist: i8,
#[allow(unused)]
	parryAttack: u8,
#[allow(unused)]
	parryDefence: u8,
#[allow(unused)]
	sfxSize: u8,
#[allow(unused)]
	pushOutCamRegionRadius: u8,
#[allow(unused)]
	hitStopType: u8,
#[allow(unused)]
	ladderEndChkOffsetTop: u8,
#[allow(unused)]
	ladderEndChkOffsetLow: u8,
#[allow(unused)]
	useRagdollCamHit: u8,
#[allow(unused)]
	disableClothRigidHit: u8,
#[allow(unused)]
	useUndulationAddAnimFB: u8,
#[allow(unused)]
	isWeakA: u8,
#[allow(unused)]
	isGhost: u8,
#[allow(unused)]
	isNoDamageMotion: u8,
#[allow(unused)]
	isUnduration: u8,
#[allow(unused)]
	isChangeWanderGhost: u8,
#[allow(unused)]
	modelDispMask0: u8,
#[allow(unused)]
	modelDispMask1: u8,
#[allow(unused)]
	modelDispMask2: u8,
#[allow(unused)]
	modelDispMask3: u8,
#[allow(unused)]
	modelDispMask4: u8,
#[allow(unused)]
	modelDispMask5: u8,
#[allow(unused)]
	modelDispMask6: u8,
#[allow(unused)]
	modelDispMask7: u8,
#[allow(unused)]
	modelDispMask8: u8,
#[allow(unused)]
	modelDispMask9: u8,
#[allow(unused)]
	modelDispMask10: u8,
#[allow(unused)]
	modelDispMask11: u8,
#[allow(unused)]
	modelDispMask12: u8,
#[allow(unused)]
	modelDispMask13: u8,
#[allow(unused)]
	modelDispMask14: u8,
#[allow(unused)]
	modelDispMask15: u8,
#[allow(unused)]
	isEnableNeckTurn: u8,
#[allow(unused)]
	disableRespawn: u8,
#[allow(unused)]
	isMoveAnimWait: u8,
#[allow(unused)]
	isCrowd: u8,
#[allow(unused)]
	isWeakB: u8,
#[allow(unused)]
	isWeakC: u8,
#[allow(unused)]
	isWeakD: u8,
#[allow(unused)]
	doesAlwaysUseSpecialTurn: u8,
#[allow(unused)]
	isRideAtkTarget: u8,
#[allow(unused)]
	isEnableStepDispInterpolate: u8,
#[allow(unused)]
	isStealthTarget: u8,
#[allow(unused)]
	disableInitializeDead: u8,
#[allow(unused)]
	isHitRumble: u8,
#[allow(unused)]
	isSmoothTurn: u8,
#[allow(unused)]
	isWeakE: u8,
#[allow(unused)]
	isWeakF: u8,
#[allow(unused)]
	modelDispMask16: u8,
#[allow(unused)]
	modelDispMask17: u8,
#[allow(unused)]
	modelDispMask18: u8,
#[allow(unused)]
	modelDispMask19: u8,
#[allow(unused)]
	modelDispMask20: u8,
#[allow(unused)]
	modelDispMask21: u8,
#[allow(unused)]
	modelDispMask22: u8,
#[allow(unused)]
	modelDispMask23: u8,
#[allow(unused)]
	modelDispMask24: u8,
#[allow(unused)]
	modelDispMask25: u8,
#[allow(unused)]
	modelDispMask26: u8,
#[allow(unused)]
	modelDispMask27: u8,
#[allow(unused)]
	modelDispMask28: u8,
#[allow(unused)]
	modelDispMask29: u8,
#[allow(unused)]
	modelDispMask30: u8,
#[allow(unused)]
	modelDispMask31: u8,
#[allow(unused)]
	itemSearchRadius: f32,
#[allow(unused)]
	chrHitHeight: f32,
#[allow(unused)]
	chrHitRadius: f32,
#[allow(unused)]
	specialTurnType: u8,
#[allow(unused)]
	isSoulGetByBoss: u8,
#[allow(unused)]
	isBulletOwner_byObject: u8,
#[allow(unused)]
	isUseLowHitFootIk: u8,
#[allow(unused)]
	isCalculatePvPDamage: u8,
#[allow(unused)]
	isHostSyncChr: u8,
#[allow(unused)]
	isSkipWeakDamageAnim: u8,
#[allow(unused)]
	isKeepHitOnRide: u8,
#[allow(unused)]
	isSpCollide: u8,
#[allow(unused)]
	def_dark: i16,
#[allow(unused)]
	threatLv: i32,
#[allow(unused)]
	specialTurnDistanceThreshold: f32,
#[allow(unused)]
	autoFootEffectSfxId: i32,
#[allow(unused)]
	materialSe1: i16,
#[allow(unused)]
	materialSfx1: i16,
#[allow(unused)]
	materialSe_Weak2: i16,
#[allow(unused)]
	materialSfx_Weak2: i16,
#[allow(unused)]
	materialSe2: i16,
#[allow(unused)]
	materialSfx2: i16,
#[allow(unused)]
	spEffectID8: i32,
#[allow(unused)]
	spEffectID9: i32,
#[allow(unused)]
	spEffectID10: i32,
#[allow(unused)]
	spEffectID11: i32,
#[allow(unused)]
	spEffectID12: i32,
#[allow(unused)]
	spEffectID13: i32,
#[allow(unused)]
	spEffectID14: i32,
#[allow(unused)]
	spEffectID15: i32,
#[allow(unused)]
	autoFootEffectDecalBaseId1: i32,
#[allow(unused)]
	toughness: i32,
#[allow(unused)]
	toughnessRecoverCorrection: f32,
#[allow(unused)]
	neutralDamageCutRate: f32,
#[allow(unused)]
	slashDamageCutRate: f32,
#[allow(unused)]
	blowDamageCutRate: f32,
#[allow(unused)]
	thrustDamageCutRate: f32,
#[allow(unused)]
	magicDamageCutRate: f32,
#[allow(unused)]
	fireDamageCutRate: f32,
#[allow(unused)]
	thunderDamageCutRate: f32,
#[allow(unused)]
	darkDamageCutRate: f32,
#[allow(unused)]
	darkGuardCutRate: f32,
#[allow(unused)]
	clothUpdateOffset: i8,
#[allow(unused)]
	npcPlayerWeightType: u8,
#[allow(unused)]
	normalChangeModelId: i16,
#[allow(unused)]
	normalChangeAnimChrId: i16,
#[allow(unused)]
	paintRenderTargetSize: i16,
#[allow(unused)]
	resistCorrectId_disease: i32,
#[allow(unused)]
	phantomShaderId: i32,
#[allow(unused)]
	multiPlayCorrectionParamId: i32,
#[allow(unused)]
	maxAnklePitchAngle: f32,
#[allow(unused)]
	resist_freeze: i16,
#[allow(unused)]
	freezeGuardResist: i8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	lockCameraParamId: i32,
#[allow(unused)]
	spEffectID16: i32,
#[allow(unused)]
	spEffectID17: i32,
#[allow(unused)]
	spEffectID18: i32,
#[allow(unused)]
	spEffectID19: i32,
#[allow(unused)]
	spEffectID20: i32,
#[allow(unused)]
	spEffectID21: i32,
#[allow(unused)]
	spEffectID22: i32,
#[allow(unused)]
	spEffectID23: i32,
#[allow(unused)]
	spEffectID24: i32,
#[allow(unused)]
	spEffectID25: i32,
#[allow(unused)]
	spEffectID26: i32,
#[allow(unused)]
	spEffectID27: i32,
#[allow(unused)]
	spEffectID28: i32,
#[allow(unused)]
	spEffectID29: i32,
#[allow(unused)]
	spEffectID30: i32,
#[allow(unused)]
	spEffectID31: i32,
#[allow(unused)]
	disableLockOnAng: f32,
#[allow(unused)]
	clothOffLodLevel: i8,
#[allow(unused)]
	isUseFootIKNormalByUnduration: u8,
#[allow(unused)]
	attachHitInitializeDead: u8,
#[allow(unused)]
	excludeGroupRewardCheck: u8,
#[allow(unused)]
	enableAILockDmyPoly_212: u8,
#[allow(unused)]
	enableAILockDmyPoly_213: u8,
#[allow(unused)]
	enableAILockDmyPoly_214: u8,
#[allow(unused)]
	disableActivateOpen_xb1: u8,
#[allow(unused)]
	disableActivateLegacy_xb1: u8,
#[allow(unused)]
	estusFlaskRecoveryParamId: i16,
#[allow(unused)]
	roleNameId: i32,
#[allow(unused)]
	estusFlaskLotPoint: i16,
#[allow(unused)]
	hpEstusFlaskLotPoint: i16,
#[allow(unused)]
	mpEstusFlaskLotPoint: i16,
#[allow(unused)]
	estusFlaskRecovery_failedLotPointAdd: i16,
#[allow(unused)]
	hpEstusFlaskRecovery_failedLotPointAdd: i16,
#[allow(unused)]
	mpEstusFlaskRecovery_failedLotPointAdd: i16,
#[allow(unused)]
	WanderGhostPhantomId: i32,
#[allow(unused)]
	hearingHeadSize: f32,
#[allow(unused)]
	SoundBankId: i16,
#[allow(unused)]
	forwardUndulationLimit: u8,
#[allow(unused)]
	sideUndulationLimit: u8,
#[allow(unused)]
	deactiveMoveSpeed: f32,
#[allow(unused)]
	deactiveMoveDist: f32,
#[allow(unused)]
	enableSoundObjDist: f32,
#[allow(unused)]
	undulationCorrectGain: f32,
#[allow(unused)]
	autoFootEffectDecalBaseId2: i16,
#[allow(unused)]
	autoFootEffectDecalBaseId3: i16,
#[allow(unused)]
	RetargetReferenceChrId: i16,
#[allow(unused)]
	SfxResBankId: i16,
#[allow(unused)]
	updateActivatePriolity: f32,
#[allow(unused)]
	chrNavimeshFlag_Alive: u8,
#[allow(unused)]
	chrNavimeshFlag_Dead: u8,
#[allow(unused)]
	pad7: [u8;1],
#[allow(unused)]
	wheelRotType: u8,
#[allow(unused)]
	wheelRotRadius: f32,
#[allow(unused)]
	retargetMoveRate: f32,
#[allow(unused)]
	ladderWarpOffset: f32,
#[allow(unused)]
	loadAssetId: i32,
#[allow(unused)]
	overlapCameraDmypolyId: i32,
#[allow(unused)]
	residentMaterialExParamId00: i32,
#[allow(unused)]
	residentMaterialExParamId01: i32,
#[allow(unused)]
	residentMaterialExParamId02: i32,
#[allow(unused)]
	residentMaterialExParamId03: i32,
#[allow(unused)]
	residentMaterialExParamId04: i32,
#[allow(unused)]
	sleepCollectorItemLotId_enemy: i32,
#[allow(unused)]
	sleepCollectorItemLotId_map: i32,
#[allow(unused)]
	footIkErrorOnGain: f32,
#[allow(unused)]
	footIkErrorOffGain: f32,
#[allow(unused)]
	SoundAddBankId: i16,
#[allow(unused)]
	materialVariationValue: u8,
#[allow(unused)]
	materialVariationValue_Weak: u8,
#[allow(unused)]
	superArmorDurability: f32,
#[allow(unused)]
	saRecoveryRate: f32,
#[allow(unused)]
	saGuardCutRate: f32,
#[allow(unused)]
	resistCorrectId_blood: i32,
#[allow(unused)]
	resistCorrectId_curse: i32,
#[allow(unused)]
	resistCorrectId_freeze: i32,
#[allow(unused)]
	resistCorrectId_sleep: i32,
#[allow(unused)]
	resistCorrectId_madness: i32,
#[allow(unused)]
	chrDeadTutorialFlagId: i32,
#[allow(unused)]
	stepDispInterpolateTime: f32,
#[allow(unused)]
	stepDispInterpolateTriggerValue: f32,
#[allow(unused)]
	lockScoreOffset: f32,
#[allow(unused)]
	pad12: [u8;8],
}

struct NPC_THINK_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	logicId: i32,
#[allow(unused)]
	battleGoalID: i32,
#[allow(unused)]
	searchEye_dist: i16,
#[allow(unused)]
	searchEye_angY: u8,
#[allow(unused)]
	isNoAvoidHugeEnemy: u8,
#[allow(unused)]
	enableWeaponOnOff: u8,
#[allow(unused)]
	targetAILockDmyPoly: u8,
#[allow(unused)]
	pad8: [u8;1],
#[allow(unused)]
	spEffectId_RangedAttack: i32,
#[allow(unused)]
	searchTargetLv1ForgetTime: f32,
#[allow(unused)]
	searchTargetLv2ForgetTime: f32,
#[allow(unused)]
	BackHomeLife_OnHitEneWal: f32,
#[allow(unused)]
	SightTargetForgetTime: f32,
#[allow(unused)]
	idAttackCannotMove: i32,
#[allow(unused)]
	ear_dist: f32,
#[allow(unused)]
	callHelp_ActionAnimId: i32,
#[allow(unused)]
	callHelp_CallActionId: i32,
#[allow(unused)]
	eye_dist: i16,
#[allow(unused)]
	isGuard_Act: u8,
#[allow(unused)]
	pad6: [u8;1],
#[allow(unused)]
	ear_soundcut_dist: i16,
#[allow(unused)]
	nose_dist: i16,
#[allow(unused)]
	maxBackhomeDist: i16,
#[allow(unused)]
	backhomeDist: i16,
#[allow(unused)]
	backhomeBattleDist: i16,
#[allow(unused)]
	nonBattleActLife: i16,
#[allow(unused)]
	BackHome_LookTargetTime: i16,
#[allow(unused)]
	BackHome_LookTargetDist: i16,
#[allow(unused)]
	SoundTargetForgetTime: f32,
#[allow(unused)]
	BattleStartDist: i16,
#[allow(unused)]
	callHelp_MyPeerId: i16,
#[allow(unused)]
	callHelp_CallPeerId: i16,
#[allow(unused)]
	targetSys_DmgEffectRate: i16,
#[allow(unused)]
	TeamAttackEffectivity: u8,
#[allow(unused)]
	eye_angX: u8,
#[allow(unused)]
	eye_angY: u8,
#[allow(unused)]
	disableDark: u8,
#[allow(unused)]
	caravanRole: u8,
#[allow(unused)]
	callHelp_CallValidMinDistTarget: u8,
#[allow(unused)]
	callHelp_CallValidRange: u8,
#[allow(unused)]
	callHelp_ForgetTimeByArrival: u8,
#[allow(unused)]
	callHelp_MinWaitTime: u8,
#[allow(unused)]
	callHelp_MaxWaitTime: u8,
#[allow(unused)]
	goalAction_ToCaution: u8,
#[allow(unused)]
	ear_listenLevel: u8,
#[allow(unused)]
	callHelp_ReplyBehaviorType: u8,
#[allow(unused)]
	disablePathMove: u8,
#[allow(unused)]
	skipArrivalVisibleCheck: u8,
#[allow(unused)]
	thinkAttr_doAdmirer: u8,
#[allow(unused)]
	enableNaviFlg_Edge: u8,
#[allow(unused)]
	enableNaviFlg_LargeSpace: u8,
#[allow(unused)]
	enableNaviFlg_Ladder: u8,
#[allow(unused)]
	enableNaviFlg_Hole: u8,
#[allow(unused)]
	enableNaviFlg_Door: u8,
#[allow(unused)]
	enableNaviFlg_InSideWall: u8,
#[allow(unused)]
	enableNaviFlg_Lava: u8,
#[allow(unused)]
	enableNaviFlg_Edge_Ordinary: u8,
#[allow(unused)]
	enableNaviFlg_reserve1: [u8;3],
#[allow(unused)]
	searchThreshold_Lv0toLv1: i32,
#[allow(unused)]
	searchThreshold_Lv1toLv2: i32,
#[allow(unused)]
	platoonReplyTime: f32,
#[allow(unused)]
	platoonReplyAddRandomTime: f32,
#[allow(unused)]
	searchEye_angX: u8,
#[allow(unused)]
	isUpdateBattleSight: u8,
#[allow(unused)]
	battleEye_updateDist: i16,
#[allow(unused)]
	battleEye_updateAngX: u8,
#[allow(unused)]
	battleEye_updateAngY: u8,
#[allow(unused)]
	pad4: [u8;16],
#[allow(unused)]
	eye_BackOffsetDist: i16,
#[allow(unused)]
	eye_BeginDist: i16,
#[allow(unused)]
	actTypeOnFailedPath: u8,
#[allow(unused)]
	goalAction_ToCautionImportant: u8,
#[allow(unused)]
	shiftAnimeId_RangedAttack: i32,
#[allow(unused)]
	actTypeOnNonBtlFailedPath: u8,
#[allow(unused)]
	isBuddyAI: u8,
#[allow(unused)]
	goalAction_ToSearchLv1: u8,
#[allow(unused)]
	goalAction_ToSearchLv2: u8,
#[allow(unused)]
	enableJumpMove: u8,
#[allow(unused)]
	disableLocalSteering: u8,
#[allow(unused)]
	goalAction_ToDisappear: u8,
#[allow(unused)]
	changeStateAction_ToNormal: u8,
#[allow(unused)]
	MemoryTargetForgetTime: f32,
#[allow(unused)]
	rangedAttackId: i32,
#[allow(unused)]
	useFall_onNormalCaution: u8,
#[allow(unused)]
	useFall_onSearchBattle: u8,
#[allow(unused)]
	enableJumpMove_onBattle: u8,
#[allow(unused)]
	backToHomeStuckAct: u8,
#[allow(unused)]
	pad3: [u8;4],
#[allow(unused)]
	soundBehaviorId01: i32,
#[allow(unused)]
	soundBehaviorId02: i32,
#[allow(unused)]
	soundBehaviorId03: i32,
#[allow(unused)]
	soundBehaviorId04: i32,
#[allow(unused)]
	soundBehaviorId05: i32,
#[allow(unused)]
	soundBehaviorId06: i32,
#[allow(unused)]
	soundBehaviorId07: i32,
#[allow(unused)]
	soundBehaviorId08: i32,
#[allow(unused)]
	weaponOffSpecialEffectId: i32,
#[allow(unused)]
	weaponOnSpecialEffectId: i32,
#[allow(unused)]
	weaponOffAnimId: i32,
#[allow(unused)]
	weaponOnAnimId: i32,
#[allow(unused)]
	surpriseAnimId: i32,
}

struct OBJ_ACT_PARAM_ST {
#[allow(unused)]
	actionEnableMsgId: i32,
#[allow(unused)]
	actionFailedMsgId: i32,
#[allow(unused)]
	spQualifiedPassEventFlag: i32,
#[allow(unused)]
	playerAnimId: i32,
#[allow(unused)]
	chrAnimId: i32,
#[allow(unused)]
	validDist: i16,
#[allow(unused)]
	spQualifiedId: i16,
#[allow(unused)]
	spQualifiedId2: i16,
#[allow(unused)]
	objDummyId: u8,
#[allow(unused)]
	isEventKickSync: u8,
#[allow(unused)]
	objAnimId: i32,
#[allow(unused)]
	validPlayerAngle: u8,
#[allow(unused)]
	spQualifiedType: u8,
#[allow(unused)]
	spQualifiedType2: u8,
#[allow(unused)]
	validObjAngle: u8,
#[allow(unused)]
	chrSorbType: u8,
#[allow(unused)]
	eventKickTiming: u8,
#[allow(unused)]
	pad1: [u8;2],
#[allow(unused)]
	actionButtonParamId: i32,
#[allow(unused)]
	enableTreasureDelaySec: f32,
#[allow(unused)]
	preActionSfxDmypolyId: i32,
#[allow(unused)]
	preActionSfxId: i32,
#[allow(unused)]
	pad2: [u8;40],
}

struct OBJECT_MATERIAL_SFX_PARAM_ST {
#[allow(unused)]
	sfxId_00: i32,
#[allow(unused)]
	sfxId_01: i32,
#[allow(unused)]
	sfxId_02: i32,
#[allow(unused)]
	sfxId_03: i32,
#[allow(unused)]
	sfxId_04: i32,
#[allow(unused)]
	sfxId_05: i32,
#[allow(unused)]
	sfxId_06: i32,
#[allow(unused)]
	sfxId_07: i32,
#[allow(unused)]
	sfxId_08: i32,
#[allow(unused)]
	sfxId_09: i32,
#[allow(unused)]
	sfxId_10: i32,
#[allow(unused)]
	sfxId_11: i32,
#[allow(unused)]
	sfxId_12: i32,
#[allow(unused)]
	sfxId_13: i32,
#[allow(unused)]
	sfxId_14: i32,
#[allow(unused)]
	sfxId_15: i32,
#[allow(unused)]
	sfxId_16: i32,
#[allow(unused)]
	sfxId_17: i32,
#[allow(unused)]
	sfxId_18: i32,
#[allow(unused)]
	sfxId_19: i32,
#[allow(unused)]
	sfxId_20: i32,
#[allow(unused)]
	sfxId_21: i32,
#[allow(unused)]
	sfxId_22: i32,
#[allow(unused)]
	sfxId_23: i32,
#[allow(unused)]
	sfxId_24: i32,
#[allow(unused)]
	sfxId_25: i32,
#[allow(unused)]
	sfxId_26: i32,
#[allow(unused)]
	sfxId_27: i32,
#[allow(unused)]
	sfxId_28: i32,
#[allow(unused)]
	sfxId_29: i32,
#[allow(unused)]
	sfxId_30: i32,
#[allow(unused)]
	sfxId_31: i32,
}

struct OBJECT_PARAM_ST {
#[allow(unused)]
	hp: i16,
#[allow(unused)]
	defense: i16,
#[allow(unused)]
	extRefTexId: i16,
#[allow(unused)]
	materialId: i16,
#[allow(unused)]
	animBreakIdMax: u8,
#[allow(unused)]
	isCamHit: u8,
#[allow(unused)]
	isBreakByPlayerCollide: u8,
#[allow(unused)]
	isAnimBreak: u8,
#[allow(unused)]
	isPenetrationBulletHit: u8,
#[allow(unused)]
	isChrHit: u8,
#[allow(unused)]
	isAttackBacklash: u8,
#[allow(unused)]
	isDisableBreakForFirstAppear: u8,
#[allow(unused)]
	isLadder: u8,
#[allow(unused)]
	isAnimPauseOnRemoPlay: u8,
#[allow(unused)]
	isDamageNoHit: u8,
#[allow(unused)]
	isMoveObj: u8,
#[allow(unused)]
	isRopeBridge: u8,
#[allow(unused)]
	isAddRigidImpulse_ByDamage: u8,
#[allow(unused)]
	isBreak_ByChrRide: u8,
#[allow(unused)]
	isBurn: u8,
#[allow(unused)]
	isBreakByEnemyCollide: u8,
#[allow(unused)]
	defaultLodParamId: i8,
#[allow(unused)]
	breakSfxId: i32,
#[allow(unused)]
	breakSfxCpId: i32,
#[allow(unused)]
	breakBulletBehaviorId: i32,
#[allow(unused)]
	breakBulletCpId: i32,
#[allow(unused)]
	breakFallHeight: u8,
#[allow(unused)]
	windEffectType_0: u8,
#[allow(unused)]
	windEffectType_1: u8,
#[allow(unused)]
	camAvoidType: u8,
#[allow(unused)]
	windEffectRate_0: f32,
#[allow(unused)]
	windEffectRate_1: f32,
#[allow(unused)]
	breakStopTime: f32,
#[allow(unused)]
	burnTime: f32,
#[allow(unused)]
	burnBraekRate: f32,
#[allow(unused)]
	burnSfxId: i32,
#[allow(unused)]
	burnSfxId_1: i32,
#[allow(unused)]
	burnSfxId_2: i32,
#[allow(unused)]
	burnSfxId_3: i32,
#[allow(unused)]
	burnBulletBehaviorId: i32,
#[allow(unused)]
	burnBulletBehaviorId_1: i32,
#[allow(unused)]
	burnBulletBehaviorId_2: i32,
#[allow(unused)]
	burnBulletBehaviorId_3: i32,
#[allow(unused)]
	burnBulletInterval: i16,
#[allow(unused)]
	navimeshFlag: u8,
#[allow(unused)]
	collisionType: u8,
#[allow(unused)]
	burnBulletDelayTime: f32,
#[allow(unused)]
	burnSfxDelayTimeMin: f32,
#[allow(unused)]
	burnSfxDelayTimeMin_1: f32,
#[allow(unused)]
	burnSfxDelayTimeMin_2: f32,
#[allow(unused)]
	burnSfxDelayTimeMin_3: f32,
#[allow(unused)]
	burnSfxDelayTimeMax: f32,
#[allow(unused)]
	burnSfxDelayTimeMax_1: f32,
#[allow(unused)]
	burnSfxDelayTimeMax_2: f32,
#[allow(unused)]
	burnSfxDelayTimeMax_3: f32,
#[allow(unused)]
	BreakAiSoundID: i32,
#[allow(unused)]
	FragmentInvisibleWaitTime: f32,
#[allow(unused)]
	FragmentInvisibleTime: f32,
#[allow(unused)]
	pad_3: [u8;16],
#[allow(unused)]
	RigidPenetrationScale_Soft: f32,
#[allow(unused)]
	RigidPenetrationScale_Normal: f32,
#[allow(unused)]
	RigidPenetrationScale_Hard: f32,
#[allow(unused)]
	LandTouchSfxId: i32,
#[allow(unused)]
	isDamageCover: u8,
#[allow(unused)]
	pad_4: [u8;1],
#[allow(unused)]
	paintDecalTargetTextureSize: i16,
#[allow(unused)]
	lifeTime_forDC: f32,
#[allow(unused)]
	clothUpdateDist: f32,
#[allow(unused)]
	contactSeId: i32,
#[allow(unused)]
	breakLandingSfxId: i32,
#[allow(unused)]
	waypointDummyPolyId_0: i32,
#[allow(unused)]
	waypointParamId_0: i32,
#[allow(unused)]
	soundBankId: i32,
#[allow(unused)]
	refDrawParamId: i32,
#[allow(unused)]
	autoCreateDynamicOffsetHeight: f32,
#[allow(unused)]
	reserved0: i32,
#[allow(unused)]
	soundBreakSEId: i32,
#[allow(unused)]
	pad_5: [u8;40],
}

struct PARTS_DRAW_PARAM_ST {
#[allow(unused)]
	lv01_BorderDist: f32,
#[allow(unused)]
	lv01_PlayDist: f32,
#[allow(unused)]
	lv12_BorderDist: f32,
#[allow(unused)]
	lv12_PlayDist: f32,
#[allow(unused)]
	lv23_BorderDist: f32,
#[allow(unused)]
	lv23_PlayDist: f32,
#[allow(unused)]
	lv34_BorderDist: f32,
#[allow(unused)]
	lv34_PlayDist: f32,
#[allow(unused)]
	lv45_BorderDist: f32,
#[allow(unused)]
	lv45_PlayDist: f32,
#[allow(unused)]
	tex_lv01_BorderDist: f32,
#[allow(unused)]
	tex_lv01_PlayDist: f32,
#[allow(unused)]
	enableCrossFade: i32,
#[allow(unused)]
	drawDist: f32,
#[allow(unused)]
	drawFadeRange: f32,
#[allow(unused)]
	shadowDrawDist: f32,
#[allow(unused)]
	shadowFadeRange: f32,
#[allow(unused)]
	motionBlur_BorderDist: f32,
#[allow(unused)]
	isPointLightShadowSrc: i8,
#[allow(unused)]
	isDirLightShadowSrc: i8,
#[allow(unused)]
	isShadowDst: i8,
#[allow(unused)]
	isShadowOnly: i8,
#[allow(unused)]
	drawByReflectCam: i8,
#[allow(unused)]
	drawOnlyReflectCam: i8,
#[allow(unused)]
	IncludeLodMapLv: i8,
#[allow(unused)]
	isNoFarClipDraw: u8,
#[allow(unused)]
	lodType: u8,
#[allow(unused)]
	shadowDrawLodOffset: i8,
#[allow(unused)]
	isTraceCameraXZ: u8,
#[allow(unused)]
	isSkydomeDrawPhase: u8,
#[allow(unused)]
	DistantViewModel_BorderDist: f32,
#[allow(unused)]
	DistantViewModel_PlayDist: f32,
#[allow(unused)]
	LimtedActivate_BorderDist_forGrid: f32,
#[allow(unused)]
	LimtedActivate_PlayDist_forGrid: f32,
#[allow(unused)]
	zSortOffsetForNoFarClipDraw: f32,
#[allow(unused)]
	shadowDrawAlphaTestDist: f32,
#[allow(unused)]
	fowardDrawEnvmapBlendType: u8,
#[allow(unused)]
	LBDrawDistScaleParamID: u8,
#[allow(unused)]
	resereve: [u8;34],
}

struct PERFORMANCE_CHECK_PARAM {
#[allow(unused)]
	workTag: u8,
#[allow(unused)]
	categoryTag: u8,
#[allow(unused)]
	compareType: u8,
#[allow(unused)]
	dummy1: [u8;1],
#[allow(unused)]
	compareValue: f32,
#[allow(unused)]
	dummy2: [u8;8],
#[allow(unused)]
	userTag: [u8;16],
}

struct PHANTOM_PARAM_ST {
#[allow(unused)]
	edgeColorA: f32,
#[allow(unused)]
	frontColorA: f32,
#[allow(unused)]
	diffMulColorA: f32,
#[allow(unused)]
	specMulColorA: f32,
#[allow(unused)]
	lightColorA: f32,
#[allow(unused)]
	edgeColorR: u8,
#[allow(unused)]
	edgeColorG: u8,
#[allow(unused)]
	edgeColorB: u8,
#[allow(unused)]
	frontColorR: u8,
#[allow(unused)]
	frontColorG: u8,
#[allow(unused)]
	frontColorB: u8,
#[allow(unused)]
	diffMulColorR: u8,
#[allow(unused)]
	diffMulColorG: u8,
#[allow(unused)]
	diffMulColorB: u8,
#[allow(unused)]
	specMulColorR: u8,
#[allow(unused)]
	specMulColorG: u8,
#[allow(unused)]
	specMulColorB: u8,
#[allow(unused)]
	lightColorR: u8,
#[allow(unused)]
	lightColorG: u8,
#[allow(unused)]
	lightColorB: u8,
#[allow(unused)]
	reserve: [u8;1],
#[allow(unused)]
	alpha: f32,
#[allow(unused)]
	blendRate: f32,
#[allow(unused)]
	blendType: u8,
#[allow(unused)]
	isEdgeSubtract: u8,
#[allow(unused)]
	isFrontSubtract: u8,
#[allow(unused)]
	isNo2Pass: u8,
#[allow(unused)]
	edgePower: f32,
#[allow(unused)]
	glowScale: f32,
}

struct PLAYER_COMMON_PARAM_ST {
#[allow(unused)]
	playerFootEffect_bySFX: i32,
#[allow(unused)]
	snipeModeDrawAlpha_FadeTime: f32,
#[allow(unused)]
	toughnessRecoverCorrection: f32,
#[allow(unused)]
	baseMagicSlotSize: u8,
#[allow(unused)]
	baseAccSlotNum: u8,
#[allow(unused)]
	reserved02: [u8;2],
#[allow(unused)]
	animeID_DropItemPick: i32,
#[allow(unused)]
	resistRecoverPoint_Sleep_Player: f32,
#[allow(unused)]
	flareOverrideHomingAngle: i32,
#[allow(unused)]
	flareOverrideHomingStopRange: f32,
#[allow(unused)]
	animeID_SleepCollectorItemPick: i32,
#[allow(unused)]
	unlockEventFlagBaseId_forWepAttr: i32,
#[allow(unused)]
	systemEnchant_BigRune: i32,
#[allow(unused)]
	lowStatus_AtkPowDown: f32,
#[allow(unused)]
	lowStatus_ConsumeStaminaRate: f32,
#[allow(unused)]
	lowStatus_AtkGuardBreak: i16,
#[allow(unused)]
	guardStatusCorrect_MaxStatusVal: i16,
#[allow(unused)]
	unlockEventFlagStepNum_forWepAttr: i16,
#[allow(unused)]
	retributionMagic_damageCountNum: i16,
#[allow(unused)]
	retributionMagic_damageCountRemainTime: i16,
#[allow(unused)]
	retributionMagic_burstDmypolyId: i16,
#[allow(unused)]
	retributionMagic_burstMagicParamId: i32,
#[allow(unused)]
	chrAimCam_rideOffsetHeight: f32,
#[allow(unused)]
	reserved23: [u8;4],
#[allow(unused)]
	arrowCaseWepBindDmypolyId: i32,
#[allow(unused)]
	boltPouchWepBindDmypolyId: i32,
#[allow(unused)]
	estusFlaskAllocateRate: f32,
#[allow(unused)]
	reserved38: [u8;2],
#[allow(unused)]
	kickAcceptanceDeg: u8,
#[allow(unused)]
	npcPlayerAnalogWeightRate_Light: u8,
#[allow(unused)]
	npcPlayerAnalogWeightRate_Normal: u8,
#[allow(unused)]
	npcPlayerAnalogWeightRate_Heavy: u8,
#[allow(unused)]
	npcPlayerAnalogWeightRate_WeightOver: u8,
#[allow(unused)]
	npcPlayerAnalogWeightRate_SuperLight: u8,
#[allow(unused)]
	reserved45: [u8;4],
#[allow(unused)]
	clearCountCorrectBaseSpEffectId: i32,
#[allow(unused)]
	arrowBoltModelIdOffset: i32,
#[allow(unused)]
	arrowBoltRemainingNumModelMaskThreshold1: i8,
#[allow(unused)]
	arrowBoltRemainingNumModelMaskThreshold2: i8,
#[allow(unused)]
	reserved27: [u8;2],
#[allow(unused)]
	resistRecoverPoint_Poision_Player: f32,
#[allow(unused)]
	resistRecoverPoint_Desease_Player: f32,
#[allow(unused)]
	resistRecoverPoint_Blood_Player: f32,
#[allow(unused)]
	resistRecoverPoint_Curse_Player: f32,
#[allow(unused)]
	resistRecoverPoint_Freeze_Player: f32,
#[allow(unused)]
	resistRecoverPoint_Poision_Enemy: f32,
#[allow(unused)]
	resistRecoverPoint_Desease_Enemy: f32,
#[allow(unused)]
	resistRecoverPoint_Blood_Enemy: f32,
#[allow(unused)]
	resistRecoverPoint_Curse_Enemy: f32,
#[allow(unused)]
	resistRecoverPoint_Freeze_Enemy: f32,
#[allow(unused)]
	requestTimeLeftBothHand: f32,
#[allow(unused)]
	resistRecoverPoint_Madness_Player: f32,
#[allow(unused)]
	animeID_MaterialItemPick: i32,
#[allow(unused)]
	hpEstusFlaskAllocateRateForYellowMonk: f32,
#[allow(unused)]
	hpEstusFlaskAllocateOffsetForYellowMonk: i32,
#[allow(unused)]
	mpEstusFlaskAllocateRateForYellowMonk: f32,
#[allow(unused)]
	mpEstusFlaskAllocateOffsetForYellowMonk: i32,
#[allow(unused)]
	resistRecoverPoint_Sleep_Enemy: f32,
#[allow(unused)]
	resistRecoverPoint_Madness_Enemy: f32,
#[allow(unused)]
	resistCurseItemId: i32,
#[allow(unused)]
	resistCurseItemMaxNum: u8,
#[allow(unused)]
	reserved_123: [u8;3],
#[allow(unused)]
	resistCurseItemSpEffectBaseId: i32,
#[allow(unused)]
	resistCurseItemLotParamId_map: i32,
#[allow(unused)]
	reserved41: [u8;52],
}

struct PLAY_REGION_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	matchAreaId: i32,
#[allow(unused)]
	multiPlayStartLimitEventFlagId: i32,
#[allow(unused)]
	otherDisableDistance: f32,
#[allow(unused)]
	pcPositionSaveLimitEventFlagId: i32,
#[allow(unused)]
	bossAreaId: i32,
#[allow(unused)]
	cultNpcWhiteGhostEntityId_byFree: i16,
#[allow(unused)]
	bMapGuradianRegion: u8,
#[allow(unused)]
	bYellowCostumeRegion: u8,
#[allow(unused)]
	multiPlayStartLimitEventFlagId_targetFlagState: u8,
#[allow(unused)]
	breakInLimitEventFlagId_1_targetFlagState: u8,
#[allow(unused)]
	whiteSignLimitEventFlagId_1_targetFlagState: u8,
#[allow(unused)]
	redSignLimitEventFlagId_1_targetFlagState: u8,
#[allow(unused)]
	breakInLimitEventFlagId_2_targetFlagState: u8,
#[allow(unused)]
	breakInLimitEventFlagId_3_targetFlagState: u8,
#[allow(unused)]
	whiteSignLimitEventFlagId_2_targetFlagState: u8,
#[allow(unused)]
	warpItemUsePermitBonfireId_1: i32,
#[allow(unused)]
	warpItemUsePermitBonfireId_2: i32,
#[allow(unused)]
	warpItemUsePermitBonfireId_3: i32,
#[allow(unused)]
	warpItemUsePermitBonfireId_4: i32,
#[allow(unused)]
	warpItemUsePermitBonfireId_5: i32,
#[allow(unused)]
	warpItemProhibitionEventFlagId_1: i32,
#[allow(unused)]
	warpItemProhibitionEventFlagId_2: i32,
#[allow(unused)]
	warpItemProhibitionEventFlagId_3: i32,
#[allow(unused)]
	warpItemProhibitionEventFlagId_4: i32,
#[allow(unused)]
	warpItemProhibitionEventFlagId_5: i32,
#[allow(unused)]
	enableBloodstain: u8,
#[allow(unused)]
	enableBloodMessage: u8,
#[allow(unused)]
	enableGhost: u8,
#[allow(unused)]
	dispMask00: u8,
#[allow(unused)]
	dispMask01: u8,
#[allow(unused)]
	whiteSignLimitEventFlagId_3_targetFlagState: u8,
#[allow(unused)]
	redSignLimitEventFlagId_2_targetFlagState: u8,
#[allow(unused)]
	redSignLimitEventFlagId_3_targetFlagState: u8,
#[allow(unused)]
	isAutoIntrudePoint: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	pad2: [u8;2],
#[allow(unused)]
	multiPlayHASHostLimitEventFlagId: i32,
#[allow(unused)]
	otherMaxDistance: f32,
#[allow(unused)]
	signPuddleOpenEventFlagId: i32,
#[allow(unused)]
	areaNo: u8,
#[allow(unused)]
	gridXNo: u8,
#[allow(unused)]
	gridZNo: u8,
#[allow(unused)]
	pad4: [u8;1],
#[allow(unused)]
	posX: f32,
#[allow(unused)]
	posY: f32,
#[allow(unused)]
	posZ: f32,
#[allow(unused)]
	breakInLimitEventFlagId_1: i32,
#[allow(unused)]
	whiteSignLimitEventFlagId_1: i32,
#[allow(unused)]
	matchAreaSignCreateLimitEventFlagId: i32,
#[allow(unused)]
	signAimId_1: i32,
#[allow(unused)]
	signAimId_2: i32,
#[allow(unused)]
	signAimId_3: i32,
#[allow(unused)]
	signAimId_4: i32,
#[allow(unused)]
	signAimId_5: i32,
#[allow(unused)]
	signAimId_6: i32,
#[allow(unused)]
	signAimId_7: i32,
#[allow(unused)]
	signAimId_8: i32,
#[allow(unused)]
	redSignLimitEventFlagId_1: i32,
#[allow(unused)]
	breakInLimitEventFlagId_2: i32,
#[allow(unused)]
	breakInLimitEventFlagId_3: i32,
#[allow(unused)]
	whiteSignLimitEventFlagId_2: i32,
#[allow(unused)]
	whiteSignLimitEventFlagId_3: i32,
#[allow(unused)]
	redSignLimitEventFlagId_2: i32,
#[allow(unused)]
	redSignLimitEventFlagId_3: i32,
#[allow(unused)]
	bossId_1: i32,
#[allow(unused)]
	bossId_2: i32,
#[allow(unused)]
	bossId_3: i32,
#[allow(unused)]
	bossId_4: i32,
#[allow(unused)]
	bossId_5: i32,
#[allow(unused)]
	bossId_6: i32,
#[allow(unused)]
	bossId_7: i32,
#[allow(unused)]
	bossId_8: i32,
#[allow(unused)]
	bossId_9: i32,
#[allow(unused)]
	bossId_10: i32,
#[allow(unused)]
	bossId_11: i32,
#[allow(unused)]
	bossId_12: i32,
#[allow(unused)]
	bossId_13: i32,
#[allow(unused)]
	bossId_14: i32,
#[allow(unused)]
	bossId_15: i32,
#[allow(unused)]
	bossId_16: i32,
#[allow(unused)]
	mapMenuUnlockEventId: i32,
#[allow(unused)]
	pad5: [u8;32],
}

struct POSTURE_CONTROL_PARAM_GENDER_ST {
#[allow(unused)]
	a000_rightElbowIO: i16,
#[allow(unused)]
	a000_leftElbowIO: i16,
#[allow(unused)]
	a000_bothLegsIO: i16,
#[allow(unused)]
	a002_rightElbowIO: i16,
#[allow(unused)]
	a002_leftElbowIO: i16,
#[allow(unused)]
	a002_bothLegsIO: i16,
#[allow(unused)]
	a003_rightElbowIO: i16,
#[allow(unused)]
	a003_leftElbowIO: i16,
#[allow(unused)]
	a003_bothLegsIO: i16,
#[allow(unused)]
	a010_rightElbowIO: i16,
#[allow(unused)]
	a010_leftElbowIO: i16,
#[allow(unused)]
	a010_bothLegsIO: i16,
#[allow(unused)]
	a012_rightElbowIO: i16,
#[allow(unused)]
	a012_leftElbowIO: i16,
#[allow(unused)]
	a012_bothLegsIO: i16,
#[allow(unused)]
	a013_rightElbowIO: i16,
#[allow(unused)]
	a013_leftElbowIO: i16,
#[allow(unused)]
	a013_bothLegsIO: i16,
#[allow(unused)]
	a014_rightElbowIO: i16,
#[allow(unused)]
	a014_leftElbowIO: i16,
#[allow(unused)]
	a014_bothLegsIO: i16,
#[allow(unused)]
	a015_rightElbowIO: i16,
#[allow(unused)]
	a015_leftElbowIO: i16,
#[allow(unused)]
	a015_bothLegsIO: i16,
#[allow(unused)]
	a016_rightElbowIO: i16,
#[allow(unused)]
	a016_leftElbowIO: i16,
#[allow(unused)]
	a016_bothLegsIO: i16,
#[allow(unused)]
	pad: [u8;10],
}

struct POSTURE_CONTROL_PARAM_PRO_ST {
#[allow(unused)]
	a000_rightArmIO: i16,
#[allow(unused)]
	a000_rightArmFB: i16,
#[allow(unused)]
	a000_leftArmIO: i16,
#[allow(unused)]
	a000_leftArmFB: i16,
#[allow(unused)]
	a002_rightArmIO: i16,
#[allow(unused)]
	a002_rightArmFB: i16,
#[allow(unused)]
	a002_leftArmIO: i16,
#[allow(unused)]
	a002_leftArmFB: i16,
#[allow(unused)]
	a003_rightArmIO: i16,
#[allow(unused)]
	a003_rightArmFB: i16,
#[allow(unused)]
	a003_leftArmIO: i16,
#[allow(unused)]
	a003_leftArmFB: i16,
#[allow(unused)]
	a010_rightArmIO: i16,
#[allow(unused)]
	a010_rightArmFB: i16,
#[allow(unused)]
	a010_leftArmIO: i16,
#[allow(unused)]
	a010_leftArmFB: i16,
#[allow(unused)]
	a012_rightArmIO: i16,
#[allow(unused)]
	a012_rightArmFB: i16,
#[allow(unused)]
	a012_leftArmIO: i16,
#[allow(unused)]
	a012_leftArmFB: i16,
#[allow(unused)]
	a013_rightArmIO: i16,
#[allow(unused)]
	a013_rightArmFB: i16,
#[allow(unused)]
	a013_leftArmIO: i16,
#[allow(unused)]
	a013_leftArmFB: i16,
#[allow(unused)]
	a014_rightArmIO: i16,
#[allow(unused)]
	a014_rightArmFB: i16,
#[allow(unused)]
	a014_leftArmIO: i16,
#[allow(unused)]
	a014_leftArmFB: i16,
#[allow(unused)]
	a015_rightArmIO: i16,
#[allow(unused)]
	a015_rightArmFB: i16,
#[allow(unused)]
	a015_leftArmIO: i16,
#[allow(unused)]
	a015_leftArmFB: i16,
#[allow(unused)]
	a016_rightArmIO: i16,
#[allow(unused)]
	a016_rightArmFB: i16,
#[allow(unused)]
	a016_leftArmIO: i16,
#[allow(unused)]
	a016_leftArmFB: i16,
#[allow(unused)]
	pad: [u8;8],
}

struct POSTURE_CONTROL_PARAM_WEP_LEFT_ST {
#[allow(unused)]
	a000_leftArmFB: i16,
#[allow(unused)]
	a000_leftWristFB: i16,
#[allow(unused)]
	a000_leftWristIO: i16,
#[allow(unused)]
	a002_leftArmFB: i16,
#[allow(unused)]
	a002_leftWristFB: i16,
#[allow(unused)]
	a002_leftWristIO: i16,
#[allow(unused)]
	a003_leftArmFB: i16,
#[allow(unused)]
	a003_leftWristFB: i16,
#[allow(unused)]
	a003_leftWristIO: i16,
#[allow(unused)]
	pad: [u8;14],
}

struct POSTURE_CONTROL_PARAM_WEP_RIGHT_ST {
#[allow(unused)]
	a000_rightArmFB: i16,
#[allow(unused)]
	a000_rightWristFB: i16,
#[allow(unused)]
	a000_rightWristIO: i16,
#[allow(unused)]
	a000_leftArmFB: i16,
#[allow(unused)]
	a000_leftWristFB: i16,
#[allow(unused)]
	a000_leftWristIO: i16,
#[allow(unused)]
	a002_rightArmFB: i16,
#[allow(unused)]
	a002_rightWristFB: i16,
#[allow(unused)]
	a002_rightWristIO: i16,
#[allow(unused)]
	a002_leftArmFB: i16,
#[allow(unused)]
	a002_leftWristFB: i16,
#[allow(unused)]
	a002_leftWristIO: i16,
#[allow(unused)]
	a003_rightArmFB: i16,
#[allow(unused)]
	a003_rightWristFB: i16,
#[allow(unused)]
	a003_rightWristIO: i16,
#[allow(unused)]
	a003_leftArmFB: i16,
#[allow(unused)]
	a003_leftWristFB: i16,
#[allow(unused)]
	a003_leftWristIO: i16,
#[allow(unused)]
	a010_rightArmFB: i16,
#[allow(unused)]
	a010_rightWristFB: i16,
#[allow(unused)]
	a010_rightWristIO: i16,
#[allow(unused)]
	a010_leftArmFB: i16,
#[allow(unused)]
	a010_leftWristFB: i16,
#[allow(unused)]
	a010_leftWristIO: i16,
#[allow(unused)]
	a012_rightArmFB: i16,
#[allow(unused)]
	a012_rightWristFB: i16,
#[allow(unused)]
	a012_rightWristIO: i16,
#[allow(unused)]
	a012_leftArmFB: i16,
#[allow(unused)]
	a012_leftWristFB: i16,
#[allow(unused)]
	a012_leftWristIO: i16,
#[allow(unused)]
	a013_rightArmFB: i16,
#[allow(unused)]
	a013_rightWristFB: i16,
#[allow(unused)]
	a013_rightWristIO: i16,
#[allow(unused)]
	a013_leftArmFB: i16,
#[allow(unused)]
	a013_leftWristFB: i16,
#[allow(unused)]
	a013_leftWristIO: i16,
#[allow(unused)]
	a014_rightArmFB: i16,
#[allow(unused)]
	a014_rightWristFB: i16,
#[allow(unused)]
	a014_rightWristIO: i16,
#[allow(unused)]
	a014_leftArmFB: i16,
#[allow(unused)]
	a014_leftWristFB: i16,
#[allow(unused)]
	a014_leftWristIO: i16,
#[allow(unused)]
	a015_rightArmFB: i16,
#[allow(unused)]
	a015_rightWristFB: i16,
#[allow(unused)]
	a015_rightWristIO: i16,
#[allow(unused)]
	a015_leftArmFB: i16,
#[allow(unused)]
	a015_leftWristFB: i16,
#[allow(unused)]
	a015_leftWristIO: i16,
#[allow(unused)]
	a016_rightArmFB: i16,
#[allow(unused)]
	a016_rightWristFB: i16,
#[allow(unused)]
	a016_rightWristIO: i16,
#[allow(unused)]
	a016_leftArmFB: i16,
#[allow(unused)]
	a016_leftWristFB: i16,
#[allow(unused)]
	a016_leftWristIO: i16,
#[allow(unused)]
	pad: [u8;4],
}

struct RANDOM_APPEAR_EDIT_PARAM_ST {
#[allow(unused)]
	appearNum: i32,
#[allow(unused)]
	paramId1: i32,
#[allow(unused)]
	rate1: i32,
#[allow(unused)]
	paramId2: i32,
#[allow(unused)]
	rate2: i32,
#[allow(unused)]
	paramId3: i32,
#[allow(unused)]
	rate3: i32,
#[allow(unused)]
	paramId4: i32,
#[allow(unused)]
	rate4: i32,
#[allow(unused)]
	paramId5: i32,
#[allow(unused)]
	rate5: i32,
#[allow(unused)]
	paramId6: i32,
#[allow(unused)]
	rate6: i32,
#[allow(unused)]
	paramId7: i32,
#[allow(unused)]
	rate7: i32,
#[allow(unused)]
	paramId8: i32,
#[allow(unused)]
	rate8: i32,
#[allow(unused)]
	paramId9: i32,
#[allow(unused)]
	rate9: i32,
#[allow(unused)]
	paramId10: i32,
#[allow(unused)]
	rate10: i32,
#[allow(unused)]
	paramId11: i32,
#[allow(unused)]
	rate11: i32,
#[allow(unused)]
	paramId12: i32,
#[allow(unused)]
	rate12: i32,
#[allow(unused)]
	paramId13: i32,
#[allow(unused)]
	rate13: i32,
#[allow(unused)]
	paramId14: i32,
#[allow(unused)]
	rate14: i32,
#[allow(unused)]
	paramId15: i32,
#[allow(unused)]
	rate15: i32,
#[allow(unused)]
	paramId16: i32,
#[allow(unused)]
	rate16: i32,
#[allow(unused)]
	paramId17: i32,
#[allow(unused)]
	rate17: i32,
#[allow(unused)]
	paramId18: i32,
#[allow(unused)]
	rate18: i32,
#[allow(unused)]
	paramId19: i32,
#[allow(unused)]
	rate19: i32,
#[allow(unused)]
	paramId20: i32,
#[allow(unused)]
	rate20: i32,
#[allow(unused)]
	paramId21: i32,
#[allow(unused)]
	rate21: i32,
#[allow(unused)]
	paramId22: i32,
#[allow(unused)]
	rate22: i32,
#[allow(unused)]
	paramId23: i32,
#[allow(unused)]
	rate23: i32,
#[allow(unused)]
	paramId24: i32,
#[allow(unused)]
	rate24: i32,
}

struct RANDOM_APPEAR_PARAM_ST {
#[allow(unused)]
	slot0: u8,
#[allow(unused)]
	slot1: u8,
#[allow(unused)]
	slot2: u8,
#[allow(unused)]
	slot3: u8,
#[allow(unused)]
	slot4: u8,
#[allow(unused)]
	slot5: u8,
#[allow(unused)]
	slot6: u8,
#[allow(unused)]
	slot7: u8,
#[allow(unused)]
	slot8: u8,
#[allow(unused)]
	slot9: u8,
#[allow(unused)]
	slot10: u8,
#[allow(unused)]
	slot11: u8,
#[allow(unused)]
	slot12: u8,
#[allow(unused)]
	slot13: u8,
#[allow(unused)]
	slot14: u8,
#[allow(unused)]
	slot15: u8,
#[allow(unused)]
	slot16: u8,
#[allow(unused)]
	slot17: u8,
#[allow(unused)]
	slot18: u8,
#[allow(unused)]
	slot19: u8,
#[allow(unused)]
	slot20: u8,
#[allow(unused)]
	slot21: u8,
#[allow(unused)]
	slot22: u8,
#[allow(unused)]
	slot23: u8,
#[allow(unused)]
	slot24: u8,
#[allow(unused)]
	slot25: u8,
#[allow(unused)]
	slot26: u8,
#[allow(unused)]
	slot27: u8,
#[allow(unused)]
	slot28: u8,
#[allow(unused)]
	slot29: u8,
#[allow(unused)]
	slot30: u8,
#[allow(unused)]
	slot31: u8,
#[allow(unused)]
	slot32: u8,
#[allow(unused)]
	slot33: u8,
#[allow(unused)]
	slot34: u8,
#[allow(unused)]
	slot35: u8,
#[allow(unused)]
	slot36: u8,
#[allow(unused)]
	slot37: u8,
#[allow(unused)]
	slot38: u8,
#[allow(unused)]
	slot39: u8,
#[allow(unused)]
	slot40: u8,
#[allow(unused)]
	slot41: u8,
#[allow(unused)]
	slot42: u8,
#[allow(unused)]
	slot43: u8,
#[allow(unused)]
	slot44: u8,
#[allow(unused)]
	slot45: u8,
#[allow(unused)]
	slot46: u8,
#[allow(unused)]
	slot47: u8,
#[allow(unused)]
	slot48: u8,
#[allow(unused)]
	slot49: u8,
#[allow(unused)]
	slot50: u8,
#[allow(unused)]
	slot51: u8,
#[allow(unused)]
	slot52: u8,
#[allow(unused)]
	slot53: u8,
#[allow(unused)]
	slot54: u8,
#[allow(unused)]
	slot55: u8,
#[allow(unused)]
	slot56: u8,
#[allow(unused)]
	slot57: u8,
#[allow(unused)]
	slot58: u8,
#[allow(unused)]
	slot59: u8,
#[allow(unused)]
	slot60: u8,
#[allow(unused)]
	slot61: u8,
#[allow(unused)]
	slot62: u8,
#[allow(unused)]
	slot63: u8,
#[allow(unused)]
	slot64: u8,
#[allow(unused)]
	slot65: u8,
#[allow(unused)]
	slot66: u8,
#[allow(unused)]
	slot67: u8,
#[allow(unused)]
	slot68: u8,
#[allow(unused)]
	slot69: u8,
#[allow(unused)]
	slot70: u8,
#[allow(unused)]
	slot71: u8,
#[allow(unused)]
	slot72: u8,
#[allow(unused)]
	slot73: u8,
#[allow(unused)]
	slot74: u8,
#[allow(unused)]
	slot75: u8,
#[allow(unused)]
	slot76: u8,
#[allow(unused)]
	slot77: u8,
#[allow(unused)]
	slot78: u8,
#[allow(unused)]
	slot79: u8,
#[allow(unused)]
	slot80: u8,
#[allow(unused)]
	slot81: u8,
#[allow(unused)]
	slot82: u8,
#[allow(unused)]
	slot83: u8,
#[allow(unused)]
	slot84: u8,
#[allow(unused)]
	slot85: u8,
#[allow(unused)]
	slot86: u8,
#[allow(unused)]
	slot87: u8,
#[allow(unused)]
	slot88: u8,
#[allow(unused)]
	slot89: u8,
#[allow(unused)]
	slot90: u8,
#[allow(unused)]
	slot91: u8,
#[allow(unused)]
	slot92: u8,
#[allow(unused)]
	slot93: u8,
#[allow(unused)]
	slot94: u8,
#[allow(unused)]
	slot95: u8,
#[allow(unused)]
	slot96: u8,
#[allow(unused)]
	slot97: u8,
#[allow(unused)]
	slot98: u8,
#[allow(unused)]
	slot99: u8,
#[allow(unused)]
	pad: [u8;1],
}

struct REINFORCE_PARAM_PROTECTOR_ST {
#[allow(unused)]
	physicsDefRate: f32,
#[allow(unused)]
	magicDefRate: f32,
#[allow(unused)]
	fireDefRate: f32,
#[allow(unused)]
	thunderDefRate: f32,
#[allow(unused)]
	slashDefRate: f32,
#[allow(unused)]
	blowDefRate: f32,
#[allow(unused)]
	thrustDefRate: f32,
#[allow(unused)]
	resistPoisonRate: f32,
#[allow(unused)]
	resistDiseaseRate: f32,
#[allow(unused)]
	resistBloodRate: f32,
#[allow(unused)]
	resistCurseRate: f32,
#[allow(unused)]
	residentSpEffectId1: u8,
#[allow(unused)]
	residentSpEffectId2: u8,
#[allow(unused)]
	residentSpEffectId3: u8,
#[allow(unused)]
	materialSetId: u8,
#[allow(unused)]
	darkDefRate: f32,
#[allow(unused)]
	resistFreezeRate: f32,
#[allow(unused)]
	resistSleepRate: f32,
#[allow(unused)]
	resistMadnessRate: f32,
}

struct REINFORCE_PARAM_WEAPON_ST {
#[allow(unused)]
	physicsAtkRate: f32,
#[allow(unused)]
	magicAtkRate: f32,
#[allow(unused)]
	fireAtkRate: f32,
#[allow(unused)]
	thunderAtkRate: f32,
#[allow(unused)]
	staminaAtkRate: f32,
#[allow(unused)]
	saWeaponAtkRate: f32,
#[allow(unused)]
	saDurabilityRate: f32,
#[allow(unused)]
	correctStrengthRate: f32,
#[allow(unused)]
	correctAgilityRate: f32,
#[allow(unused)]
	correctMagicRate: f32,
#[allow(unused)]
	correctFaithRate: f32,
#[allow(unused)]
	physicsGuardCutRate: f32,
#[allow(unused)]
	magicGuardCutRate: f32,
#[allow(unused)]
	fireGuardCutRate: f32,
#[allow(unused)]
	thunderGuardCutRate: f32,
#[allow(unused)]
	poisonGuardResistRate: f32,
#[allow(unused)]
	diseaseGuardResistRate: f32,
#[allow(unused)]
	bloodGuardResistRate: f32,
#[allow(unused)]
	curseGuardResistRate: f32,
#[allow(unused)]
	staminaGuardDefRate: f32,
#[allow(unused)]
	spEffectId1: u8,
#[allow(unused)]
	spEffectId2: u8,
#[allow(unused)]
	spEffectId3: u8,
#[allow(unused)]
	residentSpEffectId1: u8,
#[allow(unused)]
	residentSpEffectId2: u8,
#[allow(unused)]
	residentSpEffectId3: u8,
#[allow(unused)]
	materialSetId: u8,
#[allow(unused)]
	maxReinforceLevel: u8,
#[allow(unused)]
	darkAtkRate: f32,
#[allow(unused)]
	darkGuardCutRate: f32,
#[allow(unused)]
	correctLuckRate: f32,
#[allow(unused)]
	freezeGuardDefRate: f32,
#[allow(unused)]
	reinforcePriceRate: f32,
#[allow(unused)]
	baseChangePriceRate: f32,
#[allow(unused)]
	enableGemRank: i8,
#[allow(unused)]
	pad2: [u8;3],
#[allow(unused)]
	sleepGuardDefRate: f32,
#[allow(unused)]
	madnessGuardDefRate: f32,
#[allow(unused)]
	baseAtkRate: f32,
}

struct RESIST_CORRECT_PARAM_ST {
#[allow(unused)]
	addPoint1: f32,
#[allow(unused)]
	addPoint2: f32,
#[allow(unused)]
	addPoint3: f32,
#[allow(unused)]
	addPoint4: f32,
#[allow(unused)]
	addPoint5: f32,
#[allow(unused)]
	addRate1: f32,
#[allow(unused)]
	addRate2: f32,
#[allow(unused)]
	addRate3: f32,
#[allow(unused)]
	addRate4: f32,
#[allow(unused)]
	addRate5: f32,
}

struct REVERB_AUX_SEND_BUS_PARAM_ST {
#[allow(unused)]
	ReverbAuxSendBusName: [u8;32],
}

struct RIDE_PARAM_ST {
#[allow(unused)]
	atkChrId: i32,
#[allow(unused)]
	defChrId: i32,
#[allow(unused)]
	rideCamParamId: i32,
#[allow(unused)]
	atkChrAnimId: i32,
#[allow(unused)]
	defChrAnimId: i32,
#[allow(unused)]
	defAdjustDmyId: i32,
#[allow(unused)]
	defCheckDmyId: i32,
#[allow(unused)]
	diffAngMyToDef: f32,
#[allow(unused)]
	dist: f32,
#[allow(unused)]
	upperYRange: f32,
#[allow(unused)]
	lowerYRange: f32,
#[allow(unused)]
	diffAngMin: f32,
#[allow(unused)]
	diffAngMax: f32,
#[allow(unused)]
	pad: [u8;12],
}

struct ROLE_PARAM_ST {
#[allow(unused)]
	teamType: u8,
#[allow(unused)]
	pad10: [u8;3],
#[allow(unused)]
	phantomParamId: i32,
#[allow(unused)]
	spEffectID0: i32,
#[allow(unused)]
	spEffectID1: i32,
#[allow(unused)]
	spEffectID2: i32,
#[allow(unused)]
	spEffectID3: i32,
#[allow(unused)]
	spEffectID4: i32,
#[allow(unused)]
	spEffectID5: i32,
#[allow(unused)]
	spEffectID6: i32,
#[allow(unused)]
	spEffectID7: i32,
#[allow(unused)]
	spEffectID8: i32,
#[allow(unused)]
	spEffectID9: i32,
#[allow(unused)]
	sosSignSfxId: i32,
#[allow(unused)]
	mySosSignSfxId: i32,
#[allow(unused)]
	summonStartAnimId: i32,
#[allow(unused)]
	itemlotParamId: i32,
#[allow(unused)]
	voiceChatGroup: u8,
#[allow(unused)]
	roleNameColor: u8,
#[allow(unused)]
	pad1: [u8;2],
#[allow(unused)]
	roleNameId: i32,
#[allow(unused)]
	threatLv: i32,
#[allow(unused)]
	phantomParamId_vowRank1: i32,
#[allow(unused)]
	phantomParamId_vowRank2: i32,
#[allow(unused)]
	phantomParamId_vowRank3: i32,
#[allow(unused)]
	spEffectID_vowRank0: i32,
#[allow(unused)]
	spEffectID_vowRank1: i32,
#[allow(unused)]
	spEffectID_vowRank2: i32,
#[allow(unused)]
	spEffectID_vowRank3: i32,
#[allow(unused)]
	signPhantomId: i32,
#[allow(unused)]
	nonPlayerSummonStartAnimId: i32,
#[allow(unused)]
	pad2: [u8;16],
}

struct ROLLING_OBJ_LOT_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	AssetId_0: i32,
#[allow(unused)]
	AssetId_1: i32,
#[allow(unused)]
	AssetId_2: i32,
#[allow(unused)]
	AssetId_3: i32,
#[allow(unused)]
	AssetId_4: i32,
#[allow(unused)]
	AssetId_5: i32,
#[allow(unused)]
	AssetId_6: i32,
#[allow(unused)]
	AssetId_7: i32,
#[allow(unused)]
	CreateWeight_0: u8,
#[allow(unused)]
	CreateWeight_1: u8,
#[allow(unused)]
	CreateWeight_2: u8,
#[allow(unused)]
	CreateWeight_3: u8,
#[allow(unused)]
	CreateWeight_4: u8,
#[allow(unused)]
	CreateWeight_5: u8,
#[allow(unused)]
	CreateWeight_6: u8,
#[allow(unused)]
	CreateWeight_7: u8,
#[allow(unused)]
	Reserve_0: [u8;20],
}

struct RUNTIME_BONE_CONTROL_PARAM_ST {
#[allow(unused)]
	chrId: i32,
#[allow(unused)]
	ctrlType: u8,
#[allow(unused)]
	pad: [u8;11],
#[allow(unused)]
	applyBone: [u8;32],
#[allow(unused)]
	targetBone1: [u8;32],
#[allow(unused)]
	targetBone2: [u8;32],
}

struct SE_ACTIVATION_RANGE_PARAM_ST {
#[allow(unused)]
	activateRange: f32,
}

struct SE_MATERIAL_CONVERT_PARAM_ST {
#[allow(unused)]
	seMaterialId: u8,
#[allow(unused)]
	pad: [u8;3],
}

struct SFX_BLOCK_RES_SHARE_PARAM {
#[allow(unused)]
	shareBlockRsMapUidVal: i32,
}

struct SHOP_LINEUP_PARAM {
#[allow(unused)]
	equipId: i32,
#[allow(unused)]
	value: i32,
#[allow(unused)]
	mtrlId: i32,
#[allow(unused)]
	eventFlag_forStock: i32,
#[allow(unused)]
	eventFlag_forRelease: i32,
#[allow(unused)]
	sellQuantity: i16,
#[allow(unused)]
	pad3: [u8;1],
#[allow(unused)]
	equipType: u8,
#[allow(unused)]
	costType: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	setNum: i16,
#[allow(unused)]
	value_Add: i32,
#[allow(unused)]
	value_Magnification: f32,
#[allow(unused)]
	iconId: i32,
#[allow(unused)]
	nameMsgId: i32,
#[allow(unused)]
	menuTitleMsgId: i32,
#[allow(unused)]
	menuIconId: i16,
#[allow(unused)]
	pad2: [u8;2],
}

struct SIGN_PUDDLE_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	matchAreaId: i32,
#[allow(unused)]
	pad1: [u8;24],
}

struct SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST {
#[allow(unused)]
	SoundObjEnableDist: f32,
}

struct SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST {
#[allow(unused)]
	SoundNo: i32,
#[allow(unused)]
	ExpandRange: f32,
#[allow(unused)]
	FollowSpeed: f32,
#[allow(unused)]
	FollowRate: f32,
}

struct SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST {
#[allow(unused)]
	NoHitDist: f32,
#[allow(unused)]
	isCollectNoHitPoint: u8,
#[allow(unused)]
	isCollectOutdoorPoint: u8,
#[allow(unused)]
	isCollectFloorPoint: u8,
#[allow(unused)]
	distValCalcType: u8,
#[allow(unused)]
	enableLifeTime: f32,
#[allow(unused)]
	maxDistRecordNum: i32,
#[allow(unused)]
	ignoreDistNumForMax: i32,
}

struct SOUND_AUTO_REVERB_SELECT_PARAM_ST {
#[allow(unused)]
	reverbType: i32,
#[allow(unused)]
	AreaNo: i32,
#[allow(unused)]
	IndoorOutdoor: i8,
#[allow(unused)]
	useDistNoA: i8,
#[allow(unused)]
	useDistNoB: i8,
#[allow(unused)]
	pad0: [u8;1],
#[allow(unused)]
	DistMinA: f32,
#[allow(unused)]
	DistMaxA: f32,
#[allow(unused)]
	DistMinB: f32,
#[allow(unused)]
	DistMaxB: f32,
#[allow(unused)]
	NoHitNumMin: i32,
}

struct SOUND_CHR_PHYSICS_SE_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	ContactLandSeId: i32,
#[allow(unused)]
	ContactLandAddSeId: i32,
#[allow(unused)]
	ContactLandPlayNum: i32,
#[allow(unused)]
	IsEnablePlayCountPerRigid: u8,
#[allow(unused)]
	pad: [u8;3],
#[allow(unused)]
	ContactLandMinImpuse: f32,
#[allow(unused)]
	ContactLandMinSpeed: f32,
#[allow(unused)]
	ContactPlayerSeId: i32,
#[allow(unused)]
	ContactPlayerMinImpuse: f32,
#[allow(unused)]
	ContactPlayerMinSpeed: f32,
#[allow(unused)]
	ContactCheckRigidIdx0: i8,
#[allow(unused)]
	ContactCheckRigidIdx1: i8,
#[allow(unused)]
	ContactCheckRigidIdx2: i8,
#[allow(unused)]
	ContactCheckRigidIdx3: i8,
#[allow(unused)]
	ContactCheckRigidIdx4: i8,
#[allow(unused)]
	ContactCheckRigidIdx5: i8,
#[allow(unused)]
	ContactCheckRigidIdx6: i8,
#[allow(unused)]
	ContactCheckRigidIdx7: i8,
#[allow(unused)]
	ContactCheckRigidIdx8: i8,
#[allow(unused)]
	ContactCheckRigidIdx9: i8,
#[allow(unused)]
	ContactCheckRigidIdx10: i8,
#[allow(unused)]
	ContactCheckRigidIdx11: i8,
#[allow(unused)]
	ContactCheckRigidIdx12: i8,
#[allow(unused)]
	ContactCheckRigidIdx13: i8,
#[allow(unused)]
	ContactCheckRigidIdx14: i8,
#[allow(unused)]
	ContactCheckRigidIdx15: i8,
}

struct SOUND_COMMON_INGAME_PARAM_ST {
#[allow(unused)]
	ParamKeyStr: [u8;32],
#[allow(unused)]
	ParamValueStr: [u8;32],
}

struct SOUND_COMMON_SYSTEM_PARAM_ST {
#[allow(unused)]
	ParamKeyStr: [u8;32],
#[allow(unused)]
	ParamValueStr: [u8;32],
}

struct SOUND_CUTSCENE_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	ReverbType: u8,
#[allow(unused)]
	pad0: [u8;3],
#[allow(unused)]
	BgmBehaviorTypeOnStart: i16,
#[allow(unused)]
	OneShotBgmBehaviorOnStart: i16,
#[allow(unused)]
	PostPlaySeId: i32,
#[allow(unused)]
	PostPlaySeIdForSkip: i32,
#[allow(unused)]
	EnterMapMuteStopTimeOnDrawCutscene: f32,
#[allow(unused)]
	reserved: [u8;8],
#[allow(unused)]
	reserved2: [u8;4],
}

struct SPEEDTREE_MODEL_PARAM_ST {
#[allow(unused)]
	MinFadeLeaf: f32,
#[allow(unused)]
	MinFadeFrond: f32,
#[allow(unused)]
	MinFadeBranch: f32,
#[allow(unused)]
	MinTranslucencyLeaf: f32,
#[allow(unused)]
	MaxTranslucencyLeaf: f32,
#[allow(unused)]
	MinTranslucencyFrond: f32,
#[allow(unused)]
	MaxTranslucencyFrond: f32,
#[allow(unused)]
	MinTranslucencyBranch: f32,
#[allow(unused)]
	MaxTranslucencyBranch: f32,
#[allow(unused)]
	BillboardBackSpecularWeakenParam: f32,
}

struct SP_EFFECT_PARAM_ST {
#[allow(unused)]
	iconId: i32,
#[allow(unused)]
	conditionHp: f32,
#[allow(unused)]
	effectEndurance: f32,
#[allow(unused)]
	motionInterval: f32,
#[allow(unused)]
	maxHpRate: f32,
#[allow(unused)]
	maxMpRate: f32,
#[allow(unused)]
	maxStaminaRate: f32,
#[allow(unused)]
	slashDamageCutRate: f32,
#[allow(unused)]
	blowDamageCutRate: f32,
#[allow(unused)]
	thrustDamageCutRate: f32,
#[allow(unused)]
	neutralDamageCutRate: f32,
#[allow(unused)]
	magicDamageCutRate: f32,
#[allow(unused)]
	fireDamageCutRate: f32,
#[allow(unused)]
	thunderDamageCutRate: f32,
#[allow(unused)]
	physicsAttackRate: f32,
#[allow(unused)]
	magicAttackRate: f32,
#[allow(unused)]
	fireAttackRate: f32,
#[allow(unused)]
	thunderAttackRate: f32,
#[allow(unused)]
	physicsAttackPowerRate: f32,
#[allow(unused)]
	magicAttackPowerRate: f32,
#[allow(unused)]
	fireAttackPowerRate: f32,
#[allow(unused)]
	thunderAttackPowerRate: f32,
#[allow(unused)]
	physicsAttackPower: i32,
#[allow(unused)]
	magicAttackPower: i32,
#[allow(unused)]
	fireAttackPower: i32,
#[allow(unused)]
	thunderAttackPower: i32,
#[allow(unused)]
	physicsDiffenceRate: f32,
#[allow(unused)]
	magicDiffenceRate: f32,
#[allow(unused)]
	fireDiffenceRate: f32,
#[allow(unused)]
	thunderDiffenceRate: f32,
#[allow(unused)]
	physicsDiffence: i32,
#[allow(unused)]
	magicDiffence: i32,
#[allow(unused)]
	fireDiffence: i32,
#[allow(unused)]
	thunderDiffence: i32,
#[allow(unused)]
	NoGuardDamageRate: f32,
#[allow(unused)]
	vitalSpotChangeRate: f32,
#[allow(unused)]
	normalSpotChangeRate: f32,
#[allow(unused)]
	lookAtTargetPosOffset: f32,
#[allow(unused)]
	behaviorId: i32,
#[allow(unused)]
	changeHpRate: f32,
#[allow(unused)]
	changeHpPoint: i32,
#[allow(unused)]
	changeMpRate: f32,
#[allow(unused)]
	changeMpPoint: i32,
#[allow(unused)]
	mpRecoverChangeSpeed: i32,
#[allow(unused)]
	changeStaminaRate: f32,
#[allow(unused)]
	changeStaminaPoint: i32,
#[allow(unused)]
	staminaRecoverChangeSpeed: i32,
#[allow(unused)]
	magicEffectTimeChange: f32,
#[allow(unused)]
	insideDurability: i32,
#[allow(unused)]
	maxDurability: i32,
#[allow(unused)]
	staminaAttackRate: f32,
#[allow(unused)]
	poizonAttackPower: i32,
#[allow(unused)]
	diseaseAttackPower: i32,
#[allow(unused)]
	bloodAttackPower: i32,
#[allow(unused)]
	curseAttackPower: i32,
#[allow(unused)]
	fallDamageRate: f32,
#[allow(unused)]
	soulRate: f32,
#[allow(unused)]
	equipWeightChangeRate: f32,
#[allow(unused)]
	allItemWeightChangeRate: f32,
#[allow(unused)]
	soul: i32,
#[allow(unused)]
	animIdOffset: i32,
#[allow(unused)]
	haveSoulRate: f32,
#[allow(unused)]
	targetPriority: f32,
#[allow(unused)]
	sightSearchEnemyRate: f32,
#[allow(unused)]
	hearingSearchEnemyRate: f32,
#[allow(unused)]
	grabityRate: f32,
#[allow(unused)]
	registPoizonChangeRate: f32,
#[allow(unused)]
	registDiseaseChangeRate: f32,
#[allow(unused)]
	registBloodChangeRate: f32,
#[allow(unused)]
	registCurseChangeRate: f32,
#[allow(unused)]
	soulStealRate: f32,
#[allow(unused)]
	lifeReductionRate: f32,
#[allow(unused)]
	hpRecoverRate: f32,
#[allow(unused)]
	replaceSpEffectId: i32,
#[allow(unused)]
	cycleOccurrenceSpEffectId: i32,
#[allow(unused)]
	atkOccurrenceSpEffectId: i32,
#[allow(unused)]
	guardDefFlickPowerRate: f32,
#[allow(unused)]
	guardStaminaCutRate: f32,
#[allow(unused)]
	rayCastPassedTime: i16,
#[allow(unused)]
	magicSubCategoryChange1: u8,
#[allow(unused)]
	magicSubCategoryChange2: u8,
#[allow(unused)]
	bowDistRate: i16,
#[allow(unused)]
	spCategory: i16,
#[allow(unused)]
	categoryPriority: u8,
#[allow(unused)]
	saveCategory: i8,
#[allow(unused)]
	changeMagicSlot: u8,
#[allow(unused)]
	changeMiracleSlot: u8,
#[allow(unused)]
	heroPointDamage: i8,
#[allow(unused)]
	defFlickPower: u8,
#[allow(unused)]
	flickDamageCutRate: u8,
#[allow(unused)]
	bloodDamageRate: u8,
#[allow(unused)]
	dmgLv_None: i8,
#[allow(unused)]
	dmgLv_S: i8,
#[allow(unused)]
	dmgLv_M: i8,
#[allow(unused)]
	dmgLv_L: i8,
#[allow(unused)]
	dmgLv_BlowM: i8,
#[allow(unused)]
	dmgLv_Push: i8,
#[allow(unused)]
	dmgLv_Strike: i8,
#[allow(unused)]
	dmgLv_BlowS: i8,
#[allow(unused)]
	dmgLv_Min: i8,
#[allow(unused)]
	dmgLv_Uppercut: i8,
#[allow(unused)]
	dmgLv_BlowLL: i8,
#[allow(unused)]
	dmgLv_Breath: i8,
#[allow(unused)]
	atkAttribute: u8,
#[allow(unused)]
	spAttribute: u8,
#[allow(unused)]
	stateInfo: i16,
#[allow(unused)]
	wepParamChange: u8,
#[allow(unused)]
	moveType: u8,
#[allow(unused)]
	lifeReductionType: i16,
#[allow(unused)]
	throwCondition: u8,
#[allow(unused)]
	addBehaviorJudgeId_condition: i8,
#[allow(unused)]
	freezeDamageRate: u8,
#[allow(unused)]
	effectTargetSelf: u8,
#[allow(unused)]
	effectTargetFriend: u8,
#[allow(unused)]
	effectTargetEnemy: u8,
#[allow(unused)]
	effectTargetPlayer: u8,
#[allow(unused)]
	effectTargetAI: u8,
#[allow(unused)]
	effectTargetLive: u8,
#[allow(unused)]
	effectTargetGhost: u8,
#[allow(unused)]
	disableSleep: u8,
#[allow(unused)]
	disableMadness: u8,
#[allow(unused)]
	effectTargetAttacker: u8,
#[allow(unused)]
	dispIconNonactive: u8,
#[allow(unused)]
	regainGaugeDamage: u8,
#[allow(unused)]
	bAdjustMagicAblity: u8,
#[allow(unused)]
	bAdjustFaithAblity: u8,
#[allow(unused)]
	bGameClearBonus: u8,
#[allow(unused)]
	magParamChange: u8,
#[allow(unused)]
	miracleParamChange: u8,
#[allow(unused)]
	clearSoul: u8,
#[allow(unused)]
	requestSOS: u8,
#[allow(unused)]
	requestBlackSOS: u8,
#[allow(unused)]
	requestForceJoinBlackSOS: u8,
#[allow(unused)]
	requestKickSession: u8,
#[allow(unused)]
	requestLeaveSession: u8,
#[allow(unused)]
	requestNpcInveda: u8,
#[allow(unused)]
	noDead: u8,
#[allow(unused)]
	bCurrHPIndependeMaxHP: u8,
#[allow(unused)]
	corrosionIgnore: u8,
#[allow(unused)]
	sightSearchCutIgnore: u8,
#[allow(unused)]
	hearingSearchCutIgnore: u8,
#[allow(unused)]
	antiMagicIgnore: u8,
#[allow(unused)]
	fakeTargetIgnore: u8,
#[allow(unused)]
	fakeTargetIgnoreUndead: u8,
#[allow(unused)]
	fakeTargetIgnoreAnimal: u8,
#[allow(unused)]
	grabityIgnore: u8,
#[allow(unused)]
	disablePoison: u8,
#[allow(unused)]
	disableDisease: u8,
#[allow(unused)]
	disableBlood: u8,
#[allow(unused)]
	disableCurse: u8,
#[allow(unused)]
	enableCharm: u8,
#[allow(unused)]
	enableLifeTime: u8,
#[allow(unused)]
	bAdjustStrengthAblity: u8,
#[allow(unused)]
	bAdjustAgilityAblity: u8,
#[allow(unused)]
	eraseOnBonfireRecover: u8,
#[allow(unused)]
	throwAttackParamChange: u8,
#[allow(unused)]
	requestLeaveColiseumSession: u8,
#[allow(unused)]
	isExtendSpEffectLife: u8,
#[allow(unused)]
	hasTarget: u8,
#[allow(unused)]
	replanningOnFire: u8,
#[allow(unused)]
	vowType0: u8,
#[allow(unused)]
	vowType1: u8,
#[allow(unused)]
	vowType2: u8,
#[allow(unused)]
	vowType3: u8,
#[allow(unused)]
	vowType4: u8,
#[allow(unused)]
	vowType5: u8,
#[allow(unused)]
	vowType6: u8,
#[allow(unused)]
	vowType7: u8,
#[allow(unused)]
	vowType8: u8,
#[allow(unused)]
	vowType9: u8,
#[allow(unused)]
	vowType10: u8,
#[allow(unused)]
	vowType11: u8,
#[allow(unused)]
	vowType12: u8,
#[allow(unused)]
	vowType13: u8,
#[allow(unused)]
	vowType14: u8,
#[allow(unused)]
	vowType15: u8,
#[allow(unused)]
	repAtkDmgLv: i8,
#[allow(unused)]
	sightSearchRate: f32,
#[allow(unused)]
	effectTargetOpposeTarget: u8,
#[allow(unused)]
	effectTargetFriendlyTarget: u8,
#[allow(unused)]
	effectTargetSelfTarget: u8,
#[allow(unused)]
	effectTargetPcHorse: u8,
#[allow(unused)]
	effectTargetPcDeceased: u8,
#[allow(unused)]
	isContractSpEffectLife: u8,
#[allow(unused)]
	isWaitModeDelete: u8,
#[allow(unused)]
	isIgnoreNoDamage: u8,
#[allow(unused)]
	changeTeamType: i8,
#[allow(unused)]
	dmypolyId: i16,
#[allow(unused)]
	vfxId: i32,
#[allow(unused)]
	accumuOverFireId: i32,
#[allow(unused)]
	accumuOverVal: i32,
#[allow(unused)]
	accumuUnderFireId: i32,
#[allow(unused)]
	accumuUnderVal: i32,
#[allow(unused)]
	accumuVal: i32,
#[allow(unused)]
	eye_angX: u8,
#[allow(unused)]
	eye_angY: u8,
#[allow(unused)]
	addDeceasedLv: i16,
#[allow(unused)]
	vfxId1: i32,
#[allow(unused)]
	vfxId2: i32,
#[allow(unused)]
	vfxId3: i32,
#[allow(unused)]
	vfxId4: i32,
#[allow(unused)]
	vfxId5: i32,
#[allow(unused)]
	vfxId6: i32,
#[allow(unused)]
	vfxId7: i32,
#[allow(unused)]
	freezeAttackPower: i32,
#[allow(unused)]
	AppearAiSoundId: i32,
#[allow(unused)]
	addFootEffectSfxId: i16,
#[allow(unused)]
	dexterityCancelSystemOnlyAddDexterity: i8,
#[allow(unused)]
	teamOffenseEffectivity: i8,
#[allow(unused)]
	toughnessDamageCutRate: f32,
#[allow(unused)]
	weakDmgRateA: f32,
#[allow(unused)]
	weakDmgRateB: f32,
#[allow(unused)]
	weakDmgRateC: f32,
#[allow(unused)]
	weakDmgRateD: f32,
#[allow(unused)]
	weakDmgRateE: f32,
#[allow(unused)]
	weakDmgRateF: f32,
#[allow(unused)]
	darkDamageCutRate: f32,
#[allow(unused)]
	darkDiffenceRate: f32,
#[allow(unused)]
	darkDiffence: i32,
#[allow(unused)]
	darkAttackRate: f32,
#[allow(unused)]
	darkAttackPowerRate: f32,
#[allow(unused)]
	darkAttackPower: i32,
#[allow(unused)]
	antiDarkSightRadius: f32,
#[allow(unused)]
	antiDarkSightDmypolyId: i32,
#[allow(unused)]
	conditionHpRate: f32,
#[allow(unused)]
	consumeStaminaRate: f32,
#[allow(unused)]
	itemDropRate: f32,
#[allow(unused)]
	changePoisonResistPoint: i32,
#[allow(unused)]
	changeDiseaseResistPoint: i32,
#[allow(unused)]
	changeBloodResistPoint: i32,
#[allow(unused)]
	changeCurseResistPoint: i32,
#[allow(unused)]
	changeFreezeResistPoint: i32,
#[allow(unused)]
	slashAttackRate: f32,
#[allow(unused)]
	blowAttackRate: f32,
#[allow(unused)]
	thrustAttackRate: f32,
#[allow(unused)]
	neutralAttackRate: f32,
#[allow(unused)]
	slashAttackPowerRate: f32,
#[allow(unused)]
	blowAttackPowerRate: f32,
#[allow(unused)]
	thrustAttackPowerRate: f32,
#[allow(unused)]
	neutralAttackPowerRate: f32,
#[allow(unused)]
	slashAttackPower: i32,
#[allow(unused)]
	blowAttackPower: i32,
#[allow(unused)]
	thrustAttackPower: i32,
#[allow(unused)]
	neutralAttackPower: i32,
#[allow(unused)]
	changeStrengthPoint: i32,
#[allow(unused)]
	changeAgilityPoint: i32,
#[allow(unused)]
	changeMagicPoint: i32,
#[allow(unused)]
	changeFaithPoint: i32,
#[allow(unused)]
	changeLuckPoint: i32,
#[allow(unused)]
	recoverArtsPoint_Str: i8,
#[allow(unused)]
	recoverArtsPoint_Dex: i8,
#[allow(unused)]
	recoverArtsPoint_Magic: i8,
#[allow(unused)]
	recoverArtsPoint_Miracle: i8,
#[allow(unused)]
	madnessDamageRate: u8,
#[allow(unused)]
	isUseStatusAilmentAtkPowerCorrect: u8,
#[allow(unused)]
	isUseAtkParamAtkPowerCorrect: u8,
#[allow(unused)]
	dontDeleteOnDead: u8,
#[allow(unused)]
	disableFreeze: u8,
#[allow(unused)]
	isDisableNetSync: u8,
#[allow(unused)]
	shamanParamChange: u8,
#[allow(unused)]
	isStopSearchedNotify: u8,
#[allow(unused)]
	isCheckAboveShadowTest: u8,
#[allow(unused)]
	addBehaviorJudgeId_add: i16,
#[allow(unused)]
	saReceiveDamageRate: f32,
#[allow(unused)]
	defPlayerDmgCorrectRate_Physics: f32,
#[allow(unused)]
	defPlayerDmgCorrectRate_Magic: f32,
#[allow(unused)]
	defPlayerDmgCorrectRate_Fire: f32,
#[allow(unused)]
	defPlayerDmgCorrectRate_Thunder: f32,
#[allow(unused)]
	defPlayerDmgCorrectRate_Dark: f32,
#[allow(unused)]
	defEnemyDmgCorrectRate_Physics: f32,
#[allow(unused)]
	defEnemyDmgCorrectRate_Magic: f32,
#[allow(unused)]
	defEnemyDmgCorrectRate_Fire: f32,
#[allow(unused)]
	defEnemyDmgCorrectRate_Thunder: f32,
#[allow(unused)]
	defEnemyDmgCorrectRate_Dark: f32,
#[allow(unused)]
	defObjDmgCorrectRate: f32,
#[allow(unused)]
	atkPlayerDmgCorrectRate_Physics: f32,
#[allow(unused)]
	atkPlayerDmgCorrectRate_Magic: f32,
#[allow(unused)]
	atkPlayerDmgCorrectRate_Fire: f32,
#[allow(unused)]
	atkPlayerDmgCorrectRate_Thunder: f32,
#[allow(unused)]
	atkPlayerDmgCorrectRate_Dark: f32,
#[allow(unused)]
	atkEnemyDmgCorrectRate_Physics: f32,
#[allow(unused)]
	atkEnemyDmgCorrectRate_Magic: f32,
#[allow(unused)]
	atkEnemyDmgCorrectRate_Fire: f32,
#[allow(unused)]
	atkEnemyDmgCorrectRate_Thunder: f32,
#[allow(unused)]
	atkEnemyDmgCorrectRate_Dark: f32,
#[allow(unused)]
	registFreezeChangeRate: f32,
#[allow(unused)]
	invocationConditionsStateChange1: i16,
#[allow(unused)]
	invocationConditionsStateChange2: i16,
#[allow(unused)]
	invocationConditionsStateChange3: i16,
#[allow(unused)]
	hearingAiSoundLevel: i16,
#[allow(unused)]
	chrProxyHeightRate: f32,
#[allow(unused)]
	addAwarePointCorrectValue_forMe: f32,
#[allow(unused)]
	addAwarePointCorrectValue_forTarget: f32,
#[allow(unused)]
	sightSearchEnemyAdd: f32,
#[allow(unused)]
	sightSearchAdd: f32,
#[allow(unused)]
	hearingSearchAdd: f32,
#[allow(unused)]
	hearingSearchRate: f32,
#[allow(unused)]
	hearingSearchEnemyAdd: f32,
#[allow(unused)]
	value_Magnification: f32,
#[allow(unused)]
	artsConsumptionRate: f32,
#[allow(unused)]
	magicConsumptionRate: f32,
#[allow(unused)]
	shamanConsumptionRate: f32,
#[allow(unused)]
	miracleConsumptionRate: f32,
#[allow(unused)]
	changeHpEstusFlaskRate: i32,
#[allow(unused)]
	changeHpEstusFlaskPoint: i32,
#[allow(unused)]
	changeMpEstusFlaskRate: i32,
#[allow(unused)]
	changeMpEstusFlaskPoint: i32,
#[allow(unused)]
	changeHpEstusFlaskCorrectRate: f32,
#[allow(unused)]
	changeMpEstusFlaskCorrectRate: f32,
#[allow(unused)]
	applyIdOnGetSoul: i32,
#[allow(unused)]
	extendLifeRate: f32,
#[allow(unused)]
	contractLifeRate: f32,
#[allow(unused)]
	defObjectAttackPowerRate: f32,
#[allow(unused)]
	effectEndDeleteDecalGroupId: i16,
#[allow(unused)]
	addLifeForceStatus: i8,
#[allow(unused)]
	addWillpowerStatus: i8,
#[allow(unused)]
	addEndureStatus: i8,
#[allow(unused)]
	addVitalityStatus: i8,
#[allow(unused)]
	addStrengthStatus: i8,
#[allow(unused)]
	addDexterityStatus: i8,
#[allow(unused)]
	addMagicStatus: i8,
#[allow(unused)]
	addFaithStatus: i8,
#[allow(unused)]
	addLuckStatus: i8,
#[allow(unused)]
	deleteCriteriaDamage: u8,
#[allow(unused)]
	magicSubCategoryChange3: u8,
#[allow(unused)]
	spAttributeVariationValue: u8,
#[allow(unused)]
	atkFlickPower: u8,
#[allow(unused)]
	wetConditionDepth: u8,
#[allow(unused)]
	changeSaRecoveryVelocity: f32,
#[allow(unused)]
	regainRate: f32,
#[allow(unused)]
	saAttackPowerRate: f32,
#[allow(unused)]
	sleepAttackPower: i32,
#[allow(unused)]
	madnessAttackPower: i32,
#[allow(unused)]
	registSleepChangeRate: f32,
#[allow(unused)]
	registMadnessChangeRate: f32,
#[allow(unused)]
	changeSleepResistPoint: i32,
#[allow(unused)]
	changeMadnessResistPoint: i32,
#[allow(unused)]
	sleepDamageRate: u8,
#[allow(unused)]
	applyPartsGroup: u8,
#[allow(unused)]
	clearTarget: u8,
#[allow(unused)]
	fakeTargetIgnoreAjin: u8,
#[allow(unused)]
	fakeTargetIgnoreMirageArts: u8,
#[allow(unused)]
	requestForceJoinBlackSOS_B: u8,
#[allow(unused)]
	isDestinedDeathHpMult: u8,
#[allow(unused)]
	pad2: [u8;1],
#[allow(unused)]
	changeSuperArmorPoint: f32,
#[allow(unused)]
	changeSaPoint: f32,
#[allow(unused)]
	hugeEnemyPickupHeightOverwrite: f32,
#[allow(unused)]
	poisonDefDamageRate: f32,
#[allow(unused)]
	diseaseDefDamageRate: f32,
#[allow(unused)]
	bloodDefDamageRate: f32,
#[allow(unused)]
	curseDefDamageRate: f32,
#[allow(unused)]
	freezeDefDamageRate: f32,
#[allow(unused)]
	sleepDefDamageRate: f32,
#[allow(unused)]
	madnessDefDamageRate: f32,
#[allow(unused)]
	overwrite_maxBackhomeDist: i16,
#[allow(unused)]
	overwrite_backhomeDist: i16,
#[allow(unused)]
	overwrite_backhomeBattleDist: i16,
#[allow(unused)]
	overwrite_BackHome_LookTargetDist: i16,
#[allow(unused)]
	goodsConsumptionRate: f32,
#[allow(unused)]
	guardStaminaMult: f32,
#[allow(unused)]
	unk3: [u8;4],
}

struct SP_EFFECT_SET_PARAM_ST {
#[allow(unused)]
	spEffectId1: i32,
#[allow(unused)]
	spEffectId2: i32,
#[allow(unused)]
	spEffectId3: i32,
#[allow(unused)]
	spEffectId4: i32,
}

struct SP_EFFECT_VFX_PARAM_ST {
#[allow(unused)]
	midstSfxId: i32,
#[allow(unused)]
	midstSeId: i32,
#[allow(unused)]
	initSfxId: i32,
#[allow(unused)]
	initSeId: i32,
#[allow(unused)]
	finishSfxId: i32,
#[allow(unused)]
	finishSeId: i32,
#[allow(unused)]
	camouflageBeginDist: f32,
#[allow(unused)]
	camouflageEndDist: f32,
#[allow(unused)]
	transformProtectorId: i32,
#[allow(unused)]
	midstDmyId: i16,
#[allow(unused)]
	initDmyId: i16,
#[allow(unused)]
	finishDmyId: i16,
#[allow(unused)]
	effectType: u8,
#[allow(unused)]
	soulParamIdForWepEnchant: u8,
#[allow(unused)]
	playCategory: u8,
#[allow(unused)]
	playPriority: u8,
#[allow(unused)]
	existEffectForLarge: u8,
#[allow(unused)]
	existEffectForSoul: u8,
#[allow(unused)]
	effectInvisibleAtCamouflage: u8,
#[allow(unused)]
	useCamouflage: u8,
#[allow(unused)]
	invisibleAtFriendCamouflage: u8,
#[allow(unused)]
	isHideFootEffect_forCamouflage: u8,
#[allow(unused)]
	halfCamouflage: u8,
#[allow(unused)]
	isFullBodyTransformProtectorId: u8,
#[allow(unused)]
	isInvisibleWeapon: u8,
#[allow(unused)]
	isSilence: u8,
#[allow(unused)]
	isMidstFullbody: u8,
#[allow(unused)]
	isInitFullbody: u8,
#[allow(unused)]
	isFinishFullbody: u8,
#[allow(unused)]
	isVisibleDeadChr: u8,
#[allow(unused)]
	isUseOffsetEnchantSfxSize: u8,
#[allow(unused)]
	pad_1: [u8;1],
#[allow(unused)]
	decalId1: i32,
#[allow(unused)]
	decalId2: i32,
#[allow(unused)]
	footEffectPriority: u8,
#[allow(unused)]
	footEffectOffset: u8,
#[allow(unused)]
	traceSfxIdOffsetType: u8,
#[allow(unused)]
	forceDeceasedType: u8,
#[allow(unused)]
	enchantStartDmyId_0: i32,
#[allow(unused)]
	enchantEndDmyId_0: i32,
#[allow(unused)]
	enchantStartDmyId_1: i32,
#[allow(unused)]
	enchantEndDmyId_1: i32,
#[allow(unused)]
	enchantStartDmyId_2: i32,
#[allow(unused)]
	enchantEndDmyId_2: i32,
#[allow(unused)]
	enchantStartDmyId_3: i32,
#[allow(unused)]
	enchantEndDmyId_3: i32,
#[allow(unused)]
	enchantStartDmyId_4: i32,
#[allow(unused)]
	enchantEndDmyId_4: i32,
#[allow(unused)]
	enchantStartDmyId_5: i32,
#[allow(unused)]
	enchantEndDmyId_5: i32,
#[allow(unused)]
	enchantStartDmyId_6: i32,
#[allow(unused)]
	enchantEndDmyId_6: i32,
#[allow(unused)]
	enchantStartDmyId_7: i32,
#[allow(unused)]
	enchantEndDmyId_7: i32,
#[allow(unused)]
	SfxIdOffsetType: u8,
#[allow(unused)]
	phantomParamOverwriteType: u8,
#[allow(unused)]
	camouflageMinAlpha: u8,
#[allow(unused)]
	wetAspectType: u8,
#[allow(unused)]
	phantomParamOverwriteId: i32,
#[allow(unused)]
	materialParamId: i32,
#[allow(unused)]
	materialParamInitValue: f32,
#[allow(unused)]
	materialParamTargetValue: f32,
#[allow(unused)]
	materialParamFadeTime: f32,
#[allow(unused)]
	footDecalMaterialOffsetOverwriteId: i16,
#[allow(unused)]
	pad: [u8;14],
}

struct SWORD_ARTS_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	swordArtsType: u8,
#[allow(unused)]
	artsSpeedType: u8,
#[allow(unused)]
	refStatus: i8,
#[allow(unused)]
	isRefRightArts: u8,
#[allow(unused)]
	isGrayoutLeftHand: u8,
#[allow(unused)]
	isGrayoutRightHand: u8,
#[allow(unused)]
	isGrayoutBothHand: u8,
#[allow(unused)]
	reserve2: [u8;1],
#[allow(unused)]
	usePoint_L1: i8,
#[allow(unused)]
	usePoint_L2: i8,
#[allow(unused)]
	usePoint_R1: i8,
#[allow(unused)]
	usePoint_R2: i8,
#[allow(unused)]
	textId: i32,
#[allow(unused)]
	useMagicPoint_L1: i16,
#[allow(unused)]
	useMagicPoint_L2: i16,
#[allow(unused)]
	useMagicPoint_R1: i16,
#[allow(unused)]
	useMagicPoint_R2: i16,
#[allow(unused)]
	shieldIconType: i8,
#[allow(unused)]
	swordArtsTypeNew: u8,
#[allow(unused)]
	pad: [u8;1],
#[allow(unused)]
	iconId: i16,
#[allow(unused)]
	aiUsageId: i32,
}

struct TALK_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	msgId: i32,
#[allow(unused)]
	voiceId: i32,
#[allow(unused)]
	spEffectId0: i32,
#[allow(unused)]
	motionId0: i32,
#[allow(unused)]
	spEffectId1: i32,
#[allow(unused)]
	motionId1: i32,
#[allow(unused)]
	returnPos: i32,
#[allow(unused)]
	reactionId: i32,
#[allow(unused)]
	eventId: i32,
#[allow(unused)]
	msgId_female: i32,
#[allow(unused)]
	voiceId_female: i32,
#[allow(unused)]
	lipSyncStart: i16,
#[allow(unused)]
	lipSyncTime: i16,
#[allow(unused)]
	pad2: [u8;4],
#[allow(unused)]
	timeout: f32,
#[allow(unused)]
	talkAnimationId: i32,
#[allow(unused)]
	isForceDisp: u8,
#[allow(unused)]
	pad3: [u8;1],
#[allow(unused)]
	pad1: [u8;31],
}

struct THROW_DIRECTION_SFX_PARAM_ST {
#[allow(unused)]
	sfxId_00: i32,
#[allow(unused)]
	sfxId_01: i32,
#[allow(unused)]
	sfxId_02: i32,
#[allow(unused)]
	sfxId_03: i32,
#[allow(unused)]
	sfxId_04: i32,
#[allow(unused)]
	sfxId_05: i32,
#[allow(unused)]
	sfxId_06: i32,
#[allow(unused)]
	sfxId_07: i32,
#[allow(unused)]
	sfxId_08: i32,
#[allow(unused)]
	sfxId_09: i32,
#[allow(unused)]
	sfxId_10: i32,
#[allow(unused)]
	sfxId_11: i32,
#[allow(unused)]
	sfxId_12: i32,
#[allow(unused)]
	sfxId_13: i32,
#[allow(unused)]
	sfxId_14: i32,
#[allow(unused)]
	sfxId_15: i32,
#[allow(unused)]
	sfxId_16: i32,
#[allow(unused)]
	sfxId_17: i32,
#[allow(unused)]
	sfxId_18: i32,
#[allow(unused)]
	sfxId_19: i32,
#[allow(unused)]
	sfxId_20: i32,
#[allow(unused)]
	sfxId_21: i32,
#[allow(unused)]
	sfxId_22: i32,
#[allow(unused)]
	sfxId_23: i32,
#[allow(unused)]
	sfxId_24: i32,
#[allow(unused)]
	sfxId_25: i32,
#[allow(unused)]
	sfxId_26: i32,
#[allow(unused)]
	sfxId_27: i32,
#[allow(unused)]
	sfxId_28: i32,
#[allow(unused)]
	sfxId_29: i32,
#[allow(unused)]
	sfxId_30: i32,
#[allow(unused)]
	pad1: [u8;20],
}

struct THROW_PARAM_ST {
#[allow(unused)]
	AtkChrId: i32,
#[allow(unused)]
	DefChrId: i32,
#[allow(unused)]
	Dist: f32,
#[allow(unused)]
	DiffAngMin: f32,
#[allow(unused)]
	DiffAngMax: f32,
#[allow(unused)]
	upperYRange: f32,
#[allow(unused)]
	lowerYRange: f32,
#[allow(unused)]
	diffAngMyToDef: f32,
#[allow(unused)]
	throwTypeId: i32,
#[allow(unused)]
	atkAnimId: i32,
#[allow(unused)]
	defAnimId: i32,
#[allow(unused)]
	escHp: i16,
#[allow(unused)]
	selfEscCycleTime: i16,
#[allow(unused)]
	sphereCastRadiusRateTop: i16,
#[allow(unused)]
	sphereCastRadiusRateLow: i16,
#[allow(unused)]
	PadType: u8,
#[allow(unused)]
	AtkEnableState: u8,
#[allow(unused)]
	throwFollowingType: u8,
#[allow(unused)]
	pad2: [u8;1],
#[allow(unused)]
	throwType: u8,
#[allow(unused)]
	selfEscCycleCnt: u8,
#[allow(unused)]
	dmyHasChrDirType: u8,
#[allow(unused)]
	isTurnAtker: u8,
#[allow(unused)]
	isSkipWepCate: u8,
#[allow(unused)]
	isSkipSphereCast: u8,
#[allow(unused)]
	isEnableCorrectPos_forThrowAdjust: u8,
#[allow(unused)]
	isEnableThrowFollowingFallAssist: u8,
#[allow(unused)]
	isEnableThrowFollowingFeedback: u8,
#[allow(unused)]
	pad0: [u8;1],
#[allow(unused)]
	atkSorbDmyId: i16,
#[allow(unused)]
	defSorbDmyId: i16,
#[allow(unused)]
	Dist_start: f32,
#[allow(unused)]
	DiffAngMin_start: f32,
#[allow(unused)]
	DiffAngMax_start: f32,
#[allow(unused)]
	upperYRange_start: f32,
#[allow(unused)]
	lowerYRange_start: f32,
#[allow(unused)]
	diffAngMyToDef_start: f32,
#[allow(unused)]
	judgeRangeBasePosDmyId1: i32,
#[allow(unused)]
	judgeRangeBasePosDmyId2: i32,
#[allow(unused)]
	adsrobModelPosInterpolationTime: f32,
#[allow(unused)]
	throwFollowingEndEasingTime: f32,
#[allow(unused)]
	pad1: [u8;24],
}

struct TOUGHNESS_PARAM_ST {
#[allow(unused)]
	correctionRate: f32,
#[allow(unused)]
	minToughness: i16,
#[allow(unused)]
	isNonEffectiveCorrectionForMin: u8,
#[allow(unused)]
	pad2: [u8;1],
#[allow(unused)]
	spEffectId: i32,
#[allow(unused)]
	proCorrectionRate: f32,
#[allow(unused)]
	unk1: f32,
#[allow(unused)]
	unk2: f32,
#[allow(unused)]
	pad1: [u8;8],
}

struct TUTORIAL_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	menuType: u8,
#[allow(unused)]
	triggerType: u8,
#[allow(unused)]
	repeatType: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	imageId: i16,
#[allow(unused)]
	pad2: [u8;2],
#[allow(unused)]
	unlockEventFlagId: i32,
#[allow(unused)]
	textId: i32,
#[allow(unused)]
	displayMinTime: f32,
#[allow(unused)]
	displayTime: f32,
#[allow(unused)]
	pad3: [u8;4],
}

struct WAYPOINT_PARAM_ST {
#[allow(unused)]
	attribute1: i16,
#[allow(unused)]
	attribute2: i16,
#[allow(unused)]
	attribute3: i16,
#[allow(unused)]
	attribute4: i16,
#[allow(unused)]
	padding4: [u8;8],
}

struct WEATHER_ASSET_CREATE_PARAM_ST {
#[allow(unused)]
	AssetId: i32,
#[allow(unused)]
	SlotNo: i32,
#[allow(unused)]
	CreateConditionType: u8,
#[allow(unused)]
	padding0: [u8;3],
#[allow(unused)]
	TransitionSrcWeather: i16,
#[allow(unused)]
	TransitionDstWeather: i16,
#[allow(unused)]
	ElapsedTimeCheckweather: i16,
#[allow(unused)]
	padding1: [u8;2],
#[allow(unused)]
	ElapsedTime: f32,
#[allow(unused)]
	CreateDelayTime: f32,
#[allow(unused)]
	CreateProbability: i32,
#[allow(unused)]
	LifeTime: f32,
#[allow(unused)]
	FadeTime: f32,
#[allow(unused)]
	EnableCreateTimeMin: f32,
#[allow(unused)]
	EnableCreateTimeMax: f32,
#[allow(unused)]
	CreatePoint0: i16,
#[allow(unused)]
	CreatePoint1: i16,
#[allow(unused)]
	CreatePoint2: i16,
#[allow(unused)]
	CreatePoint3: i16,
#[allow(unused)]
	CreateAssetLimitId0: i8,
#[allow(unused)]
	CreateAssetLimitId1: i8,
#[allow(unused)]
	CreateAssetLimitId2: i8,
#[allow(unused)]
	CreateAssetLimitId3: i8,
#[allow(unused)]
	Reserved2: [u8;4],
}

struct WEATHER_ASSET_REPLACE_PARAM_ST {
#[allow(unused)]
	mapId: i32,
#[allow(unused)]
	TransitionSrcWeather: i16,
#[allow(unused)]
	padding0: [u8;2],
#[allow(unused)]
	isFireAsh: u8,
#[allow(unused)]
	padding1: [u8;3],
#[allow(unused)]
	reserved2: i32,
#[allow(unused)]
	AssetId0: i32,
#[allow(unused)]
	AssetId1: i32,
#[allow(unused)]
	AssetId2: i32,
#[allow(unused)]
	AssetId3: i32,
#[allow(unused)]
	AssetId4: i32,
#[allow(unused)]
	AssetId5: i32,
#[allow(unused)]
	AssetId6: i32,
#[allow(unused)]
	AssetId7: i32,
#[allow(unused)]
	reserved0: [u8;8],
#[allow(unused)]
	CreateAssetLimitId0: i8,
#[allow(unused)]
	CreateAssetLimitId1: i8,
#[allow(unused)]
	CreateAssetLimitId2: i8,
#[allow(unused)]
	CreateAssetLimitId3: i8,
#[allow(unused)]
	reserved1: [u8;4],
}

struct WEATHER_LOT_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	weatherType0: i16,
#[allow(unused)]
	weatherType1: i16,
#[allow(unused)]
	weatherType2: i16,
#[allow(unused)]
	weatherType3: i16,
#[allow(unused)]
	weatherType4: i16,
#[allow(unused)]
	weatherType5: i16,
#[allow(unused)]
	weatherType6: i16,
#[allow(unused)]
	weatherType7: i16,
#[allow(unused)]
	weatherType8: i16,
#[allow(unused)]
	weatherType9: i16,
#[allow(unused)]
	weatherType10: i16,
#[allow(unused)]
	weatherType11: i16,
#[allow(unused)]
	weatherType12: i16,
#[allow(unused)]
	weatherType13: i16,
#[allow(unused)]
	weatherType14: i16,
#[allow(unused)]
	weatherType15: i16,
#[allow(unused)]
	lotteryWeight0: i16,
#[allow(unused)]
	lotteryWeight1: i16,
#[allow(unused)]
	lotteryWeight2: i16,
#[allow(unused)]
	lotteryWeight3: i16,
#[allow(unused)]
	lotteryWeight4: i16,
#[allow(unused)]
	lotteryWeight5: i16,
#[allow(unused)]
	lotteryWeight6: i16,
#[allow(unused)]
	lotteryWeight7: i16,
#[allow(unused)]
	lotteryWeight8: i16,
#[allow(unused)]
	lotteryWeight9: i16,
#[allow(unused)]
	lotteryWeight10: i16,
#[allow(unused)]
	lotteryWeight11: i16,
#[allow(unused)]
	lotteryWeight12: i16,
#[allow(unused)]
	lotteryWeight13: i16,
#[allow(unused)]
	lotteryWeight14: i16,
#[allow(unused)]
	lotteryWeight15: i16,
#[allow(unused)]
	timezoneLimit: u8,
#[allow(unused)]
	timezoneStartHour: u8,
#[allow(unused)]
	timezoneStartMinute: u8,
#[allow(unused)]
	timezoneEndHour: u8,
#[allow(unused)]
	timezoneEndMinute: u8,
#[allow(unused)]
	reserve: [u8;9],
}

struct WEATHER_LOT_TEX_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	srcR: u8,
#[allow(unused)]
	srcG: u8,
#[allow(unused)]
	srcB: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	weatherLogId: i32,
#[allow(unused)]
	pad2: [u8;4],
}

struct WEATHER_PARAM_ST {
#[allow(unused)]
	SfxId: i32,
#[allow(unused)]
	WindSfxId: i32,
#[allow(unused)]
	GroundHitSfxId: i32,
#[allow(unused)]
	SoundId: i32,
#[allow(unused)]
	WetTime: f32,
#[allow(unused)]
	GparamId: i32,
#[allow(unused)]
	NextLotIngameSecondsMin: i32,
#[allow(unused)]
	NextLotIngameSecondsMax: i32,
#[allow(unused)]
	WetSpEffectId00: i32,
#[allow(unused)]
	WetSpEffectId01: i32,
#[allow(unused)]
	WetSpEffectId02: i32,
#[allow(unused)]
	WetSpEffectId03: i32,
#[allow(unused)]
	WetSpEffectId04: i32,
#[allow(unused)]
	SfxIdInoor: i32,
#[allow(unused)]
	SfxIdOutdoor: i32,
#[allow(unused)]
	aiSightRate: f32,
#[allow(unused)]
	DistViewWeatherGparamOverrideWeight: f32,
}

struct WEP_ABSORP_POS_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	hangPosType: u8,
#[allow(unused)]
	isSkeletonBind: u8,
#[allow(unused)]
	pad0: [u8;2],
#[allow(unused)]
	right_0: i16,
#[allow(unused)]
	left_0: i16,
#[allow(unused)]
	both_0: i16,
#[allow(unused)]
	leftHang_0: i16,
#[allow(unused)]
	rightHang_0: i16,
#[allow(unused)]
	right_1: i16,
#[allow(unused)]
	left_1: i16,
#[allow(unused)]
	both_1: i16,
#[allow(unused)]
	leftHang_1: i16,
#[allow(unused)]
	rightHang_1: i16,
#[allow(unused)]
	right_2: i16,
#[allow(unused)]
	left_2: i16,
#[allow(unused)]
	both_2: i16,
#[allow(unused)]
	leftHang_2: i16,
#[allow(unused)]
	rightHang_2: i16,
#[allow(unused)]
	right_3: i16,
#[allow(unused)]
	left_3: i16,
#[allow(unused)]
	both_3: i16,
#[allow(unused)]
	leftHang_3: i16,
#[allow(unused)]
	rightHang_3: i16,
#[allow(unused)]
	wepInvisibleType_0: u8,
#[allow(unused)]
	wepInvisibleType_1: u8,
#[allow(unused)]
	wepInvisibleType_2: u8,
#[allow(unused)]
	wepInvisibleType_3: u8,
#[allow(unused)]
	leftBoth_0: i16,
#[allow(unused)]
	leftBoth_1: i16,
#[allow(unused)]
	leftBoth_2: i16,
#[allow(unused)]
	leftBoth_3: i16,
#[allow(unused)]
	dispPosType_right_0: u8,
#[allow(unused)]
	dispPosType_left_0: u8,
#[allow(unused)]
	dispPosType_rightBoth_0: u8,
#[allow(unused)]
	dispPosType_leftBoth_0: u8,
#[allow(unused)]
	dispPosType_rightHang_0: u8,
#[allow(unused)]
	dispPosType_leftHang_0: u8,
#[allow(unused)]
	dispPosType_right_1: u8,
#[allow(unused)]
	dispPosType_left_1: u8,
#[allow(unused)]
	dispPosType_rightBoth_1: u8,
#[allow(unused)]
	dispPosType_leftBoth_1: u8,
#[allow(unused)]
	dispPosType_rightHang_1: u8,
#[allow(unused)]
	dispPosType_leftHang_1: u8,
#[allow(unused)]
	dispPosType_right_2: u8,
#[allow(unused)]
	dispPosType_left_2: u8,
#[allow(unused)]
	dispPosType_rightBoth_2: u8,
#[allow(unused)]
	dispPosType_leftBoth_2: u8,
#[allow(unused)]
	dispPosType_rightHang_2: u8,
#[allow(unused)]
	dispPosType_leftHang_2: u8,
#[allow(unused)]
	dispPosType_right_3: u8,
#[allow(unused)]
	dispPosType_left_3: u8,
#[allow(unused)]
	dispPosType_rightBoth_3: u8,
#[allow(unused)]
	dispPosType_leftBoth_3: u8,
#[allow(unused)]
	dispPosType_rightHang_3: u8,
#[allow(unused)]
	dispPosType_leftHang_3: u8,
#[allow(unused)]
	reserve: [u8;12],
}

struct WET_ASPECT_PARAM_ST {
#[allow(unused)]
	baseColorR: u8,
#[allow(unused)]
	baseColorG: u8,
#[allow(unused)]
	baseColorB: u8,
#[allow(unused)]
	reserve_0: [u8;1],
#[allow(unused)]
	baseColorA: f32,
#[allow(unused)]
	metallic: u8,
#[allow(unused)]
	reserve_1: [u8;1],
#[allow(unused)]
	reserve_2: [u8;1],
#[allow(unused)]
	reserve_3: [u8;1],
#[allow(unused)]
	metallicRate: f32,
#[allow(unused)]
	shininessRate: f32,
#[allow(unused)]
	shininess: u8,
#[allow(unused)]
	reserve_4: [u8;11],
}

struct WHITE_SIGN_COOL_TIME_PARAM_ST {
#[allow(unused)]
	limitationTime_Normal: f32,
#[allow(unused)]
	limitationTime_NormalDriedFinger: f32,
#[allow(unused)]
	limitationTime_Guardian: f32,
#[allow(unused)]
	limitationTime_GuardianDriedFinger: f32,
}

struct WORLD_MAP_LEGACY_CONV_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	srcAreaNo: u8,
#[allow(unused)]
	srcGridXNo: u8,
#[allow(unused)]
	srcGridZNo: u8,
#[allow(unused)]
	pad1: [u8;1],
#[allow(unused)]
	srcPosX: f32,
#[allow(unused)]
	srcPosY: f32,
#[allow(unused)]
	srcPosZ: f32,
#[allow(unused)]
	dstAreaNo: u8,
#[allow(unused)]
	dstGridXNo: u8,
#[allow(unused)]
	dstGridZNo: u8,
#[allow(unused)]
	pad2: [u8;1],
#[allow(unused)]
	dstPosX: f32,
#[allow(unused)]
	dstPosY: f32,
#[allow(unused)]
	dstPosZ: f32,
#[allow(unused)]
	isBasePoint: u8,
#[allow(unused)]
	pad3: [u8;1],
#[allow(unused)]
	pad4: [u8;11],
}

struct WORLD_MAP_PIECE_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	openEventFlagId: i32,
#[allow(unused)]
	openTravelAreaLeft: f32,
#[allow(unused)]
	openTravelAreaRight: f32,
#[allow(unused)]
	openTravelAreaTop: f32,
#[allow(unused)]
	openTravelAreaBottom: f32,
#[allow(unused)]
	acquisitionEventFlagId: i32,
#[allow(unused)]
	acquisitionEventScale: f32,
#[allow(unused)]
	acquisitionEventCenterX: f32,
#[allow(unused)]
	acquisitionEventCenterY: f32,
#[allow(unused)]
	acquisitionEventResScale: f32,
#[allow(unused)]
	acquisitionEventResOffsetX: f32,
#[allow(unused)]
	acquisitionEventResOffsetY: f32,
#[allow(unused)]
	pad: [u8;12],
}

struct WORLD_MAP_PLACE_NAME_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	worldMapPieceId: i32,
#[allow(unused)]
	textId: i32,
#[allow(unused)]
	pad1: [u8;4],
#[allow(unused)]
	areaNo: u8,
#[allow(unused)]
	gridXNo: u8,
#[allow(unused)]
	gridZNo: u8,
#[allow(unused)]
	pad2: [u8;1],
#[allow(unused)]
	posX: f32,
#[allow(unused)]
	posY: f32,
#[allow(unused)]
	posZ: f32,
}

struct WORLD_MAP_POINT_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	eventFlagId: i32,
#[allow(unused)]
	distViewEventFlagId: i32,
#[allow(unused)]
	iconId: i16,
#[allow(unused)]
	bgmPlaceType: i16,
#[allow(unused)]
	isAreaIcon: u8,
#[allow(unused)]
	isOverrideDistViewMarkPos: u8,
#[allow(unused)]
	isEnableNoText: u8,
#[allow(unused)]
	pad3: [u8;1],
#[allow(unused)]
	areaNo_forDistViewMark: u8,
#[allow(unused)]
	gridXNo_forDistViewMark: u8,
#[allow(unused)]
	gridZNo_forDistViewMark: u8,
#[allow(unused)]
	clearedEventFlagId: i32,
#[allow(unused)]
	dispMask00: u8,
#[allow(unused)]
	dispMask01: u8,
#[allow(unused)]
	pad2_0: [u8;1],
#[allow(unused)]
	pad2: [u8;1],
#[allow(unused)]
	distViewIconId: i16,
#[allow(unused)]
	angle: f32,
#[allow(unused)]
	areaNo: u8,
#[allow(unused)]
	gridXNo: u8,
#[allow(unused)]
	gridZNo: u8,
#[allow(unused)]
	pad: [u8;1],
#[allow(unused)]
	posX: f32,
#[allow(unused)]
	posY: f32,
#[allow(unused)]
	posZ: f32,
#[allow(unused)]
	textId1: i32,
#[allow(unused)]
	textEnableFlagId1: i32,
#[allow(unused)]
	textDisableFlagId1: i32,
#[allow(unused)]
	textId2: i32,
#[allow(unused)]
	textEnableFlagId2: i32,
#[allow(unused)]
	textDisableFlagId2: i32,
#[allow(unused)]
	textId3: i32,
#[allow(unused)]
	textEnableFlagId3: i32,
#[allow(unused)]
	textDisableFlagId3: i32,
#[allow(unused)]
	textId4: i32,
#[allow(unused)]
	textEnableFlagId4: i32,
#[allow(unused)]
	textDisableFlagId4: i32,
#[allow(unused)]
	textId5: i32,
#[allow(unused)]
	textEnableFlagId5: i32,
#[allow(unused)]
	textDisableFlagId5: i32,
#[allow(unused)]
	textId6: i32,
#[allow(unused)]
	textEnableFlagId6: i32,
#[allow(unused)]
	textDisableFlagId6: i32,
#[allow(unused)]
	textId7: i32,
#[allow(unused)]
	textEnableFlagId7: i32,
#[allow(unused)]
	textDisableFlagId7: i32,
#[allow(unused)]
	textId8: i32,
#[allow(unused)]
	textEnableFlagId8: i32,
#[allow(unused)]
	textDisableFlagId8: i32,
#[allow(unused)]
	textType1: u8,
#[allow(unused)]
	textType2: u8,
#[allow(unused)]
	textType3: u8,
#[allow(unused)]
	textType4: u8,
#[allow(unused)]
	textType5: u8,
#[allow(unused)]
	textType6: u8,
#[allow(unused)]
	textType7: u8,
#[allow(unused)]
	textType8: u8,
#[allow(unused)]
	distViewId: i32,
#[allow(unused)]
	posX_forDistViewMark: f32,
#[allow(unused)]
	posY_forDistViewMark: f32,
#[allow(unused)]
	posZ_forDistViewMark: f32,
#[allow(unused)]
	distViewId1: i32,
#[allow(unused)]
	distViewId2: i32,
#[allow(unused)]
	distViewId3: i32,
#[allow(unused)]
	dispMinZoomStep: u8,
#[allow(unused)]
	selectMinZoomStep: u8,
#[allow(unused)]
	entryFEType: u8,
#[allow(unused)]
	pad4: [u8;9],
#[allow(unused)]
	unkC0: i32,
#[allow(unused)]
	unkC4: i32,
#[allow(unused)]
	unkC8: i32,
#[allow(unused)]
	unkCC: i32,
#[allow(unused)]
	unkD0: i32,
#[allow(unused)]
	unkD4: i32,
#[allow(unused)]
	unkD8: i32,
#[allow(unused)]
	unkDC: i32,
#[allow(unused)]
	unkE0: i32,
#[allow(unused)]
	unkE4: i32,
#[allow(unused)]
	unkE8: i32,
#[allow(unused)]
	unkEC: i32,
#[allow(unused)]
	unkF0: i32,
#[allow(unused)]
	unkF4: i32,
#[allow(unused)]
	unkF8: i32,
#[allow(unused)]
	unkFC: i32,
}

struct WWISE_VALUE_TO_STR_CONVERT_PARAM_ST {
#[allow(unused)]
	disableParam_NT: u8,
#[allow(unused)]
	disableParamReserve1: [u8;1],
#[allow(unused)]
	disableParamReserve2: [u8;3],
#[allow(unused)]
	ParamStr: [u8;32],
}

