use std::any::Any;

use nucleotide_converter::{CodeConverter, CodeConverterInPlace};

fn demo_converter<T: CodeConverter + Any>(code: &str, converter: &T) {
    let mut out = vec![0; code.len()];
    converter.convert(code.as_bytes(), &mut out);
    println!("[{}] {:?}", std::any::type_name::<T>(), out);
}

fn demo_pack_unpack<P: CodeConverterInPlace, U: CodeConverter>(
    code: &str,
    packer: &P,
    unpacker: &U,
) {
    let mut packed = code.as_bytes().to_vec();
    let packed = packer.convert_in_place(&mut packed);
    let mut unpacked = vec![0; code.len()];
    unpacker.convert(&packed, &mut unpacked);
    println!(
        "[{} -> {}] packed -> {} = {:02x?}",
        std::any::type_name::<U>(),
        std::any::type_name::<P>(),
        code,
        packed
    );
}

fn main() {
    let code = std::env::args()
        .nth(1)
        .unwrap_or("ATCGatcgATCGatcgNn".to_string());
    let converter = nucleotide_converter::NaiveCodeConverter::default();
    demo_converter(&code, &converter);
    let converter = nucleotide_converter::LUTCodeConverter::default();
    demo_converter(&code, &converter);
    let converter = nucleotide_converter::SSE2CodeConverter::default();
    demo_converter(&code, &converter);
    let converter = nucleotide_converter::SSSE3CodeConverter::default();
    demo_converter(&code, &converter);
    let converter = nucleotide_converter::AVX2CodeConverter::default();
    demo_converter(&code, &converter);
    let converter = nucleotide_converter::AVX512VbmiCodeConverter::default();
    demo_converter(&code, &converter);

    let (packer, unpacker) = (
        nucleotide_converter::custom_alphabet::LUTInPlacePacker::default(),
        nucleotide_converter::custom_alphabet::LUTUnpacker::default(),
    );
    demo_pack_unpack(&code, &packer, &unpacker);

    let (packer, unpacker) = (
        nucleotide_converter::custom_alphabet::SSE41InPlacePacker::default(),
        nucleotide_converter::custom_alphabet::SSSE3Unpacker::default(),
    );
    demo_pack_unpack(&code, &packer, &unpacker);
}
