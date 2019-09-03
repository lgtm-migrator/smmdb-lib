#![allow(dead_code)]

pub const VERSION: u32 = 1;

pub const COURSE_DATA_PREFIX: &str = "course_data_";
pub const COURSE_DATA_SUFFIX: &str = ".bcd";

// level header
pub const LEVEL_HEADER_OFFSET: usize = 0x10;

pub const YEAR_OFFSET: usize = LEVEL_HEADER_OFFSET + 0x8;
pub const MONTH_OFFSET: usize = LEVEL_HEADER_OFFSET + 0xA;
pub const DAY_OFFSET: usize = LEVEL_HEADER_OFFSET + 0xB;
pub const HOUR_OFFSET: usize = LEVEL_HEADER_OFFSET + 0xC;
pub const MINUTE_OFFSET: usize = LEVEL_HEADER_OFFSET + 0xD;

pub const TITLE_OFFSET: usize = LEVEL_HEADER_OFFSET + 0xF4;
pub const TITLE_LENGTH: usize = 0x40;
pub const TITLE_OFFSET_END: usize = TITLE_OFFSET + TITLE_LENGTH;

pub const DESCRIPTION_OFFSET: usize = LEVEL_HEADER_OFFSET + 0x136;
pub const DESCRIPTION_LENGTH: usize = 0xC8;
pub const DESCRIPTION_OFFSET_END: usize = DESCRIPTION_OFFSET + DESCRIPTION_LENGTH;

pub const START_Y_OFFSET: usize = LEVEL_HEADER_OFFSET + 0x0;

pub const FINISH_Y_OFFSET: usize = LEVEL_HEADER_OFFSET + 0x1;
pub const FINISH_X_OFFSET: usize = LEVEL_HEADER_OFFSET + 0x2;

pub const TIME_OFFSET: usize = LEVEL_HEADER_OFFSET + 0x4;

pub const GAME_STYLE_OFFSET: usize = LEVEL_HEADER_OFFSET + 0xF1;
pub const GAME_STYLE_OFFSET_END: usize = GAME_STYLE_OFFSET + 2;

pub const CLEAR_CONDITION_TYPE_OFFSET: usize = LEVEL_HEADER_OFFSET + 0xF;
pub const CLEAR_CONDITION_OFFSET: usize = LEVEL_HEADER_OFFSET + 0x10;
pub const CLEAR_CONDITION_AMOUNT_OFFSET: usize = LEVEL_HEADER_OFFSET + 0x6;

// level area
pub const LEVEL_AREA_OFFSET: usize = 0x210;
pub const LEVEL_AREA_SUB_OFFSET: usize = 0x2E0F0;

pub const COURSE_THEME_OFFSET: [usize; 2] = [LEVEL_AREA_OFFSET + 0x0, LEVEL_AREA_SUB_OFFSET + 0x0];

pub const AUTO_SCROLL_OFFSET: [usize; 2] = [LEVEL_AREA_OFFSET + 0x1, LEVEL_AREA_SUB_OFFSET + 0x1];

pub const WATER_MAX_OFFSET: [usize; 2] = [LEVEL_AREA_OFFSET + 0x4, LEVEL_AREA_SUB_OFFSET + 0x4];
pub const WATER_MODE_OFFSET: [usize; 2] = [LEVEL_AREA_OFFSET + 0x5, LEVEL_AREA_SUB_OFFSET + 0x5];
pub const WATER_SPEED_OFFSET: [usize; 2] = [LEVEL_AREA_OFFSET + 0x6, LEVEL_AREA_SUB_OFFSET + 0x6];
pub const WATER_MIN_OFFSET: [usize; 2] = [LEVEL_AREA_OFFSET + 0x7, LEVEL_AREA_SUB_OFFSET + 0x7];
