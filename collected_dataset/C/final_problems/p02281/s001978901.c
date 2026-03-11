#include<stdio.h>
#define MAX 10000
#define N -1
struct node{
  int a,b,c;
};
struct node T[MAX];
int n;
void preparse(int);
void inparse(int);
void postparse(int);

int main(){
  int i,v,l,r,root;
  scanf("%d",&n);
  for(i=0;i<n;i++){
    T[i].a = N;
  }
  for(i=0;i<n;i++){
    scanf("%d %d %d",&v,&l,&r);
    T[v].b=l;
    T[v].c=r;

    if(l != N){
      T[l].a=v;
    }
    if(r != N){
      T[r].a=v;
    }
  }
    for(i=0;i<n;i++){
      if(T[i].a == N){
        root = i;
      }
    }

    printf("Preorder\n");
    preparse(root);
    printf("\n");
    printf("Inorder\n");
    inparse(root);
    printf("\n");
    printf("Postorder\n");
    postparse(root);
    printf("\n");
    return 0;
}
void preparse(int y){
  if(y == N){
    return;
  }
  printf(" %d",y);
  preparse(T[y].b);
  preparse(T[y].c);
}
void inparse(int u){
  if(u == N){
    return;
  }
  inparse(T[u].b);
  printf(" %d",u);
  inparse(T[u].c);
}
void postparse(int u){
  if(u == N){
    return;
  }
  postparse(T[u].b);
  postparse(T[u].c);
  printf(" %d",u);
}

