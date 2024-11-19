use proc_macro::TokenStream;
use quote::quote; // 데이터 구조 -> 문자열 러스트 코드 변환
use syn; // 문자열 러스트 코드 -> 데이터 구조로 파싱

/// 사용자가 타입에 `#[derive(HelloMacro)]` 지정 시 호출됨
/// `TokenStream` 출력으로 크레이트 사용자가 작성하는 코드에 추가됨
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 조작 가능한 구문 트리로 러스트 코드의 표현을 구성합니다
    // DeriveInput 구조체를 반환받음
    // `TokenStream` 반환을 해야하기 때문에 `unwrap()` 실패 시 `panic!`
    // 실제 제품 코드에서는 `panic!` 또는 `expect`를 통해 내용을 제공해야 함
    let ast = syn::parse(input).unwrap();

    // 트레이트 구현체를 생성합니다
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident; // 실제 어노테이션된 타입의 이름 식별자를 담고 있음
    // `quote!` 매크로는 ㅂ나환하고자 하는 러스트 코드를 정의하도록 해줌
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // `#name`을 통해 `name`에 담겨있는 값을 치환
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}