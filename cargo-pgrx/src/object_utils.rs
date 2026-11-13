use crate::target::target_arch;
use eyre::{WrapErr, bail};
use object::read::macho::{FatArch, MachOFatFile32, MachOFatFile64};
use object::{Object, ObjectSection};
use pgrx_sql_entity_graph::section::{
    MACHO_SECTION_NAME, MACHO_SEGMENT_NAME, is_schema_section_name,
};

#[derive(Clone, Copy)]
enum MachBits {
    Bits32,
    Bits64,
}

#[derive(Clone, Copy)]
enum MachEndian {
    Little,
    Big,
}

pub(crate) fn schema_section_data<'a>(
    data: &'a [u8],
    target: Option<&str>,
) -> eyre::Result<Option<&'a [u8]>> {
    let kind = object::FileKind::parse(data).wrap_err("couldn't parse binary kind")?;

    match kind {
        object::FileKind::MachOFat32 => {
            let fat = MachOFatFile32::parse(data).wrap_err("couldn't parse universal Mach-O")?;
            schema_section_data(slice_arch(data, fat.arches(), target_arch(target))?, target)
        }
        object::FileKind::MachOFat64 => {
            let fat = MachOFatFile64::parse(data).wrap_err("couldn't parse universal Mach-O")?;
            schema_section_data(slice_arch(data, fat.arches(), target_arch(target))?, target)
        }
        object::FileKind::MachO32 | object::FileKind::MachO64 => macho_schema_section_data(data),
        _ => schema_section_data_from_object(data),
    }
}

fn schema_section_data_from_object(data: &[u8]) -> eyre::Result<Option<&[u8]>> {
    let object = object::File::parse(data).wrap_err("couldn't parse binary object")?;

    for section in object.sections() {
        let name = section.name().wrap_err("couldn't read section name")?;
        if is_schema_section_name(name) {
            return section.data().wrap_err("couldn't read pgrx schema section").map(Some);
        }
    }

    Ok(None)
}

fn macho_schema_section_data(data: &[u8]) -> eyre::Result<Option<&[u8]>> {
    let (bits, endian, header_size) = parse_macho_header(data)?;
    let ncmds = read_u32(data, 16, endian)?;
    let mut cursor = header_size;

    for _ in 0..ncmds {
        let cmd = object::macho::LoadCommandType(read_u32(data, cursor, endian)?);
        let cmdsize = read_u32(data, cursor + 4, endian)? as usize;
        if cmdsize < 8 {
            bail!("invalid Mach-O load command size");
        }

        let command_end = cursor
            .checked_add(cmdsize)
            .filter(|end| *end <= data.len())
            .ok_or_else(|| eyre::eyre!("invalid Mach-O load command range"))?;
        let command = &data[cursor..command_end];

        let section = match bits {
            MachBits::Bits32 if cmd == object::macho::LC_SEGMENT => {
                macho_schema_section_from_segment32(command, data, endian)?
            }
            MachBits::Bits64 if cmd == object::macho::LC_SEGMENT_64 => {
                macho_schema_section_from_segment64(command, data, endian)?
            }
            _ => None,
        };

        if section.is_some() {
            return Ok(section);
        }

        cursor = command_end;
    }

    Ok(None)
}

fn macho_schema_section_from_segment32<'a>(
    command: &'a [u8],
    data: &'a [u8],
    endian: MachEndian,
) -> eyre::Result<Option<&'a [u8]>> {
    const SEGMENT_LEN: usize = 56;
    const SECTION_LEN: usize = 68;

    if command.len() < SEGMENT_LEN {
        bail!("invalid Mach-O 32-bit segment command");
    }

    if trim_nul(&command[8..24]) != MACHO_SEGMENT_NAME.as_bytes() {
        return Ok(None);
    }

    let nsects = read_u32(command, 48, endian)? as usize;
    let sections = &command[SEGMENT_LEN..];
    let expected_len = nsects
        .checked_mul(SECTION_LEN)
        .ok_or_else(|| eyre::eyre!("invalid Mach-O 32-bit section count"))?;
    if sections.len() < expected_len {
        bail!("invalid Mach-O 32-bit section table");
    }

    for index in 0..nsects {
        let start = index * SECTION_LEN;
        let section = &sections[start..start + SECTION_LEN];
        if trim_nul(&section[..16]) == MACHO_SECTION_NAME.as_bytes()
            && trim_nul(&section[16..32]) == MACHO_SEGMENT_NAME.as_bytes()
        {
            let size = read_u32(section, 36, endian)? as u64;
            let offset = read_u32(section, 40, endian)? as u64;
            return slice_range(data, offset, size).map(Some);
        }
    }

    Ok(None)
}
fn macho_schema_section_from_segment64<'a>(
    command: &'a [u8],
    data: &'a [u8],
    endian: MachEndian,
) -> eyre::Result<Option<&'a [u8]>> {
    const SEGMENT_LEN: usize = 72;
    const SECTION_LEN: usize = 80;

    if command.len() < SEGMENT_LEN {
        bail!("invalid Mach-O 64-bit segment command");
    }

    if trim_nul(&command[8..24]) != MACHO_SEGMENT_NAME.as_bytes() {
        return Ok(None);
    }

    let nsects = read_u32(command, 64, endian)? as usize;
    let sections = &command[SEGMENT_LEN..];
    let expected_len = nsects
        .checked_mul(SECTION_LEN)
        .ok_or_else(|| eyre::eyre!("invalid Mach-O 64-bit section count"))?;
    if sections.len() < expected_len {
        bail!("invalid Mach-O 64-bit section table");
    }

    for index in 0..nsects {
        let start = index * SECTION_LEN;
        let section = &sections[start..start + SECTION_LEN];
        if trim_nul(&section[..16]) == MACHO_SECTION_NAME.as_bytes()
            && trim_nul(&section[16..32]) == MACHO_SEGMENT_NAME.as_bytes()
        {
            let size = read_u64(section, 40, endian)?;
            let offset = read_u32(section, 48, endian)? as u64;
            return slice_range(data, offset, size).map(Some);
        }
    }

    Ok(None)
}

fn parse_macho_header(data: &[u8]) -> eyre::Result<(MachBits, MachEndian, usize)> {
    let magic = data.get(..4).ok_or_else(|| eyre::eyre!("Mach-O file is too small"))?;

    match u32::from_be_bytes(magic.try_into().expect("already bounds-checked to 4 bytes")) {
        object::macho::MH_CIGAM => Ok((MachBits::Bits32, MachEndian::Little, 28)),
        object::macho::MH_MAGIC => Ok((MachBits::Bits32, MachEndian::Big, 28)),
        object::macho::MH_CIGAM_64 => Ok((MachBits::Bits64, MachEndian::Little, 32)),
        object::macho::MH_MAGIC_64 => Ok((MachBits::Bits64, MachEndian::Big, 32)),
        _ => {
            bail!("invalid Mach-O magic");
        }
    }
}

fn read_u32(data: &[u8], offset: usize, endian: MachEndian) -> eyre::Result<u32> {
    let bytes =
        data.get(offset..offset + 4).ok_or_else(|| eyre::eyre!("unexpected end of Mach-O data"))?;
    let bytes: [u8; 4] = bytes.try_into().expect("already bounds-checked to 4 bytes");

    Ok(match endian {
        MachEndian::Little => u32::from_le_bytes(bytes),
        MachEndian::Big => u32::from_be_bytes(bytes),
    })
}

fn read_u64(data: &[u8], offset: usize, endian: MachEndian) -> eyre::Result<u64> {
    let bytes =
        data.get(offset..offset + 8).ok_or_else(|| eyre::eyre!("unexpected end of Mach-O data"))?;
    let bytes: [u8; 8] = bytes.try_into().expect("already bounds-checked to 8 bytes");

    Ok(match endian {
        MachEndian::Little => u64::from_le_bytes(bytes),
        MachEndian::Big => u64::from_be_bytes(bytes),
    })
}

fn slice_range(data: &[u8], offset: u64, size: u64) -> eyre::Result<&[u8]> {
    let end =
        offset.checked_add(size).ok_or_else(|| eyre::eyre!("invalid Mach-O section range"))?;
    let start = usize::try_from(offset).wrap_err("Mach-O section offset overflowed usize")?;
    let end = usize::try_from(end).wrap_err("Mach-O section end overflowed usize")?;

    data.get(start..end).ok_or_else(|| eyre::eyre!("invalid Mach-O section bounds"))
}

fn trim_nul(bytes: &[u8]) -> &[u8] {
    let len = bytes.iter().position(|byte| *byte == 0).unwrap_or(bytes.len());
    &bytes[..len]
}

fn slice_arch<'a>(data: &'a [u8], arches: &[impl FatArch], arch: &str) -> eyre::Result<&'a [u8]> {
    use object::macho::{
        CPU_SUBTYPE_ARM64_ALL, CPU_SUBTYPE_ARM64_V8, CPU_SUBTYPE_ARM64E, CPU_SUBTYPE_X86_64_ALL,
        CPU_SUBTYPE_X86_64_H,
    };
    let (architecture, subtype) = match arch {
        "x86" | "i386" | "i586" | "i686" => (object::Architecture::I386, None),
        "x86_64" => (object::Architecture::X86_64, Some(CPU_SUBTYPE_X86_64_ALL)),
        "x86_64h" => (object::Architecture::X86_64, Some(CPU_SUBTYPE_X86_64_H)),
        "arm" => (object::Architecture::Arm, None),
        "aarch64" => (object::Architecture::Aarch64, Some(CPU_SUBTYPE_ARM64_ALL)),
        "arm64e" => (object::Architecture::Aarch64, Some(CPU_SUBTYPE_ARM64E)),
        "mips" => (object::Architecture::Mips, None),
        "powerpc" => (object::Architecture::PowerPc, None),
        "powerpc64" => (object::Architecture::PowerPc64, None),
        _ => bail!("unsupported target architecture `{arch}` for universal Mach-O"),
    };
    let slice = arches
        .iter()
        .find(|candidate| {
            // Subtype IDs exclude capability bits such as LIB64 and the arm64e ABI version.
            candidate.architecture() == architecture
                && subtype.is_none_or(|subtype| candidate.cpusubtype().id() == subtype)
        })
        .or_else(|| {
            // ARM64_V8 is compatible with aarch64, but prefer its generic slice when present.
            if arch != "aarch64" {
                return None;
            }
            arches.iter().find(|candidate| {
                candidate.architecture() == architecture
                    && candidate.cpusubtype().id() == CPU_SUBTYPE_ARM64_V8
            })
        })
        .ok_or_else(|| {
            eyre::eyre!("universal Mach-O has no slice for target architecture `{arch}`")
        })?;
    slice.data(data).wrap_err_with(|| format!("couldn't read Mach-O slice for `{arch}`"))
}

#[cfg(test)]
mod tests {
    use super::schema_section_data;
    use object::macho::{
        CPU_SUBTYPE_ARM64_ALL, CPU_SUBTYPE_ARM64_V8, CPU_SUBTYPE_ARM64E, CPU_SUBTYPE_LIB64,
        CPU_SUBTYPE_PTRAUTH_ABI, CPU_SUBTYPE_X86_64_ALL, CPU_SUBTYPE_X86_64_H, CPU_TYPE_ARM64,
        CPU_TYPE_X86_64, CpuSubtype, CpuType, FAT_MAGIC, FAT_MAGIC_64,
    };
    use object::read::macho::{FatArch, MachOFatFile32};
    use pgrx_pg_config::{PgConfigSelector, Pgrx};
    use pgrx_sql_entity_graph::section::{
        MACHO_SECTION_NAME, MACHO_SEGMENT_NAME, schema_section_sentinel_entry,
    };

    fn parse_object(data: &[u8]) -> object::Result<object::File<'_>> {
        let kind = object::FileKind::parse(data)?;

        match kind {
            object::FileKind::MachOFat32 => {
                let arch = std::env::consts::ARCH;

                match slice_arch32(data, arch) {
                    Some(slice) => parse_object(slice),
                    None => {
                        panic!("Failed to slice architecture '{arch}' from universal binary.")
                    }
                }
            }
            _ => object::File::parse(data),
        }
    }

    fn slice_arch32<'a>(data: &'a [u8], arch: &str) -> Option<&'a [u8]> {
        use object::Architecture;

        let target = match arch {
            "x86" => Architecture::I386,
            "x86_64" => Architecture::X86_64,
            "arm" => Architecture::Arm,
            "aarch64" => Architecture::Aarch64,
            "mips" => Architecture::Mips,
            "powerpc" => Architecture::PowerPc,
            "powerpc64" => Architecture::PowerPc64,
            _ => Architecture::Unknown,
        };

        let candidates = MachOFatFile32::parse(data).ok()?;
        let architecture = candidates.arches().iter().find(|a| a.architecture() == target)?;

        architecture.data(data).ok()
    }

    fn minimal_macho64(cpu: CpuType, subtype: CpuSubtype, payload: &[u8]) -> Vec<u8> {
        const HEADER_LEN: usize = 32;
        const SEGMENT_LEN: usize = 72;
        const SECTION_LEN: usize = 80;
        let command_len = SEGMENT_LEN + SECTION_LEN;
        let fileoff = (HEADER_LEN + command_len) as u64;

        fn push_padded_name(bytes: &mut Vec<u8>, name: &str, width: usize) {
            assert!(name.len() <= width, "name `{name}` must fit in {width} bytes");
            bytes.extend_from_slice(name.as_bytes());
            bytes.resize(bytes.len() + (width - name.len()), 0);
        }

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&[0xcf, 0xfa, 0xed, 0xfe]);
        bytes.extend_from_slice(&cpu.0.to_le_bytes());
        bytes.extend_from_slice(&subtype.0.to_le_bytes());
        bytes.extend_from_slice(&object::macho::MH_DYLIB.0.to_le_bytes());
        bytes.extend_from_slice(&1u32.to_le_bytes());
        bytes.extend_from_slice(&(command_len as u32).to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());

        bytes.extend_from_slice(&object::macho::LC_SEGMENT_64.0.to_le_bytes());
        bytes.extend_from_slice(&(command_len as u32).to_le_bytes());
        push_padded_name(&mut bytes, MACHO_SEGMENT_NAME, 16);
        bytes.extend_from_slice(&0u64.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&fileoff.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&1u32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());

        push_padded_name(&mut bytes, MACHO_SECTION_NAME, 16);
        push_padded_name(&mut bytes, MACHO_SEGMENT_NAME, 16);
        bytes.extend_from_slice(&0u64.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&(fileoff as u32).to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());

        bytes.extend_from_slice(payload);
        bytes
    }

    #[test]
    fn reads_schema_section_from_minimal_macho64() {
        const PAYLOAD: &[u8] = b"\x04\x00\x00\x00test";
        let bytes = minimal_macho64(CPU_TYPE_X86_64, CPU_SUBTYPE_X86_64_ALL.into(), PAYLOAD);

        assert_eq!(schema_section_data(&bytes, None).unwrap(), Some(PAYLOAD));
    }

    fn universal_macho(bits64: bool) -> Vec<u8> {
        universal_macho_with_slices(
            bits64,
            &[
                (CPU_TYPE_X86_64, CPU_SUBTYPE_X86_64_ALL.into(), b"x86 schema"),
                (CPU_TYPE_ARM64, CPU_SUBTYPE_ARM64_ALL.into(), b"arm schema"),
            ],
        )
    }

    fn universal_macho_with_slices(
        bits64: bool,
        slices: &[(CpuType, CpuSubtype, &[u8])],
    ) -> Vec<u8> {
        let slices: Vec<_> = slices
            .iter()
            .map(|&(cpu, subtype, payload)| (cpu, subtype, minimal_macho64(cpu, subtype, payload)))
            .collect();
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&if bits64 { FAT_MAGIC_64 } else { FAT_MAGIC }.to_be_bytes());
        bytes.extend_from_slice(&(slices.len() as u32).to_be_bytes());
        let mut offset = 8 + slices.len() * if bits64 { 32 } else { 20 };
        for (cpu, subtype, slice) in &slices {
            bytes.extend_from_slice(&cpu.0.to_be_bytes());
            bytes.extend_from_slice(&subtype.0.to_be_bytes());
            if bits64 {
                bytes.extend_from_slice(&(offset as u64).to_be_bytes());
                bytes.extend_from_slice(&(slice.len() as u64).to_be_bytes());
                bytes.extend_from_slice(&0u32.to_be_bytes());
                bytes.extend_from_slice(&0u32.to_be_bytes());
            } else {
                bytes.extend_from_slice(&(offset as u32).to_be_bytes());
                bytes.extend_from_slice(&(slice.len() as u32).to_be_bytes());
                bytes.extend_from_slice(&0u32.to_be_bytes());
            }
            offset += slice.len();
        }
        for (_, _, slice) in slices {
            bytes.extend_from_slice(&slice);
        }
        bytes
    }

    #[test]
    fn universal_schema_uses_requested_architecture() {
        for bits64 in [false, true] {
            let bytes = universal_macho(bits64);
            for (target, expected) in
                [("x86_64-apple-darwin", b"x86 schema"), ("aarch64-apple-darwin", b"arm schema")]
            {
                assert_eq!(
                    schema_section_data(&bytes, Some(target)).unwrap(),
                    Some(expected.as_slice()),
                    "{target}, fat64={bits64}"
                );
            }
            let expected = match std::env::consts::ARCH {
                "x86_64" => Some(b"x86 schema".as_slice()),
                "aarch64" => Some(b"arm schema".as_slice()),
                _ => continue,
            };
            assert_eq!(schema_section_data(&bytes, None).unwrap(), expected);
        }
    }

    #[test]
    fn universal_schema_reports_missing_target_slice() {
        for bits64 in [false, true] {
            let bytes = universal_macho(bits64);
            let error = schema_section_data(&bytes, Some("i686-apple-darwin")).unwrap_err();
            assert!(error.to_string().contains("i686"), "{error}");
        }
    }

    #[test]
    fn universal_schema_uses_requested_cpu_subtype() {
        let mut slices: [(CpuType, CpuSubtype, &[u8]); 4] = [
            (CPU_TYPE_X86_64, CPU_SUBTYPE_X86_64_H.into(), b"haswell schema"),
            (CPU_TYPE_ARM64, CPU_SUBTYPE_ARM64E | CPU_SUBTYPE_PTRAUTH_ABI, b"arm64e schema"),
            (CPU_TYPE_X86_64, CPU_SUBTYPE_X86_64_ALL | CPU_SUBTYPE_LIB64, b"x86 schema"),
            (CPU_TYPE_ARM64, CPU_SUBTYPE_ARM64_ALL.into(), b"arm schema"),
        ];
        for bits64 in [false, true] {
            for _ in 0..2 {
                let bytes = universal_macho_with_slices(bits64, &slices);
                for (target, expected) in [
                    ("x86_64-apple-darwin", b"x86 schema".as_slice()),
                    ("aarch64-apple-darwin", b"arm schema".as_slice()),
                    ("x86_64h-apple-darwin", b"haswell schema".as_slice()),
                    ("arm64e-apple-darwin", b"arm64e schema".as_slice()),
                ] {
                    assert_eq!(
                        schema_section_data(&bytes, Some(target)).unwrap(),
                        Some(expected),
                        "{target}, fat64={bits64}"
                    );
                }
                let native = match std::env::consts::ARCH {
                    "x86_64" => Some(b"x86 schema".as_slice()),
                    "aarch64" => Some(b"arm schema".as_slice()),
                    _ => None,
                };
                if let Some(native) = native {
                    assert_eq!(schema_section_data(&bytes, None).unwrap(), Some(native));
                }
                slices.reverse();
            }
        }
    }

    #[test]
    fn universal_schema_rejects_other_cpu_subtypes() {
        for bits64 in [false, true] {
            for (target, cpu, subtype) in [
                ("x86_64-apple-darwin", CPU_TYPE_X86_64, CPU_SUBTYPE_X86_64_H),
                ("x86_64h-apple-darwin", CPU_TYPE_X86_64, CPU_SUBTYPE_X86_64_ALL),
                ("aarch64-apple-darwin", CPU_TYPE_ARM64, CPU_SUBTYPE_ARM64E),
                ("arm64e-apple-darwin", CPU_TYPE_ARM64, CPU_SUBTYPE_ARM64_ALL),
            ] {
                let bytes =
                    universal_macho_with_slices(bits64, &[(cpu, subtype.into(), b"wrong schema")]);
                let error = schema_section_data(&bytes, Some(target)).unwrap_err();
                assert!(
                    error.to_string().contains("no slice"),
                    "{target}, fat64={bits64}: {error}"
                );
            }
        }
    }

    #[test]
    fn universal_schema_prefers_generic_arm64_to_v8() {
        for bits64 in [false, true] {
            let v8 = (CPU_TYPE_ARM64, CPU_SUBTYPE_ARM64_V8.into(), b"v8 schema".as_slice());
            let generic = (CPU_TYPE_ARM64, CPU_SUBTYPE_ARM64_ALL.into(), b"arm schema".as_slice());
            let bytes = universal_macho_with_slices(bits64, &[v8, generic]);
            assert_eq!(
                schema_section_data(&bytes, Some("aarch64-apple-darwin")).unwrap(),
                Some(b"arm schema".as_slice())
            );
            let bytes = universal_macho_with_slices(bits64, &[v8]);
            assert_eq!(
                schema_section_data(&bytes, Some("aarch64-apple-darwin")).unwrap(),
                Some(b"v8 schema".as_slice())
            );
        }
    }

    #[test]
    fn universal_schema_rejects_invalid_slice_bounds() {
        for bits64 in [false, true] {
            let mut bytes = universal_macho(bits64);
            bytes.truncate(bytes.len() - 1);
            let error = schema_section_data(&bytes, Some("aarch64-apple-darwin")).unwrap_err();
            assert!(format!("{error:#}").contains("Invalid fat arch offset or size"), "{error:#}");
        }
    }

    #[test]
    fn reads_sentinel_schema_section_from_minimal_macho64() {
        let payload = schema_section_sentinel_entry();
        let bytes = minimal_macho64(CPU_TYPE_X86_64, CPU_SUBTYPE_X86_64_ALL.into(), &payload);

        assert_eq!(schema_section_data(&bytes, None).unwrap(), Some(payload.as_slice()));
    }

    #[test]
    fn returns_none_for_valid_binary_without_schema_section() {
        let root_path = env!("CARGO_MANIFEST_DIR");
        let fixture_path = format!("{root_path}/tests/fixtures/macos-universal-binary");
        let bin = std::fs::read(fixture_path).unwrap();

        assert!(schema_section_data(&bin, None).unwrap().is_none());
    }

    #[test]
    fn parses_managed_postmasters() {
        let pgrx = Pgrx::from_config().unwrap();
        let mut results = pgrx
            .iter(PgConfigSelector::All)
            .map(|pg_config| {
                let fixture_path = pg_config.unwrap().postmaster_path().unwrap();
                let bin = std::fs::read(fixture_path).unwrap();

                parse_object(&bin).is_ok()
            })
            .peekable();

        assert!(results.peek().is_some());
        assert!(results.all(|r| r));
    }

    #[test]
    fn parses_universal_binary_slice() {
        let root_path = env!("CARGO_MANIFEST_DIR");
        let fixture_path = format!("{root_path}/tests/fixtures/macos-universal-binary");
        let bin = std::fs::read(fixture_path).unwrap();

        let slice = slice_arch32(&bin, "aarch64")
            .expect("Failed to slice architecture 'aarch64' from universal binary.");
        assert!(parse_object(slice).is_ok());
    }

    #[test]
    fn slice_unknown_architecture_returns_none() {
        let root_path = env!("CARGO_MANIFEST_DIR");
        let fixture_path = format!("{root_path}/tests/fixtures/macos-universal-binary");
        let bin = std::fs::read(fixture_path).unwrap();

        assert!(slice_arch32(&bin, "foo").is_none());
    }
}
