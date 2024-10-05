// ./common.rs 를 사용할 경우 테스트 출력 결과에 common 테스트 결과가 포함됨
// ./common/mod.rs 를 사용할 경우 테스트 출력 결과에 common 이 제외됨
//      이러한 서브모듈 명명규칙을 따르는 파일은 통합 테스트 파일로 취급하지 않음

pub fn setup() {
    // 라이브러리 테스트와 관련된 공통 설정 코드
}