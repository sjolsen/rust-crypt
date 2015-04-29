#![feature(collections)]

pub mod crypt;

#[test]
fn foo_bar ()
{
    fn test_crypt (key: &str, salt: &str, expected: &str)
    {
        use std::ffi::CString;
        let foo    = CString::new (key).unwrap ();
        let bar    = CString::new (salt).unwrap ();
        let result = crypt::crypt (foo, bar);
        assert! (result.is_some ());
        assert! (result.unwrap () == Vec::<u8>::from (expected));
    }

    let foo = "foo";
    let bar = "$6$bar";
    let expected = "$6$bar$fsDFaQrp1MI9yhNnoKlXJFfXSLpz/dpqWaA2NP/71WJUzfqQxPVvFY6Px7VlDppW/NB6Cbz6BjF2b9bD.riFX1";
    test_crypt (foo, bar, expected);
}
