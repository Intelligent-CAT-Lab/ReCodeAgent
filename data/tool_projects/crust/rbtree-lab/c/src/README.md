## RED-BLACK TREE :red_circle::black_circle:

- 자가 균형 이진 탐색 트리로서 연관 배열 등을 구현하는데 쓰이는 자료 구조이다.
- 높이가 h 인 이진 탐색 트리에서 _삽입_, _검색_, _삭제_ 등과 같은 동작이 O(h) 의 시간에 수행될 수 있는데, 최악의 경우 (이진 탐색 트리의 높이가 클 경우) 실행 속도가 일반적인 연결 리스트와 비슷한 정도에 블가하다.
- 레드 블랙 트리는 트리가 균형을 이루도록 설계된 검색 트리 구조 중 하나이다. 자료의 _삽입_ 과 _삭제_, _검색_ 의 동작이 최악의 경우에도 레드 블랙 트리의 높이인 O(logn) 시간에 수행되도록 보장한다.

![Red-Black TREE](https://user-images.githubusercontent.com/72483138/144753423-b57c8286-3d7d-417d-8c8f-7f9990db21a5.png)

### 특징
- 각각의 노드가 _레드_ 나 _블랙_ 인 색상 속성을 가지고 있는 이진 탐색 트리이다. 따라서 이진 탐색 트리의 일반적인 조건을 만족한다.
- 트리의 각 노드는 `color`, `key`, `left`, `right` 등의 필드를 갖는다.
- 레드-블랙 트리 코드에서 한계 조건을 다루기 편리하도록 `NIL`을 표현하기 위해 하나의 `경계노드(SENTINEL)`을 사용한다.
- `NIL노드`의 색은 검정색이다. 
- 한 노드의 자식 또는 부모가 존재하지 않으면 그에 대응되는 노드의 포인터 필드는 `NIL` 값으로 채워진다.
- 한 `노드 x`에서 리프노드까지의 경로에 있는 모든 검정 노드(x 자신은 제외)를 그 노드의 `흑색 높이(black-height)`라 하고 `bh(x)` 라고 한다. 

- 아래 **조건**을 만족해야 유효한 **레드-블랙 트리**가 된다.
1. 노드의 색은 _레드_ 혹은 _블랙_ 중의 하나이다.
2. 루트 노드 색은 _블랙_ 이다
3. 모든 리프 노드들(NIL) 은 _블랙_ 이다.
4. 레드 노드의 자식노드 양쪽은 언제나 모두 _블랙_ 이다. (_레드_ 노드는 연달아 나타날 수 없으며, _블랙_ 노드만이 _레드_ 노드의 부모 노드가 될 수 있다)
5. 어떤 노드로부터 시작되어 그에 속한 하위 리프 노드에 도달하는 모든 경로에는 리프 노드를 제외하면 모두 같은 개수의 블랙 노드가 있다.

---
## 구현 범위
다음 기능들을 수행할 수 있도록 RB Tree 구현

#### 생성
- [x] tree = `new_tree()`: RB Tree 구조체 생성

#### 회전
- [x] `left_rotation(tree, node)`: 해당 node를 기준으로 RB Tree 왼쪽 회, 기준 node가 없다면 반환
- [x] `right_rotation(tree, node)`: 해당 node를 기준으로 RB Tree 오른쪽 회전, 기준 node가 없다면 반환
#### 노드 삽입
- [x] tree = `rbtree_insert(tree, key)`: RB Tree에 키 값이 key인 새로운 노드 삽입 후 루트 반환
- [x] `rbtree_insert_fixup(tree, node)` : 노드 삽입 후 RB Tree 특성 만족시키기 위해 노드의 색 변경 및 회전 수행

#### 노드 삭제
- [x] `rbtree_erase(tree, node)` : 노드의 키를 이용하여 RB Tree에서 노드 삭제
- [x] `rbtree_delete_fixup(tree, node)` : 삭제 수행 후 RB Tree 특성을 만족시키기 위해 노드의 색 변경 및 회전 수행
#### 유틸
- [x] ptr = `tree_find(tree, key)`
    - RB tree내에 해당 key가 있는지 탐색하여 있으면 해당 node pointer 반환
    - 없으면 NIL 반환
- [x] `delete_rbtree(tree)` : RB Tree에 할당되었던 메모리 반환 및 트리 삭제
- [x] `free_node(tree, node)` : RB Tree 내부에 할당되었던 노드의 메모리 반환 및 노드 삭제
- [x] node = `rbtree_min(tree)` : RB Tree 의 노드중 키 값이 가장 작은 노드 반환
- [x] node = `rbtree_max(tree)` : RB Tree 의 노드중 키 값이 가장 큰 노드 반환

---
### 테스트 케이스
- [x] `test_init()`
- [x] `test_rotate()`
- [x] `test_insert()` 
- [x] `test_insert_single(1024)` 
- [x] `test_find_single(512, 1024)` 
- [x] `test_erase_root(128)` 
- [x] `test_minmax_suite()` 
- [x] `test_to_array_suite()` 
- [x] `test_distinct_values()` 
- [x] `test_duplicate_values()` 
- [x] `test_multi_instance()` 

--- 
#### 회전 개요
- `RB-TREE-INSERT`와 `RB-TREE-DELETE`의 연산에서 트리를 수정하기 때문에 레드블랙 트리의 특성을 위반할 수 있다. 
- 레드 블랙의 특성을 복구하기 위해 일부 노드의 색과 포인터를 변경해줘야 하는데, 포인터를 변경할 때 회전이 사용된다.
- 회전에는 좌회전과 우회전 두 종류가 있다. 좌회전과 우회전은 서로 대칭이다.

<center><img src="https://user-images.githubusercontent.com/72483138/145193420-7a0f9dd0-e3a7-4dad-b707-67bce63727a4.png"></center>

##### 좌회전 의사 코드
```c
LEFT-ROTATE(T, x)
1 y = x.right 
2 x.right = y.left 
3 if y.left != T.nil
4   y.left.p = x
5 y.p = x.p 
6 if x.p == T.nil
7 T.root = y
8 elseif x == x.p.left
9   x.p.left = y
10 else x.p.right = y
11 y.left = x
12 x.p = y
```

#### 삽입 개요
- `RB-INSERT(T, z)` 를 호출하면 `노드 z`가 `RB트리 T` 에 삽입된다.
- `이진 탐색 트리`의 삽입처럼 `노드 z`가 들어갈 위치를 탐색하여 `트리 T`에 삽입한 뒤 `삽입된 노드 Z`를 빨간 노드로 칠한다.
- 삽입 된 새로운 빨간색 노드로 인해 `RB 트리`의 조건을 위반하였을 수도 있으므로 `RB-INSERT-FIXUP(T, z)`에서 회전과 색 변환을 통해 `RB트리`의 조건을 만족할 수 있도록 한다.

##### `RB-TREE-INSERT` 의사 코드
```c
RB-INSERT(T, z)
1 y = T.nil
2 x = T.root
3 while x != T.nil
4   y = x
5   if z.key < x.key
6     x = x.left
7   else x = x.right
8 z.p = y
9 if y == T.nil
10  T.root = z
11 elseif  z.key < y.key
12  y.left = z
13 else y.right = z
14 z.left = T.nil
15 z.right = T.nil
16 z.color = RED
17 RB-INSERT-FIXUP(T, z)
```

##### `BINARY-TREE-INSERT`와 `RB-TREE-INSERT` 의 차이
- `TREE-INSERT`에 나타나는 모든 `NIL`은 `T.nil`로 교체된다.
- 새로 삽입되는 `노드 z`의 `left`와 `right`는 `T.nil`로 지정된다.
- 새로 삽입되는 `노드 z`의 색깔을 빨간색으로 칠한다.
- 새로 삽입되는 `노드 z`의 색이 `빨간색`으로 되면서 레드블랙트리의 특성을 위반할 수 있으므로 `RB-INSERT-FIXUP(tree, node)`를 호출하여 특성을 만족할 수 있도록 한다.

##### `RB-INSERT-FIXUP(tree, node)` 의 동작
- 새로운 `노드 z`가 삽입되면서 발생 가능한 `레드블랙트리` 조건의 위반 사항
  - 루트 노드는 `검정색` 이어야한다.
    - `노드 z`가 루트일 때 위반
  - `빨간색` 노드는 `빨간색` 자식을 가질 수 없다.
    - `노드 z`의 부모가 `빨간색`일 때 위반 

##### `RB-INSERT-FIXUP` 의사 코드
```c
RB-INSERT-FIXUP(T, z)
1 while z.p.color == RED
2   if z.p == z.p.p.left
3     y = z.p.p.right
4     if y.color == RED
5       z.p.color = BLACK // case 1
6       y.color = BLACK // case 1
7       z.p.p.color = RED // case 1
8       z = z.p.p // case 1
9     else if z == z.p.right
10        z = z.p // case 2
11        LEFT-ROTATE(T, z)/ // case 2
12      z.p.color = BLACK // case 3
13      z.p.p.color = RED // case 3
14      RIGHT-ROTATE(T, z.p.p) // case 3
15   else (same for z.p as a z.p.p.right subtree) // case 4 ~ 6
16 T.root.color = BLACK
```

##### `RB-INSERT-FIXUP(tree, node)` 동작 및 위반 경우
- 새로운 `노드 z`가 삽입되면서 발생 가능한 `레드블랙트리` 조건의 위반 사항을 확인하고 각 경우에 맞는 노드의 색 변경 및 회전을 통해 `레드블랙트리` 조건을 만족할 수 있도록 한다.
  - 레드블랙트리에서 루트 노드는 `검정색` 이어야하므로 `노드 z`가 루트일 때 위반된다.
  - `빨간색` 노드는 `빨간색` 자식을 가질 수 없다.`노드 z`의 부모가 `빨간색`일 때 위반된다. 
    
- 경우 1 : `노드 z`의 `삼촌 y`가 적색인 경우
  - `z.p.p`는 흑색이고, `z.p` 와 `y`는 적색이므로, 
  `z.p.p`를 적색으로 칠하고 `z.p` 와 `y`를 흑색하여 특성 5를 만족시킬 수 있다.
  - 그 다음 `z.p.p`를 `z`로 설정하여 위의 색 변화로 `RBTree` 의 위반 사항이 생겼는지 while문을 통해 다시 확인한다.
- 경우 2 : `노드 z`의 `삼촌 y`가 흑색이며 `노드 z`가 오른쪽 자식인 경우
  - 경우 2의 경우  `LEFT-ROTATE`을 통해 경우 3으로 변경하여 위반 사항을 해결한다.
- 경우 3 : `노드 z`의 `삼촌 y`가 흑색이며 `노드 z`가 왼쪽 자식인 경우
  - `z.p`의 색을 검정으로 바꾸고, `z.p.p`의 색을 빨강으로 바꾼뒤 `RIGHT-ROTATION`을 통해 레드블랙 트리 위반 사항을 해결한다. 

![image](https://user-images.githubusercontent.com/72483138/145197093-b9be2f7c-0f49-488e-b738-ec60a3065dd6.png)

> 경우 1 ~ 3 은 z의 부모가 조부모의 왼쪽 서브 트리일 경우와, 오른쪽 서브 트리일 경우로 나뉠 수 있기 때문에 총 6가지의 경우가 생긴다.

### 삭제 개요
- 보통의 이진 탐색 트리와 비슷하게 삭제한다. 하지만 삽입과 같이 삭제 후 경우들을 보면서 레드블랙트리의 특성을 위반한 경우 색변환과 회전을 통해 포인터 변경을 통해 레드 블랙 트리의 특징을 만족시킨다.
- 기본적으로 RB Tree의 삭제를 코드로 구현할 때 
  - `삭제할 노드 z`와 `대체 노드 y`의 부모, 자식등의 연결 정보를 수정하는 방법과
  - `대체 노드 y`의 키 값을 `원래 삭제할 노드 z`에 복사 후 `대체 노드 y`를 대신 삭제 시키는 방법이 있는데
  전자의 방법으로 삭제를 구현했다.

##### `TRANSPLANT(T, u, v)` 의사 코드
- `TRANSPLANT(T, u, v)`는 `RB-DELETE(T, z)` 에서 `삭제할 노드 z`와 `z를 대체할 노드 y`(z보다 키 값이 큰 가장 작은 노드)의 연결 관계를 바꿔주는 역할을 한다.
```c
RB-TRANSPLANT(T, u, v)
1 if u.p == T.nil
2   T.root = v
3 elseif u == u.p.left
4   u.p.left = v
5 else u.p.right = v
6 v.p = u:p
```

##### `RB-DELETE(T, z)` 의사 코드

```c
RB-DELETE(T, z)
1 y = z
2 y-original-color = y.color
3 if z.left == T.nil
4     x = z.right
5     RB-TRANSPLANT(T, z, z.right)
6 elseif z.right == T.nil
7     x = z.left
8     RB-TRANSPLANT(T, z, z.left)
9 else y = TREE-MINIMUM(z.right)
10    y-original-color = y.color
11    x = y.right
12    if y.p == z
13        x.p = y
14    else RB-TRANSPLANT(T, y, y.right)
15        y.right = z.right
16        y.right.p = y
17    RB-TRANSPLANT(T, z, y)
18    y.left = z.left
19    y.left.p = y
20    y.color = z.color
21 if y-original-color == BLACK
22    RB-DELETE-FIXUP(T, x)
```

##### `RB-DELETE-FIXUP(T, x)` 동작 및 위반 경우
- `RB-DELETE(T, z)`에서 새로운 `노드 z`가 삭제되면서 `대체 노드 y`가 대체되었다. 따라서 `y->right인 x`에 대해서 부모로 올라가면서 발생 가능한 `레드블랙트리` 조건의 위반 사항을 확인하고 각 경우에 맞는 노드의 색 변경 및 회전을 통해 `레드블랙트리` 조건을 만족할 수 있도록 한다.
- 대체 노드 y의 자식 노드인 x를 기준으로 부모노드와 삼촌 노드의 색을 확인하며 각 경우에 맞는 색 변환 및 회전한다.

- 경우 1: x의 형제 w가 적색인 경우
- 경우 2: x의 형제 w는 흑색이고 w의 두 자식이 모두 흑색인 경우
- 경우 3: x의 형제 w는 흑색, w의 왼쪽 자식은 적색, w의 오른쪽 자식은 흑색인 경우
- 경우 4: x의 형제 w는 흑색이고 w의 오른쪽 자식은 적색인 경우

> 경우 1 ~ 4는 `노드 x`가 부모의 왼쪽 자식일 때의 경우이므로 오른쪽 자식인 경우에 경우 1 ~ 5가 대칭적으로 발생가능 하다.
##### `RB-DELETE-FIXUP(T, x)` 의사 코드

```c
1 while x != T.root and x.color == BLACK
2     if x == x.p.left
3         w = x.p.right
4         if w.color == RED
5             w.color = BLACK // case 1
6             x.p.color . RED // case 1
7             LEFT-ROTATE(T, x.p) // case 1
8             w = x.p.right // case 1
9         if w.left.color == BLACK and w.right.color == BLACK
10            w .color = RED // case 2
11            x = x.p // case 2
12        else if w.right.color == BLACK
13                w.left.color = BLACK // case 3
14                w.color = RED // case 3
15                RIGHT-ROTATE(T, w) // case 3
16                w = x.p.right // case 3
17            w.color = x.p.color // case 4
18            x.p.color = BLACK // case 4
19            w.right.color = BLACK // case 4
20            LEFT-ROTATE(T, x.p) // case 4
21            x = T.root // case 4
22    else (same thing for the opossite direction “right” and “left” exchanged) // case 5 ~ 8
23 x.color = BLACK
```

--- 
## 참고문헌
- [위키백과: 레드-블랙 트리](https://ko.wikipedia.org/wiki/%EB%A0%88%EB%93%9C-%EB%B8%94%EB%9E%99_%ED%8A%B8%EB%A6%AC) ([영어](https://en.wikipedia.org/wiki/Red%E2%80%93black_tree))
- CLRS book (Introduction to Algorithms) 13장 레드 블랙 트리
- [Wikipedia:균형 이진 트리의 구현 방법들](https://en.wikipedia.org/wiki/Self-balancing_binary_search_tree#Implementations)

---
### ETC

#### `malloc`
- 인자로 전달된 크기 만큼 메모리를 할당한 후에, 그 메모리의 시작 주소값을 반환한다.
- 할당된 메모리는 초기화 되지 않았을 수도 있다. (기존의 있었단 내용이 남아 있을 수도 있다)
- 만일 `size`가 0 이라면, `malloc` 이 무엇을 리턴할지는 구현한 라이브러리 마다 다르다. 어떤 경우 널 포인터를 리턴할 수도 있고, 어떤 경우, 널이 아닌 포인터를 리턴하지만, 사용 불가능한 부분을 가리키고 있을 수도 있다. 어떤 경우든, 해당 주소값을 사용하면 안된다.
- 할당된 메모리는 반드시 `free` 로 해제해줘야 한다.
- **인자**
  - `size` : 메모리 블록의 크기(바이트 단위)
- **반환 값**
  - 메모리 할당에 성공하였을 경우, 할당한 메모리 블록을 가리키는 포인터를 리턴한다. 해당 포인터의 타입은 언제나 `void*` 이므로 사용자가 원하는 타입으로 캐스팅 해줘야 한다.
  - `malloc` 함수가 메모리 할당에 실패하였을 경우, `NULL` 포인터를 리턴한다.

#### `calloc`과 `malloc`의 차이
- `calloc`은 동적으로 메모리를 할당하고 할당된 메모리 블록을 `0`으로 초기화한다. 
- `malloc`은 동적으로 메모리를 할당하지만 할당된 메모리를 `0`으로 초기화하지않는다.
