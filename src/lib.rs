#![feature(collections)]
#![feature(convert)]

pub mod crypt;

#[test]
fn test_dollarsix ()
{
    fn test_crypt (key: &str, salt: &str, expected: &str)
    {
        use std::ffi::CString;
        use std::str::from_utf8;
        let foo    = CString::new (key).unwrap ();
        let bar    = CString::new (salt).unwrap ();
        let result = crypt::crypt (foo, bar);
        assert! (result.is_some (), "Internal `crypt' error");
        match from_utf8 (result.unwrap ().as_slice ()) {
            Err (e)  => assert! (false, "Couldn't parse result as UTF-8: {}", e),
            Ok (got) => assert! (got == expected,
                                 "Incorrect hash:
  Key:      {}
  Salt:     {}
  Expected: {}
  Got:      {}",
                                 key, salt, expected, got)
        }
    }

    let tests = [["secretions",     "$6$Schultzs",      "$6$Schultzs$hNWYcmh8RuCjDuv.Sjb/m8C98FULPwHhvNHrnMUmFez8/y2PFjzx4O2Cipvb1MzZKyGC0mF7aGQnWdPu.zp/F."     ],
                 ["telemetrys",     "$6$sincere",       "$6$sincere$H5aejF7tCWzn/ZsD6lX7PIjzus7TBvNM2qwMa8VEmVKZJROJPMQgwHGJUzi0vSrl1yOsD9l.4MQp6fv3SK/W/0"      ],
                 ["Regulus",        "$6$viscid",        "$6$viscid$4ml9MJPuPaUamU6qfsp5YGJliTGJh5hK7lgiB5dP8WsWUixQByZzhuaYxrnlz4ExAJ52q9vMMRZPvqQUqizPz0"       ],
                 ["thralls",        "$6$bartered",      "$6$bartered$Dix0Z.ESdGmf5qUg9iiKoC1dV1ZPCcwoo/65v6qRIDZIretF7ppYNikUzQoZY4hZYB8rgzSvXO98tODMsmAOV."     ],
                 ["monograms",      "$6$compel",        "$6$compel$M9uD0.Xwf4dkettKZzTtAW6Jh2zLJ3kDZOa551rZsVFgWzH/ePMXZ1mBaLijXqWfezAoiXzseVDk6FWWUAEz//"       ],
                 ["Massasoit",      "$6$insolubilitys", "$6$insolubilitys$LXaju60D6Cz7psjJVceMUOFzeRpkycPlJpWqJfzRbhyK.F4z7Wj.VbY.MzrNsUXaYb6vJE/VNFZwH1iRZfk1g/"],
                 ["censured",       "$6$amoralitys",    "$6$amoralitys$LpYTffkWFCAsPsfPRexNmIkwbf7xc45gyHwlZRqUYyZpSS3tND0xHOo3WxxW/yllSvht4qoJFggnt05ZplLKd/"   ],
                 ["insignias",      "$6$worriers",      "$6$worriers$o0Ao9fh9MtIER/NnVfml7KqFMaTM5APwCnD9MDUIuaOyfsdyw0dS7cinzaAeg8zJYlG00q5VESktbh6/Tjqse."     ],
                 ["upsets",         "$6$yearned",       "$6$yearned$vucq8Cl57jeO.Rkg91.CK4XzPLaYqGm89Ir.mem6fxJhoePS9vzwHJLRBpREOyowHmoWfYLgQR/x0YugeRufO/"      ],
                 ["versifying",     "$6$vindictive",    "$6$vindictive$p4a2/Ct7Ftg90hirGaBZPkNJlj0wuZlG3QJv3m72RTaR4oH//qZ0TPRGLR6YlYuBpDJSqiQzFhhftOGPx8hyD/"   ],
                 ["Villarreal",     "$6$prefaces",      "$6$prefaces$c6Asw36qBZgMZ2BoCvv.6L.qDWXAaU1W.VDzNSScXJzBgXshD7PN7YwjQH3r1Tv9lyYhD0CJyRzNmj7LNhsSH."     ],
                 ["voile",          "$6$westbound",     "$6$westbound$m2ng9p4svFpNwwOSqPYW1diOy0lcSx1b3wqC7nV/cQR1MwYqENTo0L70QCxMjMU9DuLVv5Q7HZJfSPdYiZntk."    ],
                 ["uppercases",     "$6$Newcastle",     "$6$Newcastle$J.3NHyl2JYwdRDxIO4YWT97nPwCkUYmZsoBld4mk55YbrfizJf0ocvxGWj4UsAxYv78Gh19sYgnMAZ1cWKd4w/"    ],
                 ["graphite",       "$6$sacraments",    "$6$sacraments$2cLmGC3lz7jwFb/xUriMOrvtQPSrw1y7dz997/DNZTzY5l8IwyVb1ykNzGmAMSWpGHwyDCY/OniWuKRs6GVtu1"   ],
                 ["salmons",        "$6$clones",        "$6$clones$GKhyY3cRdH7W9pkWbk4TMljGbbUNdTuSO4XjhtP5Tb3NRayAUVIigv2Co4n6MAgwhftQHT/QpvriDky/9P0QA1"       ],
                 ["Souths",         "$6$Alstons",       "$6$Alstons$wOvzXQtKst8pJ96l.0FvrfGIjIIdEzS1Ggzqc7Mbzo4EJovfmKOBFILmDAx0c9baSLrFb.7Hrfpf9OgdCSj.T/"      ],
                 ["exclusivenesss", "$6$ODonnell",      "$6$ODonnell$UVYaLkBOl4Na.gvqBrGHXhgzyjKXBu1S.N2vgc1NW7JUMUBmp9/yeD9SeTnInfASX6ZYDM26Adjf91FayxCMF0"     ],
                 ["ticket",         "$6$multiplicity",  "$6$multiplicity$vczzXdQevD2zRUQISdmR7w8BzrIELfgVaUfqR4XCOU0SRhBwZciPorjltfHrzt/obtqvfnDoxypCbWnc23J961" ],
                 ["rooked",         "$6$striker",       "$6$striker$90VOzhQ98d77xb4qoKFq3hrfW4e4VKe6SLhKMf8Ykjx6YlbnhQQ7llAPxA7JNK2gnvxHG0vfrbIMcQ/gx6yHA."      ],
                 ["vacancy",        "$6$healthiness",   "$6$healthiness$t7DokVqVlZFlAq.3ozy9GP7gDQxhYf6HF384cEKtfB67rBXS8IyqPJKG02cTAu7.yEYnundrQQL6HI89xgLdC1"  ]];

    for &test in tests.iter () {
        test_crypt (test [0], test [1], test [2]);
    }
}

#[test]
fn test_dollarsix_bad ()
{
    fn test_crypt (key: &str, salt: &str, expected: &str)
    {
        use std::ffi::CString;
        use std::str::from_utf8;
        let foo    = CString::new (key).unwrap ();
        let bar    = CString::new (salt).unwrap ();
        let result = crypt::crypt (foo, bar);
        assert! (result.is_some (), "Internal `crypt' error");
        match from_utf8 (result.unwrap ().as_slice ()) {
            Err (e)  => assert! (false, "Couldn't parse result as UTF-8: {}", e),
            Ok (got) => assert! (got != expected,
                                 "Hashes shouldn't match:
  Key:      {}
  Salt:     {}
  Expected: {}
  Got:      {}",
                                 key, salt, expected, got)
        }
    }

    let tests = [["secretions",     "$6$Schultzs",      "$6$Schultzs$hNWYcmh8RuCjDuv.Sjb/m8C98FULPwHhvNHrnMUm"],
                 ["telemetrys",     "$6$sincere",       "$6$sincere$H5aejF7tCWzn/ZsD6lX7PIjzus7TBvNM2qwMa8VEm"],
                 ["Regulus",        "$6$viscid",        "$6$viscid$4ml9MJPuPaUamU6qfsp5YGJliTGJh5hK7lgiB5dP8W"],
                 ["thralls",        "$6$bartered",      "$6$bartered$Dix0Z.ESdGmf5qUg9iiKoC1dV1ZPCcwoo/65v6qR"],
                 ["monograms",      "$6$compel",        "$6$compel$M9uD0.Xwf4dkettKZzTtAW6Jh2zLJ3kDZOa551rZsV"],
                 ["Massasoit",      "$6$insolubilitys", "$6$insolubilitys$LXaju60D6Cz7psjJVceMUOFzeRpkycPlJpW"],
                 ["censured",       "$6$amoralitys",    "$6$amoralitys$LpYTffkWFCAsPsfPRexNmIkwbf7xc45gyHwlZR"],
                 ["insignias",      "$6$worriers",      "$6$worriers$o0Ao9fh9MtIER/NnVfml7KqFMaTM5APwCnD9MDUI"],
                 ["upsets",         "$6$yearned",       "$6$yearned$vucq8Cl57jeO.Rkg91.CK4XzPLaYqGm89Ir.mem6f"],
                 ["versifying",     "$6$vindictive",    "$6$vindictive$p4a2/Ct7Ftg90hirGaBZPkNJlj0wuZlG3QJv3m"],
                 ["Villarreal",     "$6$prefaces",      "$6$prefaces$c6Asw36qBZgMZ2BoCvv.6L.qDWXAaU1W.VDzNSSc"],
                 ["voile",          "$6$westbound",     "$6$westbound$m2ng9p4svFpNwwOSqPYW1diOy0lcSx1b3wqC7nV"],
                 ["uppercases",     "$6$Newcastle",     "$6$Newcastle$J.3NHyl2JYwdRDxIO4YWT97nPwCkUYmZsoBld4m"],
                 ["graphite",       "$6$sacraments",    "$6$sacraments$2cLmGC3lz7jwFb/xUriMOrvtQPSrw1y7dz997/"],
                 ["salmons",        "$6$clones",        "$6$clones$GKhyY3cRdH7W9pkWbk4TMljGbbUNdTuSO4XjhtP5Tb"],
                 ["Souths",         "$6$Alstons",       "$6$Alstons$wOvzXQtKst8pJ96l.0FvrfGIjIIdEzS1Ggzqc7Mbz"],
                 ["exclusivenesss", "$6$ODonnell",      "$6$ODonnell$UVYaLkBOl4Na.gvqBrGHXhgzyjKXBu1S.N2vgc1N"],
                 ["ticket",         "$6$multiplicity",  "$6$multiplicity$vczzXdQevD2zRUQISdmR7w8BzrIELfgVaUfq"],
                 ["rooked",         "$6$striker",       "$6$striker$90VOzhQ98d77xb4qoKFq3hrfW4e4VKe6SLhKMf8Yk"],
                 ["vacancy",        "$6$healthiness",   "$6$healthiness$t7DokVqVlZFlAq.3ozy9GP7gDQxhYf6HF384c"]];

    for &test in tests.iter () {
        test_crypt (test [0], test [1], test [2]);
    }
}
