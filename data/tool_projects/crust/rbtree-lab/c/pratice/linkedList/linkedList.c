#include <stdio.h>
#include <stdlib.h>

struct NODE { // 연결 리스트의 노드 구조체
    struct NODE *next;  // 다음 노드의 주소를 저장할 포인터
    int data;           // 데이터를 저장할 멤버 변수
};

/* 
연결 리스트에서 노드를 추가하는 규칙
1. 노드에 메모리 할당
2. next 멤버에 다음 노드의 메모리 주소 저장
3. data 멤버에 데이터 저장
4. 마지막 노드라면 next 멤버의 NULL 저장
*/
int main(void)
{
    // 연결 리스트의 시작노드를 가리킬 head 노드 생성
    struct NODE *head = malloc(sizeof(struct NODE));  

    // 첫 번째 노드 생성 및 head->next가 첫번째 노드를 가리키게 함
    struct NODE *node1 = malloc(sizeof(struct NODE));
    head->next = node1;

    // 첫 번째 노드에 데이터 저장
    node1->data = 10;

    // 두 번째 노드 생성 및 첫번째 노드가 두번째 노드를 가리키게 함
    struct NODE *node2 = malloc(sizeof(struct NODE));
    node1->next = node2;

    // 두 번째 노드에 데이터 저장
    node2->data = 20;

    // 두 번째 노드가 마지막 노드이므로 NULL을 가리키게 함
    node2->next=NULL;

    // 리스트 순회용 포인터(curr)에 첫 번째 노드의 주소 가리키게 함 
    struct NODE *curr = head->next;

    // 포인터가 NULL이 아닐때까지 반복
    while (curr != NULL)
    {
        // 현재 포인터가 가리키는 노드의 데이터 출력
        printf("%d\n", curr->data);
        // 현재 노드에서 다음 노드로 이동
        curr = curr->next;
    }

    free(node2); // 두번째 노드 메모리 해제
    free(node1); // 첫번째 노드 메모리 해제
    free(head);  // head 노드 메모리 해제

    return 0;
    
}