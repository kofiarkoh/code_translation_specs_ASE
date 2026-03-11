#include<stdio.h>
#include<stdlib.h>
#define LARGE 100000


int V,E;
int color=0;
int C[LARGE];

typedef struct list{
	int id;
	struct list *next;
}list;


list * makelist(int x){
	list *new=malloc(sizeof(list));
	new->id=x;
	new->next=NULL;
	return new;
}

void list_insert(list* L[],int x,list *new){
	list **p=&L[x];
	
	while((*p)!=NULL){
		p=&((*p)->next);
	}
	(*p)=new;
	
	return;
}

int push(int stack[],int x,int top){
	stack[top++]=x;
	return top;
}

int pop(int stack[],int *top){
	(*top)--;
	return stack[*top];
}

void dfs(int C[],int x,list* L[]){
	int stack[LARGE];
	int top=0;
	int temp;
	list *p;
	
	color++;
	top=push(stack,x,top);
	while(top>0){
		temp=pop(stack,&top);
		if(C[temp]==0)C[temp]=color;
		p=L[temp];
		while(p!=NULL){
			if(C[p->id]==0){
				top=push(stack,p->id,top);
				C[p->id]=color;
			}
			p=p->next;
		}
	}
	
	return;
}

int main(){
	int i,j;
	int s,t,q;
	int x,y;
	list *L[LARGE];
	list *new;
	
	scanf("%d %d",&V,&E);
	
	for(i=0;i<V;i++){
		L[i]=NULL;
		C[i]=0;
	}
	
	for(i=0;i<E;i++){
		scanf("%d %d",&s,&t);
		new=makelist(t);
		list_insert(L,s,new);
		new=makelist(s);
		list_insert(L,t,new);
	}
	
	scanf("%d",&q);
	for(i=0;i<q;i++){
		scanf("%d %d",&x,&y);
		if(C[x]==0)dfs(C,x,L);
		if(C[y]==0)dfs(C,y,L);
		if(C[x]==C[y])printf("yes\n");
		else printf("no\n");
	}
		
	
	return 0;
	
}

