// =========================================================================
// [포트폴리오 주석] 프로젝트명: 저사양 환경을 위한 Rust 기반 초경량 파일 검색 엔진
// 분석자: limhiru (Nmap 기여자)
// 분석 목적: 가비지 컬렉터(GC)가 없는 Rust의 메모리 효율성과 속도 체감
// =========================================================================

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    // 1. 성능 측정을 위한 타이머 시작 (Rust의 정밀 타임 스탬프 기능)
    let start_time = Instant::now();

    // 2. 검색 대상 파일과 찾을 단어 설정
    // (분석 메모: 테스트를 위해 우선 Cargo.toml 파일을 읽어 "mini"라는 단어를 검색하도록 설정함)
    let file_path = "Cargo.toml"; 
    let search_target = "mini";   

    println!("🔍 분석 시작: [{}] 파일에서 '{}' 단어를 탐색합니다...", file_path, search_target);

    // 3. 파일 열기 (안전장치 기능: 파일이 없으면 에러를 뿜고 안전하게 종료됨)
    let file = File::open(file_path).expect("❌ 파일을 열 수 없습니다. 경로를 확인하세요.");
    
    // 4. 저사양 PC를 위한 '버퍼 리더(BufReader)' 도입 
    // ※ 중요: 파일을 통째로 램(RAM)에 올리면 저사양 컴퓨터는 멈춥니다. 
    //    이 방식은 파일을 한 줄씩 쪼개서 읽기 때문에 램을 거의 0MB 수준으로 아낍니다!
    let reader = BufReader::new(file);
    let mut line_count = 0;
    let mut match_count = 0;

    // 5. 고속 루프 탐색 체계
    for (index, line) in reader.lines().enumerate() {
        line_count += 1;
        if let Ok(line_text) = line {
            // 해당 줄에 내가 찾는 단어가 포함되어 있는지 포인터 추적
            if line_text.contains(search_target) {
                match_count += 1;
                println!("   [{}번째 줄 발견]: {}", index + 1, line_text.trim());
            }
        }
    }

    // 6. 타이머 종료 및 벤치마크 결과 출력
    let duration = start_time.elapsed();
    
    println!("==================================================");
    println!("✅ 탐색 완료!");
    println!("📊 총 읽은 줄 수: {}줄", line_count);
    println!("🎯 발견된 단어 수: {}개", match_count);
    println!("⚡ 총 소요 시간: {:?}", duration); // 러스트의 압도적인 속도가 찍히는 구간
    println!("==================================================");
}