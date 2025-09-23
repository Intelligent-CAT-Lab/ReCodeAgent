#include <stdio.h>
#include <stdlib.h>

// typedef char data;
typedef struct _Node {
    char key;
    struct _Node* left;
    struct _Node* right;
} Node;

Node* search(Node* root, char x) {
    Node* p = root;
    while (p != NULL) {
        if (p -> key == x)
            return p;
        else if (p -> key < x)
            p = p -> right;
        else
            p = p -> left;
    }
    return NULL;
}
Node* insert(Node* root, char x){
    Node* p = root;
    Node* parent = NULL;
    while (p != NULL) {
        parent = p;
        if (p -> key == x){
            printf("같은 키가 있습니다. \n");
            return p;
        }
        else if (p -> key < x)
            p = p -> right;
        else
            p = p -> left;
    }
    // 새 노드 할당
    Node* newNode = (Node*)malloc(sizeof(Node));
    newNode -> key = x;
    newNode -> left = newNode -> right = NULL;
    // parent 의 자식으로 새 노드 붙이기
    if (parent != NULL) { // 맨처음 삽입하는 상황일 수도 있어서 (루트노드 삽입시)
        if (parent -> key < newNode -> key){
            parent -> right = newNode;
        }
        else {
            parent -> left = newNode;
        }
    }
    return newNode;
}
Node* delete(Node* root, char x){
    Node* p = root;
    Node* parent = NULL;
    while ((p != NULL) && (p -> key != x)) {
        parent = p;
        if (p -> key < x){
            p = p -> right;
        }
        else
            p = p -> left;
    }
    if (p == NULL) {
        printf("찾는 노드가 없습니다. \n");
        return root;
    }

    if (p -> left == NULL && p -> right == NULL) { // 차수가 0
        if (parent == NULL) // 차수가 0(자식 x), 부모노드x (루트노드 하나만 있을 때)
            root = NULL;
        else {
            if (parent -> left == p) // 자식이 없으면서, 자신이 왼쪽 노드일 때.
                parent -> left = NULL;
            else // 자식이 없으면서, 자신이 오른쪽 노드일 때.
                parent -> right = NULL;
        }
    }
    else if (p -> left == NULL || p -> right == NULL){ // 차수가 1
        Node *child = (p -> left != NULL) ? p -> left : p -> right; // 차수가 1일 때, 자식의 노드를 child 변수가 참조함.
        if (parent == NULL){
            root = child; // 자식이 1명인데 부모가 없으므로, 루트 노드를 삭제하는 중이었다는 뜻이니까 자식을 루트로 둔다.
        }
        else{
            if (parent -> left == p){ // 자신이 부모 노드의 왼쪽 자식 이었으면, 그 자리에 본인이 드감.
                parent -> left = child;
            }else{ // 오른쪽 자식이었으면 부모노드의 오른쪽 자식에 (지금 내자리) 외동딸로 등록.
                parent -> right = child;
            }
        }
    }
    else{ // 차수가 2
        Node * succ_parent = p;  // 지울 놈.
        Node* succ = p -> right;  // 지울 놈의 오른쪽 자식의 왼쪽 노드를 쭉 타고 들어가서(Null 만날때까지) 그놈을 지울 놈자리에 앉혀야 한다.
        while (succ -> left != NULL) {
            succ_parent = succ;
            succ = succ -> left;
        }
        p -> key = succ -> key;
        if (succ_parent -> left == succ){
            succ_parent -> left = succ -> right;
        }else{
            succ_parent -> right = succ -> right;
        }
        p = succ; // free(p)를 통해 놓아주기 위해 설정.
    }

    free(p);
    return 0;
}
void inorder(Node* root){
    if (root == NULL){
        return;
    }
    inorder(root -> left);
    printf("%c", root -> key);
    inorder(root -> right);
}
int main(){
    Node *root = insert(NULL, 'D');
    insert(root, 'B');
    insert(root, 'A');
    insert(root, 'E');
    insert(root, 'C');
    insert(root, 'F');
    inorder(root); printf("\n");

    delete(root, 'A');
    inorder(root); printf("\n");
    return 0;
}