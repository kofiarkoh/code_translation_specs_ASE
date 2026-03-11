#include <stdio.h>
#include <stdlib.h>

#define xMAX 1000000000
#define xMIN -1000000000

typedef struct _tree{
    int data;
    struct _tree *upper;
    struct _tree *lower;
}TREE;

TREE* setTree(TREE *p,int x){
    p->data=x;
    p->upper=NULL;
    p->lower=NULL;
    return p;
}
TREE* insert(TREE *tree,int x){
    TREE *p=tree;
    
    if(p==NULL){
        tree=(TREE*)malloc(sizeof(TREE));
        tree=setTree(tree,x);
    }
    else{
        while(1){
            
            if(x<p->data){
                if(p->lower!=NULL)p=p->lower;
                else{
                    p->lower=(TREE*)malloc(sizeof(TREE));
                    p=p->lower;
                    break;
                }
            }
            
            else if(x>=p->data){
                if(p->upper!=NULL)p=p->upper;
                else{
                    p->upper=(TREE*)malloc(sizeof(TREE));
                    p=p->upper;
                    break;
                }
            }
        }
    
        p=setTree(p,x);
    }

    return tree;
}
void getMax(TREE *tree){
    TREE *p=tree;
    if(p==NULL)return;
    
    while(p->upper!=NULL)p=p->upper;
    printf("%d\n",p->data);
    return;
}
TREE* deleteMax(TREE *tree){
    TREE *p=tree,*pp;
    if(p==NULL)return tree;
    
    while(p->upper!=NULL){
        pp=p;
        p=p->upper;
    }
    pp->upper=p->lower;
    return tree;
}

void printTree(TREE *tree){
    if(tree==NULL)return;
    
    if(tree->lower!=NULL)printTree(tree->lower);
    printf("%d ",tree->data);
    if(tree->upper!=NULL)printTree(tree->upper);
    
    return;
}

int main(void){
    int i;
    
    int n,q;
    scanf("%d %d\n",&n,&q);
    
    TREE *tree[n];
    for(i=0;i<n;i++){
        tree[i]=NULL;
    }
    
    int com,t,x;
    while(q--){
        scanf("%d %d \n",&com,&t);
        switch(com){
            case 0:
                //insert
                scanf("%d\n",&x);
                tree[t]=insert(tree[t],x);
                break;
            case 1:
                //getMax
                getMax(tree[t]);
                break;
            case 2:
                //deleteMax
                tree[t]=deleteMax(tree[t]);
                break;
                
        }
    }
    
    return 0;
}

