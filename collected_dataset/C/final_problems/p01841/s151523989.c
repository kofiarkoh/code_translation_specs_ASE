#include <stdio.h>
#include <stdlib.h>

struct node{
    int value;
    struct node *left,*right;
};

typedef struct node node;

node *build(){
    if(getchar()==')')return NULL; //skip (
    node *res = (node *)malloc(sizeof(node));
    if(res->left = build())getchar(); //skip )
    scanf("[%d](",&res->value);
    if(res->right = build())getchar(); //skip )
    return res;
}

node *unite(node *a, node *b){
    if(a==NULL || b==NULL)return NULL;
    node *res = (node *)malloc(sizeof(node));
    res->value = a->value+b->value;
    res->left = unite(a->left, b->left);
    res->right = unite(a->right, b->right);
    return res;
}

void output(node *n){
    if(n==NULL)return;
    printf("(");
    output(n->left);
    printf(")[%d](", n->value);
    output(n->right);
    printf(")");
}

int main(){
    node *a = build();
    getchar(); //skip \n
    node *b = build();
    getchar(); //skip \n
    output(unite(a,b));
    printf("\n");
    return 0;
}