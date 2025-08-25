use std::any::Any;

use nucleotide_converter::CodeConverter;

fn demo_converter<T: CodeConverter + Any>(code: &str, converter: &T) {
    let mut out = [0u8; 16];
    converter.convert(code.as_bytes(), &mut out);
    println!("[{}] {:?}", std::any::type_name::<T>(), out);
}

fn main() {
    let code = std::env::args()
        .nth(1)
        .unwrap_or("ATCGatcgATCGatcg".to_string());
    let converter = nucleotide_converter::NaiveCodeConverter;
    demo_converter(&code, &converter);
    let converter = nucleotide_converter::LUTCodeConverter;
    demo_converter(&code, &converter);
    let converter = nucleotide_converter::SSE2CodeConverter::default();
    demo_converter(&code, &converter);
    let converter = nucleotide_converter::AVX2CodeConverter::default();
    demo_converter(&code, &converter);
}
