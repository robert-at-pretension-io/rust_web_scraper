-- ********************************************
-- BEGIN INSERTS FOR: AF1BO
-- ********************************************
-- -------------------------------------------------------------
--  inserting AF1BO into akc.t_cde_form_identifier_akc
-- -------------------------------------------------------------
insert into
    akc.t_cde_form_identifier_akc (
        CDE_FORM_IDENTIFIER,
        DATE_FORM_EFFECTIVE,
        DATE_FORM_END,
        TEXT_BARCODE,
        NUM_ORDER,
        TIMESTAMP,
        DESC_FORM_IDENTIFIER,
        CDE_DOCUMENT_TYPE,
        IND_KEY_VERIFY_DOCUMENT,
        IND_DOCUMENT_BACK,
        TEXT_HURLER_ALT_BARCODE_REGEX,
        IND_CONSOLE_DEFAULT,
        IND_PRINT_DEFAULT,
        TEXT_HURLER_ALT_BARCODE2_REGEX
    )
values
    (
        'AF1BO',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        'AF1BO',
        400,
        SYSDATE,
        'AF1BO (10/24)',
        'REGAPP',
        'N',
        'N',
        '[a-zA-Z]{2}[0-9]{8}',
        'Y',
        'Y',
        null
    );

-- -----------------------------------------------------------------
--  inserting AF1BO into akc.t_form_id_to_fee_type_akc
-- -----------------------------------------------------------------
insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'ADDOWN',
        13,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        null,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'HSHIPREG',
        0,
        'Y',
        'Y',
        'Y',
        null,
        0,
        'N',
        null,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'PEDC04',
        25,
        'N',
        'Y',
        'Y',
        null,
        0,
        'N',
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'PEDP03',
        46,
        'N',
        'Y',
        'Y',
        null,
        0,
        'N',
        1,
        'Export Pedigree',
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'RCAR_FEE',
        17,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        null,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'REGAPP',
        45.98,
        'Y',
        'N',
        'N',
        null,
        0,
        null,
        null,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'REGNAMEL',
        11,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        null,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'RLATE',
        0,
        'N',
        'N',
        'N',
        null,
        0,
        null,
        null,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'RLATE1',
        36,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        null,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'RLATE2',
        66,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        null,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'SILVER',
        18,
        'N',
        'Y',
        'Y',
        '01',
        0,
        'N',
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'SUPXFR',
        0,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        null,
        null,
        null,
        null,
        null,
        null
    );

-- -----------------------------------------------------------------
--  inserting AF1BO into akc.t_form_id_to_fee_type_dsp_akc
-- -----------------------------------------------------------------
insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'ADDOWN',
        'Co-Ownership Fee',
        null,
        null,
        null,
        7
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'HSHIPREG',
        'Hardship Registration Fee',
        null,
        null,
        null,
        8
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'PEDC04',
        '4 Generation Pedigree',
        null,
        null,
        null,
        4
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'PEDP03',
        'Export Pedigree',
        null,
        null,
        null,
        3
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'RCAR_FEE',
        'AKC Reunite Rider',
        null,
        null,
        null,
        5
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'REGAPP',
        'Dog Registration',
        null,
        null,
        null,
        1
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'REGNAMEL',
        'Extended Name',
        null,
        null,
        null,
        6
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'RLATE',
        'Late Fee - REG',
        null,
        null,
        null,
        10
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'RLATE1',
        'Late 1 Year - REG',
        null,
        null,
        null,
        10
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'RLATE2',
        'Late 2 Year - REG',
        null,
        null,
        null,
        10
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'SILVER',
        'Silver',
        null,
        null,
        null,
        2
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'AF1BO',
        'SUPXFR',
        'Supplemental TXFR',
        null,
        null,
        null,
        1
    );

-- ********************************************
-- BEGIN INSERTS FOR: WEBREG_034
-- ********************************************
-- -------------------------------------------------------------
--  inserting WEBREG_034 into akc.t_cde_form_identifier_akc
-- -------------------------------------------------------------
insert into
    akc.t_cde_form_identifier_akc (
        CDE_FORM_IDENTIFIER,
        DATE_FORM_EFFECTIVE,
        DATE_FORM_END,
        TEXT_BARCODE,
        NUM_ORDER,
        TIMESTAMP,
        DESC_FORM_IDENTIFIER,
        CDE_DOCUMENT_TYPE,
        IND_KEY_VERIFY_DOCUMENT,
        IND_DOCUMENT_BACK,
        TEXT_HURLER_ALT_BARCODE_REGEX,
        IND_CONSOLE_DEFAULT,
        IND_PRINT_DEFAULT,
        TEXT_HURLER_ALT_BARCODE2_REGEX
    )
values
    (
        'WEBREG_034',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        'None',
        500,
        SYSDATE,
        'WEBREG_034 (10/24)',
        'WEBREG',
        'N',
        'N',
        null,
        null,
        null,
        null
    );

-- -----------------------------------------------------------------
--  inserting WEBREG_034 into akc.t_form_id_to_fee_type_akc
-- -----------------------------------------------------------------
insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ADDOWN',
        13,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'CCPBASIC',
        40,
        'Y',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        40,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'CCPPLAT',
        86.99,
        'Y',
        'Y',
        'Y',
        null,
        0,
        null,
        2,
        null,
        86.99,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'CCPSILV',
        49.99,
        'Y',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        49.99,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'FAMDOG02',
        18.95,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        'AKC Family Dog Digital Magazine',
        18.95,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'GDH1YROR',
        45.99,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        'One-year Access Training',
        45.99,
        null,
        null,
        1
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'GKDNA8',
        55,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        'AKC DNA Kit',
        55,
        null,
        null,
        1
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'GKDNA9',
        135.99,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        'AKC DNA + Health Kit',
        135.99,
        null,
        null,
        1
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ORC2PFEE',
        17,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ORC2_FEE',
        17,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ORC3PFEE',
        26,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ORC3_FEE',
        26,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ORC4PFEE',
        30,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ORC4_FEE',
        30,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ORC5PFEE',
        39,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ORC5_FEE',
        39,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'PEDC03',
        20,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        '3 Generation Pedigree',
        20,
        null,
        null,
        1
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'PEDC04',
        25,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'PEDP03',
        46,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        '3 Generation Export Pedigree',
        46,
.33,
        'PERCENT',
        1
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'PPHTLNRD',
        13.5,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'REGNAMEL',
        11,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'RGBXPUP1',
        59.99,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        'AKC Personalized Puppy Box',
        59.99,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'RLATE',
        0,
        'N',
        'N',
        'N',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'RLATE1',
        36,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        null,
        36,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'RLATE2',
        66,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        null,
        66,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'WEBREG',
        0,
        'Y',
        'N',
        'N',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

-- -----------------------------------------------------------------
--  inserting WEBREG_034 into akc.t_form_id_to_fee_type_dsp_akc
-- -----------------------------------------------------------------
insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ADDOWN',
        'Co-Ownership',
        13,
        0,
        'CURRENCY',
        6
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'CCPBASIC',
        'Registration',
        40,
        0,
        'PERCENT',
        2
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'CCPPLAT',
        'Registration Essentials',
        86.99,
        0,
        'PERCENT',
        2
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'CCPSILV',
        'Registration Basics',
        49.99,
        0,
        'PERCENT',
        2
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'FAMDOG02',
        'AKC Family Dog Digital Magazine',
        18.95,
.16,
        'PERCENT',
        25
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'GDH1YROR',
        'One-year Access Training',
        45.99,
        null,
        null,
        1
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'GKDNA8',
        'AKC DNA Kit',
        55,
        null,
        null,
        1
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'GKDNA9',
        'AKC DNA + Health Kit',
        135.99,
        null,
        null,
        1
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ORC2PFEE',
        'AKC Reunite Enrollment, AKC Logo Collar Tag and Pet Poison Helpline',
        17,
.32,
        'PERCENT',
        10
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ORC2_FEE',
        'AKC Reunite Enrollment, AKC Logo Collar Tag',
        17,
.32,
        'PERCENT',
        10
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ORC3PFEE',
        'AKC Reunite Enrollment, Engraved AKC Logo Collar Tag and Pet Poison Helpline',
        26,
.23,
        'PERCENT',
        11
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ORC3_FEE',
        'AKC Reunite Enrollment and Engraved AKC Logo Collar Tag',
        26,
.23,
        'PERCENT',
        11
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ORC4PFEE',
        'AKC Reunite Enrollment, Engraved Collar Tag, Lost Pet Alert, Pet Poison Helpline',
        30,
.21,
        'PERCENT',
        12
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ORC4_FEE',
        'AKC Reunite Enrollment, AKC Logo Collar Tag and Lost Pet Alert',
        30,
.21,
        'PERCENT',
        12
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ORC5PFEE',
        'AKC Reunite Enrollment, Engraved Collar Tag, Lost Pet Alert, Pet Poison Helpline',
        39,
.17,
        'PERCENT',
        13
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'ORC5_FEE',
        'AKC Reunite Enrollment, Engraved AKC Logo Collar Tag and Lost Pet Alert',
        39,
.17,
        'PERCENT',
        13
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'PEDC03',
        '3 Generation Pedigree',
        20,
.2,
        'PERCENT',
        1
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'PEDC04',
        '4 Generation Pedigree',
        25,
.26,
        'PERCENT',
        5
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'PEDP03',
        '3 Generation Export Pedigree',
        46,
.33,
        'PERCENT',
        1
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'PPHTLNRD',
        'Pet Poison Helpline',
        13.5,
        0,
        'CURRENCY',
        20
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'REGNAMEL',
        'Extended Name',
        11,
        0,
        'CURRENCY',
        8
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'RGBXPUP1',
        'AKC Personalized Puppy Box',
        59.99,
        0,
        'CURRENCY',
        31
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'RLATE',
        'Late Fee',
        null,
        null,
        null,
        99
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'RLATE1',
        'Late Fee',
        36,
        null,
        null,
        99
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'RLATE2',
        'Late Fee',
        66,
        null,
        null,
        99
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'WEBREG',
        'Dog Registration',
        0,
        0,
        'CURRENCY',
        1
    );

-- -----------------------------------------------------------------
--  inserting WEBREG_034 into akc.t_cde_form_id_mapping_xref_akc
-- -----------------------------------------------------------------
insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_000',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_001',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_002',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_003',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_004',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_005',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_006',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_007',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_008',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_009',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_010',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_011',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_012',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_013',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_014',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_015',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_016',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_017',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_018',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_019',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_020',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_021',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_022',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_023',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_024',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_027',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_029',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_030',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_031',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_032',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_WEB',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_033',
        'WEBREG_034',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

-- Update current form mappings
update
    akc.t_cde_form_id_mapping_xref_akc
set
    DATE_END = trunc(sysdate)
where
    CDE_FORM_IDENTIFIER_TO = 'WEBREG_033'
    and sysdate between DATE_EFFECTIVE
    and DATE_END;

-- -----------------------------------------------------------
--  inserting and updating AKC.T_REGISTRATION_PROGRAM_AKC
-- -----------------------------------------------------------
insert into
    AKC.T_REGISTRATION_PROGRAM_AKC (
        KEY_REGISTRATION_PROGRAM,
        CDE_REGISTRATION_PROGRAM,
        DESC_REGISTRATION_PROGRAM,
        CDE_FORM_IDENTIFIER,
        TEXT_MARKETING_CODE,
        CDE_COUPON_PROGRAM,
        DATE_EFFECTIVE,
        DATE_END,
        CDE_ONLINE_FORM_ID
    )
values
    (
        AKC.S_REGISTRATION_PROGRAM_AKC.nextval,
        'ELK',
        'Electronic Litter Kit',
        'AF1BO',
        'NONE',
        null,
        sysdate,
        to_date('12/31/2999 23:59:00', 'mm/dd/rrrr hh24:mi:ss'),
        'WEBREG_034'
    );

update
    AKC.T_REGISTRATION_PROGRAM_AKC
set
    date_end = sysdate
where
    CDE_REGISTRATION_PROGRAM = 'ELK'
    and CDE_ONLINE_FORM_ID = 'WEBREG_033';

insert into
    AKC.T_REGISTRATION_PROGRAM_AKC (
        KEY_REGISTRATION_PROGRAM,
        CDE_REGISTRATION_PROGRAM,
        DESC_REGISTRATION_PROGRAM,
        CDE_FORM_IDENTIFIER,
        TEXT_MARKETING_CODE,
        CDE_COUPON_PROGRAM,
        DATE_EFFECTIVE,
        DATE_END,
        CDE_ONLINE_FORM_ID
    )
values
    (
        AKC.S_REGISTRATION_PROGRAM_AKC.nextval,
        'LITKIT',
        'Litter Kit',
        'AF1BO',
        'NONE',
        null,
        sysdate,
        to_date('12/31/2999 23:59:00', 'mm/dd/rrrr hh24:mi:ss'),
        'WEBREG_034'
    );

update
    AKC.T_REGISTRATION_PROGRAM_AKC
set
    date_end = sysdate
where
    CDE_REGISTRATION_PROGRAM = 'LITKIT'
    and CDE_ONLINE_FORM_ID = 'WEBREG_033';

-- Add CKCREG to WEBREG_034
Insert into
    AKC.T_FORM_ID_TO_FEE_TYPE_AKC (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'CKCREG',
        125,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        'CKC Reciprocal Registration',
        125,
        null,
        null,
        1
    );

Insert into
    AKC.T_FORM_ID_TO_FEE_TYPE_DSP_AKC (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_034',
        'CKCREG',
        'CKC Reciprocal Registration',
        125,
        null,
        null,
        1
    );

-- ********************************************
-- BEGIN INSERTS FOR: WEBREG_SELFBG
-- ********************************************
-- -------------------------------------------------------------
--  inserting WEBREG_SELFBG into akc.t_cde_form_identifier_akc
-- -------------------------------------------------------------
insert into
    akc.t_cde_form_identifier_akc (
        CDE_FORM_IDENTIFIER,
        DATE_FORM_EFFECTIVE,
        DATE_FORM_END,
        TEXT_BARCODE,
        NUM_ORDER,
        TIMESTAMP,
        DESC_FORM_IDENTIFIER,
        CDE_DOCUMENT_TYPE,
        IND_KEY_VERIFY_DOCUMENT,
        IND_DOCUMENT_BACK,
        TEXT_HURLER_ALT_BARCODE_REGEX,
        IND_CONSOLE_DEFAULT,
        IND_PRINT_DEFAULT,
        TEXT_HURLER_ALT_BARCODE2_REGEX
    )
values
    (
        'WEBREG_SELFBG',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        'None',
        500,
        SYSDATE,
        'WEBREG_SELFBG (10/24)',
        'WEBREG',
        'N',
        'N',
        null,
        null,
        null,
        null
    );

-- -----------------------------------------------------------------
--  inserting WEBREG_SELFBG into akc.t_form_id_to_fee_type_akc
-- -----------------------------------------------------------------
insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ADDOWN',
        13,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'CCPBASIC',
        40,
        'Y',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        40,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'CCPPLAT',
        86.99,
        'Y',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        86.99,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'CCPSILV',
        49.99,
        'Y',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        49.99,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'FAMDOG02',
        18.95,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        'AKC Family Dog Digital Magazine',
        18.95,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'GDH1YROR',
        45.99,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        'One-year Access Training',
        45.99,
        null,
        null,
        1
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'GKDNA8',
        55,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        'AKC DNA Kit',
        55,
        null,
        null,
        1
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'GKDNA9',
        135.99,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        'AKC DNA + Health Kit',
        135.99,
        null,
        null,
        1
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ORC2PFEE',
        17,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ORC2_FEE',
        17,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ORC3PFEE',
        26,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ORC3_FEE',
        26,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ORC4PFEE',
        30,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ORC4_FEE',
        30,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ORC5PFEE',
        39,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ORC5_FEE',
        39,
        'N',
        'Y',
        'Y',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'PEDC03',
        20,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        '3 Generation Pedigree',
        20,
        null,
        null,
        1
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'PEDC04',
        25,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'PEDP03',
        46,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        '3 Generation Export Pedigree',
        46,
.33,
        'PERCENT',
        1
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'PPHTLNRD',
        13.5,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'REGNAMEL',
        11,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'RGBXPUP1',
        59.99,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        'AKC Personalized Puppy Box',
        59.99,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'RLATE',
        0,
        'N',
        'N',
        'N',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'RLATE1',
        36,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        null,
        36,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'RLATE2',
        66,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        null,
        66,
        null,
        null,
        null
    );

insert into
    akc.t_form_id_to_fee_type_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'WEBREG',
        0,
        'Y',
        'N',
        'N',
        null,
        0,
        null,
        1,
        null,
        null,
        null,
        null,
        null
    );

-- -----------------------------------------------------------------
--  inserting WEBREG_SELFBG into akc.t_form_id_to_fee_type_dsp_akc
-- -----------------------------------------------------------------
insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ADDOWN',
        'Co-Ownership',
        13,
        0,
        'CURRENCY',
        6
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'CCPBASIC',
        'Registration',
        40,
        0,
        'PERCENT',
        2
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'CCPPLAT',
        'Registration Essentials',
        86.99,
.73,
        'PERCENT',
        2
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'CCPSILV',
        'Registration Basics',
        49.99,
.42,
        'PERCENT',
        2
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'FAMDOG02',
        'AKC Family Dog Digital Magazine',
        18.95,
.16,
        'PERCENT',
        25
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'GDH1YROR',
        'One-year Access Training',
        45.99,
        null,
        null,
        1
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'GKDNA8',
        'AKC DNA Kit',
        55,
        null,
        null,
        1
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'GKDNA9',
        'AKC DNA + Health Kit',
        135.99,
        null,
        null,
        1
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ORC2PFEE',
        'AKC Reunite Enrollment, AKC Logo Collar Tag and Pet Poison Helpline',
        17,
.32,
        'PERCENT',
        10
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ORC2_FEE',
        'AKC Reunite Enrollment, AKC Logo Collar Tag',
        17,
.32,
        'PERCENT',
        10
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ORC3PFEE',
        'AKC Reunite Enrollment, Engraved AKC Logo Collar Tag and Pet Poison Helpline',
        26,
.23,
        'PERCENT',
        11
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ORC3_FEE',
        'AKC Reunite Enrollment and Engraved AKC Logo Collar Tag',
        26,
.23,
        'PERCENT',
        11
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ORC4PFEE',
        'AKC Reunite Enrollment, Engraved Collar Tag, Lost Pet Alert, Pet Poison Helpline',
        30,
.21,
        'PERCENT',
        12
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ORC4_FEE',
        'AKC Reunite Enrollment, AKC Logo Collar Tag and Lost Pet Alert',
        30,
.21,
        'PERCENT',
        12
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ORC5PFEE',
        'AKC Reunite Enrollment, Engraved Collar Tag, Lost Pet Alert, Pet Poison Helpline',
        39,
.17,
        'PERCENT',
        13
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'ORC5_FEE',
        'AKC Reunite Enrollment, Engraved AKC Logo Collar Tag and Lost Pet Alert',
        39,
.17,
        'PERCENT',
        13
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'PEDC03',
        '3 Generation Pedigree',
        20,
.2,
        'PERCENT',
        1
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'PEDC04',
        '4 Generation Pedigree',
        25,
.26,
        'PERCENT',
        5
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'PEDP03',
        '3 Generation Export Pedigree',
        46,
.33,
        'PERCENT',
        1
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'PPHTLNRD',
        'Pet Poison Helpline',
        13.5,
        0,
        'CURRENCY',
        20
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'REGNAMEL',
        'Extended Name',
        11,
        0,
        'CURRENCY',
        8
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'RGBXPUP1',
        'AKC Personalized Puppy Box',
        59.99,
        0,
        'CURRENCY',
        31
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'RLATE',
        'Late Fee',
        null,
        null,
        null,
        99
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'RLATE1',
        'Late Fee',
        36,
        null,
        null,
        99
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'RLATE2',
        'Late Fee',
        66,
        null,
        null,
        99
    );

insert into
    akc.t_form_id_to_fee_type_dsp_akc (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'WEBREG',
        'Dog Registration',
        0,
        0,
        'CURRENCY',
        1
    );

-- -----------------------------------------------------------------
--  inserting WEBREG_SELFBG into akc.t_cde_form_id_mapping_xref_akc
-- -----------------------------------------------------------------
insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAA',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAB',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAC',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAD',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAE',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAF',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAG',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAH',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAI',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAJ',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAK',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAL',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAM',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAN',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAO',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAP',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAQ',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAR',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAS',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAT',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAU',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAV',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFAZ',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFBB',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFBC',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFBD',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFBE',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

insert into
    akc.t_cde_form_id_mapping_xref_akc (
        CDE_FORM_IDENTIFIER_FROM,
        CDE_FORM_IDENTIFIER_TO,
        TEXT_NOTES,
        DATE_EFFECTIVE,
        DATE_END,
        TIMESTAMP
    )
values
    (
        'WEBREG_SELFBF',
        'WEBREG_SELFBG',
        'REG-18980: 2025 Price Increases',
        trunc(sysdate),
        to_date('12/31/2999 00:00:00', 'mm/dd/rrrr hh24:mi:ss'),
        SYSDATE
    );

-- Update current form mappings
update
    akc.t_cde_form_id_mapping_xref_akc
set
    DATE_END = trunc(sysdate)
where
    CDE_FORM_IDENTIFIER_TO = 'WEBREG_SELFBF'
    and sysdate between DATE_EFFECTIVE
    and DATE_END;

-- -----------------------------------------------------------
--  inserting and updating AKC.T_REGISTRATION_PROGRAM_AKC
-- -----------------------------------------------------------
insert into
    AKC.T_REGISTRATION_PROGRAM_AKC (
        KEY_REGISTRATION_PROGRAM,
        CDE_REGISTRATION_PROGRAM,
        DESC_REGISTRATION_PROGRAM,
        CDE_FORM_IDENTIFIER,
        TEXT_MARKETING_CODE,
        CDE_COUPON_PROGRAM,
        DATE_EFFECTIVE,
        DATE_END,
        CDE_ONLINE_FORM_ID
    )
values
    (
        AKC.S_REGISTRATION_PROGRAM_AKC.nextval,
        'WEBREG_SELF',
        'Program for Self-Registration via the website',
        'AF1BO',
        'NONE',
        null,
        sysdate,
        to_date('12/31/2999 23:59:00', 'mm/dd/rrrr hh24:mi:ss'),
        'WEBREG_SELFBG'
    );

update
    AKC.T_REGISTRATION_PROGRAM_AKC
set
    date_end = sysdate
where
    CDE_REGISTRATION_PROGRAM = 'WEBREG_SELF'
    and CDE_ONLINE_FORM_ID = 'WEBREG_SELFBF';

-- Add CKCREG to WEBREG_SELFBG
Insert into
    AKC.T_FORM_ID_TO_FEE_TYPE_AKC (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        NUM_AMOUNT_FEE,
        IND_REQUIRED,
        IND_REFUND_ON_CANCEL,
        IND_FULFILLMENT,
        TEXT_CONVERT_STRING,
        NUM_OPUS_FEE_FIELD,
        IND_AUTO_ORDER,
        NUM_FEE_TYPE_VERSION,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'CKCREG',
        125,
        'N',
        'Y',
        'N',
        null,
        0,
        null,
        1,
        'CKC Reciprocal Registration',
        125,
        null,
        null,
        1
    );

Insert into
    AKC.T_FORM_ID_TO_FEE_TYPE_DSP_AKC (
        CDE_FORM_IDENTIFIER,
        CDE_FEE_TYPE,
        TEXT_DISPLAY_FEE_TYPE,
        NUM_BASE_PRICE,
        NUM_SAVINGS,
        CDE_SAVINGS_TYPE,
        NUM_SORT_ORDER
    )
values
    (
        'WEBREG_SELFBG',
        'CKCREG',
        'CKC Reciprocal Registration',
        125,
        null,
        null,
        1
    );

--
--
--------------------------------------------------------------------------------
-- Increase Online Reg Package Prices
--------------------------------------------------------------------------------
--
--
update
    akc.t_form_id_to_fee_type_akc
set
    num_amount_fee = 55,
    num_base_price = 55
where
    cde_form_identifier in ('WEBREG_034', 'WEBREG_SELFBG')
    and cde_fee_type = 'CCPSILV';

update
    akc.t_form_id_to_fee_type_dsp_akc
set
    num_base_price = 55
where
    cde_form_identifier in ('WEBREG_034', 'WEBREG_SELFBG')
    and cde_fee_type = 'CCPSILV';

update
    akc.t_form_id_to_fee_type_akc
set
    num_amount_fee = 90,
    num_base_price = 90
where
    cde_form_identifier in ('WEBREG_034', 'WEBREG_SELFBG')
    and cde_fee_type = 'CCPPLAT';

update
    akc.t_form_id_to_fee_type_dsp_akc
set
    num_base_price = 90
where
    cde_form_identifier in ('WEBREG_034', 'WEBREG_SELFBG')
    and cde_fee_type = 'CCPPLAT';

--
--
--------------------------------------------------------------------------------
-- Update Console Default for paper forms
--------------------------------------------------------------------------------
--
--
update
    akc.t_cde_form_identifier_akc
set
    ind_console_default = 'N'
where
    cde_form_identifier like 'AF1B%'
    and ind_console_default = 'Y'
    and cde_form_identifier <> 'AF1BO';

--
--
--------------------------------------------------------------------------------
-- Refresh the console cache
--------------------------------------------------------------------------------
--
update
    akc.t_preference_akc
set
    text_property_value = to_char (sysdate, 'YYYYMMDD_HH24:MI:SS'),
    modified_by = user,
    timestamp = sysdate
where
    key_property_name = 'org.akc.reg.formserver.SerializerUtil.refreshTimestamp';