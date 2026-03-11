#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define DEBUG 0
#define true 1
#define false 0

//typedef struct Node Node;
struct Node;
typedef struct Node Node;

struct Node{
    int key;
    int priority;
    Node *root;
    Node *left;
    Node *right;
};


Node* rightRotate(Node* t);
Node* leftRotate(Node* t);
Node* mkNode(int key,int priority);
Node* insert(Node* t, int key, int priority);
Node* delete(Node* t, int key);
Node* _delete(Node* t, int key);
void print(Node* t);
void _print_in(Node* t);
void _print_pre(Node* t);
void search(Node* t, int key);
int _search(Node* t, int key);

int main (){
    //printf("hellocatcat\n");
    int m;
    Node* root = NULL;
    char cmd[32];
    char* ins="insert";
    char* del="delete";
    char* pri="print";
    char* fin="find";
    int a;
    int b;
    memset(cmd,0,strlen(cmd));
    scanf("%d",&m);
    while(m>0){
        scanf("%s",cmd);
        if(!strcmp(cmd,ins)){
            scanf("%d %d",&a,&b);
            if(DEBUG)printf("%s %d %d\n",cmd,a,b);
            root =insert(root,a,b);
        }else if(!strcmp(cmd,del)){
            scanf("%d",&a);
            if(DEBUG)printf("%s %d\n",cmd,a);
            root = delete(root,a);
        }else if(!strcmp(cmd,pri)){
            if(DEBUG)printf("%s\n",cmd);
            print(root);
        }else if(!strcmp(cmd,fin)){
            scanf("%d",&a);
            if(DEBUG)printf("%s %d\n",cmd,a);
            search(root,a);
        }
        m--;
    }

    return 0;
}

Node* rightRotate(Node* t){
    Node* s = t->left;
    t->left = s->right;
    s->right = t;
    return s; //the new root of subtree
}

Node* leftRotate(Node* t){
    Node* s = t->right;
    t->right = s->left;
    s->left = t;
    return s; //the new root of subtree
}

Node* mkNode(int key, int priority){
    Node *s = malloc(sizeof(Node));
    if(s){
        s->key = key;
        s->priority = priority;
        s->root = NULL;
        s->left = NULL;
        s->right = NULL;
    }
    return s;
}

Node* insert(Node* t, int key, int priority){
    if(t == NULL)
        return mkNode(key, priority);
    if(key == t->key)
        return t;
    
    if(key < t->key){
        t->left = insert(t->left, key, priority);
        if(t->priority < t->left->priority){
            t = rightRotate(t);
        }
    }else{
        t->right = insert(t->right, key, priority);
        if(t->priority < t->right->priority){
            t = leftRotate(t);
        }
    }

    return t;
}

Node* delete(Node* t, int key){
    if(t == NULL)
        return NULL;
    if(key< t->key){
        t->left = delete(t->left, key);
    }else{
        if( key > t->key){
            t->right= delete(t->right, key);
        }else{
            return _delete(t,key);
        }
    }
    return t;
}

Node* _delete (Node* t, int key){
    if(t->left == NULL && t->right == NULL)
        return NULL;
    else if (t-> left == NULL)
        t = leftRotate(t);
    else if (t->right == NULL)
        t = rightRotate(t);
    else{
        if(t->left->priority > t->right->priority)
            t = rightRotate(t);
        else
        {
            t = leftRotate(t);
        }
    }
    return delete(t,key);
}

void print(Node* t){
    _print_in(t);
    printf("\n");
    _print_pre(t);
    printf("\n");
}

void _print_in(Node* t){
    if(t->left == NULL && t->right == NULL){
        printf(" %d",t->key);
        return;
    }
    
    if(t->left != NULL){
        _print_in(t->left);
    }
    printf(" %d",t->key);
    if(t->right != NULL){
        _print_in(t->right);
    }

    return;
}

void _print_pre(Node* t){
    if(t->left == NULL && t->right == NULL){
        printf(" %d",t->key);
        return;
    }
    
    printf(" %d",t->key);
    if(t->left != NULL){
        _print_pre(t->left);
    }
    if(t->right != NULL){
        _print_pre(t->right);
    }
    return;
}

void search(Node* t, int key){
    if(t == NULL){
        printf("no\n");
        return;
    }else{
        if(_search(t,key)){
            printf("yes\n");
            return;
        }else{
            printf("no\n");
            return;
        }
    }
}

int _search(Node* t, int key){
    if(t == NULL){
        return false;
    }
    if(t->key == key){
        return true;
    }
    if(key < t->key){
        return _search(t->left,key);
    }else{
        return _search(t->right,key);
    }
}

