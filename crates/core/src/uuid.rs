// -------------------------------------------------------------------------------------------------
//  Copyright (c) 2015-2025 dyntrait. All rights reserved.
//
//  @File         : uuid.rs
//  @Author       : dyntrait
//  @Description  :
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
// -------------------------------------------------------------------------------------------------
//! A `UUID4` Universally Unique Identifier (UUID) version 4 (RFC 4122).

use std::{
    ffi::CStr,
    fmt::{Debug, Display, Formatter},
    hash::Hash,
    io::{Cursor, Write},
    str::FromStr,
};

use rand::RngCore;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

/// The maximum length of ASCII characters for a `UUID4` string value (includes null terminator).
pub const UUID4_LEN: usize = 37;

/// Represents a Universally Unique Identifier (UUID)
/// version 4 based on a 128-bit label as specified in RFC 4122.
/// repr 是 "Representation"（表示、布局）的缩写,请按照指定的某种规则来安排这个类型在内存中各字段的先后顺序和对齐方式
/// Rust 编译器为了优化性能，默认会对结构体字段进行重排（Field Reordering），以减少内存对齐带来的空白填充。但是，C 语言的内存布局是确定的（按定义顺序，遵循 C 的对齐规则）。
/// 如果 Rust 需要把一个结构体传递给 C 语言写的库（比如你在前面看到的 staticlib 场景），就必须使用 #[repr(C)] 来保证双方对内存的理解是一致的
/// 使用 #[repr(C)] 后，结构体字段的排列顺序严格按照代码中定义的顺序，对齐方式也与 C 标准一致。这在编写底层系统代码、序列化或与硬件交互时非常重要

#[repr(C)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct UUID4 {
    /// The UUID v4 value as a fixed-length C string byte array (includes null terminator).
    pub value: [u8; 37], // cbindgen issue using the constant in the array
}

impl UUID4 {
    /// Creates a new [`UUID4`] instance.
    ///
    /// The UUID value is stored as a fixed-length C string byte array.
    #[must_use]
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut bytes = [0u8; 16];  //uuid实际是一个128位数字 16*8=128位
        rng.fill_bytes(&mut bytes);
        // 版本 4：基于随机数 (UUIDv4) 理论上存在碰撞的可能，但由于位数极多（$2^{128}$），在实际应用中碰撞的概率低到可以忽略不计。它是目前应用最广泛的 UUID 类型
        bytes[6] = (bytes[6] & 0x0F) | 0x40; // Set the version to 4
        bytes[8] = (bytes[8] & 0x3F) | 0x80; // Set the variant to RFC 4122

        let mut value = [0u8; UUID4_LEN];
        // Cursor 是一种零成本抽象（Zero-cost abstraction）。它利用 Rust 的特征系统，在不改变底层数据结构的前提下，为内存操作提供了与文件 I/O 一致的接口，极大地简化了数据的读写、定位和格式化操作
        // 它是一个包装器（Wrapper），用于将普通的内存块（如字节数组 [u8] 或 Vec<u8>）包装成一个逻辑上的文件
        let mut cursor = Cursor::new(&mut value[..36]);
        // UUID 以 32 个十六进制数字表示，以连字符分隔成 5 组 550e8400-e29b-41d4-a716-446655440000
        // UUID 的 8-4-4-4-12 标准十六进制格式 08x: 表示十六进制格式，宽度为 8，如果不够，前面补 0
        write!(
            cursor,
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), // u32::from_be_bytes(...): UUID 的前 4 个字节被视为一个大端序（Big-Endian）的 32 位无符号整数
            u16::from_be_bytes([bytes[4], bytes[5]]), // 分组 2, 3, 4 分别是 2 字节的大端序无符号整数
            u16::from_be_bytes([bytes[6], bytes[7]]),
            u16::from_be_bytes([bytes[8], bytes[9]]),
            u64::from_be_bytes([
                bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15], 0, 0
            ]) >> 16 // 最后 6 个字节加上两个 0 组成 8 字节，视为 u64，然后右移 16 位，截取高 6 字节作为最后的 12 个十六进制字符
        )
            .expect("Error writing UUID string to buffer");

        value[36] = 0; // Add the null terminator

        Self { value }
    }

    /// Converts the [`UUID4`] to a C string reference.
    ///
    /// # Panics
    ///
    /// Panics if the internal byte array is not a valid C string (does not end with a null terminator).
    #[must_use]
    pub fn to_cstr(&self) -> &CStr {
        // SAFETY: We always store valid C strings
        CStr::from_bytes_with_nul(&self.value)
            .expect("UUID byte representation should be a valid C string")
    }

    /// Returns the UUID as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        // SAFETY: We always store valid ASCII UUID strings
        self.to_cstr().to_str().expect("UUID should be valid UTF-8")
    }

    /// Returns the raw UUID bytes (16 bytes).
    ///
    /// This method is optimized for serialization where the UUID bytes
    /// are needed directly without string conversion overhead.
    #[must_use]
    pub fn as_bytes(&self) -> [u8; 16] {
        // Parse the string representation to extract the raw bytes
        // This is done once at read time to avoid repeated parsing
        let uuid_str = self.to_cstr().to_str().expect("Valid UTF-8");
        let uuid = Uuid::parse_str(uuid_str).expect("Valid UUID4");
        *(uuid.as_bytes())
    }

    fn validate_v4(uuid: &Uuid) {
        // 无论在什么模式下编译，assert_eq! 都会被包含在生成的二进制文件中。
        // Validate this is a v4 UUID
        assert_eq!(
            uuid.get_version(),
            Some(uuid::Version::Random),
            "UUID is not version 4"
        );

        // Validate RFC4122 variant
        assert_eq!(
            uuid.get_variant(),
            uuid::Variant::RFC4122,
            "UUID is not RFC 4122 variant"
        );
    }

    fn from_validated_uuid(uuid: &Uuid) -> Self {
        let mut value = [0; UUID4_LEN];
        let uuid_str = uuid.to_string();
        value[..uuid_str.len()].copy_from_slice(uuid_str.as_bytes());
        value[uuid_str.len()] = 0; // Add null terminator
        Self { value }
    }
}
// std::str::FromStr 必须处理错误 返回 Err，让调用者决定如何处理
impl FromStr for UUID4 {
    type Err = uuid::Error;

    /// Attempts to create a [`UUID4`] from a string representation.
    ///
    /// The string should be a valid UUID in the standard format (e.g., "2d89666b-1a1e-4a75-b193-4eb3b454c757").
    ///
    /// # Panics
    ///
    /// Panics if the `value` is not a valid UUID version 4 RFC 4122.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::try_parse(value)?;
        Self::validate_v4(&uuid);
        Ok(Self::from_validated_uuid(&uuid))
    }
}

// From<&str> (来自于 std::convert::From) 不应该失败 引发 Panic (引发崩溃)
impl From<&str> for UUID4 {
    /// Creates a [`UUID4`] from a string slice.
    ///
    /// # Panics
    ///
    /// Panics if the `value` string is not a valid UUID version 4 RFC 4122.
    //  只要类型 T 实现了 FromStr，字符串就可以调用 parse() 转为 T
    // fn parse<T: FromStr>(&self) -> Result<T, T::Err>;
    fn from(value: &str) -> Self {
        value
            .parse()
            .expect("`value` should be a valid UUID version 4 (RFC 4122)")
    }
}

impl From<String> for UUID4 {
    /// Creates a [`UUID4`] from a string.
    ///
    /// # Panics
    ///
    /// Panics if the `value` string is not a valid UUID version 4 RFC 4122.
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<uuid::Uuid> for UUID4 {
    /// Creates a [`UUID4`] from a [`uuid::Uuid`].
    ///
    /// # Panics
    ///
    /// Panics if the `value` is not a valid UUID version 4 RFC 4122.
    fn from(value: uuid::Uuid) -> Self {
        Self::validate_v4(&value);
        Self::from_validated_uuid(&value)
    }
}

impl From<UUID4> for uuid::Uuid {
    /// Creates a [`uuid::Uuid`] from a [`UUID4`].
    fn from(value: UUID4) -> Self {
        Self::from_bytes(value.as_bytes())
    }
}

impl Default for UUID4 {
    /// Creates a new default [`UUID4`] instance.
    ///
    /// The default UUID4 is simply a newly generated UUID version 4.
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for UUID4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", stringify!(UUID4), self) // stringify!(UUID4) 会被替换为 "UUID4" 仅仅是把变量的值转为字符串，它是把你写在括号里的那段代码字面量转为字符串
    }
}

impl Display for UUID4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_cstr().to_string_lossy())  // 非法的 UTF-8：如果 C 字符串内部包含不合法的 UTF-8 字节序列（例如 C 语言可能允许的非法编码），to_string_lossy() 会把这些字节替换为 ``，从而丢失了原始的非法字节信息
    }
}

impl Serialize for UUID4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
    // to_string() 是 std::string::ToString 特征（Trait）中定义的方法 该类型必须实现了 std::fmt::Display 特征
    // 当你调用 to_string() 时，Rust 会在后台调用 fmt::Display::fmt 方法，将你的数据按照“用户友好”的格式渲染到内存中，并将其封装成一个 String 对象
}

impl<'de> Deserialize<'de> for UUID4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // &str 不是未卜先知得到的，而是序列化器（Deserializer）被要求寻找一个字符串，然后它在数据源中找到并直接“借用”了这块内存的结果
        // Rust 编译器看到这个明确的类型标注，就会推断出你要求 Deserialize::deserialize(deserializer) 方法返回一个 &str
        // 你没有在“未卜先知”，你是在向反序列化器发出一项具体的请求：“请给我一个指向原始数据中字符串部分的引用”。
        // 如果序列化器无法做到（例如数据是被压缩或编码过的，必须先解压才能找到字符串），serde 就会在运行时报错
        let uuid4_str: &str = Deserialize::deserialize(deserializer)?;
        let uuid4: Self = uuid4_str.into();
        Ok(uuid4)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    use rstest::*;
    use uuid;

    use super::*;

    #[rstest]
    fn test_new() {
        let uuid = UUID4::new();
        let uuid_string = uuid.to_string();
        let uuid_parsed = Uuid::parse_str(&uuid_string).unwrap();
        assert_eq!(uuid_parsed.get_version().unwrap(), uuid::Version::Random);
        assert_eq!(uuid_parsed.to_string().len(), 36);

        // Version 4 requires bits: 0b0100xxxx
        assert_eq!(&uuid_string[14..15], "4");
        // RFC4122 variant requires bits: 0b10xxxxxx
        let variant_char = &uuid_string[19..20];
        assert!(matches!(variant_char, "8" | "9" | "a" | "b" | "A" | "B"));
    }

    #[rstest]
    fn test_uuid_format() {
        let uuid = UUID4::new();
        let bytes = uuid.value;

        // Check null termination
        assert_eq!(bytes[36], 0);

        // Verify dash positions
        assert_eq!(bytes[8] as char, '-');
        assert_eq!(bytes[13] as char, '-');
        assert_eq!(bytes[18] as char, '-');
        assert_eq!(bytes[23] as char, '-');

        let s = uuid.to_string();
        assert_eq!(s.chars().nth(14).unwrap(), '4');
    }

    #[rstest]
    #[should_panic(expected = "UUID is not version 4")]
    fn test_from_str_with_non_version_4_uuid_panics() {
        let uuid_string = "6ba7b810-9dad-11d1-80b4-00c04fd430c8"; // v1 UUID
        let _ = UUID4::from(uuid_string);
    }

    #[rstest]
    fn test_case_insensitive_parsing() {
        let upper = "2D89666B-1A1E-4A75-B193-4EB3B454C757";
        let lower = "2d89666b-1a1e-4a75-b193-4eb3b454c757";
        let uuid_upper = UUID4::from(upper);
        let uuid_lower = UUID4::from(lower);

        assert_eq!(uuid_upper, uuid_lower);
        assert_eq!(uuid_upper.to_string(), lower);
    }

    #[rstest]
    #[case("6ba7b810-9dad-11d1-80b4-00c04fd430c8")] // v1 (time-based)
    #[case("000001f5-8fa9-21d1-9df3-00e098032b8c")] // v2 (DCE Security)
    #[case("3d813cbb-47fb-32ba-91df-831e1593ac29")] // v3 (MD5 hash)
    #[case("fb4f37c1-4ba3-5173-9812-2b90e76a06f7")] // v5 (SHA-1 hash)
    #[should_panic(expected = "UUID is not version 4")]
    fn test_invalid_version(#[case] uuid_string: &str) {
        let _ = UUID4::from(uuid_string);
    }

    #[rstest]
    #[should_panic(expected = "UUID is not RFC 4122 variant")]
    fn test_non_rfc4122_variant() {
        // Valid v4 but wrong variant
        let uuid = "550e8400-e29b-41d4-0000-446655440000";
        let _ = UUID4::from(uuid);
    }

    #[rstest]
    #[case("")] // Empty string
    #[case("not-a-uuid-at-all")] // Invalid format
    #[case("6ba7b810-9dad-11d1-80b4")] // Too short
    #[case("6ba7b810-9dad-11d1-80b4-00c04fd430c8-extra")] // Too long
    #[case("6ba7b810-9dad-11d1-80b4=00c04fd430c8")] // Wrong separator
    #[case("6ba7b81019dad111d180b400c04fd430c8")] // No separators
    #[case("6ba7b810-9dad-11d1-80b4-00c04fd430")] // Truncated
    #[case("6ba7b810-9dad-11d1-80b4-00c04fd430cg")] // Invalid hex character
    fn test_invalid_uuid_cases(#[case] invalid_uuid: &str) {
        assert!(UUID4::from_str(invalid_uuid).is_err());
    }

    #[rstest]
    fn test_default() {
        let uuid: UUID4 = UUID4::default();
        let uuid_string = uuid.to_string();
        let uuid_parsed = Uuid::parse_str(&uuid_string).unwrap();
        assert_eq!(uuid_parsed.get_version().unwrap(), uuid::Version::Random);
    }

    #[rstest]
    fn test_from_str() {
        let uuid_string = "2d89666b-1a1e-4a75-b193-4eb3b454c757";
        let uuid = UUID4::from(uuid_string);
        let result_string = uuid.to_string();
        let result_parsed = Uuid::parse_str(&result_string).unwrap();
        let expected_parsed = Uuid::parse_str(uuid_string).unwrap();
        assert_eq!(result_parsed, expected_parsed);
    }

    #[rstest]
    fn test_from_uuid() {
        let original = uuid::Uuid::new_v4();
        let uuid4 = UUID4::from(original);
        assert_eq!(uuid4.to_string(), original.to_string());
    }

    #[rstest]
    fn test_equality() {
        let uuid1 = UUID4::from("2d89666b-1a1e-4a75-b193-4eb3b454c757");
        let uuid2 = UUID4::from("46922ecb-4324-4e40-a56c-841e0d774cef");
        assert_eq!(uuid1, uuid1);
        assert_ne!(uuid1, uuid2);
    }

    #[rstest]
    fn test_debug() {
        let uuid_string = "2d89666b-1a1e-4a75-b193-4eb3b454c757";
        let uuid = UUID4::from(uuid_string);
        assert_eq!(format!("{uuid:?}"), format!("UUID4({uuid_string})"));
    }

    #[rstest]
    fn test_display() {
        let uuid_string = "2d89666b-1a1e-4a75-b193-4eb3b454c757";
        let uuid = UUID4::from(uuid_string);
        assert_eq!(format!("{uuid}"), uuid_string);
    }

    #[rstest]
    fn test_to_cstr() {
        let uuid = UUID4::new();
        let cstr = uuid.to_cstr();

        assert_eq!(cstr.to_str().unwrap(), uuid.to_string());
        assert_eq!(cstr.to_bytes_with_nul()[36], 0);
    }

    #[rstest]
    fn test_as_str() {
        let uuid = UUID4::new();
        let s = uuid.as_str();

        assert_eq!(s, uuid.to_string());
        assert_eq!(s.len(), 36);
    }

    #[rstest]
    fn test_hash_consistency() {
        let uuid = UUID4::new();

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        uuid.hash(&mut hasher1);
        uuid.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[rstest]
    fn test_serialize_json() {
        let uuid_string = "2d89666b-1a1e-4a75-b193-4eb3b454c757";
        let uuid = UUID4::from(uuid_string);

        let serialized = serde_json::to_string(&uuid).unwrap();
        let expected_json = format!("\"{uuid_string}\"");
        assert_eq!(serialized, expected_json);
    }

    #[rstest]
    fn test_deserialize_json() {
        let uuid_string = "2d89666b-1a1e-4a75-b193-4eb3b454c757";
        let serialized = format!("\"{uuid_string}\"");

        let deserialized: UUID4 = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.to_string(), uuid_string);
    }

    #[rstest]
    fn test_serialize_deserialize_round_trip() {
        let uuid = UUID4::new();

        let serialized = serde_json::to_string(&uuid).unwrap();
        let deserialized: UUID4 = serde_json::from_str(&serialized).unwrap();

        assert_eq!(uuid, deserialized);
    }

    #[rstest]
    fn test_as_bytes() {
        let uuid_string = "2d89666b-1a1e-4a75-b193-4eb3b454c757";
        let uuid = UUID4::from(uuid_string);

        let bytes = uuid.as_bytes();
        assert_eq!(bytes.len(), 16);

        // Reconstruct UUID from bytes and verify it matches
        let reconstructed = Uuid::from_bytes(bytes);
        assert_eq!(reconstructed.to_string(), uuid_string);

        // Verify version 4
        assert_eq!(reconstructed.get_version().unwrap(), uuid::Version::Random);
    }

    #[rstest]
    fn test_as_bytes_round_trip() {
        let uuid1 = UUID4::new();
        let bytes = uuid1.as_bytes();
        let uuid2 = UUID4::from(Uuid::from_bytes(bytes));

        assert_eq!(uuid1, uuid2);
    }
}
