#include <stdio.h>
#include <stdlib.h>

int get_uint() {
    int n = 0;
    int c = getchar_unlocked();
    if(c < 48 || 57 < c) return c;
    while(47 < c && c < 58) {
        n = 10 * n + (c & 0xf);
        c = getchar_unlocked();
    }
    return n;
}
int get_str(char *str) {
    int c;
    while((c = getchar_unlocked()) > 32) *str++ = (char)c;
    *str = 0;
    return c;
}
void put_uint(int n) {
    if(!n) {
        putchar_unlocked('0');
        return;
    }
    char buf[11];
    int i = 0;
    while(n) buf[i++] = (char)(n % 10 + '0'), n /= 10;
    while(i--)putchar_unlocked(buf[i]);
}
void put_str(char *str) { while(*str) putchar_unlocked(*str++); }

typedef struct Node {
    int key;
    int priority;
    struct Node *left, *right;
} Node;
typedef struct BST {
    Node *root;
    size_t size;
} BST;
void bst_init(BST *BST) {
    BST->root = NULL;
    BST->size = 0;
}
Node *bst_rotate_right(Node *root) {
    Node *tmp = root->left;
    root->left = tmp->right;
    tmp->right = root;
    return tmp;
}
Node *bst_rotate_left(Node *root) {
    Node *tmp = root->right;
    root->right = tmp->left;
    tmp->left = root;
    return tmp;
}
Node *bst__insert(Node *root, int key, int priority) {
    if(root == NULL) {
        Node *new_node = malloc(sizeof(Node));
        new_node->key = key;
        new_node->priority = priority;
        new_node->left = new_node->right = NULL;
        return new_node;
    }
    if(key == root->key) return root;
    if(key < root->key) {
        root->left = bst__insert(root->left, key, priority);
        if(root->priority < root->left->priority) root = bst_rotate_right(root);
    } else {
        root->right = bst__insert(root->right, key, priority);
        if(root->priority < root->right->priority) root = bst_rotate_left(root);
    }
    return root;
}
void bst_insert(BST *BST, int key, int priority) {
    BST->root = bst__insert(BST->root, key, priority);
    BST->size++;
}
Node *bst_find(const Node *root, const int key) {
    while(root != NULL && root->key != key) {
        if(key < root->key) root = root->left;
        else root = root->right;
    }
    return root;
}
Node *bst_get_min(Node *root) {
    while(root->left != NULL) root = root->left;
    return root;
}
Node *bst__delete(Node *root, int key) {
    if(root == NULL) return NULL;
    if(key < root->key) root->left = bst__delete(root->left, key);
    else if(key > root->key) root->right = bst__delete(root->right, key);
    else if(root->left && root->right) {
        if(root->left->priority > root->right->priority) root = bst_rotate_right(root);
        else root = bst_rotate_left(root);
        root = bst__delete(root, key);
    } else {
        Node *tmp = root;
        if(root->left) {
            root = bst_rotate_right(root);
            root->right = bst__delete(root->right, key);
        }
        else if(root->right) {
            root = bst_rotate_left(root);
            root->left = bst__delete(root->left, key);
        }
        else {
            root = NULL;
            free(tmp);
        }
    }
    return root;
}
void bst_delete(BST *BST, int key) {
    BST->root = bst__delete(BST->root, key);
    BST->size--;
}
void bst_inorder(Node *root) {
    if(root == NULL) return;
    bst_inorder(root->left);
    putchar_unlocked(' '), put_uint(root->key);
    bst_inorder(root->right);
}
void bst_preorder(Node *root) {
    if(root == NULL) return;
    putchar_unlocked(' '), put_uint(root->key);
    bst_preorder(root->left);
    bst_preorder(root->right);
}
int main(int argc, char **argv) {
    BST BST;
    bst_init(&BST);
    int m = get_uint();
    int key, priority;
    char c[8];
    for(int i = 0; i < m; ++i) {
        get_str(c);
        switch(*c) {
        case 'i':
            key = get_uint();
            priority = get_uint();
            bst_insert(&BST, key, priority);
            break;
        case 'f':
            key = get_uint();
            put_str(bst_find(BST.root, key) ? "yes\n" : "no\n");
            break;
        case 'd':
            key = get_uint();
            bst_delete(&BST, key);
            break;
        default:
            bst_inorder(BST.root);
            putchar_unlocked('\n');
            bst_preorder(BST.root);
            putchar_unlocked('\n');
            break;
        }
    }
    return 0;
}

