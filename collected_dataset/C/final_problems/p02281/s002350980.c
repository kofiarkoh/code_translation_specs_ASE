#include<stdio.h>

#define N 100000

typedef struct
{
    int parent;
    int left;
    int right;
} node;

int finddepth(int);
void preorder(int);
void inorder(int);
void postorder(int);

node T[N];

int main()
{
    int pa, x, y, i, j, n, k, c, root;
    
    scanf("%d", &n);
    for(i=0; i<n; i++)
    {
        T[i].parent = -1;
        T[i].left = -1;
        T[i].right = -1;
    }
    
    
    for(i=0; i<n; i++)
    {
        scanf("%d", &pa);
        
        scanf("%d%d", &x, &y);
        if(x!=-1)T[x].parent = pa;
        if(y!=-1)T[y].parent = pa;
        T[pa].left = x;
        T[pa].right = y;
        
    }
    
    for(i=0; i<n; i++)
    {
        if(T[i].parent == -1)
        {
            root = i;
            break;
        }
    }
    
    printf("Preorder\n");
    preorder(root);
    printf("\n");
    
    printf("Inorder\n");
    inorder(root);
    printf("\n");
    
    printf("Postorder\n");
    postorder(root);
    printf("\n");
    
    return 0;
}
int finddepth(int x)
{
    int c=0;
    while(T[x].parent >= 0)
    {
        c++;
        x = T[x].parent;
    }
    return c;
}
void preorder(int root)
{
    if(root == -1) return;
    
    printf(" %d", root);
    preorder(T[root].left);
    preorder(T[root].right);
}
void inorder(int root)
{
    if(root == -1) return;
    
    inorder(T[root].left);
    printf(" %d", root);
    inorder(T[root].right);
}
void postorder(int root)
{
    if(root == -1) return;
    
    postorder(T[root].left);
    postorder(T[root].right);
    printf(" %d", root);
}