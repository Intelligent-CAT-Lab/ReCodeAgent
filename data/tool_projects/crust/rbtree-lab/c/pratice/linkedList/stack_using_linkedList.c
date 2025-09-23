/*
ADT Stack (ADT, Abstract Data Type, 추상 자료형)
object :
    - 0 개 이상의 원소가 이루는 유한 순서 리스트
public : 
    - push(data) : data를 스택에 삽입
    - pop() : 스택이 empty가 아닐 떄 top의 data를 반환 후 제거
    - peek() : 스택이 empty가 아닐 때 top의 data를 반환 (제거 x)
    - isEmpty() : 스택이 공백 상태인지 검사
    - printStack() : 스택의 원소 차례로 출력
*/

#include <stdio.h>
#include <stdlib.h>

/* 1. 연결 리스트의 노드 정의 */
typedef struct _stack {
    char data;
    struct _stack *link;
} stack;

stack *top; // 스택의 맨 상위 노드의 주소를 가리키는 포인터 변수

/* 2. 스택이 비었는지 아닌지를 판단 함수 */
int isEmpty()
{
    if (top == NULL) 
    {
        printf("ERROR : Stack is empty. \n");
        return 1;
    }
    return 0;
}

/* 3. 스택에 노드 삽입 함수 */
void push(char data)
{
    stack *newNode = malloc(sizeof(stack));
    newNode->data = data;
    newNode->link = top;
    top = newNode;
}

/* 4. 스택에 최상단 노드 삭제 함수 */ 
char pop()
{
    if (!isEmpty())
    {
        stack *temp = top;
        
        char data = top->data;
        
        top = top->link;
        
        free(temp);
        return data;
    }
}

/* 5. 스택의 맨 위 노드 확인 함수 */
char peek()
{
    if (!isEmpty())
    {
        return top->data;
    }
}

/* 6. 스택 노드에 있는 데이터 출력 */
void printStack()
{
    if (!isEmpty())
    {
        stack *curr = top;
        while (curr != NULL)
        {
            printf("%c ", curr->data);
            curr = curr->link;
        }
        printf("\n");
    }
}


int main(void)
{
    /* 실행 예시 */
    char check;

    printStack();
    push('A');
    push('B');
    check = peek();
    printf("top : %c\n", check);
    push('C');
    printStack();
    pop();
    push('D');
    check = peek();
    printf("top : %c\n", check);
    push('E');
    push('F');
    printStack();
    pop();
    pop();
    printStack();
    pop();

    return 0;
}