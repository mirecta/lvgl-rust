/**
 * @file lv_conf.h
 * @brief LVGL configuration for ESP32
 * 
 * Optimized for minimal RAM usage on ESP32 without PSRAM
 */

#ifndef LV_CONF_H
#define LV_CONF_H

#include <stdint.h>

/*====================
   COLOR SETTINGS
 *====================*/

/* Color depth: 16 (RGB565) is best for most displays */
#define LV_COLOR_DEPTH 16

/* Swap the 2 bytes of RGB565 color. Useful if display uses big-endian */
#define LV_COLOR_16_SWAP 0

/*====================
   MEMORY SETTINGS
 *====================*/

/* Use system malloc (LVGL 9.x) */
#define LV_USE_STDLIB_MALLOC    LV_STDLIB_CLIB
#define LV_USE_STDLIB_STRING    LV_STDLIB_CLIB
#define LV_USE_STDLIB_SPRINTF   LV_STDLIB_CLIB

/* Fallback pool size (only used if builtin allocator) */
#define LV_MEM_SIZE (48 * 1024U)

/*====================
   HAL SETTINGS
 *====================*/

/* Default display refresh, input read and animation step period */
#define LV_DEF_REFR_PERIOD 33  /* 30 FPS */

/* Default DPI (dots per inch) */
#define LV_DPI_DEF 130

/* Use a custom tick source (REQUIRED for ESP32) */
#define LV_TICK_CUSTOM 1
#if LV_TICK_CUSTOM
    #define LV_TICK_CUSTOM_INCLUDE "esp_timer.h"
    #define LV_TICK_CUSTOM_SYS_TIME_EXPR ((esp_timer_get_time() / 1000ULL))
#endif

/*====================
   FEATURE CONFIG
 *====================*/

/* Enable the log module */
#define LV_USE_LOG 1
#if LV_USE_LOG
    #define LV_LOG_LEVEL LV_LOG_LEVEL_WARN
    #define LV_LOG_PRINTF 1
#endif

/* Enable asserts */
#define LV_USE_ASSERT_NULL          1
#define LV_USE_ASSERT_MALLOC        1
#define LV_USE_ASSERT_STYLE         0
#define LV_USE_ASSERT_MEM_INTEGRITY 0
#define LV_USE_ASSERT_OBJ           0

/* Enable performance monitor */
#define LV_USE_PERF_MONITOR 0

/* Enable memory monitor */
#define LV_USE_MEM_MONITOR 0

/*====================
   FONT CONFIG
 *====================*/

/* Montserrat fonts - enable only what you need */
#define LV_FONT_MONTSERRAT_8  0
#define LV_FONT_MONTSERRAT_10 0
#define LV_FONT_MONTSERRAT_12 1
#define LV_FONT_MONTSERRAT_14 1
#define LV_FONT_MONTSERRAT_16 1
#define LV_FONT_MONTSERRAT_18 0
#define LV_FONT_MONTSERRAT_20 0
#define LV_FONT_MONTSERRAT_22 0
#define LV_FONT_MONTSERRAT_24 0
#define LV_FONT_MONTSERRAT_26 0
#define LV_FONT_MONTSERRAT_28 0
#define LV_FONT_MONTSERRAT_30 0
#define LV_FONT_MONTSERRAT_32 0
#define LV_FONT_MONTSERRAT_34 0
#define LV_FONT_MONTSERRAT_36 0
#define LV_FONT_MONTSERRAT_38 0
#define LV_FONT_MONTSERRAT_40 0
#define LV_FONT_MONTSERRAT_42 0
#define LV_FONT_MONTSERRAT_44 0
#define LV_FONT_MONTSERRAT_46 0
#define LV_FONT_MONTSERRAT_48 0

/* Default font */
#define LV_FONT_DEFAULT &lv_font_montserrat_14

/* Enable FreeType (disabled to save space) */
#define LV_USE_FREETYPE 0

/*====================
   TEXT SETTINGS
 *====================*/

#define LV_TXT_ENC LV_TXT_ENC_UTF8

/* Enable bidirectional text support (Hebrew, Arabic) */
#define LV_USE_BIDI 0

/* Enable Arabic/Persian processing */
#define LV_USE_ARABIC_PERSIAN_CHARS 0

/*====================
   WIDGET CONFIG
 *====================*/

/* Core widgets - enable what you need */
#define LV_USE_ARC        1
#define LV_USE_BAR        1
#define LV_USE_BUTTON     1
#define LV_USE_BUTTONMATRIX 1
#define LV_USE_CANVAS     0  /* Uses lots of RAM */
#define LV_USE_CHECKBOX   1
#define LV_USE_DROPDOWN   1
#define LV_USE_IMAGE      1
#define LV_USE_LABEL      1
#define LV_USE_LINE       1
#define LV_USE_ROLLER     1
#define LV_USE_SCALE      1
#define LV_USE_SLIDER     1
#define LV_USE_SWITCH     1
#define LV_USE_TEXTAREA   1
#define LV_USE_TABLE      1

/* Extra widgets */
#define LV_USE_ANIMIMG    0
#define LV_USE_CALENDAR   0
#define LV_USE_CHART      1
#define LV_USE_COLORWHEEL 0
#define LV_USE_IMGBTN     0
#define LV_USE_KEYBOARD   1
#define LV_USE_LED        1
#define LV_USE_LIST       1
#define LV_USE_MENU       0
#define LV_USE_METER      1
#define LV_USE_MSGBOX     1
#define LV_USE_SPAN       0
#define LV_USE_SPINBOX    0
#define LV_USE_SPINNER    1
#define LV_USE_TABVIEW    1
#define LV_USE_TILEVIEW   0
#define LV_USE_WIN        0

/*====================
   LAYOUTS
 *====================*/

#define LV_USE_FLEX 1
#define LV_USE_GRID 1

/*====================
   DRAW
 *====================*/

/* Use SW renderer (no GPU) */
#define LV_USE_DRAW_SW 1

/* Use ARM2D acceleration (for some ESP32 variants) */
#define LV_USE_DRAW_ARM2D 0

/* Vector graphics */
#define LV_USE_DRAW_VG_LITE 0
#define LV_USE_VECTOR_GRAPHIC 0

/*====================
   LIBS
 *====================*/

/* File system - disable to save space */
#define LV_USE_FS_STDIO 0
#define LV_USE_FS_POSIX 0
#define LV_USE_FS_FATFS 0

/* PNG decoder */
#define LV_USE_PNG 0

/* BMP decoder */
#define LV_USE_BMP 0

/* JPG decoder */
#define LV_USE_TJPGD 0

/* GIF decoder */
#define LV_USE_GIF 0

/* QR code */
#define LV_USE_QRCODE 0

/* Snapshot */
#define LV_USE_SNAPSHOT 0

/*====================
   OTHERS
 *====================*/

/* Enable GPU interface (not needed for SW rendering) */
#define LV_USE_GPU_SDL 0
#define LV_USE_GPU_STM32_DMA2D 0

/* Observer pattern for data binding */
#define LV_USE_OBSERVER 1

/* Enable obj ID for debugging */
#define LV_USE_OBJ_ID 0

#endif /* LV_CONF_H */
