#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Point{
  int next;
  struct Point *P,*L,*R;
};

struct Point *squrt,*setsu;
void inorder(struct Point *st);
void preorder (struct Point *st);
void insert (int a);
struct Point *find (struct Point *st,int b);
struct Point *treeMinimum(struct Point *x);
struct Point *treeSuccessor(struct Point *x);
void Delete (struct Point *z);

int main()
{
  int i,n,p;
  char hi[10];

  scanf("%d",&n);

  for(i=0;i<n;i++){
    scanf("%s",hi);
    if( hi[0] == 'f'){
     scanf("%d",&p);
     struct Point *q =find(squrt,p);
     if(q!=setsu)
       printf("yes\n");
     else printf("no\n");
    }
    else  if(hi[0]=='i'){        /*insert*/
      scanf("%d",&p);
      insert(p);
    }else if(hi[0]=='p'){   /*print*/
      inorder(squrt);
      printf("\n");
      preorder(squrt);
      printf("\n");
    }
    else if(hi[0]=='d'){    /*print*/
      scanf("%d",&p);
      Delete(find(squrt,p));
    }
  }
  return 0;
}

void inorder(struct Point *st){
  if(st==setsu)
    return;
  inorder(st->L);
  printf(" %d",st->next);
  inorder(st->R);
  
}

void preorder(struct Point *st){
  if(st==setsu)
    return;
  printf(" %d",st->next);
  preorder(st->L);
  preorder(st->R);

}

void insert (int a){
  struct Point *x=squrt;
  struct Point *y=setsu;
  struct Point *z;

  z=(struct Point *)malloc(sizeof(struct Point));
  z->next=a;
  z->L=setsu;
  z->R=setsu;

  while(x!=setsu){
    y=x;
    if(x->next > z->next){
      x=x->L;
    }else
      x=x->R;
    
  }

  z->P=y;
  if(y==setsu){
    squrt=z;
  }else{
    if(y->next > z->next){
      y->L=z;
    }else
      y->R=z;
    
  }
}
    
    
struct Point *find (struct Point *st,int b){     /*keyの存在を見つける*/
  while(st !=setsu && b !=st->next){
    if(b < st->next)
      st=st->L;
    else st=st->R;
  }
  return st;
}

struct Point *treeMinimum(struct Point *x){

  while(x->L!=setsu)
    x=x->L;

  return x;
}


struct Point *treeSuccessor(struct Point *x){   
  if(x->R!=setsu)
    return treeMinimum (x->R);

  struct Point *y =x->P;
  
  while(y!=setsu && x==y->R){
    x=y;
    y=y->P;
  }

  return y;
}
    
void Delete (struct Point *z){
  struct Point *y;                   /*削除*/
  struct Point *x;                   /*子*/

  if(z->L == setsu || z->R == setsu )
    y=z;
  else 
    y=treeSuccessor(z);

  if(y->L!=setsu){
    x=y->L;
  }else{
    x=y->R;
  }

  if (x!=setsu){
    x->P=y->P;
  }

  if(y->P==setsu){
    squrt=x;
  }else{
    if(y==y->P->L){
      y->P->L=x;
    }else{
      y->P->R=x;
    }
  }
  if(y!=z){
    z->next = y->next;
  }

  free(y);

}
  

