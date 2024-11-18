Rust 공부를 위해 채팅 서버는 Rust로 만들어 보겠습니다.

과연 할 수 있을지...

# 읽고 고려 해볼 글
- 대규모 채팅서버 아키텍처
https://engineering.linecorp.com/ko/blog/the-architecture-behind-chatting-on-line-live
- 어떤 DBMS를 사용할까?
https://velog.io/@murphytklee/%EC%B1%84%ED%8C%85-%EC%8B%9C%EC%8A%A4%ED%85%9C-NoSQL-%ED%8A%B9%EC%84%B1-%EB%B0%8F-%EB%B9%84%EA%B5%90-%EB%B6%84%EC%84%9D-CAP-PACELC

### 로그인
{"category": "": data:["id":"", "password":""]}

### DB
Redis(인메모리 NoSQL), MongoDB(문서 지향 NoSQL), PostgreSQL, graphQL, Cassandra(분산 NoSQL)
공부를 1차 적인 목적으로 채팅 서버에 사용하기 좋은 DBMS들을 생각해 뽑은 목록입니다.
이 중 **MongoDB**로 해볼까 합니다.
목록에 있는 DBMS들 중에서도 커뮤니티가 상당히 활발하고 자료가 많을 것이라 생각했고, 다른 것들보다
학습의 난이도 좀 낮아보였습니다.

