use ordnl;

#[test]
pub fn test_u128() {
    for (num, expected) in vec![
        // < 100
        (0u128, "0th"),
        (1u128, "1st"),
        (2u128, "2nd"),
        (3u128, "3rd"),
        (4u128, "4th"),
        (5u128, "5th"),
        (6u128, "6th"),
        (7u128, "7th"),
        (8u128, "8th"),
        (9u128, "9th"),
        (10u128, "10th"),
        (11u128, "11th"),
        (12u128, "12th"),
        (13u128, "13th"),
        (14u128, "14th"),
        (15u128, "15th"),
        (16u128, "16th"),
        (17u128, "17th"),
        (18u128, "18th"),
        (19u128, "19th"),
        (20u128, "20th"),
        (21u128, "21st"),
        (22u128, "22nd"),
        (23u128, "23rd"),
        (24u128, "24th"),
        (25u128, "25th"),
        (26u128, "26th"),
        (27u128, "27th"),
        (28u128, "28th"),
        (29u128, "29th"),
        // 100+
        (100u128, "100th"),
        (101u128, "101st"),
        (102u128, "102nd"),
        (103u128, "103rd"),
        (104u128, "104th"),
        (105u128, "105th"),
        (106u128, "106th"),
        (107u128, "107th"),
        (108u128, "108th"),
        (109u128, "109th"),
        (111u128, "111th"),
        (112u128, "112th"),
        (113u128, "113th"),
        (114u128, "114th"),
        (115u128, "115th"),
        (116u128, "116th"),
        (117u128, "117th"),
        (118u128, "118th"),
        (119u128, "119th"),
        (200u128, "200th"),
        (211u128, "211th"),
        (212u128, "212th"),
        (213u128, "213th"),
        (214u128, "214th"),
        (215u128, "215th"),
        (216u128, "216th"),
        (217u128, "217th"),
        (218u128, "218th"),
        (219u128, "219th"),
        // 1000+
        (1000u128, "1000th"),
        (1001u128, "1001st"),
        (1002u128, "1002nd"),
        (1003u128, "1003rd"),
        (1004u128, "1004th"),
        (1005u128, "1005th"),
        (1006u128, "1006th"),
        (1007u128, "1007th"),
        (1008u128, "1008th"),
        (1009u128, "1009th"),
        (1011u128, "1011th"),
        (1012u128, "1012th"),
        (1013u128, "1013th"),
        (1014u128, "1014th"),
        (1015u128, "1015th"),
        (1016u128, "1016th"),
        (1017u128, "1017th"),
        (1018u128, "1018th"),
        (1019u128, "1019th"),
        (1100u128, "1100th"),
        (1101u128, "1101st"),
        (1102u128, "1102nd"),
        (1103u128, "1103rd"),
        (1104u128, "1104th"),
        (1105u128, "1105th"),
        (1106u128, "1106th"),
        (1107u128, "1107th"),
        (1108u128, "1108th"),
        (1109u128, "1109th"),
        (1111u128, "1111th"),
        (1112u128, "1112th"),
        (1113u128, "1113th"),
        (1114u128, "1114th"),
        (1115u128, "1115th"),
        (1116u128, "1116th"),
        (1117u128, "1117th"),
        (1118u128, "1118th"),
        (1119u128, "1119th"),
        // 10,000+
        (10000u128, "10000th"),
        (10001u128, "10001st"),
        (10002u128, "10002nd"),
        (10003u128, "10003rd"),
        (10004u128, "10004th"),
        (10005u128, "10005th"),
        (10006u128, "10006th"),
        (10007u128, "10007th"),
        (10008u128, "10008th"),
        (10009u128, "10009th"),
        (10011u128, "10011th"),
        (10012u128, "10012th"),
        (10013u128, "10013th"),
        (10014u128, "10014th"),
        (10015u128, "10015th"),
        (10016u128, "10016th"),
        (10017u128, "10017th"),
        (10018u128, "10018th"),
        (10019u128, "10019th"),
        (10100u128, "10100th"),
        (10101u128, "10101st"),
        (10102u128, "10102nd"),
        (10103u128, "10103rd"),
        (10104u128, "10104th"),
        (10105u128, "10105th"),
        (10106u128, "10106th"),
        (10107u128, "10107th"),
        (10108u128, "10108th"),
        (10109u128, "10109th"),
        (10111u128, "10111th"),
        (10112u128, "10112th"),
        (10113u128, "10113th"),
        (10114u128, "10114th"),
        (10115u128, "10115th"),
        (10116u128, "10116th"),
        (10117u128, "10117th"),
        (10118u128, "10118th"),
        (10119u128, "10119th"),
        // 1,000,000,000+
        (1000000000u128, "1000000000th"),
        (1000000001u128, "1000000001st"),
        (1000000002u128, "1000000002nd"),
        (1000000003u128, "1000000003rd"),
        (1000000004u128, "1000000004th"),
        (1000000005u128, "1000000005th"),
        (1000000006u128, "1000000006th"),
        (1000000007u128, "1000000007th"),
        (1000000008u128, "1000000008th"),
        (1000000009u128, "1000000009th"),
        (1000000011u128, "1000000011th"),
        (1000000012u128, "1000000012th"),
        (1000000013u128, "1000000013th"),
        (1000000014u128, "1000000014th"),
        (1000000015u128, "1000000015th"),
        (1000000016u128, "1000000016th"),
        (1000000017u128, "1000000017th"),
        (1000000018u128, "1000000018th"),
        (1000000019u128, "1000000019th"),
        (1000000100u128, "1000000100th"),
        (1000000101u128, "1000000101st"),
        (1000000102u128, "1000000102nd"),
        (1000000103u128, "1000000103rd"),
        (1000000104u128, "1000000104th"),
        (1000000105u128, "1000000105th"),
        (1000000106u128, "1000000106th"),
        (1000000107u128, "1000000107th"),
        (1000000108u128, "1000000108th"),
        (1000000109u128, "1000000109th"),
        (1000000111u128, "1000000111th"),
        (1000000112u128, "1000000112th"),
        (1000000113u128, "1000000113th"),
        (1000000114u128, "1000000114th"),
        (1000000115u128, "1000000115th"),
        (1000000116u128, "1000000116th"),
        (1000000117u128, "1000000117th"),
        (1000000118u128, "1000000118th"),
        (1000000119u128, "1000000119th"),
        //  10,000,000,000,000,000,000+
        (10000000000000000000u128, "10000000000000000000th"),
        (10000000000000000001u128, "10000000000000000001st"),
        (10000000000000000002u128, "10000000000000000002nd"),
        (10000000000000000003u128, "10000000000000000003rd"),
        (10000000000000000004u128, "10000000000000000004th"),
        (10000000000000000005u128, "10000000000000000005th"),
        (10000000000000000006u128, "10000000000000000006th"),
        (10000000000000000007u128, "10000000000000000007th"),
        (10000000000000000008u128, "10000000000000000008th"),
        (10000000000000000009u128, "10000000000000000009th"),
        (10000000000000000011u128, "10000000000000000011th"),
        (10000000000000000012u128, "10000000000000000012th"),
        (10000000000000000013u128, "10000000000000000013th"),
        (10000000000000000014u128, "10000000000000000014th"),
        (10000000000000000015u128, "10000000000000000015th"),
        (10000000000000000016u128, "10000000000000000016th"),
        (10000000000000000017u128, "10000000000000000017th"),
        (10000000000000000018u128, "10000000000000000018th"),
        (10000000000000000019u128, "10000000000000000019th"),
        (10000000000000000100u128, "10000000000000000100th"),
        (10000000000000000101u128, "10000000000000000101st"),
        (10000000000000000102u128, "10000000000000000102nd"),
        (10000000000000000103u128, "10000000000000000103rd"),
        (10000000000000000104u128, "10000000000000000104th"),
        (10000000000000000105u128, "10000000000000000105th"),
        (10000000000000000106u128, "10000000000000000106th"),
        (10000000000000000107u128, "10000000000000000107th"),
        (10000000000000000108u128, "10000000000000000108th"),
        (10000000000000000109u128, "10000000000000000109th"),
        (10000000000000000111u128, "10000000000000000111th"),
        (10000000000000000112u128, "10000000000000000112th"),
        (10000000000000000113u128, "10000000000000000113th"),
        (10000000000000000114u128, "10000000000000000114th"),
        (10000000000000000115u128, "10000000000000000115th"),
        (10000000000000000116u128, "10000000000000000116th"),
        (10000000000000000117u128, "10000000000000000117th"),
        (10000000000000000118u128, "10000000000000000118th"),
        (10000000000000000119u128, "10000000000000000119th"),
        // 100,000,000,000,000,000,000,000,000,000,000,000,000+
        (
            100000000000000000000000000000000000000u128,
            "100000000000000000000000000000000000000th",
        ),
        (
            100000000000000000000000000000000000001u128,
            "100000000000000000000000000000000000001st",
        ),
        (
            100000000000000000000000000000000000002u128,
            "100000000000000000000000000000000000002nd",
        ),
        (
            100000000000000000000000000000000000003u128,
            "100000000000000000000000000000000000003rd",
        ),
        (
            100000000000000000000000000000000000004u128,
            "100000000000000000000000000000000000004th",
        ),
        (
            100000000000000000000000000000000000005u128,
            "100000000000000000000000000000000000005th",
        ),
        (
            100000000000000000000000000000000000006u128,
            "100000000000000000000000000000000000006th",
        ),
        (
            100000000000000000000000000000000000007u128,
            "100000000000000000000000000000000000007th",
        ),
        (
            100000000000000000000000000000000000008u128,
            "100000000000000000000000000000000000008th",
        ),
        (
            100000000000000000000000000000000000009u128,
            "100000000000000000000000000000000000009th",
        ),
        (
            100000000000000000000000000000000000011u128,
            "100000000000000000000000000000000000011th",
        ),
        (
            100000000000000000000000000000000000012u128,
            "100000000000000000000000000000000000012th",
        ),
        (
            100000000000000000000000000000000000013u128,
            "100000000000000000000000000000000000013th",
        ),
        (
            100000000000000000000000000000000000014u128,
            "100000000000000000000000000000000000014th",
        ),
        (
            100000000000000000000000000000000000015u128,
            "100000000000000000000000000000000000015th",
        ),
        (
            100000000000000000000000000000000000016u128,
            "100000000000000000000000000000000000016th",
        ),
        (
            100000000000000000000000000000000000017u128,
            "100000000000000000000000000000000000017th",
        ),
        (
            100000000000000000000000000000000000018u128,
            "100000000000000000000000000000000000018th",
        ),
        (
            100000000000000000000000000000000000019u128,
            "100000000000000000000000000000000000019th",
        ),
        (
            100000000000000000000000000000000000100u128,
            "100000000000000000000000000000000000100th",
        ),
        (
            100000000000000000000000000000000000101u128,
            "100000000000000000000000000000000000101st",
        ),
        (
            100000000000000000000000000000000000102u128,
            "100000000000000000000000000000000000102nd",
        ),
        (
            100000000000000000000000000000000000103u128,
            "100000000000000000000000000000000000103rd",
        ),
        (
            100000000000000000000000000000000000104u128,
            "100000000000000000000000000000000000104th",
        ),
        (
            100000000000000000000000000000000000105u128,
            "100000000000000000000000000000000000105th",
        ),
        (
            100000000000000000000000000000000000106u128,
            "100000000000000000000000000000000000106th",
        ),
        (
            100000000000000000000000000000000000107u128,
            "100000000000000000000000000000000000107th",
        ),
        (
            100000000000000000000000000000000000108u128,
            "100000000000000000000000000000000000108th",
        ),
        (
            100000000000000000000000000000000000109u128,
            "100000000000000000000000000000000000109th",
        ),
        (
            100000000000000000000000000000000000111u128,
            "100000000000000000000000000000000000111th",
        ),
        (
            100000000000000000000000000000000000112u128,
            "100000000000000000000000000000000000112th",
        ),
        (
            100000000000000000000000000000000000113u128,
            "100000000000000000000000000000000000113th",
        ),
        (
            100000000000000000000000000000000000114u128,
            "100000000000000000000000000000000000114th",
        ),
        (
            100000000000000000000000000000000000115u128,
            "100000000000000000000000000000000000115th",
        ),
        (
            100000000000000000000000000000000000116u128,
            "100000000000000000000000000000000000116th",
        ),
        (
            100000000000000000000000000000000000117u128,
            "100000000000000000000000000000000000117th",
        ),
        (
            100000000000000000000000000000000000118u128,
            "100000000000000000000000000000000000118th",
        ),
        (
            100000000000000000000000000000000000119u128,
            "100000000000000000000000000000000000119th",
        ),
    ] {
        assert_eq!(ordnl::of_u128(num), expected);
    }
}