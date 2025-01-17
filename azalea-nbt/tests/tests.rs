use ahash::AHashMap;
use azalea_nbt::Tag;
use std::io::Cursor;

#[test]
fn test_decode_hello_world() {
    // read hello_world.nbt
    let buf = include_bytes!("hello_world.nbt").to_vec();
    let tag = Tag::read(&mut Cursor::new(&buf[..])).unwrap();
    assert_eq!(
        tag,
        Tag::Compound(AHashMap::from_iter(vec![(
            "hello world".to_string(),
            Tag::Compound(AHashMap::from_iter(vec![(
                "name".to_string(),
                Tag::String("Bananrama".to_string()),
            )]))
        )]))
    );
}

#[test]
fn test_roundtrip_hello_world() {
    let original = include_bytes!("hello_world.nbt").to_vec();

    let mut original_stream = Cursor::new(&original[..]);
    let tag = Tag::read(&mut original_stream).unwrap();

    // write hello_world.nbt
    let mut result = Vec::new();
    tag.write(&mut result).unwrap();

    assert_eq!(result, original);
}

#[test]
fn test_bigtest() {
    // read bigtest.nbt
    let original = include_bytes!("bigtest.nbt").to_vec();

    let mut original_stream = Cursor::new(original);
    let original_tag = Tag::read_gzip(&mut original_stream).unwrap();

    let mut result = Vec::new();
    original_tag.write(&mut result).unwrap();

    let decoded_tag = Tag::read(&mut Cursor::new(&result)).unwrap();

    assert_eq!(decoded_tag, original_tag);
}

#[test]
fn test_stringtest() {
    let correct_tag = Tag::Compound(AHashMap::from_iter(vec![(
        "😃".to_string(),
        Tag::List(vec![
            Tag::String("asdfkghasfjgihsdfogjsndfg".to_string()),
            Tag::String("jnabsfdgihsabguiqwrntgretqwejirhbiqw".to_string()),
            Tag::String("asd".to_string()),
            Tag::String("wqierjgt7wqy8u4rtbwreithwretiwerutbwenryq8uwervqwer9iuqwbrgyuqrbtwierotugqewrtqwropethert".to_string()),
            Tag::String("asdf".to_string()),
            Tag::String("alsdkjiqwoe".to_string()),
            Tag::String("lmqi9hyqd".to_string()),
            Tag::String("qwertyuiop".to_string()),
            Tag::String("asdfghjkl".to_string()),
            Tag::String("zxcvbnm".to_string()),
            Tag::String("                               ".to_string()),
            Tag::String("words words words words words words".to_string()),
            Tag::String("aaaaaaaaaaaaaaaaaaaa".to_string()),
            Tag::String("♥".to_string()),
            Tag::String("a\nb\n\n\nc\r\rd".to_string()),
            Tag::String("😁".to_string()),
        ])
    )]));
    let original = include_bytes!("stringtest.nbt").to_vec();

    let mut original_stream = Cursor::new(original);
    let original_tag = Tag::read_gzip(&mut original_stream).unwrap();

    assert_eq!(original_tag, correct_tag);
}

#[test]
fn test_complex_player() {
    let original = include_bytes!("complex_player.dat").to_vec();

    let mut original_stream = Cursor::new(original);
    let original_tag = Tag::read_gzip(&mut original_stream).unwrap();

    let mut result = Vec::new();
    original_tag.write(&mut result).unwrap();

    let decoded_tag = Tag::read(&mut Cursor::new(&result)).unwrap();

    assert_eq!(decoded_tag, original_tag);
}

#[test]
fn test_simple_player() {
    let original = include_bytes!("simple_player.dat").to_vec();

    let mut original_stream = Cursor::new(original);
    let original_tag = Tag::read_gzip(&mut original_stream).unwrap();

    let mut result = Vec::new();
    original_tag.write(&mut result).unwrap();

    let decoded_tag = Tag::read(&mut Cursor::new(&result)).unwrap();

    assert_eq!(decoded_tag, original_tag);
}
